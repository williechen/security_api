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

pub async fn add_next_year(db_url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let now = Local::now().date_naive();
    if 10 == now.month() && 1 == now.day() {
        calendar_data::service::insert_calendar_data(db_url, true).await?;
        calendar_data::service::insert_calendar_data(db_url, false).await?;
    } else {
        calendar_data::service::insert_calendar_data(db_url, false).await?;
    }
    Ok(())
}

pub async fn add_daily_task(db_url: &str) -> Result<(), Box<dyn std::error::Error>> {
    daily_task::service::insert_task_data(db_url).await?;
    Ok(())
}

pub async fn run_daily_task(db_url: &str, is_renew: bool) -> Result<(), Box<dyn std::error::Error>> {
    if is_renew {
    listen_flow::service::delete_flow_data(db_url, "security").await;
    }
    daily_task::service::exec_daily_task(db_url).await?;
    Ok(())
}

pub async fn run_price_task(db_url: &str, is_renew: bool) -> Result<(), Box<dyn std::error::Error>> {
    if is_renew {
    listen_flow::service::delete_flow_data(db_url, "price").await;
    }
    daily_task::service::exec_price_task(db_url).await?;
    Ok(())
}
