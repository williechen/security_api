use core::time;
use std::env;

use sqlx::postgres::PgPoolOptions;
use tracing::{event, Level};
use tracing_subscriber::fmt::format::FmtSpan;

#[tokio::main]
async fn main() {
    let log_filter =
        std::env::var("RUST_LOG").unwrap_or_else(|_| "my_api=debug,sqlx=debug".to_owned());

    let file_appender = tracing_appender::rolling::hourly("", "security_api.log");

    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    tracing_subscriber::fmt()
        .json()
        .with_env_filter(log_filter)
        .with_span_events(FmtSpan::CLOSE)
        .with_writer(non_blocking)
        .init();

    let db_pool = match PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(time::Duration::from_secs(4))
        .connect("postgres://willie:Gn220304@localhost:5432/my_security")
        .await
    {
        Ok(pool) => pool,
        Err(e) => panic!("Couldn't establish DB connection: {}", e),
    };

    sqlx::migrate!()
        .run(&db_pool)
        .await
        .expect("Cannot run migration");

    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let action_code = args[1].as_str();
        match action_code {
            "get_web_security" => security_api::get_security_all_code(&db_pool).await.unwrap(),
            "res_to_temp" => security_api::get_security_to_temp(&db_pool).await.unwrap(),
            "temp_to_task" => security_api::get_temp_to_task(&db_pool).await.unwrap(),
            "task_run" => security_api::get_task_run(&db_pool).await.unwrap(),
            _ => event!(target: "my_api", Level::INFO, "{:?}", args[1]),
        }
    }
}
