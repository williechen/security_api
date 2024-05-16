use tracing::{event, Level};

mod calendar_data;
mod daily_task;
mod response_data;
mod security_price;
mod security_task;
mod security_temp;
mod task_setting;

pub async fn add_next_year(pool: &sqlx::PgPool) -> Result<(), Box<dyn std::error::Error>> {
    event!(target: "security_api", Level::INFO, "call add_next_year");
    calendar_data::service::insert_calendar_data(pool).await?;
    Ok(())
}

pub async fn add_daily_task(pool: &sqlx::PgPool) -> Result<(), Box<dyn std::error::Error>> {
    event!(target: "security_api", Level::INFO, "call add_daily_task");
    daily_task::service::insert_task_data(pool).await?;
    Ok(())
}

pub async fn run_daily_task(pool: &sqlx::PgPool) -> Result<(), Box<dyn std::error::Error>> {
    event!(target: "security_api", Level::INFO, "call run_daily_task");
    daily_task::service::exec_daily_task(pool).await?;
    Ok(())
}
