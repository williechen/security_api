use std::env;

use sqlx::postgres::PgPoolOptions;
use tracing::{event, Level};

#[tokio::main]
async fn main() {
    let log_filter =
        std::env::var("RUST_LOG").unwrap_or_else(|_| "security_api=debug,sqlx=info".to_owned());

    let file_appender = tracing_appender::rolling::hourly("logs", "security_api.log");

    let (file_non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    tracing_subscriber::fmt()
        .json()
        .with_env_filter(log_filter)
        .with_writer(file_non_blocking)
        .init();

    let db_pool = match PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://willie:Gn220304@localhost:5432/security_api")
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
            "add_next_year" => match security_api::add_next_year(&db_pool).await {
                Ok(_) => event!(target: "security_api", Level::INFO, "add_next_year Done"),
                Err(e) => {
                    event!(target: "security_api", Level::ERROR, "{:?}", e);
                    panic!("add_next_year Error {}", e)
                }
            },
            "add_daily_task" => match security_api::add_daily_task(&db_pool).await {
                Ok(_) => event!(target: "security_api", Level::INFO, "add_daily_task Done"),
                Err(e) => {
                    event!(target: "security_api", Level::ERROR, "{:?}", e);
                    panic!("add_daily_task Error {}", e)
                }
            },
            "run_daily_task" => match security_api::run_daily_task(&db_pool).await {
                Ok(_) => event!(target: "security_api", Level::INFO, "run_daily_task Done"),
                Err(e) => {
                    event!(target: "security_api", Level::ERROR, "{:?}", e);
                    panic!("run_daily_task Error {}", e)
                }
            },
            _ => event!(target: "security_api", Level::INFO, "{:?}", args[1]),
        }
    }
}
