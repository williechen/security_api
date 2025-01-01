#![warn(clippy::all, clippy::pedantic)]

use chrono::{Datelike, Local};

mod calendar_data;
mod daily_task;
pub mod listen_flow;
pub mod repository;
mod response_data;
mod security_price;
mod security_task;
mod security_temp;
mod task_setting;
mod database_backup;

pub fn backup_insert() -> Result<(), Box<dyn std::error::Error>> {
    database_backup::DatabaseBackup.backup_insert("security_api", "security_api_insert_backup");
    database_backup::DatabaseBackup.backup_copy("security_api", "security_api_copy_backup");
    Ok(())
}

pub async fn add_init_year() -> Result<(), sqlx::Error> {
    calendar_data::service::init_calendar_data().await?;
    Ok(())
}

pub async fn add_next_year() -> Result<(), sqlx::Error> {
    let now = Local::now().date_naive();
    if 10 == now.month() && 1 == now.day() {
        calendar_data::service::insert_calendar_data(true).await?;
        calendar_data::service::insert_calendar_data(false).await?;
    } else {
        calendar_data::service::insert_calendar_data(false).await?;
    }
    Ok(())
}

pub async fn add_daily_task() -> Result<(), sqlx::Error> {
    daily_task::service::insert_task_data().await?;
    Ok(())
}

pub async fn run_daily_task(is_renew: bool) -> Result<(), sqlx::Error> {
    if is_renew {
        listen_flow::service::delete_flow_data("security").await;
    }
    daily_task::service::exec_daily_task().await?;
    Ok(())
}

pub async fn run_price_task(is_renew: bool) -> Result<(), Box<dyn std::error::Error>> {
    if is_renew {
        listen_flow::service::delete_flow_data("price").await;
    }
    daily_task::service::exec_price_task().await?;
    Ok(())
}
