use chrono::{Datelike, Local, NaiveDate};
use rand::{thread_rng, Rng};
use tokio::time;
use tokio_retry::{
    strategy::{jitter, ExponentialBackoff},
    Retry,
};
use tracing::{event, Level};

use super::{dao, model::SecurityTask};
use crate::{
    response_data::{self, model::ResponseData},
    security_temp::{self, model::SecurityTemp},
};

pub async fn insert_task_data(pool: &sqlx::PgPool) -> Result<(), Box<dyn std::error::Error>> {
    let mut transaction = pool.begin().await?;

    let twse_list = select_temp_to_twse(&mut transaction, Local::now().date_naive()).await?;
    event!(target: "my_api", Level::DEBUG, "{:?}", twse_list);
    loop_date_temp_data(pool, &twse_list, 1).await?;

    let tpex_list = select_temp_to_tpex(&mut transaction, Local::now().date_naive()).await?;
    event!(target: "my_api", Level::DEBUG, "{:?}", tpex_list);
    loop_date_temp_data(pool, &tpex_list, 2).await?;

    Ok(())
}

async fn loop_date_temp_data(
    pool: &sqlx::PgPool,
    data_list: &Vec<SecurityTemp>,
    index: i32,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut item_index = index;

    for data in data_list {
        let now = Local::now().naive_local();
        let issue_date = match &data.issue_date {
            Some(t) => NaiveDate::parse_from_str(t, "%Y/%m/%d")?,
            None => Local::now().date_naive(),
        };

        for y in (issue_date.year()..=now.year()).rev() {
            let mut me = (1..13).rev();
            if issue_date.year() == y && issue_date.year() == now.year() {
                me = (issue_date.month()..now.month()).rev();
            } else if now.year() == y {
                me = (1..now.month()).rev();
            } else if issue_date.year() == y {
                me = (issue_date.month()..13).rev();
            }

            for m in me {
                let mut transaction = pool.begin().await?;

                let mut rng = thread_rng();
                let seed: i64 = rng.gen_range(1000000000000..=9999999999999);

                let twse_date = format!("{:04}{:02}01", y, m);
                let tpex_date = format!("{:03}/{:02}", y - 1911, m);

                let mut query_security_task = SecurityTask::new();
                query_security_task.security_code = match &data.security_code {
                    Some(t) => Some(t.to_string()),
                    None => None,
                };
                query_security_task.twse_date = Some(twse_date.clone());

                let task_list = dao::read_all(&mut transaction, &query_security_task).await?;
                if task_list.0 <= 0 {
                    let security_seed = seed.to_string();
                    let sort_no = item_index;

                    let security_task = SecurityTask {
                        row_id: None,
                        market_type: match &data.market_type {
                            Some(t) => Some(t.to_string()),
                            None => None,
                        },
                        security_code: match &data.security_code {
                            Some(t) => Some(t.to_string()),
                            None => None,
                        },
                        issue_date: match &data.issue_date {
                            Some(t) => Some(t.to_string()),
                            None => None,
                        },
                        twse_date: Some(twse_date),
                        tpex_date: Some(tpex_date),
                        security_seed: Some(security_seed),
                        is_enabled: Some(1),
                        sort_no: Some(sort_no),
                        retry_count: Some(0),
                        created_date: Some(now),
                        updated_date: Some(now),
                    };

                    match dao::create(&mut transaction, security_task).await {
                        Ok(_) => transaction.commit().await?,
                        Err(e) => {
                            transaction.rollback().await?;
                            event!(target: "my_api", Level::ERROR, "{:?}", &e);
                        }
                    };
                }
            }
        }
        item_index = item_index + 2;
    }

    Ok(())
}

async fn select_temp_to_twse(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    date: NaiveDate,
) -> Result<Vec<SecurityTemp>, Box<dyn std::error::Error>> {
    match security_temp::dao::read_all_by_sql(transaction,
        &format!(r#" SELECT row_id
                      , version_code 
                      , international_code
                      , security_code
                      , security_name
                      , market_type
                      , security_type
                      , industry_type
                      , issue_date
                      , cfi_code
                      , remark
                      , is_enabled
                      , created_date
                      , updated_date
                   FROM security_temp 
                  WHERE version_code='{}' 
                    AND market_type in ('上市')
                    AND security_type in ('ETF', 'ETN', '股票', '特別股', '轉換公司債', '交換公司債')
                  ORDER BY security_code, issue_date, market_type, security_type
            "#, date.format("%Y%m%d"))).await{
            Ok(rows) => Ok(rows.1),
            Err(e) => {
                event!(target: "my_api", Level::ERROR, "{:?}", &e);
                Ok(vec![])
            }
                    }
}

async fn select_temp_to_tpex(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    date: NaiveDate,
) -> Result<Vec<SecurityTemp>, Box<dyn std::error::Error>> {
    match security_temp::dao::read_all_by_sql(transaction,
        &format!(r#" SELECT row_id
        , version_code 
        , international_code
        , security_code
        , security_name
        , market_type
        , security_type
        , industry_type
        , issue_date
        , cfi_code
        , remark
        , is_enabled
        , created_date
        , updated_date
                   FROM security_temp 
                  WHERE version_code='{}' 
                    AND market_type in ('上櫃', '興櫃')
                    AND security_type in ('ETF', 'ETN', '股票', '特別股', '轉換公司債', '交換公司債')
                    ORDER BY security_code, issue_date, market_type, security_type
            "#,date.format("%Y%m%d"))).await{
                Ok(rows) => Ok(rows.1),
                Err(e) => {
                    event!(target: "my_api", Level::ERROR, "{:?}", &e);
                    Ok(vec![])
                }
                        }
}

pub async fn get_all_task(pool: &sqlx::PgPool) -> Result<(), Box<dyn std::error::Error>> {
    let retry_strategy = ExponentialBackoff::from_millis(100)
        .map(jitter) // add jitter to delays
        .take(3);

    let mut transaction = pool.begin().await?;

    let mut last_market_type = Some(String::new());

    let task_datas = select_all_task(&mut transaction).await?;
    for security in task_datas {
        let mut transaction_loop = pool.begin().await?;

        let market_type = security.market_type.clone();
        let security_code = security.security_code.clone();
        let twse_date = security.twse_date.clone();

        let mut query_response_data = ResponseData::new();
        query_response_data.read_date = twse_date.clone();
        query_response_data.data_code = security_code.clone();

        let res_list =
            response_data::dao::read_all(&mut transaction_loop, &query_response_data).await?;
        if res_list.0 <= 0 {
            if Some("上市".to_string()) == market_type {
                let data = Retry::spawn(retry_strategy.clone(), || async {
                    event!(target: "my_api", Level::DEBUG, "retry 上市");
                    response_data::service::get_twse_json(&security).await
                })
                .await?;

                match add_res_data(&mut transaction_loop, &security, &data).await {
                    Ok(_) => transaction_loop.commit().await?,
                    Err(e) => {
                        transaction_loop.rollback().await?;
                        event!(target: "my_api", Level::ERROR, "{:?}", &e);
                    }
                };
            } else if Some("上櫃".to_string()) == market_type {
                let data = Retry::spawn(retry_strategy.clone(), || async {
                    event!(target: "my_api", Level::DEBUG, "retry 上櫃");
                    response_data::service::get_tpex1_json(&security).await
                })
                .await?;

                match add_res_data(&mut transaction_loop, &security, &data).await {
                    Ok(_) => transaction_loop.commit().await?,
                    Err(e) => {
                        transaction_loop.rollback().await?;
                        event!(target: "my_api", Level::ERROR, "{:?}", &e);
                    }
                };
            } else if Some("興櫃".to_string()) == market_type {
                let data = Retry::spawn(retry_strategy.clone(), || async {
                    event!(target: "my_api", Level::DEBUG, "retry 興櫃");
                    response_data::service::get_tpex2_html(&security).await
                })
                .await?;

                match add_res_data(&mut transaction_loop, &security, &data).await {
                    Ok(_) => transaction_loop.commit().await?,
                    Err(e) => {
                        transaction_loop.rollback().await?;
                        event!(target: "my_api", Level::ERROR, "{:?}", &e);
                    }
                };
            }

            let mut rng = thread_rng();
            if last_market_type == market_type {
                event!(target: "my_api", Level::DEBUG, "{:?}={:?}", last_market_type, market_type);
                time::sleep(time::Duration::from_secs(rng.gen_range(4..8))).await;
            } else if last_market_type != market_type
                && (Some("上櫃".to_string()) == last_market_type
                    || Some("興欏".to_string()) == last_market_type)
                && (Some("上櫃".to_string()) == market_type
                    || Some("興欏".to_string()) == market_type)
            {
                event!(target: "my_api", Level::DEBUG, "{:?}<>{:?}", last_market_type, market_type);
                time::sleep(time::Duration::from_secs(rng.gen_range(4..8))).await;
            } else {
                event!(target: "my_api", Level::DEBUG, "{:?}<>{:?}", last_market_type, market_type);
                time::sleep(time::Duration::from_secs(rng.gen_range(3..6))).await;
            }
        }
        last_market_type = security.market_type;
    }

    Ok(())
}

async fn select_all_task(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
) -> Result<Vec<SecurityTask>, Box<dyn std::error::Error>> {
    match dao::read_all_by_sql(
        transaction,
        &r#" SELECT row_id
                      , market_type
                      , security_code
                      , issue_date
                      , twse_date
                      , tpex_date
                      , security_seed 
                      , is_enabled
                      , sort_no
                      , retry_count
                      , created_date
                      , updated_date
                   FROM security_task
                  WHERE is_enabled = 1
                  ORDER BY twse_date DESC, sort_no 
        "#,
    )
    .await
    {
        Ok(rows) => Ok(rows.1),
        Err(e) => {
            event!(target: "my_api", Level::ERROR, "{:?}", &e);
            Ok(vec![])
        }
    }
}

async fn add_res_data(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    data: &SecurityTask,
    html: &String,
) -> Result<(), Box<dyn std::error::Error>> {
    let response_data = ResponseData {
        row_id: None,
        data_content: Some(html.to_string()),
        data_code: data.security_code.clone(),
        read_date: data.twse_date.clone(),
        created_date: Some(Local::now().naive_local()),
        updated_date: Some(Local::now().naive_local()),
    };

    response_data::dao::create(transaction, response_data).await?;

    let mut security_task = data.clone();
    security_task.is_enabled = Some(0);
    security_task.retry_count = match security_task.retry_count {
        Some(v) => Some(v + 1),
        None => Some(0),
    };

    dao::update(transaction, security_task.to_owned()).await?;

    Ok(())
}
