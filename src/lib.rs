use chrono::Local;
use tracing::{event, Level};

use crate::response_data::{model::ResponseData, service};

mod response_data;
mod security_task;
mod security_temp;

pub async fn get_security_all_code(pool: &sqlx::PgPool) -> Result<(), Box<dyn std::error::Error>> {
    event!(target: "my_api", Level::DEBUG, "call get_security_all_code");

    let mut transaction = pool.begin().await?;

    let now = Local::now().naive_local();

    let query_response_data = ResponseData {
        row_id: None,
        data_content: None,
        data_code: Some("seecurity".to_string()),
        read_date: Some(now.format("%Y%m%d").to_string()),
        created_date: Some(now),
        updated_date: Some(now),
    };

    let data_list = response_data::dao::read_all(&mut transaction, &query_response_data).await?;
    if data_list.0 <= 0 {
        let content = service::get_web_security_data().await?;

        let response_data = ResponseData {
            row_id: None,
            data_content: Some(content),
            data_code: Some("seecurity".to_string()),
            read_date: Some(now.format("%Y%m%d").to_string()),
            created_date: Some(now),
            updated_date: Some(now),
        };

        match response_data::dao::create(&mut transaction, response_data).await {
            Ok(_) => transaction.commit().await?,
            Err(e) => {
                transaction.rollback().await?;
                event!(target: "my_api", Level::ERROR, "{:?}", e);
            }
        };
    }

    Ok(())
}

pub async fn get_security_to_temp(pool: &sqlx::PgPool) -> Result<(), Box<dyn std::error::Error>> {
    event!(target: "my_api", Level::DEBUG, "call get_security_to_temp");

    let mut transaction = pool.begin().await?;

    let now = Local::now().naive_local();

    let query_response_data = ResponseData {
        row_id: None,
        data_content: None,
        data_code: Some("seecurity".to_string()),
        read_date: Some(now.format("%Y%m%d").to_string()),
        created_date: Some(now),
        updated_date: Some(now),
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
    event!(target: "my_api", Level::DEBUG, "call get_temp_to_task");
    security_task::service::insert_task_data(pool).await?;
    Ok(())
}

pub async fn get_task_run(pool: &sqlx::PgPool) -> Result<(), Box<dyn std::error::Error>> {
    event!(target: "my_api", Level::DEBUG, "call get_task_run");
    security_task::service::get_all_task(pool).await?;
    Ok(())
}
