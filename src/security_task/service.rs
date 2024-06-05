use chrono::{Datelike, Local, NaiveDate, Timelike};
use rand::{thread_rng, Rng};
use serde_json::Value;
use sqlx::Acquire;
use tokio::time;
use tokio_retry::{strategy::ExponentialBackoff, Retry};
use tracing::{event, instrument, Level};

use super::{dao, model::SecurityTask};
use crate::{
    daily_task::model::DailyTaskInfo,
    response_data::{self, model::ResponseData},
    security_temp::{self, model::SecurityTemp},
};

#[instrument]
pub async fn insert_task_data(
    pool: sqlx::PgPool,
    task_info: &DailyTaskInfo,
) -> Result<(), Box<dyn std::error::Error>> {
    let open_date = task_info.open_date.clone().unwrap();

    let twse_list = select_temp_to_twse(pool.clone(), open_date.clone()).await?;
    event!(target: "security_api", Level::DEBUG, "{:?}", twse_list);
    let mut item_index = 1;

    for data in twse_list {
        let mut conn = pool.acquire().await?;
        let mut transaction = conn.begin().await?;
        match loop_data_temp_data(&mut transaction, data, task_info.clone(), item_index).await {
            Ok(_) => transaction.commit().await?,
            Err(e) => {
                transaction.rollback().await?;
                event!(target: "security_api", Level::ERROR, "security_task.insert_task_data: {}", &e);
                panic!("security_task.insert_task_data Error {}", &e);
            }
        }
        item_index = item_index + 2;
    }

    let tpex_list = select_temp_to_tpex(pool.clone(), open_date.clone()).await?;
    event!(target: "security_api", Level::DEBUG, "{:?}", tpex_list);
    let mut item_index = 2;

    for data in tpex_list {
        let mut conn = pool.acquire().await?;
        let mut transaction = conn.begin().await?;
        match loop_data_temp_data(&mut transaction, data, task_info.clone(), item_index).await {
            Ok(_) => transaction.commit().await?,
            Err(e) => {
                transaction.rollback().await?;
                event!(target: "security_api", Level::ERROR, "security_task.insert_task_data: {}", &e);
                panic!("security_task.insert_task_data Error {}", &e);
            }
        }
        item_index = item_index + 2;
    }

    Ok(())
}

async fn loop_data_temp_data(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    data: SecurityTemp,
    task_info: DailyTaskInfo,
    item_index: i32,
) -> Result<(), Box<dyn std::error::Error>> {
    if data.issue_date <= task_info.ce_date {
        let query_security_task = match data.market_type.clone().unwrap().as_str() {
            "上市" => SecurityTask {
                row_id: None,
                open_date: task_info.open_date.clone(),
                security_code: data.security_code.clone(),
                security_name: None,
                market_type: data.market_type.clone(),
                issue_date: data.issue_date.clone(),
                security_date: task_info.open_date.clone(),
                security_seed: None,
                exec_count: None,
                is_enabled: None,
                sort_no: None,
            },
            "上櫃" => SecurityTask {
                row_id: None,
                open_date: task_info.open_date.clone(),
                security_code: data.security_code.clone(),
                security_name: None,
                market_type: data.market_type.clone(),
                issue_date: data.issue_date.clone(),
                security_date: task_info.tw_date.clone(),
                security_seed: None,
                exec_count: None,
                is_enabled: None,
                sort_no: None,
            },
            "興櫃" => SecurityTask {
                row_id: None,
                open_date: task_info.open_date.clone(),
                security_code: data.security_code.clone(),
                security_name: None,
                market_type: data.market_type.clone(),
                issue_date: data.issue_date.clone(),
                security_date: task_info.tw_date.clone(),
                security_seed: None,
                exec_count: None,
                is_enabled: None,
                sort_no: None,
            },
            _ => SecurityTask::new(),
        };

        let task_list = dao::read_all(transaction, &query_security_task).await?;
        if task_list.0 <= 0 {
            let seed: i64 = thread_rng().gen_range(1..=9999999999999);
            let security_seed = format!("{:013}", seed);
            let sort_no = item_index;

            let security_task = match data.market_type.clone().unwrap().as_str() {
                "上市" => SecurityTask {
                    row_id: None,
                    open_date: task_info.open_date.clone(),
                    security_code: data.security_code.clone(),
                    security_name: data.security_name.clone(),
                    market_type: data.market_type.clone(),
                    issue_date: data.issue_date.clone(),
                    security_date: task_info.open_date.clone(),
                    security_seed: Some(security_seed),
                    exec_count: Some(0),
                    is_enabled: Some(1),
                    sort_no: Some(sort_no),
                },
                "上櫃" => SecurityTask {
                    row_id: None,
                    open_date: task_info.open_date.clone(),
                    security_code: data.security_code.clone(),
                    security_name: data.security_name.clone(),
                    market_type: data.market_type.clone(),
                    issue_date: data.issue_date.clone(),
                    security_date: task_info.tw_date.clone(),
                    security_seed: Some(security_seed),
                    exec_count: Some(0),
                    is_enabled: Some(1),
                    sort_no: Some(sort_no),
                },
                "興櫃" => SecurityTask {
                    row_id: None,
                    open_date: task_info.open_date.clone(),
                    security_code: data.security_code.clone(),
                    security_name: data.security_name.clone(),
                    market_type: data.market_type.clone(),
                    issue_date: data.issue_date.clone(),
                    security_date: task_info.tw_date.clone(),
                    security_seed: Some(security_seed),
                    exec_count: Some(0),
                    is_enabled: Some(1),
                    sort_no: Some(sort_no),
                },
                _ => SecurityTask::new(),
            };

            dao::create(transaction, security_task).await?;
        }
    }

    Ok(())
}

async fn select_temp_to_twse(
    pool: sqlx::PgPool,
    open_date: String,
) -> Result<Vec<SecurityTemp>, Box<dyn std::error::Error>> {
    let mut conn = pool.acquire().await?;
    let mut transaction = conn.begin().await?;

    match security_temp::dao::read_all_by_sql(
        &mut transaction,
        &format!(
            r#" SELECT row_id
                      , open_date
                      , international_code
                      , security_code
                      , security_name
                      , market_type
                      , security_type
                      , industry_type
                      , issue_date
                      , cfi_code
                      , remark
                      , created_date
                      , updated_date
                   FROM security_temp 
                  WHERE open_date >= '{}' 
                    AND market_type in ('上市')
                    AND security_type in ('ETF', 'ETN', '股票', '特別股')
                  ORDER BY security_code, issue_date, market_type, security_type
            "#,
            open_date
        ),
    )
    .await
    {
        Ok(rows) => Ok(rows.1),
        Err(e) => {
            event!(target: "security_api", Level::ERROR, "security_task.select_temp_to_twse: {}", &e);
            panic!("security_task.select_temp_to_twse Error {}", &e)
        }
    }
}

async fn select_temp_to_tpex(
    pool: sqlx::PgPool,
    open_date: String,
) -> Result<Vec<SecurityTemp>, Box<dyn std::error::Error>> {
    let mut conn = pool.acquire().await?;
    let mut transaction = conn.begin().await?;

    match security_temp::dao::read_all_by_sql(
        &mut transaction,
        &format!(
            r#" SELECT row_id
                     , open_date
                     , international_code
                     , security_code
                     , security_name
                     , market_type
                     , security_type
                     , industry_type
                     , issue_date
                     , cfi_code
                     , remark
                     , created_date
                     , updated_date
                  FROM security_temp 
                 WHERE open_date >= '{}' 
                   AND market_type in ('上櫃', '興櫃')
                   AND security_type in ('ETF', 'ETN', '股票', '特別股')
                 ORDER BY security_code, issue_date, market_type, security_type
            "#,
            open_date
        ),
    )
    .await
    {
        Ok(rows) => Ok(rows.1),
        Err(e) => {
            event!(target: "security_api", Level::ERROR, "security_task.select_temp_to_tpex: {}", &e);
            panic!("security_task.select_temp_to_tpex Error {}", &e)
        }
    }
}

#[instrument]
pub async fn get_all_task(
    pool: sqlx::PgPool,
    task_info: &DailyTaskInfo,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut conn = pool.acquire().await?;
    let mut transaction = conn.begin().await?;

    let query_security_task = SecurityTask {
        row_id: None,
        open_date: task_info.open_date.clone(),
        security_code: None,
        security_name: None,
        market_type: None,
        issue_date: None,
        security_date: None,
        security_seed: None,
        exec_count: None,
        is_enabled: Some(1),
        sort_no: None,
    };

    let task_datas = dao::read_all(&mut transaction, &query_security_task).await?;
    if task_datas.0 > 0 {
        for data in task_datas.1 {
            let mut conn = pool.acquire().await?;
            let mut transaction = conn.begin().await?;

            let security_code = data.security_code.clone().unwrap();
            let open_date = data.open_date.clone().unwrap();
            let od = NaiveDate::parse_from_str(&open_date, "%Y%m%d")?;
            let nod = od.and_hms_opt(15, 30, 0).unwrap();

            let nd = Local::now().date_naive();
            let ndt = Local::now().naive_local();

            // 今天且下午三點半
            if nd == od && nod < ndt {
                let start_time = Local::now().time();

                match loop_data_security_task(&mut transaction, data).await {
                    Ok(_) => transaction.commit().await?,
                    Err(e) => {
                        transaction.rollback().await?;
                        event!(target: "security_api", Level::ERROR, "security_task.insert_task_data: {}", &e);
                        panic!("security_task.insert_task_data Error {}", &e);
                    }
                }

                let end_time = Local::now().time();
                let seconds = 6 - (end_time - start_time).num_seconds();

                let sleep_num = if seconds > 1 {
                    thread_rng().gen_range(2..=seconds)
                } else {
                    4
                };
                time::sleep(time::Duration::from_secs(sleep_num.try_into().unwrap())).await;

            // 小於今天的日期
            } else if nd > od {
                let year_str = format!("{:04}", od.year());
                let month_str = format!("{:02}", od.month());
                let day_str = format!("{:02}", od.day());

                let res_data = response_data::dao::read_by_max_day(
                    &mut transaction,
                    &security_code,
                    &year_str,
                    &month_str,
                    &day_str,
                )
                .await?;
                if res_data.is_none() {
                    let start_time = Local::now().time();

                    match loop_data_security_task(&mut transaction, data).await {
                        Ok(_) => transaction.commit().await?,
                        Err(e) => {
                            transaction.rollback().await?;
                            event!(target: "security_api", Level::ERROR, "security_task.insert_task_data: {}", &e);
                            panic!("security_task.insert_task_data Error {}", &e);
                        }
                    }

                    let end_time = Local::now().time();
                    let seconds = 6 - (end_time - start_time).num_seconds();

                    let sleep_num = if seconds > 1 {
                        thread_rng().gen_range(2..=seconds)
                    } else {
                        4
                    };
                    time::sleep(time::Duration::from_secs(sleep_num.try_into().unwrap())).await;
                } else {
                    let mut security_task = data.clone();
                    security_task.exec_count = match security_task.exec_count {
                        Some(v) => Some(v + 1),
                        None => Some(0),
                    };

                    match dao::update(&mut transaction, security_task).await {
                        Ok(_) => transaction.commit().await?,
                        Err(e) => {
                            transaction.rollback().await?;
                            event!(target: "security_api", Level::ERROR, "security_task.insert_task_data: {}", &e);
                            panic!("security_task.insert_task_data Error {}", &e);
                        }
                    }
                }
            }
        }
    }

    Ok(())
}

async fn loop_data_security_task(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    security: SecurityTask,
) -> Result<(), Box<dyn std::error::Error>> {
    // 重試設定
    let retry_strategy = ExponentialBackoff::from_millis(2000)
        .max_delay(time::Duration::from_secs(10))
        .take(5);

    let open_date = security.open_date.clone().unwrap();
    let security_code = security.security_code.clone().unwrap();

    match security.market_type.clone().unwrap().as_str() {
        "上市" => {
            let data = Retry::spawn(retry_strategy.clone(), || async {
                        event!(target: "security_api", Level::INFO, "try 上市 {} {}", &security_code, &open_date);
                        response_data::service::get_twse_avg_json(&security).await
                    })
                    .await?;

            let json_value: Value = serde_json::from_str(&data)?;
            let data_status = match json_value.get("stat") {
                Some(t) => "OK" == t.as_str().unwrap_or(""),
                None => false,
            };

            add_res_data(transaction, &security, &data, data_status).await?;
        }
        "上櫃" => {
            let data = Retry::spawn(retry_strategy.clone(), || async {
                event!(target: "security_api", Level::INFO, "try 上櫃 {} {}", &security_code, &open_date);
                response_data::service::get_tpex1_json(&security).await
            })
            .await?;

            let json_value: Value = serde_json::from_str(&data)?;
            let data_status = match json_value.get("iTotalRecords") {
                Some(t) => 0 < t.as_i64().unwrap_or(0),
                None => false,
            };

            add_res_data(transaction, &security, &data, data_status).await?;
        }
        "興櫃" => {
            let data = Retry::spawn(retry_strategy.clone(), || async {
                    event!(target: "security_api", Level::INFO, "try 興櫃 {} {}", &security_code, &open_date);
                    response_data::service::get_tpex2_html(&security).await
                })
                .await?;

            let json_value: Value = serde_json::from_str(&data)?;
            let data_status = match json_value.get("iTotalRecords") {
                Some(t) => 0 < t.as_i64().unwrap_or(0),
                None => false,
            };

            add_res_data(transaction, &security, &data, data_status).await?;
        }
        _ => (),
    }

    Ok(())
}

async fn add_res_data(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    security: &SecurityTask,
    html: &String,
    data_status: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    if data_status {
        let security_code = security.security_code.clone().unwrap();
        let open_date = security.open_date.clone().unwrap();

        let open_month = NaiveDate::parse_from_str(&open_date, "%Y%m%d")?;
        let year_str = format!("{:04}", open_month.year());
        let month_str = format!("{:02}", open_month.month());

        let res_data = ResponseData {
            row_id: None,
            data_content: Some(html.to_string()),
            open_date: security.open_date.clone(),
            exec_code: security.security_code.clone(),
        };

        let cnt = response_data::dao::update_by_max_day(
            transaction,
            res_data.clone(),
            &security_code,
            &year_str,
            &month_str,
        )
        .await?;
        if cnt <= 0 {
            response_data::dao::create(transaction, res_data.clone()).await?;
        }

        let mut security_task = security.clone();
        security_task.is_enabled = Some(0);
        security_task.exec_count = match security_task.exec_count {
            Some(v) => Some(v + 1),
            None => Some(0),
        };

        dao::update(transaction, security_task).await?;
    } else {
        let mut security_task = security.clone();
        security_task.exec_count = match security_task.exec_count {
            Some(v) => Some(v + 1),
            None => Some(0),
        };

        dao::update(transaction, security_task).await?;
    }
    Ok(())
}
