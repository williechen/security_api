use chrono::{Datelike, Local, NaiveDate};
use rand::{thread_rng, Rng};
use serde_json::Value;
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
    event!(target: "security_api", Level::DEBUG, "{:?}", twse_list);
    loop_date_temp_data(pool, &twse_list, 1).await?;

    let tpex_list = select_temp_to_tpex(&mut transaction, Local::now().date_naive()).await?;
    event!(target: "security_api", Level::DEBUG, "{:?}", tpex_list);
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
        let mut transaction = pool.begin().await?;

        let mut rng = thread_rng();
        let seed: i64 = rng.gen_range(1000000000000..=9999999999999);

        let mut query_security_task = SecurityTask::new();
        query_security_task.security_code = match &data.security_code {
            Some(t) => Some(t.to_string()),
            None => None,
        };
        query_security_task.security_date = Some(security_date.clone());

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
                security_date: Some(twse_date),
                security_seed: Some(security_seed),
                is_enabled: Some(1),
                sort_no: Some(sort_no),
                retry_count: Some(0),
            };

            match dao::create(&mut transaction, security_task).await {
                Ok(_) => transaction.commit().await?,
                Err(e) => {
                    transaction.rollback().await?;
                    event!(target: "security_api", Level::ERROR, "{:?}", &e);
                }
            };
        }
    }
    item_index = item_index + 2;

    Ok(())
}

async fn select_temp_to_twse(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    date: NaiveDate,
) -> Result<Vec<SecurityTemp>, Box<dyn std::error::Error>> {
    match security_temp::dao::read_all_by_sql(
        transaction,
        &format!(
            r#" SELECT row_id
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
                    AND security_type in ('ETF', 'ETN', '股票', '特別股')
                  ORDER BY security_code, issue_date, market_type, security_type
            "#,
            date.format("%Y%m%d")
        ),
    )
    .await
    {
        Ok(rows) => Ok(rows.1),
        Err(e) => {
            event!(target: "security_api", Level::ERROR, "{:?}", &e);
            Ok(vec![])
        }
    }
}

async fn select_temp_to_tpex(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    date: NaiveDate,
) -> Result<Vec<SecurityTemp>, Box<dyn std::error::Error>> {
    match security_temp::dao::read_all_by_sql(
        transaction,
        &format!(
            r#" SELECT row_id
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
                    AND security_type in ('ETF', 'ETN', '股票', '特別股')
                    ORDER BY security_code, issue_date, market_type, security_type
            "#,
            date.format("%Y%m%d")
        ),
    )
    .await
    {
        Ok(rows) => Ok(rows.1),
        Err(e) => {
            event!(target: "security_api", Level::ERROR, "{:?}", &e);
            Ok(vec![])
        }
    }
}

pub async fn get_all_task(pool: &sqlx::PgPool) -> Result<(), Box<dyn std::error::Error>> {
    let retry_strategy = ExponentialBackoff::from_millis(100)
        .max_delay(time::Duration::from_secs(10))
        .map(jitter) // add jitter to delays
        .take(5);

    let mut transaction = pool.begin().await?;

    let mut last_market_type = Some(String::new());

    let now = Local::now().naive_local();

    for year in (1962..=now.year()).rev() {
        let task_datas = select_all_task(&mut transaction, &year).await?;
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
                        event!(target: "security_api", Level::INFO, "try 上市 {:?} {:?}", &security_code, &twse_date);
                        response_data::service::get_twse_avg_json(&security).await
                    })
                    .await?;

                    let json_value: Value = serde_json::from_str(&data)?;
                    let data_status = match json_value.get("stat") {
                        Some(t) => "OK" == t.as_str().unwrap_or(""),
                        None => false,
                    };

                    match add_res_data(&mut transaction_loop, &security, &data, data_status).await {
                        Ok(_) => transaction_loop.commit().await?,
                        Err(e) => {
                            transaction_loop.rollback().await?;
                            event!(target: "security_api", Level::ERROR, "{:?}", &e);
                        }
                    };
                } else if Some("上櫃".to_string()) == market_type {
                    let data = Retry::spawn(retry_strategy.clone(), || async {
                        event!(target: "security_api", Level::INFO, "try 上櫃 {:?} {:?}", &security_code, &twse_date);
                        response_data::service::get_tpex1_json(&security).await
                    })
                    .await?;

                    let json_value: Value = serde_json::from_str(&data)?;
                    let data_status = match json_value.get("iTotalRecords") {
                        Some(t) => 0 < t.as_i64().unwrap_or(0),
                        None => false,
                    };

                    match add_res_data(&mut transaction_loop, &security, &data, data_status).await {
                        Ok(_) => transaction_loop.commit().await?,
                        Err(e) => {
                            transaction_loop.rollback().await?;
                            event!(target: "security_api", Level::ERROR, "{:?}", &e);
                        }
                    };
                } else if Some("興櫃".to_string()) == market_type {
                    let data = Retry::spawn(retry_strategy.clone(), || async {
                        event!(target: "security_api", Level::INFO, "try 興櫃 {:?} {:?}", &security_code, &twse_date);
                        response_data::service::get_tpex2_html(&security).await
                    })
                    .await?;

                    let json_value: Value = serde_json::from_str(&data)?;
                    let data_status = match json_value.get("data_cnt") {
                        Some(t) => 0 < t.as_i64().unwrap_or(0),
                        None => false,
                    };

                    match add_res_data(&mut transaction_loop, &security, &data, data_status).await {
                        Ok(_) => transaction_loop.commit().await?,
                        Err(e) => {
                            transaction_loop.rollback().await?;
                            event!(target: "security_api", Level::ERROR, "{:?}", &e);
                        }
                    };
                }

                let mut rng = thread_rng();
                if last_market_type == market_type {
                    event!(target: "security_api", Level::DEBUG, "{:?}={:?}", last_market_type, market_type);
                    time::sleep(time::Duration::from_secs(rng.gen_range(4..8))).await;
                } else if last_market_type != market_type
                    && (Some("上櫃".to_string()) == last_market_type
                        || Some("興欏".to_string()) == last_market_type)
                    && (Some("上櫃".to_string()) == market_type
                        || Some("興欏".to_string()) == market_type)
                {
                    event!(target: "security_api", Level::DEBUG, "{:?}<>{:?}", last_market_type, market_type);
                    time::sleep(time::Duration::from_secs(rng.gen_range(4..8))).await;
                } else {
                    event!(target: "security_api", Level::DEBUG, "{:?}<>{:?}", last_market_type, market_type);
                    time::sleep(time::Duration::from_secs(rng.gen_range(3..6))).await;
                }
            }
            last_market_type = security.market_type;
        }
    }

    Ok(())
}

async fn select_all_task(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    year: &i32,
) -> Result<Vec<SecurityTask>, Box<dyn std::error::Error>> {
    match dao::read_all_by_sql(
        transaction,
        &format!(
            r#" SELECT row_id
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
                    AND twse_date >= '{}0101'
                    AND retry_count < 10
                  ORDER BY twse_date DESC, sort_no 
        "#,
            year
        ),
    )
    .await
    {
        Ok(rows) => Ok(rows.1),
        Err(e) => {
            event!(target: "security_api", Level::ERROR, "{:?}", &e);
            Ok(vec![])
        }
    }
}

async fn add_res_data(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    data: &SecurityTask,
    html: &String,
    data_status: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    if data_status {
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
    } else {
        let mut security_task = data.clone();
        security_task.retry_count = match security_task.retry_count {
            Some(v) => Some(v + 1),
            None => Some(0),
        };

        dao::update(transaction, security_task.to_owned()).await?;
    }
    Ok(())
}
