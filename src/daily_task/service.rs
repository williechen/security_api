#![warn(clippy::all, clippy::pedantic)]
use std::{future::Future, pin::Pin, process};

use tracing::{event, Level};

use crate::{
    listen_flow::{self, model::ListenFlow},
    response_data, security_price, security_task, security_temp,
};

use super::{dao, model::DailyTask};

pub async fn insert_task_data() -> Result<(), sqlx::Error> {
    let task_list = dao::find_all().await;
    for data in task_list {
        event!(target: "security_api", Level::DEBUG, "DailyTask: {}", &data);

        let q_year = &data.open_date_year;
        let q_month = &data.open_date_month;
        let q_day = &data.open_date_day;
        let q_job_code = &data.job_code;

        let task = dao::find_one(q_year, q_month, q_day, q_job_code).await;
        if task.is_none() {
            let new_date = DailyTask {
                open_date_year: data.open_date_year,
                open_date_month: data.open_date_month,
                open_date_day: data.open_date_day,
                job_code: data.job_code,
                exec_status: "WAIT".to_string(),
                row_id: "".to_string(),
            };
            dao::create(new_date).await?;
        }
    }
    Ok(())
}

pub async fn exec_daily_task() -> Result<(), sqlx::Error> {
    let mut exec_task = dao::find_one_by_exec_desc("security").await;
    while exec_task.is_some() {
        let e_open_date = start_open_data("security", &exec_task.unwrap()).await;

        let (year, month) = e_open_date.await;

        let task_list = dao::find_all_by_exec_desc(&year, &month).await;
        for task in task_list {
            event!(target: "security_api", Level::INFO, "DailyTaskInfo: {0}", &task);
            update_task_status(&task, "OPEN").await;

            let job_code = &task.job_code;
            let ref_job_code = job_code.as_str();

            // 執行任務
            match ref_job_code {
                "delete_temp" => init_security_data(&task).await,
                "get_web_security" => reply_security_data(&task).await,
                "res_to_temp" => response_to_temp(&task).await,
                "temp_to_task" => temp_to_daily_security(&task).await,
                "task_run" => executive_daily_security(&task).await,
                _ => (),
            }
        }

        end_open_date("security", &year, &month).await;
        exec_task = dao::find_one_by_exec_desc("security").await;
    }
    Ok(())
}

pub async fn exec_price_task() -> Result<(), Box<dyn std::error::Error>> {
    let mut exec_task = dao::find_one_by_exec_asc("price").await;
    while exec_task.is_some() {
        let e_open_date = start_open_data("price", &exec_task.unwrap()).await;

        let (year, month) = e_open_date.await;

        let task_list = dao::find_all_by_exec_asc(&year, &month).await;
        for task in task_list {
            event!(target: "security_api", Level::INFO, "DailyTaskInfo {0}", &task);
            update_task_status(&task, "OPEN").await;

            let job_code = &task.job_code;
            let ref_job_code = job_code.as_str();

            // 執行任務
            match ref_job_code {
                "res_price" => parse_security_price(&task).await,
                "price_value" => statistics_average_price(&task).await,
                _ => (),
            }
        }

        end_open_date("price", &year, &month).await;
        exec_task = dao::find_one_by_exec_asc("price").await;
    }
    Ok(())
}

async fn init_security_data(task: &DailyTask) {
    match security_temp::service::delete_temp().await {
        Ok(_) => {
            update_task_status(task, "EXIT").await;
            event!(target: "security_api", Level::INFO, "daily_task.delete_temp Done");
        }
        Err(e) => {
            update_task_status(task, "EXEC").await;
            event!(target: "security_api", Level::ERROR, "daily_task.delete_temp {}", &e);
            panic!("daily_task.delete_temp Error {}", &e)
        }
    }
}

async fn reply_security_data(task: &DailyTask) {
    match response_data::service::get_security_all_code(task).await {
        Ok(_) => {
            update_task_status(task, "EXIT").await;
            event!(target: "security_api", Level::INFO, "daily_task.get_web_security Done");
        }
        Err(e) => {
            update_task_status(task, "EXEC").await;
            event!(target: "security_api", Level::ERROR, "daily_task.get_web_security {}", &e);
            panic!("daily_task.get_web_security Error {}", &e)
        }
    }
}

async fn response_to_temp(task: &DailyTask) {
    match security_temp::service::get_security_to_temp(task).await {
        Ok(_) => {
            update_task_status(task, "EXIT").await;
            event!(target: "security_api", Level::INFO, "daily_task.res_to_temp Done");
        }
        Err(e) => {
            update_task_status(task, "EXEC").await;
            event!(target: "security_api", Level::ERROR, "daily_task.res_to_temp {}", &e);
            panic!("daily_task.res_to_temp Error {}", &e)
        }
    }
}

async fn temp_to_daily_security(task: &DailyTask) {
    match security_task::service::insert_task_data(task).await {
        Ok(_) => {
            update_task_status(task, "EXIT").await;
            event!(target: "security_api", Level::INFO, "daily_task.temp_to_task Done");
        }
        Err(e) => {
            update_task_status(task, "EXEC").await;
            event!(target: "security_api", Level::ERROR, "daily_task.temp_to_task {}", &e);
            panic!("daily_task.temp_to_task Error {}", &e)
        }
    }

    security_task::service_range::update_task_data(task)
        .await
        .unwrap();
}

async fn executive_daily_security(task: &DailyTask) {
    match security_task::service::get_all_task(task).await {
        Ok(_) => {
            update_task_status(task, "EXIT").await;
            event!(target: "security_api", Level::INFO, "daily_task.task_run Done");
        }
        Err(e) => {
            update_task_status(task, "EXEC").await;
            event!(target: "security_api", Level::ERROR, "daily_task.task_run {}", &e);
            panic!("daily_task.task_run Error {}", &e)
        }
    }
}

async fn parse_security_price(task: &DailyTask) {
    match security_price::service::get_security_to_price(task).await {
        Ok(_) => {
            update_task_status(task, "EXIT").await;
            event!(target: "security_api", Level::INFO,  "daily_task.res_price Done");
        }
        Err(e) => {
            update_task_status(task, "EXEC").await;
            event!(target: "security_api", Level::ERROR, "daily_task.res_price {}", &e);
            panic!("daily_task.res_price Error {}", &e)
        }
    }
}

async fn statistics_average_price(task: &DailyTask) {
    match security_price::service::get_calculator_to_price(task).await {
        Ok(_) => {
            update_task_status(task, "EXIT").await;
            event!(target: "security_api", Level::INFO,  "daily_task.price_value Done");
        }
        Err(e) => {
            update_task_status(task, "EXEC").await;
            event!(target: "security_api", Level::ERROR,  "daily_task.price_value {}", &e);
            panic!("daily_task.price_value Error {}", &e)
        }
    }
}

async fn update_task_status(task: &DailyTask, status: &str) {
    let mut daily_task = task.clone();
    daily_task.exec_status = status.to_string();

    dao::modify(daily_task).await.unwrap();
}

async fn start_open_data(
    flow_code: &str,
    task: &DailyTask,
) -> Pin<Box<dyn Future<Output = (String, String)>>> {
    let pid = process::id() as i32;
    let year = &task.open_date_year;
    let month = &task.open_date_month;

    let results = listen_flow::service::read_flow_data(flow_code, &year, &month).await;
    let current_tasks = results
        .into_iter()
        .filter(|x| x.pid == pid && x.pstatus != "EXIT".to_string())
        .collect::<Vec<ListenFlow>>();
    if current_tasks.len() > 0 {
        let exec_task = dao::find_one_by_exec_asc(flow_code).await;
        return Box::pin(start_open_data(flow_code, &exec_task.unwrap())).await;
    } else {
        listen_flow::service::insert_flow_data2(pid, flow_code, &year, &month).await;
        let y = year.clone();
        let m = month.clone();
        return Box::pin(async move { (y, m) });
    }
}

async fn end_open_date(flow_code: &str, year: &str, month: &str) {
    let pid = process::id() as i32;
    listen_flow::service::modify_flow_data2(pid, flow_code, year, month).await;
}
