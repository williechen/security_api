#![warn(clippy::all, clippy::pedantic)]

use std::cmp::max;

use chrono::{Datelike, Local, NaiveDate};
use rand::{thread_rng, Rng};
use serde_json::Value;
use sqlx::PgConnection;
use tokio::time;
use tokio_retry::{strategy::ExponentialBackoff, Retry};
use tracing::{event, Level};

use super::{dao, model::SecurityTask};
use crate::{
    daily_task::model::DailyTaskInfo,
    repository::Repository,
    response_data::{self, model::ResponseData},
    security_temp::{self, model::SecurityTemp},
};

pub async fn insert_task_data(
    db_url: &str,
    task_info: &DailyTaskInfo,
) -> Result<(), Box<dyn std::error::Error>> {
    event!(target: "security_api", Level::INFO, "call daily_task.temp_to_task");
    let pool = Repository::new(db_url).await;
    let mut transaction = pool.connection.acquire().await?;

    let open_date = task_info.open_date.clone().unwrap();
    let ce_date = task_info.ce_date.clone().unwrap();

    let twse_list =
        select_temp_to_twse(&mut *transaction, open_date.clone(), ce_date.clone()).await?;
    let tpex_list =
        select_temp_to_tpex(&mut *transaction, open_date.clone(), ce_date.clone()).await?;

    let max_count = max(twse_list.len(), tpex_list.len());

    let mut sort_num = 0;
    for i in 0..max_count {
        if i < twse_list.len() {
            sort_num = sort_num + 1;

            let twse_data = &twse_list[i];
            let mut transaction = pool.connection.acquire().await?;
            loop_data_temp_data(
                &mut *transaction,
                twse_data.clone(),
                task_info.clone(),
                sort_num,
            )
            .await?;
        }
        if i < tpex_list.len() {
            sort_num = sort_num + 1;

            let tpex_data = &tpex_list[i];
            let mut transaction = pool.connection.acquire().await?;
            loop_data_temp_data(
                &mut *transaction,
                tpex_data.clone(),
                task_info.clone(),
                sort_num,
            )
            .await?;
        }
    }

    Ok(())
}

async fn loop_data_temp_data(
    transaction: &mut PgConnection,
    data: SecurityTemp,
    task_info: DailyTaskInfo,
    item_index: i32,
) -> Result<(), Box<dyn std::error::Error>> {
    let market_type = data.market_type.clone().unwrap_or("".to_string());
    let ref_market_type = market_type.as_str();

    let query_security_task = match ref_market_type {
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

        let security_task = match ref_market_type {
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

    Ok(())
}

async fn select_temp_to_twse(
    transaction: &mut PgConnection,
    open_date: String,
    ce_date: String,
) -> Result<Vec<SecurityTemp>, Box<dyn std::error::Error>> {
    match security_temp::dao::read_all_by_sql(
        transaction,
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
                  WHERE open_date >= '{0}' 
                    AND issue_date <= '{1}'
                    AND market_type in ('上市')
                    AND security_type in ('ETF', 'ETN', '股票', '特別股')
                  ORDER BY security_code, issue_date, market_type, security_type
            "#,
            open_date, ce_date
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
    transaction: &mut PgConnection,
    open_date: String,
    ce_date: String,
) -> Result<Vec<SecurityTemp>, Box<dyn std::error::Error>> {
    match security_temp::dao::read_all_by_sql(
        transaction,
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
                 WHERE open_date >= '{0}' 
                   AND issue_date <= '{1}'
                   AND market_type in ('上櫃', '興櫃')
                   AND security_type in ('ETF', 'ETN', '股票', '特別股')
                 ORDER BY security_code, issue_date, market_type, security_type
            "#,
            open_date, ce_date
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

pub async fn get_all_task(
    db_url: &str,
    task_info: &DailyTaskInfo,
) -> Result<(), Box<dyn std::error::Error>> {
    event!(target: "security_api", Level::INFO, "call daily_task.task_run");

    let pool = Repository::new(db_url).await;
    let mut transaction = pool.connection.acquire().await?;

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

    let task_datas = dao::read_all(&mut *transaction, &query_security_task).await?;
    if task_datas.0 > 0 {
        for data in task_datas.1 {
            event!(target: "security_api", Level::DEBUG, "SecurityTask: {}", &data);
            let security_code = data.security_code.clone().unwrap();
            let open_date = data.open_date.clone().unwrap();
            let od = NaiveDate::parse_from_str(&open_date, "%Y%m%d")?;
            let nod = od.and_hms_opt(15, 30, 0).unwrap();

            let nd = Local::now().date_naive();
            let ndt = Local::now().naive_local();

            // 今天且下午三點半
            if nd == od && nod < ndt {
                let mut transaction = pool.connection.begin().await?;

                let start_time = Local::now().time();

                match loop_data_security_task(&mut *transaction, data).await {
                    Ok(_) => {
                        transaction.commit().await?;
                        let end_time = Local::now().time();
                        let seconds = 8 - (end_time - start_time).num_seconds();

                        let sleep_num = if seconds > 4 {
                            thread_rng().gen_range(4..=seconds)
                        } else {
                            4
                        };
                        time::sleep(time::Duration::from_secs(sleep_num.try_into().unwrap())).await;
                    }
                    Err(e) => {
                        transaction.rollback().await?;
                        event!(target: "security_api", Level::ERROR, "daily_task.get_all_task {}", &e);
                        continue;
                    }
                }

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
                    let mut transaction = pool.connection.begin().await?;

                    let start_time = Local::now().time();

                    match loop_data_security_task(&mut *transaction, data).await {
                        Ok(_) => {
                            transaction.commit().await?;
                            let end_time = Local::now().time();
                            let seconds = 8 - (end_time - start_time).num_seconds();

                            let sleep_num = if seconds > 4 {
                                thread_rng().gen_range(4..=seconds)
                            } else {
                                4
                            };
                            time::sleep(time::Duration::from_secs(sleep_num.try_into().unwrap()))
                                .await;
                        }
                        Err(e) => {
                            transaction.rollback().await?;
                            event!(target: "security_api", Level::ERROR, "daily_task.get_all_task {}", &e);
                            continue;
                        }
                    }
                } else {
                    let mut transaction = pool.connection.acquire().await?;
                    update_data(&mut *transaction, &data, true).await;
                }
            }
        }
    }
    Ok(())
}

async fn loop_data_security_task(
    transaction: &mut PgConnection,
    security: SecurityTask,
) -> Result<(), Box<dyn std::error::Error>> {
    // 重試設定
    let retry_strategy = ExponentialBackoff::from_millis(2000)
        .max_delay(time::Duration::from_secs(10))
        .take(5);

    let open_date = security.open_date.clone().unwrap();
    let security_code = security.security_code.clone().unwrap();

    let market_type = security.market_type.clone().unwrap_or("".to_string());
    let ref_market_type = market_type.as_str();

    match ref_market_type {
        "上市" => {
            let data = Retry::spawn(retry_strategy.clone(), || async {
                response_data::service::get_twse_avg_json(&security).await
            })
            .await?;

            let json_value: Value = serde_json::from_str(&data)?;
            match json_value.get("stat") {
                Some(t) => {
                    if "OK" == t.as_str().unwrap_or("") {
                        add_res_data(transaction, &security, &data).await;
                        update_data(transaction, &security, true).await;
                    } else {
                        update_data(transaction, &security, false).await;
                    }
                }
                None => update_data(transaction, &security, false).await,
            };
        }
        "上櫃" => {
            let data = Retry::spawn(retry_strategy.clone(), || async {
                response_data::service::get_tpex1_json(&security).await
            })
            .await?;

            let json_value: Value = serde_json::from_str(&data)?;
            match json_value.get("iTotalRecords") {
                Some(t) => {
                    if 0 < t.as_i64().unwrap_or(0) {
                        add_res_data(transaction, &security, &data).await;
                        update_data(transaction, &security, true).await;
                    } else {
                        update_data(transaction, &security, false).await;
                    }
                }
                None => update_data(transaction, &security, false).await,
            };
        }
        "興櫃" => {
            let data = Retry::spawn(retry_strategy.clone(), || async {
                    response_data::service::get_tpex2_html(&security).await
                })
                .await?;

            let json_value: Value = serde_json::from_str(&data)?;
            match json_value.get("iTotalRecords") {
                Some(t) => {
                    if 0 < t.as_i64().unwrap_or(0) {
                        add_res_data(transaction, &security, &data).await;
                        update_data(transaction, &security, true).await;
                    } else {
                        update_data(transaction, &security, false).await;
                    }
                }
                None => update_data(transaction, &security, false).await,
            };
        }
        _ => (),
    }

    Ok(())
}

async fn add_res_data(transaction: &mut PgConnection, security: &SecurityTask, html: &String) {
    let security_code = security.security_code.clone().unwrap();
    let open_date = security.open_date.clone().unwrap();

    let open_month = NaiveDate::parse_from_str(&open_date, "%Y%m%d").unwrap();
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
    .await
    .unwrap();
    if cnt <= 0 {
        response_data::dao::create(transaction, res_data.clone())
            .await
            .unwrap();
    }
}

async fn update_data(transaction: &mut PgConnection, data: &SecurityTask, is_action: bool) {
    let mut security_task = data.clone();
    security_task.exec_count = match security_task.exec_count {
        Some(v) => Some(v + 1),
        None => Some(0),
    };

    if is_action {
        security_task.is_enabled = Some(0);
    }

    dao::update(transaction, security_task).await.unwrap();
}
