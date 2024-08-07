#![warn(clippy::all, clippy::pedantic)]
use std::process;

use chrono::{Local, NaiveDate};
use rand::{thread_rng, Rng};
use sqlx::PgConnection;
use tokio::time;
use tracing::{event, Level};

use crate::{
    listen_flow, repository::Repository, response_data, security_price, security_task,
    security_temp,
};

use super::{
    dao,
    model::{DailyTask, DailyTaskInfo},
};

pub async fn insert_task_data(db_url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let pool = Repository::new(db_url).await;
    let mut transaction = pool.connection.acquire().await?;

    let task_list = select_task(&mut *transaction, Local::now().date_naive()).await?;
    for data in task_list {
        event!(target: "security_api", Level::DEBUG, "DailyTask: {}", &data);
        let query_daily_task = DailyTask {
            row_id: None,
            open_date: data.open_date.clone(),
            job_code: data.job_code.clone(),
            exec_status: None,
        };
        let task_list = dao::read_all(&mut *transaction, query_daily_task).await?;
        if task_list.0 <= 0 {
            dao::create(&mut *transaction, data.clone()).await?;
        } else {
            dao::update(&mut *transaction, data.clone()).await?;
        }
    }
    Ok(())
}

async fn select_task(
    transaction: &mut PgConnection,
    date: NaiveDate,
) -> Result<Vec<DailyTask>, Box<dyn std::error::Error>> {
    match dao::read_all_by_sql(
        transaction,
        &format!(
            r#"
            SELECT '' AS row_id
                 , CONCAT(cd.ce_year, cd.ce_month, cd.ce_day) AS open_date
                 , ts.job_code 
                 , 'WAIT' AS exec_status
              FROM calendar_data cd
              JOIN task_setting ts
                ON cd.group_task = ts.group_code
             WHERE NOT EXISTS (
                   SELECT 1 
                     FROM daily_task dt
                    WHERE dt.open_date = CONCAT(cd.ce_year, cd.ce_month, cd.ce_day)
                      AND dt.job_code = ts.job_code
             )
              AND CONCAT(cd.ce_year, cd.ce_month, cd.ce_day) <= '{0}'
              AND cd.date_status = 'O'
            ORDER BY 2 desc, ts.sort_no  
            "#,
            date.format("%Y%m%d").to_string()
        ),
    )
    .await
    {
        Ok(rows) => Ok(rows.1),
        Err(e) => {
            event!(target: "security_api", Level::ERROR, "daily_task.select_task: {}", &e);
            panic!("daily_task.select_task Error {}", &e)
        }
    }
}

pub async fn exec_daily_task(db_url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let pool = Repository::new(db_url).await;
    let mut transaction = pool.connection.acquire().await?;

    let mut exec_open_date =
        dao::read_by_exec(&mut *transaction, "security", "dt.open_date desc").await?;
    while exec_open_date.len() > 0 {
        let e_open_date = get_open_data(db_url, &exec_open_date[0], "security").await;

        let task_info_list = dao::read_all_by_daily(
            &mut *transaction,
            &e_open_date[0..4],
            &e_open_date[4..6],
            "dt.open_date desc",
        )
        .await?;
        for task_info in task_info_list {
            if task_info.job_code.is_some() {
                event!(target: "security_api", Level::DEBUG, "DailyTaskInfo: {:?}", &task_info);
                update_task_status(&mut *transaction, &task_info, "OPEN").await;

                let job_code = task_info.job_code.clone().unwrap();
                let open_date = task_info.open_date.clone().unwrap();

                let ref_job_code = job_code.as_str();

                // 執行任務
                match ref_job_code {
                    "get_web_security" => {
                        match response_data::service::get_security_all_code(db_url, &task_info)
                            .await
                        {
                            Ok(_) => {
                                update_task_status(&mut *transaction, &task_info, "EXIT").await;
                                event!(target: "security_api", Level::INFO, "daily_task.get_web_security Done");
                            }
                            Err(e) => {
                                update_task_status(&mut *transaction, &task_info, "EXEC").await;
                                event!(target: "security_api", Level::ERROR, "daily_task.get_web_security {}", &e);
                                panic!("daily_task.get_web_security Error {}", &e)
                            }
                        }
                    }
                    "res_to_temp" => {
                        match security_temp::service::get_security_to_temp(db_url, &task_info).await
                        {
                            Ok(_) => {
                                update_task_status(&mut *transaction, &task_info, "EXIT").await;
                                event!(target: "security_api", Level::INFO, "daily_task.res_to_temp Done");
                            }
                            Err(e) => {
                                update_task_status(&mut *transaction, &task_info, "EXEC").await;
                                event!(target: "security_api", Level::ERROR, "daily_task.res_to_temp {}", &e);
                                panic!("daily_task.res_to_temp Error {}", &e)
                            }
                        }
                    }
                    "temp_to_task" => {
                        match security_task::service::insert_task_data(db_url, &task_info).await {
                            Ok(_) => {
                                update_task_status(&mut *transaction, &task_info, "EXIT").await;
                                event!(target: "security_api", Level::INFO, "daily_task.temp_to_task Done");
                            }
                            Err(e) => {
                                update_task_status(&mut *transaction, &task_info, "EXEC").await;
                                event!(target: "security_api", Level::ERROR, "daily_task.temp_to_task {}", &e);
                                panic!("daily_task.temp_to_task Error {}", &e)
                            }
                        }

                        security_task::service_range::update_task_data(db_url, &task_info).await?;
                    }
                    "delete_temp" => match security_temp::service::delete_temp(db_url).await {
                        Ok(_) => {
                            update_task_status(&mut *transaction, &task_info, "EXIT").await;
                            event!(target: "security_api", Level::INFO, "daily_task.delete_temp Done");
                        }
                        Err(e) => {
                            update_task_status(&mut *transaction, &task_info, "EXEC").await;
                            event!(target: "security_api", Level::ERROR, "daily_task.delete_temp {}", &e);
                            panic!("daily_task.delete_temp Error {}", &e)
                        }
                    },
                    "task_run" => {
                        match security_task::service::get_all_task(db_url, &task_info).await {
                            Ok(_) => {
                                update_task_status(&mut *transaction, &task_info, "EXIT").await;
                                event!(target: "security_api", Level::INFO, "daily_task.task_run Done");
                            }
                            Err(e) => {
                                update_task_status(&mut *transaction, &task_info, "EXEC").await;
                                event!(target: "security_api", Level::ERROR, "daily_task.task_run {}", &e);
                                panic!("daily_task.task_run Error {}", &e)
                            }
                        }
                    }
                    _ => {
                        event!(target: "security_api", Level::INFO, "daily_task.other_job: {0} {1}", &job_code, &open_date)
                    }
                };
            }
            // 等待數量
            let wait_number = task_info.wait_number.unwrap_or(0);

            // 等待單位
            if task_info.wait_type.is_some() {
                match task_info.wait_type.clone().unwrap().as_str() {
                    "DM" => {
                        time::sleep(time::Duration::from_secs(
                            (60 * 60 * 24 * 30 * wait_number).try_into().unwrap(),
                        ))
                        .await
                    }
                    "DW" => {
                        time::sleep(time::Duration::from_secs(
                            (60 * 60 * 24 * 7 * wait_number).try_into().unwrap(),
                        ))
                        .await
                    }
                    "DD" => {
                        time::sleep(time::Duration::from_secs(
                            (60 * 60 * 24 * wait_number).try_into().unwrap(),
                        ))
                        .await
                    }
                    "TH" => {
                        time::sleep(time::Duration::from_secs(
                            (60 * 60 * wait_number).try_into().unwrap(),
                        ))
                        .await
                    }
                    "TM" => {
                        time::sleep(time::Duration::from_secs(
                            (60 * wait_number).try_into().unwrap(),
                        ))
                        .await
                    }
                    "TS" => {
                        time::sleep(time::Duration::from_secs(wait_number.try_into().unwrap()))
                            .await
                    }
                    _ => time::sleep(time::Duration::from_secs(1)).await,
                };
            }
        }
        exec_open_date =
            dao::read_by_exec(&mut *transaction, "security", "dt.open_date desc").await?;
    }
    Ok(())
}

pub async fn exec_price_task(db_url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let pool = Repository::new(db_url).await;
    let mut transaction = pool.connection.acquire().await?;

    let mut exec_open_date = dao::read_by_exec(&mut *transaction, "price", "dt.open_date").await?;
    while exec_open_date.len() > 0 {
        let e_open_date = get_open_data(db_url, &exec_open_date[0], "price").await;

        let task_info_list = dao::read_all_by_daily(
            &mut *transaction,
            &e_open_date[0..4],
            &e_open_date[4..6],
            "dt.open_date",
        )
        .await?;
        for task_info in task_info_list {
            if task_info.job_code.is_some() {
                event!(target: "security_api", Level::DEBUG, "DailyTaskInfo {:?}", &task_info);
                update_task_status(&mut *transaction, &task_info, "OPEN").await;

                let job_code = task_info.job_code.clone().unwrap();
                let open_date = task_info.open_date.clone().unwrap();

                let ref_job_code = job_code.as_str();

                // 執行任務
                match ref_job_code {
                    "res_price" => {
                        match security_price::service::get_security_to_price(db_url, &task_info)
                            .await
                        {
                            Ok(_) => {
                                update_task_status(&mut *transaction, &task_info, "EXIT").await;
                                event!(target: "security_api", Level::INFO, "daily_task.res_price Done");
                            }
                            Err(e) => {
                                update_task_status(&mut *transaction, &task_info, "EXEC").await;
                                event!(target: "security_api", Level::ERROR, "daily_task.res_price {}", &e);
                                panic!("daily_task.res_price Error {}", &e)
                            }
                        }
                    }
                    "price_value" => {
                        match security_price::service::get_calculator_to_price(db_url, &task_info)
                            .await
                        {
                            Ok(_) => {
                                update_task_status(&mut *transaction, &task_info, "EXIT").await;
                                event!(target: "security_api", Level::INFO, "daily_task.price_value Done");
                            }
                            Err(e) => {
                                update_task_status(&mut *transaction, &task_info, "EXEC").await;
                                event!(target: "security_api", Level::ERROR, "daily_task.price_value {}", &e);
                                panic!("daily_task.price_value Error {}", &e)
                            }
                        }
                    }
                    _ => {
                        event!(target: "security_api", Level::INFO, "price_task.other_job: {0} {1}", &job_code, &open_date)
                    }
                };
            }
            // 等待數量
            let wait_number = task_info.wait_number.unwrap_or(0);

            // 等待單位
            if task_info.wait_type.is_some() {
                match task_info.wait_type.clone().unwrap().as_str() {
                    "DM" => {
                        time::sleep(time::Duration::from_secs(
                            (60 * 60 * 24 * 30 * wait_number).try_into().unwrap(),
                        ))
                        .await
                    }
                    "DW" => {
                        time::sleep(time::Duration::from_secs(
                            (60 * 60 * 24 * 7 * wait_number).try_into().unwrap(),
                        ))
                        .await
                    }
                    "DD" => {
                        time::sleep(time::Duration::from_secs(
                            (60 * 60 * 24 * wait_number).try_into().unwrap(),
                        ))
                        .await
                    }
                    "TH" => {
                        time::sleep(time::Duration::from_secs(
                            (60 * 60 * wait_number).try_into().unwrap(),
                        ))
                        .await
                    }
                    "TM" => {
                        time::sleep(time::Duration::from_secs(
                            (60 * wait_number).try_into().unwrap(),
                        ))
                        .await
                    }
                    "TS" => {
                        time::sleep(time::Duration::from_secs(wait_number.try_into().unwrap()))
                            .await
                    }
                    _ => time::sleep(time::Duration::from_secs(1)).await,
                };
            }
        }
        exec_open_date = dao::read_by_exec(&mut *transaction, "price", "dt.open_date").await?;
    }
    Ok(())
}

async fn update_task_status(
    transaction: &mut PgConnection,
    task_info: &DailyTaskInfo,
    status: &str,
) {
    let daily_task = DailyTask {
        row_id: task_info.row_id.clone(),
        open_date: task_info.open_date.clone(),
        job_code: task_info.job_code.clone(),
        exec_status: Some(status.to_string()),
    };
    dao::update(transaction, daily_task).await.unwrap();
}

async fn get_open_data(db_url: &str, open_date: &str, flow_code: &str) -> String {
    let pid = process::id() as i32;
    let year = &open_date[0..4];
    let month = &open_date[4..6];

    time::sleep(time::Duration::from_secs(thread_rng().gen_range(2..=4))).await;
    let results = listen_flow::service::read_flow_data(db_url, flow_code, year, month).await;
    if results.len() > 0 {
        if Some(pid) == results[0].pid {
            open_date.to_string()
        } else {
            let pool = Repository::new(db_url).await;
            let mut transaction = pool.connection.acquire().await.unwrap();
            if "price" == flow_code {
                let res = dao::read_by_exec(&mut *transaction, flow_code, "dt.open_date")
                    .await
                    .unwrap();
                let year = &res[0][0..4];
                let month = &res[0][4..6];
                listen_flow::service::insert_flow_data2(db_url, pid.into(), flow_code, year, month)
                    .await;
                res[0].clone()
            } else {
                let res = dao::read_by_exec(&mut *transaction, flow_code, "dt.open_date desc")
                    .await
                    .unwrap();
                let year = &res[0][0..4];
                let month = &res[0][4..6];
                listen_flow::service::insert_flow_data2(db_url, pid.into(), flow_code, year, month)
                    .await;
                res[0].clone()
            }
        }
    } else {
        listen_flow::service::insert_flow_data2(db_url, pid.into(), flow_code, year, month).await;
        open_date.to_string()
    }
}
