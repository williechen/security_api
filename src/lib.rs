#![warn(clippy::all, clippy::pedantic)]

use chrono::{Datelike, Local};
pub mod repository;
pub mod schema;

mod calendar_data;
mod daily_task;
mod listen_flow;
mod response_data;
mod security_task;
mod security_temp;

pub fn add_next_year() -> Result<(), Box<dyn std::error::Error>> {
    let now = Local::now().date_naive();
    if 10 == now.month() && 1 == now.day() {
        calendar_data::service::insert_calendar_data(true)?;
        calendar_data::service::insert_calendar_data(false)?;
    } else {
        calendar_data::service::insert_calendar_data(false)?;
    }
    Ok(())
}

pub fn add_daily_task() -> Result<(), Box<dyn std::error::Error>> {
    daily_task::service::insert_task_data()?;
    Ok(())
}

pub fn run_daily_task(is_renew: bool) -> Result<(), Box<dyn std::error::Error>> {
    if is_renew {
        listen_flow::service::delete_flow_data("security");
    }
    daily_task::service::exec_daily_task()?;
    Ok(())
}

pub fn run_price_task(is_renew: bool) -> Result<(), Box<dyn std::error::Error>> {
    /*if is_renew {
        listen_flow::service::delete_flow_data(db_url, "price").await;
    }
    daily_task::service::exec_price_task(db_url).await?;*/
    Ok(())
}
