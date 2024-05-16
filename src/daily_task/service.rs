use chrono::{Local, NaiveDate};
use tokio::time;
use tracing::{event, Level};

use crate::{
    response_data::{self, model::ResponseData},
    security_task, security_temp,
};

use super::{
    dao,
    model::{DailyTask, DailyTaskInfo},
};

pub async fn insert_task_data(pool: &sqlx::PgPool) -> Result<(), Box<dyn std::error::Error>> {
    let mut transaction = pool.begin().await?;

    let task_list = select_task(&mut transaction, Local::now().date_naive()).await?;
    event!(target: "security_api", Level::DEBUG, "{:?}", &task_list);
    loop_date_task_data(pool, &task_list).await?;

    Ok(())
}

pub async fn exec_daily_task(pool: &sqlx::PgPool) -> Result<(), Box<dyn std::error::Error>> {
    let mut transaction = pool.begin().await?;
    let task_info_list =
        dao::read_all_by_daily(&mut transaction, Local::now().date_naive()).await?;
    for task_info in task_info_list {
        if task_info.job_code.is_some() {
            update_task_status(pool, &task_info, "OPEN").await;
            // 執行任務
            match task_info.job_code.clone().unwrap().as_str() {
                "get_web_security" => match get_security_all_code(&pool, &task_info).await {
                    Ok(_) => {
                        update_task_status(pool, &task_info, "EXIT").await;
                        event!(target: "security_api", Level::INFO, "get_web_security Done");
                    }
                    Err(e) => {
                        update_task_status(pool, &task_info, "EXEC").await;
                        event!(target: "security_api", Level::ERROR, "get_web_security {}", &e);
                        panic!("get_web_security Error {}", &e)
                    }
                },
                "res_to_temp" => match get_security_to_temp(&pool, &task_info).await {
                    Ok(_) => {
                        update_task_status(pool, &task_info, "EXIT").await;
                        event!(target: "security_api", Level::INFO, "res_to_temp Done");
                    }
                    Err(e) => {
                        update_task_status(pool, &task_info, "EXEC").await;
                        event!(target: "security_api", Level::ERROR, "res_to_temp {}", &e);
                        panic!("res_to_temp Error {}", &e)
                    }
                },
                "temp_to_task" => match get_temp_to_task(&pool, &task_info).await {
                    Ok(_) => {
                        update_task_status(pool, &task_info, "EXIT").await;
                        event!(target: "security_api", Level::INFO, "temp_to_task Done");
                    }
                    Err(e) => {
                        update_task_status(pool, &task_info, "EXEC").await;
                        event!(target: "security_api", Level::ERROR, "temp_to_task {}", &e);
                        panic!("temp_to_task Error {}", &e)
                    }
                },
                "delete_temp" => match delete_temp(&pool, &task_info).await {
                    Ok(_) => {
                        update_task_status(pool, &task_info, "EXIT").await;
                        event!(target: "security_api", Level::INFO, "delete_temp Done");
                    }
                    Err(e) => {
                        update_task_status(pool, &task_info, "EXEC").await;
                        event!(target: "security_api", Level::ERROR, "delete_temp {}", &e);
                        panic!("delete_temp Error {}", &e)
                    }
                },
                "task_run" => match get_task_run(&pool, &task_info).await {
                    Ok(_) => {
                        update_task_status(pool, &task_info, "EXIT").await;
                        event!(target: "security_api", Level::INFO, "task_run Done");
                    }
                    Err(e) => {
                        update_task_status(pool, &task_info, "EXEC").await;
                        event!(target: "security_api", Level::ERROR, "task_run {}", &e);
                        panic!("task_run Error {}", &e)
                    }
                },
                _ => event!(target: "security_api", Level::INFO, "othen job"),
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
                    time::sleep(time::Duration::from_secs(wait_number.try_into().unwrap())).await
                }
                _ => time::sleep(time::Duration::from_secs(0)).await,
            };
        }
    }

    Ok(())
}

async fn loop_date_task_data(
    pool: &sqlx::PgPool,
    data_list: &Vec<DailyTask>,
) -> Result<(), Box<dyn std::error::Error>> {
    for data in data_list {
        let mut transaction = pool.begin().await?;

        let query_daily_task = DailyTask {
            row_id: None,
            open_date: data.open_date.clone(),
            job_code: data.job_code.clone(),
            exec_status: None,
        };

        let task_list = dao::read_all(&mut transaction, &query_daily_task).await?;
        if task_list.0 <= 0 {
            match dao::create(&mut transaction, data.clone()).await {
                Ok(_) => transaction.commit().await?,
                Err(e) => {
                    transaction.rollback().await?;
                    event!(target: "security_api", Level::ERROR, "loop_date_task_data {}", &e);
                    panic!("loop_date_task_data Error {}", &e)
                }
            };
        } else {
            match dao::update(&mut transaction, data.clone()).await {
                Ok(_) => transaction.commit().await?,
                Err(e) => {
                    transaction.rollback().await?;
                    event!(target: "security_api", Level::ERROR, "loop_date_task_data {}", &e);
                    panic!("loop_date_task_data Error {}", &e)
                }
            };
        }
    }

    Ok(())
}

async fn update_task_status(pool: &sqlx::PgPool, task_info: &DailyTaskInfo, status: &str) {
    let mut transaction = pool.begin().await.unwrap();

    let daily_task = DailyTask {
        row_id: task_info.row_id.clone(),
        open_date: task_info.open_date.clone(),
        job_code: task_info.job_code.clone(),
        exec_status: Some(status.to_string()),
    };

    match dao::update(&mut transaction, daily_task).await {
        Ok(_) => transaction.commit().await.unwrap(),
        Err(e) => {
            transaction.rollback().await.unwrap();
            event!(target: "security_api", Level::ERROR, "update_task_status {}", &e);
            panic!("update_task_status Error {}", &e)
        }
    };
}

async fn select_task(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    date: NaiveDate,
) -> Result<Vec<DailyTask>, Box<dyn std::error::Error>> {
    match dao::read_all_by_sql(
        transaction,
        &format!(
            r#"
            SELECT '' AS row_id
                 , '{0}' AS open_date
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
                   AND dt.exec_status = 'EXIT'
              )
                AND CONCAT(cd.ce_year, cd.ce_month, cd.ce_day) <= '{0}'
                AND cd.date_status = 'O'
             ORDER BY cd.ce_month desc, cd.ce_day desc, cd.ce_year desc, ts.sort_no  
            "#,
            date.format("%Y%m%d").to_string()
        ),
    )
    .await
    {
        Ok(rows) => Ok(rows.1),
        Err(e) => {
            event!(target: "security_api", Level::ERROR, "select_task {}", &e);
            panic!("select_task Error {}", &e)
        }
    }
}

pub async fn get_security_all_code(
    pool: &sqlx::PgPool,
    task_info: &DailyTaskInfo,
) -> Result<(), Box<dyn std::error::Error>> {
    event!(target: "security_api", Level::INFO, "call get_security_all_code");

    let mut transaction = pool.begin().await?;

    let query_response_data = ResponseData {
        row_id: None,
        open_date: task_info.open_date.clone(),
        exec_code: Some("seecurity".to_string()),
        data_content: None,
    };

    let data_list = response_data::dao::read_all(&mut transaction, &query_response_data).await?;
    if data_list.0 <= 0 {
        let content = response_data::service::get_web_security_data().await?;

        let response_data = ResponseData {
            row_id: None,
            open_date: task_info.open_date.clone(),
            exec_code: Some("seecurity".to_string()),
            data_content: Some(content),
        };

        match response_data::dao::create(&mut transaction, response_data).await {
            Ok(_) => transaction.commit().await?,
            Err(e) => {
                transaction.rollback().await?;
                event!(target: "security_api", Level::ERROR, "get_security_all_code {}", e);
                panic!("get_security_all_code Error {}", &e)
            }
        };
    }

    Ok(())
}

pub async fn get_security_to_temp(
    pool: &sqlx::PgPool,
    task_info: &DailyTaskInfo,
) -> Result<(), Box<dyn std::error::Error>> {
    event!(target: "security_api", Level::INFO, "call get_security_to_temp");

    let mut transaction = pool.begin().await?;

    let query_response_data = ResponseData {
        row_id: None,
        open_date: task_info.open_date.clone(),
        exec_code: Some("seecurity".to_string()),
        data_content: None,
    };

    let data_list = response_data::dao::read_all(&mut transaction, &query_response_data).await?;
    if data_list.0 > 0 {
        if let Some(data) = data_list.1.get(0) {
            if let Some(data_content) = &data.data_content {
                security_temp::service::insert_temp_data(pool, data_content).await?
            }
        }
    }

    Ok(())
}

pub async fn get_temp_to_task(
    pool: &sqlx::PgPool,
    task_info: &DailyTaskInfo,
) -> Result<(), Box<dyn std::error::Error>> {
    event!(target: "security_api", Level::INFO, "call get_temp_to_task");
    security_task::service::insert_task_data(pool, task_info).await?;
    Ok(())
}

pub async fn delete_temp(
    pool: &sqlx::PgPool,
    task_info: &DailyTaskInfo,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut transaction = pool.begin().await?;
    security_temp::dao::truncate(&mut transaction).await?;
    Ok(())
}

pub async fn get_task_run(
    pool: &sqlx::PgPool,
    task_info: &DailyTaskInfo,
) -> Result<(), Box<dyn std::error::Error>> {
    event!(target: "security_api", Level::INFO, "call get_task_run");
    security_task::service::get_all_task(pool, task_info).await?;
    Ok(())
}
