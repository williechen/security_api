#![warn(clippy::all, clippy::pedantic)]

use std::{cmp::max, error::Error};

use chrono::{Local, NaiveDate};
use rand::{thread_rng, Rng};
use tokio::time::{self, sleep};
use tokio_retry::{strategy::ExponentialBackoff, Retry};
use tracing::{event, Level};

use super::{dao, model::SecurityTask};
use crate::{
    daily_task::model::DailyTask,
    response_data::{
        self,
        model::{ResponseData, SecurityPriceTpex1, SecurityPriceTpex2, SecurityPriceTwse},
    },
    security_temp::{self, model::SecurityTemp},
};

pub async fn insert_task_data(task: &DailyTask) -> Result<(), sqlx::Error> {
    event!(target: "security_api", Level::INFO, "call daily_task.temp_to_task");

    let twse_list = security_temp::dao::find_all_by_twse(task).await;
    let tpex_list = security_temp::dao::find_all_by_tpex(task).await;

    let max_count = max(twse_list.len(), tpex_list.len());

    let mut sort_num = 0;
    for i in 0..max_count {
        if i < twse_list.len() {
            sort_num = sort_num + 1;

            let twse_data = &twse_list[i];
            loop_data_temp_data(twse_data, &task, sort_num).await?;
        }
        if i < tpex_list.len() {
            sort_num = sort_num + 1;

            let tpex_data = &tpex_list[i];
            loop_data_temp_data(tpex_data, &task, sort_num).await?;
        }
    }

    Ok(())
}

async fn loop_data_temp_data(
    data: &SecurityTemp,
    task: &DailyTask,
    item_index: i32,
) -> Result<(), sqlx::Error> {
    let q_year = task.open_date_year.clone();
    let q_month = task.open_date_month.clone();
    let q_day = task.open_date_day.clone();
    let q_security_code = data.security_code.clone();
    let q_market_type = data.market_type.clone();
    let q_issue_date = data.issue_date.clone();

    let security = dao::find_one(
        q_year,
        q_month,
        q_day,
        q_security_code,
        q_market_type,
        q_issue_date,
    )
    .await;
    if security.is_none() {
        let seed: i64 = thread_rng().gen_range(1..=9999999999999);
        let security_seed = format!("{:013}", seed);
        let sort_no = item_index;

        let new_security_task = SecurityTask {
            row_id: String::new(),
            security_code: data.security_code.clone(),
            security_name: data.security_name.clone(),
            market_type: data.market_type.clone(),
            issue_date: data.issue_date.clone(),
            exec_count: 0,
            is_enabled: 1,
            sort_no,
            open_date_year: task.open_date_year.clone(),
            open_date_month: task.open_date_month.clone(),
            open_date_day: task.open_date_day.clone(),
            exec_seed: security_seed,
        };

        dao::create(new_security_task).await?;
    }

    Ok(())
}

pub async fn get_all_task(task: &DailyTask) -> Result<(), Box<dyn std::error::Error>> {
    event!(target: "security_api", Level::INFO, "call daily_task.task_run");

    let q_year = task.open_date_year.clone();
    let q_month = task.open_date_month.clone();
    let q_day = task.open_date_day.clone();

    let securitys = dao::find_all_by_times(q_year.clone(), q_month.clone(), q_day.clone()).await;

    let mut old_market_type = String::new();

    let mut index = 0;
    while securitys.len() > index {
        let security = &securitys[index];
        event!(target: "security_api", Level::DEBUG, "SecurityTask: {}", &security);

        let y = task.open_date_year.clone().parse().unwrap();
        let m = task.open_date_month.clone().parse().unwrap();
        let d = task.open_date_day.clone().parse().unwrap();

        let od = NaiveDate::from_ymd_opt(y, m, d).unwrap();
        let nod = od.and_hms_opt(15, 30, 0).unwrap();

        let nd = Local::now().date_naive();
        let ndt = Local::now().naive_local();

        let market_type = security.market_type.clone();

        // 今天且下午三點半
        if nd == od && nod < ndt {
            let start_time = Local::now();

            match loop_data_security_task(security.clone()).await {
                Ok(_) => {
                    let end_time = Local::now();

                    sleep(time::Duration::from_secs(sleep_time(
                        (end_time - start_time).num_seconds(),
                        old_market_type,
                        market_type,
                    )))
                    .await;

                    index += 1;
                    old_market_type = security.market_type.clone();
                }
                Err(e) => {
                    event!(target: "security_api", Level::ERROR, "daily_task.get_all_task {}", &e);
                    continue;
                }
            }
        // 小於今天的日期
        } else if nd > od {
            let res_data = response_data::dao::find_one_by_max(&security).await;
            if res_data.is_none() {
                let start_time = Local::now();

                match loop_data_security_task(security.clone()).await {
                    Ok(_) => {
                        let end_time = Local::now();

                        sleep(time::Duration::from_secs(sleep_time(
                            (end_time - start_time).num_seconds(),
                            old_market_type,
                            market_type,
                        )))
                        .await;
                        index += 1;
                        old_market_type = security.market_type.clone();
                    }
                    Err(e) => {
                        event!(target: "security_api", Level::ERROR, "daily_task.get_all_task {}", &e);
                        continue;
                    }
                }
            } else {
                update_data(&security.clone(), true).await;
                index += 1;
            }
        }
    }

    Ok(())
}

fn sleep_time(seconds: i64, old_market_type: String, new_market_type: String) -> u64 {
    event!(target: "security_api", Level::DEBUG,"{0},{1},{2}", seconds, old_market_type, new_market_type);
    match (old_market_type.as_ref(), new_market_type.as_ref()) {
        ("上市", "上櫃") => {
            if 4 - seconds > 0 {
                (4 - seconds) as u64
            } else if 4 - seconds <= 0 {
                0
            } else {
                4
            }
        }
        ("上市", "興櫃") => {
            if 4 - seconds > 0 {
                (4 - seconds) as u64
            } else if 4 - seconds <= 0 {
                0
            } else {
                4
            }
        }
        ("上櫃", "上市") => {
            if 4 - seconds > 0 {
                (4 - seconds) as u64
            } else if 4 - seconds <= 0 {
                0
            } else {
                4
            }
        }
        ("興櫃", "上市") => {
            if 4 - seconds > 0 {
                (4 - seconds) as u64
            } else if 4 - seconds <= 0 {
                0
            } else {
                4
            }
        }
        (_, _) => {
            if 8 - seconds > 0 {
                (8 - seconds) as u64
            } else if 8 - seconds <= 0 {
                0
            } else {
                8
            }
        }
    }
}

async fn loop_data_security_task(security: SecurityTask) -> Result<(), Box<dyn Error>> {
    // 重試設定
    let retry_strategy = ExponentialBackoff::from_millis(2000)
        .max_delay(time::Duration::from_secs(10))
        .take(5);

    let market_type = security.market_type.clone();
    let ref_market_type = market_type.as_str();

    let y = security.open_date_year.clone().parse::<i32>().unwrap();
    let m = security.open_date_month.clone();
    let tw_ym = format!("{0}/{1:02}", y - 1911, m);

    match ref_market_type {
        "上市" => {
            match Retry::spawn(retry_strategy.clone(), || async {
                response_data::service::get_twse_avg_json(&security).await
            })
            .await
            {
                Ok(res) => {
                    match serde_json::from_str::<SecurityPriceTwse>(&res) {
                        Ok(price) => {
                            let stat = price.stat;
                            let mut date = "000/00/00".to_string();
                            if price.data.is_some() {
                                let row = price.data.clone().unwrap();
                                if row.first().is_some() {
                                    date = row[0][0].clone();
                                }
                            }

                            if "OK" == stat && date.trim().starts_with(&tw_ym) {
                                add_res_data(&security, res).await;
                                update_data(&security, true).await;
                            } else {
                                event!(target: "security_api", Level::INFO,"{0}", res);
                                update_data(&security, false).await;
                            }

                            return Ok(());
                        }
                        Err(e) => {
                            event!(target: "security_api", Level::ERROR, "{0} => {1}", res, e);
                            return Ok(());
                        }
                    };
                }
                Err(e) => return Err(e),
            };
        }
        "上櫃" => {
            match Retry::spawn(retry_strategy.clone(), || async {
                response_data::service::get_tpex1_json(&security).await
            })
            .await
            {
                Ok(res) => {
                    match serde_json::from_str::<SecurityPriceTpex1>(&res) {
                        Ok(price) => {
                            let cnt = price.i_total_records;
                            let mut date = "000/00".to_string();
                            if !price.aa_data.is_empty() {
                                if price.aa_data[0].first().is_some() {
                                    date = price.aa_data[0][0].clone();
                                }
                            }

                            if cnt > 0 && date.trim().starts_with(&tw_ym) {
                                add_res_data(&security, res).await;
                                update_data(&security, true).await;
                            } else {
                                event!(target: "security_api", Level::INFO, "{0}", res);
                                update_data(&security, false).await;
                            }

                            return Ok(());
                        }
                        Err(e) => {
                            event!(target: "security_api", Level::ERROR, "{0} => {1}", res, e);
                            return Ok(());
                        }
                    };
                }
                Err(e) => return Err(e),
            };
        }
        "興櫃" => {
            match Retry::spawn(retry_strategy.clone(), || async {
                response_data::service::get_tpex2_html(&security).await
            })
            .await
            {
                Ok(res) => {
                    match serde_json::from_str::<SecurityPriceTpex2>(&res) {
                        Ok(price) => {
                            let cnt = price.i_total_records;
                            let mut date = "000/00".to_string();
                            if !price.aa_data.is_empty() {
                                if price.aa_data[0].first().is_some() {
                                    date = price.aa_data[0][0].clone();
                                }
                            }

                            if cnt > 0 && date.trim().starts_with(&tw_ym) {
                                add_res_data(&security, res).await;
                                update_data(&security, true).await;
                            } else {
                                event!(target: "security_api", Level::INFO, "{0}", res);
                                update_data(&security, false).await;
                            }

                            return Ok(());
                        }
                        Err(e) => {
                            event!(target: "security_api", Level::ERROR, "{0} => {1}", res, e);
                            return Ok(());
                        }
                    };
                }
                Err(e) => return Err(e),
            };
        }
        _ => (),
    }

    Ok(())
}

async fn add_res_data(security: &SecurityTask, html: String) {
    let res_data = response_data::dao::find_one_by_min(&security).await;
    if res_data.is_none() {
        let new_res_data = ResponseData {
            row_id: String::new(),
            open_date_year: security.clone().open_date_year,
            open_date_month: security.clone().open_date_month,
            open_date_day: security.clone().open_date_day,
            exec_code: security.clone().security_code,
            data_content: html,
        };
        response_data::dao::create(new_res_data).await.unwrap();
    } else {
        let new_res_data = ResponseData {
            row_id: res_data.clone().unwrap().row_id,
            open_date_year: security.clone().open_date_year,
            open_date_month: security.clone().open_date_month,
            open_date_day: security.clone().open_date_day,
            exec_code: res_data.clone().unwrap().exec_code,
            data_content: html,
        };
        response_data::dao::modify(new_res_data).await.unwrap();
    }
}

async fn update_data(security: &SecurityTask, is_action: bool) {
    let mut security_task = security.clone();
    security_task.exec_count = security_task.exec_count + 1;

    if is_action {
        security_task.is_enabled = 0;
    }

    dao::modify(security_task).await.unwrap();
}
