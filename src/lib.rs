use chrono::{Datelike, Local};
use tracing::{event, Level};

mod calendar_data;
mod daily_task;
mod response_data;
mod security_price;
mod security_task;
mod security_temp;
mod task_setting;

pub async fn add_next_year(db_url: &str) -> Result<(), Box<dyn std::error::Error>> {
    event!(target: "security_api", Level::INFO, "call add_next_year");

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
    event!(target: "security_api", Level::INFO, "call add_daily_task");
    daily_task::service::insert_task_data(db_url).await?;
    Ok(())
}

pub async fn run_daily_task(db_url: &str) -> Result<(), Box<dyn std::error::Error>> {
    event!(target: "security_api", Level::INFO, "call run_daily_task");
    daily_task::service::exec_daily_task(db_url).await?;
    Ok(())
}
