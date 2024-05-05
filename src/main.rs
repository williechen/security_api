use std::env;

use sqlx::postgres::PgPoolOptions;
use tracing::{event, Level};

#[tokio::main]
async fn main() {
    let log_filter =
        std::env::var("RUST_LOG").unwrap_or_else(|_| "my_api=info,sqlx=info".to_owned());

    let file_appender = tracing_appender::rolling::hourly("", "security_api.log");

    let (file_non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    tracing_subscriber::fmt()
        .json()
        .with_env_filter(log_filter)
        .with_writer(file_non_blocking)
        .init();

    let db_pool = match PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://willie:Gn220304@localhost:5432/my_securiOk(Ok(ty")
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
            "get_web_security" => match security_api::get_security_all_code(&db_pool).await {
                Ok(_) => event!(target: "my_api", Level::INFO, "get_web_security Done"),
                Err(e) => {
                    event!(target: "my_api", Level::ERROR, "{:?}", e);
                    panic!("get_web_security Error {}", e)
                }
            },
            "res_to_temp" => match security_api::get_security_to_temp(&db_pool).await {
                Ok(_) => event!(target: "my_api", Level::INFO, "res_to_temp Done"),
                Err(e) => {
                    event!(target: "my_api", Level::ERROR, "{:?}", e);
                    panic!("res_to_temp Error {}", e)
                }
            },
            "temp_to_task" => match security_api::get_temp_to_task(&db_pool).await {
                Ok(_) => event!(target: "my_api", Level::INFO, "temp_to_task Done"),
                Err(e) => {
                    event!(target: "my_api", Level::ERROR, "{:?}", e);
                    panic!("temp_to_task Error {}", e)
                }
            },
            "task_run" => match security_api::get_task_run(&db_pool).await {
                Ok(_) => event!(target: "my_api", Level::INFO, "task_run Done"),
                Err(e) => {
                    event!(target: "my_api", Level::ERROR, "{:?}", e);
                    panic!("task_run Error {}", e)
                }
            },
            _ => event!(target: "my_api", Level::INFO, "{:?}", args[1]),
        }
    }
}
