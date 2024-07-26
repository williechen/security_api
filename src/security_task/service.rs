#![warn(clippy::all, clippy::pedantic)]

use std::{cmp::max, thread::sleep, time};

use chrono::{Local, NaiveDate};
use log::{debug, error, info};
use rand::{thread_rng, Rng};
use retry::delay::{jitter, Exponential};
use retry::retry;
use serde_json::Value;

use super::{
    dao,
    model::{NewSecurityTask, SecurityTask},
};
use crate::response_data::model::NewResponseData;
use crate::{
    daily_task::model::DailyTask,
    response_data::{self, model::ResponseData},
    security_temp::{self, model::SecurityTemp},
};

pub fn insert_task_data(task: &DailyTask) -> Result<(), Box<dyn std::error::Error>> {
    info!(target: "security_api", "call daily_task.temp_to_task");

    let twse_list = security_temp::dao::find_all_by_twse(&task);
    let tpex_list = security_temp::dao::find_all_by_tpex(&task);

    let max_count = max(twse_list.len(), tpex_list.len());

    let mut sort_num = 0;
    for i in 0..max_count {
        if i < twse_list.len() {
            sort_num = sort_num + 1;

            let twse_data = &twse_list[i];
            loop_data_temp_data(twse_data, &task, sort_num)?;
        }
        if i < tpex_list.len() {
            sort_num = sort_num + 1;

            let tpex_data = &tpex_list[i];
            loop_data_temp_data(tpex_data, &task, sort_num)?;
        }
    }

    Ok(())
}

fn loop_data_temp_data(
    data: &SecurityTemp,
    task: &DailyTask,
    item_index: i32,
) -> Result<(), Box<dyn std::error::Error>> {
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
    );
    if security.is_none() {
        let seed: i64 = thread_rng().gen_range(1..=9999999999999);
        let security_seed = format!("{:013}", seed);
        let sort_no = item_index;

        let new_security_task = NewSecurityTask {
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
            created_date: Local::now().naive_local(),
            updated_date: Local::now().naive_local(),
        };

        dao::create(new_security_task)?;
    }

    Ok(())
}

pub fn get_all_task(task: &DailyTask) -> Result<(), Box<dyn std::error::Error>> {
    info!(target: "security_api", "call daily_task.task_run");

    let q_year = task.open_date_year.clone();
    let q_month = task.open_date_month.clone();
    let q_day = task.open_date_day.clone();

    let mut security = dao::find_one_by_times(q_year.clone(), q_month.clone(), q_day.clone());
    while security.is_some() {
        debug!(target: "security_api", "SecurityTask: {}", &security.clone().unwrap());

        let y = task.open_date_year.clone().parse().unwrap();
        let m = task.open_date_month.clone().parse().unwrap();
        let d = task.open_date_day.clone().parse().unwrap();

        let od = NaiveDate::from_ymd_opt(y, m, d).unwrap();
        let nod = od.and_hms_opt(15, 30, 0).unwrap();

        let nd = Local::now().date_naive();
        let ndt = Local::now().naive_local();

        // 今天且下午三點半
        if nd == od && nod < ndt {
            let start_time = Local::now().time();

            match loop_data_security_task(security.clone().unwrap()) {
                Ok(_) => {
                    let end_time = Local::now().time();
                    let seconds = 8 - (end_time - start_time).num_seconds();

                    let sleep_num = if seconds > 4 {
                        thread_rng().gen_range(4..=seconds)
                    } else {
                        4
                    };
                    sleep(time::Duration::from_secs(sleep_num.try_into().unwrap()));
                }
                Err(e) => {
                    error!(target: "security_api", "daily_task.get_all_task {}", &e);
                }
            }

        // 小於今天的日期
        } else if nd > od {
            let res_data = response_data::dao::find_one_by_max(&security.clone().unwrap());
            if res_data.is_none() {
                let start_time = Local::now().time();

                match loop_data_security_task(security.clone().unwrap()) {
                    Ok(_) => {
                        let end_time = Local::now().time();
                        let seconds = 8 - (end_time - start_time).num_seconds();

                        let sleep_num = if seconds > 4 {
                            thread_rng().gen_range(4..=seconds)
                        } else {
                            4
                        };
                        sleep(time::Duration::from_secs(sleep_num.try_into().unwrap()));
                    }
                    Err(e) => {
                        error!(target: "security_api", "daily_task.get_all_task {}", &e);
                    }
                }
            } else {
                update_data(&security.clone().unwrap(), true);
            }
        }
        security = dao::find_one_by_times(q_year.clone(), q_month.clone(), q_day.clone());
    }
    Ok(())
}

fn loop_data_security_task(security: SecurityTask) -> Result<(), retry::Error<Box<(dyn std::error::Error + 'static)>>> {
    // 重試設定
    let retry_strategy = Exponential::from_millis(2000).map(jitter).take(5);

    let market_type = security.market_type.clone();
    let ref_market_type = market_type.as_str();

    match ref_market_type {
        "上市" => {
            let data = retry(retry_strategy, || {
                response_data::service::get_twse_avg_json(&security)
            })?;

            let json_value: Value = serde_json::from_str(&data).expect("twse json parse error");
            match json_value.get("stat") {
                Some(t) => {
                    if "OK" == t.as_str().unwrap_or("") {
                        add_res_data(&security, &data);
                        update_data(&security, true);
                    } else {
                        update_data(&security, false);
                    }
                }
                None => update_data(&security, false),
            };
        }
        "上櫃" => {
            let data = retry(retry_strategy, || {
                response_data::service::get_tpex1_json(&security)
            })?;

            let json_value: Value = serde_json::from_str(&data).expect("tpex1 json parse error");
            match json_value.get("iTotalRecords") {
                Some(t) => {
                    if 0 < t.as_i64().unwrap_or(0) {
                        add_res_data(&security, &data);
                        update_data(&security, true);
                    } else {
                        update_data(&security, false);
                    }
                }
                None => update_data(&security, false),
            };
        }
        "興櫃" => {
            let data = retry(retry_strategy, || {
                response_data::service::get_tpex2_html(&security)
            })?;

            let json_value: Value = serde_json::from_str(&data).expect("tpex2 json parse error");
            match json_value.get("iTotalRecords") {
                Some(t) => {
                    if 0 < t.as_i64().unwrap_or(0) {
                        add_res_data(&security, &data);
                        update_data(&security, true);
                    } else {
                        update_data(&security, false);
                    }
                }
                None => update_data(&security, false),
            };
        }
        _ => (),
    }

    Ok(())
}

fn add_res_data(security: &SecurityTask, html: &String) {
    let res_data = response_data::dao::find_one_by_max(&security);
    if res_data.is_none() {
        let new_res_data = NewResponseData {
            open_date_year: security.clone().open_date_year,
            open_date_month: security.clone().open_date_month,
            open_date_day: security.clone().open_date_day,
            exec_code: security.clone().security_code,
            data_content: html.to_string(),
            created_date: Local::now().naive_local(),
            updated_date: Local::now().naive_local(),
        };
        response_data::dao::create(new_res_data).unwrap();
    } else {
        let new_res_data = ResponseData {
            row_id: res_data.clone().unwrap().row_id,
            open_date_year: security.clone().open_date_year,
            open_date_month: security.clone().open_date_month,
            open_date_day: security.clone().open_date_day,
            exec_code: res_data.clone().unwrap().exec_code,
            data_content: html.to_string(),
            created_date: res_data.clone().unwrap().created_date,
            updated_date: Local::now().naive_local(),
        };
        response_data::dao::modify(new_res_data).unwrap();
    }
}

fn update_data(security: &SecurityTask, is_action: bool) {
    
    let mut security_task = security.clone();
    security_task.exec_count = security_task.exec_count + 1;
    security_task.updated_date = Local::now().naive_local();

    if is_action {
        security_task.is_enabled = 0;
    }

    dao::modify(security_task).unwrap();
}
