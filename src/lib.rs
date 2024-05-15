use chrono::Local;
use tracing::{event, Level};

use crate::response_data::{model::ResponseData, service};

mod calendar_data;
mod daily_task;
mod response_data;
mod security_price;
mod security_task;
mod security_temp;
mod task_setting;

pub async fn add_next_year(pool: &sqlx::PgPool) -> Result<(), Box<dyn std::error::Error>> {
    event!(target: "security_api", Level::INFO, "call add_next_year");
    calendar_data::service::init_calendar_data(pool).await?;
    Ok(())
}

pub async fn add_daily_task(pool: &sqlx::PgPool) -> Result<(), Box<dyn std::error::Error>> {
    event!(target: "security_api", Level::INFO, "call add_daily_task");
    let mut transaction = pool.begin().await?;
    Ok(())
}

pub async fn run_daily_task(pool: &sqlx::PgPool) -> Result<(), Box<dyn std::error::Error>> {
    event!(target: "security_api", Level::INFO, "call run_daily_task");

    Ok(())
}

pub async fn get_security_all_code(pool: &sqlx::PgPool) -> Result<(), Box<dyn std::error::Error>> {
    event!(target: "security_api", Level::INFO, "call get_security_all_code");

    let mut transaction = pool.begin().await?;

    let now = Local::now().naive_local();

    let query_response_data = ResponseData {
        row_id: None,
        version_code: Some(now.format("%Y%m%d").to_string()),
        exec_code: Some("seecurity".to_string()),
        data_content: None,
    };

    let data_list = response_data::dao::read_all(&mut transaction, &query_response_data).await?;
    if data_list.0 <= 0 {
        let content = service::get_web_security_data().await?;

        let response_data = ResponseData {
            row_id: None,
            version_code: Some(now.format("%Y%m%d").to_string()),
            exec_code: Some("seecurity".to_string()),
            data_content: Some(content),
        };

        match response_data::dao::create(&mut transaction, response_data).await {
            Ok(_) => transaction.commit().await?,
            Err(e) => {
                transaction.rollback().await?;
                event!(target: "security_api", Level::ERROR, "{:?}", e);
            }
        };
    }

    Ok(())
}

pub async fn get_security_to_temp(pool: &sqlx::PgPool) -> Result<(), Box<dyn std::error::Error>> {
    event!(target: "security_api", Level::INFO, "call get_security_to_temp");

    let mut transaction = pool.begin().await?;

    let now = Local::now().naive_local();

    let query_response_data = ResponseData {
        row_id: None,
        version_code: Some(now.format("%Y%m%d").to_string()),
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

pub async fn get_temp_to_task(pool: &sqlx::PgPool) -> Result<(), Box<dyn std::error::Error>> {
    event!(target: "security_api", Level::INFO, "call get_temp_to_task");
    //security_task::service::insert_task_data(pool).await?;
    Ok(())
}

pub async fn get_task_run(pool: &sqlx::PgPool) -> Result<(), Box<dyn std::error::Error>> {
    event!(target: "security_api", Level::INFO, "call get_task_run");
    //security_task::service::get_all_task(pool).await?;
    Ok(())
}
