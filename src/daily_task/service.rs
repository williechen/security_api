#![warn(clippy::all, clippy::pedantic)]
use std::process;

use rand::{thread_rng, Rng};
use tokio::time::{self, sleep};
use tracing::{event, Level};

use crate::{listen_flow, response_data, security_price, security_task, security_temp};

use super::{dao, model::DailyTask};

pub async fn insert_task_data() -> Result<(), sqlx::Error> {
    let task_list = dao::find_all().await;
    for data in task_list {
        event!(target: "security_api", Level::DEBUG, "DailyTask: {}", &data);

        let q_year = data.open_date_year.clone();
        let q_month = data.open_date_month.clone();
        let q_day = data.open_date_day.clone();
        let q_job_code = data.job_code.clone();

        let task = dao::find_one(q_year, q_month, q_day, q_job_code).await;
        if task.is_none() {
            let new_date = DailyTask {
                row_id: String::new(),
                open_date_year: data.open_date_year.clone(),
                open_date_month: data.open_date_month.clone(),
                open_date_day: data.open_date_day.clone(),
                job_code: data.job_code.clone(),
                exec_status: "WAIT".to_string(),
            };
            dao::create(new_date).await?;
        }
    }
    Ok(())
}

pub async fn exec_daily_task() -> Result<(), sqlx::Error> {
    let mut exec_task = dao::find_one_by_exec_desc("security".to_string()).await;
    while exec_task.is_some() {
        let e_open_date = start_open_data("security", &exec_task.clone().unwrap()).await;

        let task_list =
            dao::find_all_by_exec_desc(e_open_date.0.clone(), e_open_date.1.clone()).await;
        for task in task_list {
            event!(target: "security_api", Level::INFO, "DailyTaskInfo: {0}", &task);
            update_task_status(&task, "OPEN").await;

            let job_code = task.job_code.clone();
            let ref_job_code = job_code.as_str();

            // 執行任務
            match ref_job_code {
                "get_web_security" => {
                    match response_data::service::get_security_all_code(&task).await {
                        Ok(_) => {
                            update_task_status(&task, "EXIT").await;
                            event!(target: "security_api", Level::INFO, "daily_task.get_web_security Done");
                        }
                        Err(e) => {
                            update_task_status(&task, "EXEC").await;
                            event!(target: "security_api", Level::ERROR, "daily_task.get_web_security {}", &e);
                            panic!("daily_task.get_web_security Error {}", &e)
                        }
                    }
                }
                "res_to_temp" => match security_temp::service::get_security_to_temp(&task).await {
                    Ok(_) => {
                        update_task_status(&task, "EXIT").await;
                        event!(target: "security_api", Level::INFO,"daily_task.res_to_temp Done");
                    }
                    Err(e) => {
                        update_task_status(&task, "EXEC").await;
                        event!(target: "security_api", Level::ERROR, "daily_task.res_to_temp {}", &e);
                        panic!("daily_task.res_to_temp Error {}", &e)
                    }
                },
                "temp_to_task" => {
                    match security_task::service::insert_task_data(&task).await {
                        Ok(_) => {
                            update_task_status(&task, "EXIT").await;
                            event!(target: "security_api", Level::INFO, "daily_task.temp_to_task Done");
                        }
                        Err(e) => {
                            update_task_status(&task, "EXEC").await;
                            event!(target: "security_api", Level::ERROR, "daily_task.temp_to_task {}", &e);
                            panic!("daily_task.temp_to_task Error {}", &e)
                        }
                    }

                    security_task::service_range::update_task_data(&task).await?;
                }
                "delete_temp" => match security_temp::service::delete_temp().await {
                    Ok(_) => {
                        update_task_status(&task, "EXIT").await;
                        event!(target: "security_api", Level::INFO, "daily_task.delete_temp Done");
                    }
                    Err(e) => {
                        update_task_status(&task, "EXEC").await;
                        event!(target: "security_api", Level::ERROR, "daily_task.delete_temp {}", &e);
                        panic!("daily_task.delete_temp Error {}", &e)
                    }
                },
                "task_run" => match security_task::service::get_all_task(&task).await {
                    Ok(_) => {
                        update_task_status(&task, "EXIT").await;
                        event!(target: "security_api", Level::INFO, "daily_task.task_run Done");
                    }
                    Err(e) => {
                        update_task_status(&task, "EXEC").await;
                        event!(target: "security_api", Level::ERROR, "daily_task.task_run {}", &e);
                        panic!("daily_task.task_run Error {}", &e)
                    }
                },
                _ => event!(target: "security_api", Level::DEBUG, "daily_task.other_job: {0}", &job_code)
        
            };
        }

        end_open_date("security", &e_open_date.0, &e_open_date.1).await;
        exec_task = dao::find_one_by_exec_desc("security".to_string()).await;
    }
    Ok(())
}

pub async fn exec_price_task() -> Result<(), Box<dyn std::error::Error>> {
    let mut exec_task = dao::find_one_by_exec_asc("price".to_string()).await;
    while exec_task.is_some() {
        let e_open_date = start_open_data("price", &exec_task.clone().unwrap()).await;

        let task_list =
            dao::find_all_by_exec_asc(e_open_date.0.clone(), e_open_date.1.clone()).await;
        for task in task_list {
            event!(target: "security_api", Level::INFO, "DailyTaskInfo {0}", &task);
            update_task_status(&task, "OPEN").await;

            let job_code = task.job_code.clone();
            let ref_job_code = job_code.as_str();

            // 執行任務
            match ref_job_code {
                "res_price" => match security_price::service::get_security_to_price(&task).await {
                    Ok(_) => {
                        update_task_status(&task, "EXIT").await;
                        event!(target: "security_api", Level::INFO,  "daily_task.res_price Done");
                    }
                    Err(e) => {
                        update_task_status(&task, "EXEC").await;
                        event!(target: "security_api", Level::ERROR,  "daily_task.res_price {}", &e);
                    }
                },
                "price_value" => {
                    match security_price::service::get_calculator_to_price(&task).await {
                        Ok(_) => {
                            update_task_status(&task, "EXIT").await;
                            event!(target: "security_api", Level::INFO,  "daily_task.price_value Done");
                        }
                        Err(e) => {
                            update_task_status(&task, "EXEC").await;
                            event!(target: "security_api", Level::ERROR,  "daily_task.price_value {}", &e);
                        }
                    }
                }
                _ => event!(target: "security_api", Level::DEBUG,  "price_task.other_job: {0}", &job_code)
                
            };
        }

        end_open_date("price", &e_open_date.0, &e_open_date.1).await;
        exec_task = dao::find_one_by_exec_asc("price".to_string()).await;
    }
    Ok(())
}

async fn update_task_status(task: &DailyTask, status: &str) {
    let mut daily_task = task.clone();
    daily_task.exec_status = status.to_string();

    dao::modify(daily_task).await.unwrap();
}

async fn start_open_data(flow_code: &str, task: &DailyTask) -> (String, String) {
    let pid = process::id() as i32;
    let year = task.open_date_year.clone();
    let month = task.open_date_month.clone();

    sleep(time::Duration::from_secs(thread_rng().gen_range(2..=4))).await;
    let results = listen_flow::service::read_flow_data(flow_code, &year, &month).await;
    if results.len() > 0 {
        if pid == results[0].pid {
            (year, month)
        } else {
            end_open_date(flow_code, &year, &month).await;
            if "price" == flow_code {
                let res = dao::find_one_by_exec_asc("price".to_string()).await;
                let year = res.clone().unwrap().open_date_year;
                let month = res.clone().unwrap().open_date_month;
                listen_flow::service::insert_flow_data2(pid, flow_code, &year, &month).await;
                (year, month)
            } else {
                let res = dao::find_one_by_exec_desc("security".to_string()).await;
                let year = res.clone().unwrap().open_date_year;
                let month = res.clone().unwrap().open_date_month;
                listen_flow::service::insert_flow_data2(pid, flow_code, &year, &month).await;
                (year, month)
            }
        }
    } else {
        listen_flow::service::insert_flow_data2(pid, flow_code, &year, &month).await;
        (year, month)
    }
}

async fn end_open_date(flow_code: &str, year: &str, month: &str) {
    let pid = process::id() as i32;
    listen_flow::service::modify_flow_data2(pid, flow_code, year, month).await;
}
