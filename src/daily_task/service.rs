#![warn(clippy::all, clippy::pedantic)]
use std::{process, thread::sleep, time};

use chrono::Local;
use log::{debug, error, info};
use rand::{thread_rng, Rng};

use crate::{
    daily_task::model::NewDailyTask, listen_flow, response_data, security_error::SecurityError,
    security_price, security_task, security_temp,
};

use super::{dao, model::DailyTask};

pub fn insert_task_data() -> Result<(), SecurityError> {
    let task_list = dao::find_all();
    for data in task_list {
        debug!(target: "security_api", "DailyTask: {}", &data);

        let q_year = data.open_date_year.clone();
        let q_month = data.open_date_month.clone();
        let q_day = data.open_date_day.clone();
        let q_job_code = data.job_code.clone();

        let task = dao::find_one(q_year, q_month, q_day, q_job_code);
        if task.is_none() {
            let new_date = NewDailyTask {
                open_date_year: data.open_date_year.clone(),
                open_date_month: data.open_date_month.clone(),
                open_date_day: data.open_date_day.clone(),
                job_code: data.job_code.clone(),
                exec_status: "WAIT".to_string(),
                created_date: Local::now().naive_local(),
                updated_date: Local::now().naive_local(),
            };
            dao::create(new_date)?;
        }
    }
    Ok(())
}

pub fn exec_daily_task() -> Result<(), SecurityError> {
    let mut exec_task = dao::find_one_by_exec_desc("security".to_string());
    while exec_task.is_some() {
        let e_open_date = start_open_data("security", &exec_task.clone().unwrap());

        let task_list = dao::find_all_by_exec_desc(e_open_date.0.clone(), e_open_date.1.clone());
        for task in task_list {
            info!(target: "security_api", "DailyTaskInfo: {0}", &task);
            update_task_status(&task, "OPEN");

            let job_code = task.job_code.clone();
            let ref_job_code = job_code.as_str();
        }

        end_open_date("security", &e_open_date.0, &e_open_date.1);
        exec_task = dao::find_one_by_exec_desc("security".to_string());
    }
    Ok(())
}

pub fn exec_price_task() -> Result<(), Box<dyn std::error::Error>> {
    let mut exec_task = dao::find_one_by_exec_asc("price".to_string());
    while exec_task.is_some() {
        let e_open_date = start_open_data("price", &exec_task.clone().unwrap());

        let task_list = dao::find_all_by_exec_asc(e_open_date.0.clone(), e_open_date.1.clone());
        for task in task_list {
            info!(target: "security_api", "DailyTaskInfo {0}", &task);
            update_task_status(&task, "OPEN");

            // 執行任務
        }

        end_open_date("price", &e_open_date.0, &e_open_date.1);
        exec_task = dao::find_one_by_exec_asc("price".to_string());
    }
    Ok(())
}

fn init_security_data(task: &DailyTask) {
    match security_temp::service::delete_temp() {
        Ok(_) => {
            update_task_status(task, "EXIT");
            info!(target: "security_api", "daily_task.delete_temp Done");
        }
        Err(e) => {
            update_task_status(task, "EXEC");
            error!(target: "security_api", "daily_task.delete_temp {}", &e);
            panic!("daily_task.delete_temp Error {}", &e)
        }
    }
}

fn reply_security_data(task: &DailyTask) {
    match response_data::service::get_security_all_code(task) {
        Ok(_) => {
            update_task_status(task, "EXIT");
            info!(target: "security_api", "daily_task.get_web_security Done");
        }
        Err(e) => {
            update_task_status(task, "EXEC");
            error!(target: "security_api", "daily_task.get_web_security {}", &e);
            panic!("daily_task.get_web_security Error {}", &e)
        }
    }
}

fn response_to_temp(task: &DailyTask) {
    match security_temp::service::get_security_to_temp(task) {
        Ok(_) => {
            update_task_status(task, "EXIT");
            info!(target: "security_api","daily_task.res_to_temp Done");
        }
        Err(e) => {
            update_task_status(task, "EXEC");
            error!(target: "security_api", "daily_task.res_to_temp {}", &e);
            panic!("daily_task.res_to_temp Error {}", &e)
        }
    }
}

fn temp_to_daily_security(task: &DailyTask) {
    match security_task::service::insert_task_data(task) {
        Ok(_) => {
            update_task_status(task, "EXIT");
            info!(target: "security_api", "daily_task.temp_to_task Done");
        }
        Err(e) => {
            update_task_status(task, "EXEC");
            error!(target: "security_api", "daily_task.temp_to_task {}", &e);
            panic!("daily_task.temp_to_task Error {}", &e)
        }
    }

    security_task::service_range::update_task_data(task).unwrap();
}

fn executive_daily_security(task: &DailyTask) {
    match security_task::service::get_all_task(task) {
        Ok(_) => {
            update_task_status(task, "EXIT");
            info!(target: "security_api", "daily_task.task_run Done");
        }
        Err(e) => {
            update_task_status(task, "EXEC");
            error!(target: "security_api", "daily_task.task_run {}", &e);
            panic!("daily_task.task_run Error {}", &e)
        }
    }
}

fn parse_security_price(task: &DailyTask) {
    match security_price::service::get_security_to_price(task) {
        Ok(_) => {
            update_task_status(task, "EXIT");
            info!(target: "security_api",  "daily_task.res_price Done");
        }
        Err(e) => {
            update_task_status(task, "EXEC");
            error!(target: "security_api",  "daily_task.res_price {}", &e);
            panic!("daily_task.res_price Error {}", &e)
        }
    }
}

fn statistics_average_price(task: &DailyTask) {
    match security_price::service::get_calculator_to_price(task) {
        Ok(_) => {
            update_task_status(task, "EXIT");
            info!(target: "security_api",  "daily_task.price_value Done");
        }
        Err(e) => {
            update_task_status(task, "EXEC");
            error!(target: "security_api",  "daily_task.price_value {}", &e);
            panic!("daily_task.price_value Error {}", &e)
        }
    }
}

fn update_task_status(task: &DailyTask, status: &str) {
    let mut daily_task = task.clone();
    daily_task.exec_status = status.to_string();
    daily_task.updated_date = Local::now().naive_local();

    dao::modify(daily_task).unwrap();
}

fn start_open_data(flow_code: &str, task: &DailyTask) -> (String, String) {
    let pid = process::id() as i32;
    let year = task.open_date_year.clone();
    let month = task.open_date_month.clone();

    sleep(time::Duration::from_secs(thread_rng().gen_range(2..=4)));
    let results = listen_flow::service::read_flow_data(flow_code, &year, &month);
    if results.len() > 0 {
        if pid == results[0].pid {
            (year, month)
        } else {
            end_open_date(flow_code, &year, &month);
            if "price" == flow_code {
                let res = dao::find_one_by_exec_asc("price".to_string());
                let year = res.clone().unwrap().open_date_year;
                let month = res.clone().unwrap().open_date_month;
                listen_flow::service::insert_flow_data2(pid, flow_code, &year, &month);
                (year, month)
            } else {
                let res = dao::find_one_by_exec_desc("security".to_string());
                let year = res.clone().unwrap().open_date_year;
                let month = res.clone().unwrap().open_date_month;
                listen_flow::service::insert_flow_data2(pid, flow_code, &year, &month);
                (year, month)
            }
        }
    } else {
        listen_flow::service::insert_flow_data2(pid, flow_code, &year, &month);
        (year, month)
    }
}

fn end_open_date(flow_code: &str, year: &str, month: &str) {
    let pid = process::id() as i32;
    listen_flow::service::modify_flow_data2(pid, flow_code, year, month);
}
