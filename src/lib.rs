#![warn(clippy::all, clippy::pedantic)]

use std::fs;

use chrono::{Datelike, Local};

mod calendar_data;
mod daily_task;
mod database_backup;
pub mod listen_flow;
pub mod repository;
mod response_data;
mod security_price;
mod security_task;
mod security_temp;
mod task_setting;

pub fn backup_insert() -> Result<(), Box<dyn std::error::Error>> {
    let now_str = Local::now().format("%Y%m%d");

    let insert_backup = format!("security_api_insert_backup_{0}", &now_str);
    let copy_backup = format!("security_api_copy_backup_{0}", &now_str);

    let mut is_insert_backup = false;
    let mut is_copy_backup = false;

    let files = fs::read_dir("./")?;
    for file in files {
        let file = file?;

        if !is_insert_backup && file
            .file_name()
            .to_str()
            .map_or(false, |s| s.starts_with(&insert_backup)){
                is_insert_backup = true;
        }

        if !is_copy_backup && file
            .file_name()
            .to_str()
            .map_or(false, |s| s.starts_with(&copy_backup)){
                is_copy_backup = true;
        }
    }

    if !is_insert_backup {
        database_backup::DatabaseBackup.backup_insert("security_api", "security_api_insert_backup");
    }

    if !is_copy_backup {
        database_backup::DatabaseBackup.backup_copy("security_api", "security_api_copy_backup");
    }

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
