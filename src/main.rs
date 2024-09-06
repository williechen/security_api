#![warn(clippy::all, clippy::pedantic)]

use std::env;

use tracing::{event, Level};

#[tokio::main]
async fn main() {
    let log_filter =
        std::env::var("RUST_LOG").unwrap_or_else(|_| "security_api=info,sqlx=error".to_owned());

    // console log
    let (_console_non_blocking, _guard) = tracing_appender::non_blocking(std::io::stdout());
    // log file
    let file_appender = tracing_appender::rolling::hourly("logs", "security_api.log");
    let (file_non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    tracing_subscriber::fmt()
        .json()
        .with_env_filter(log_filter)
        .with_writer(file_non_blocking)
        .init();

    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let action_code = args[1].as_str();
        match action_code {
            "add_init_year" => match security_api::add_init_year().await {
                Ok(_) => event!(target: "security_api", Level::INFO, "add_init_year Done"),
                Err(e) => {
                    event!(target: "security_api", Level::ERROR, "add_init_year {}", &e);
                    panic!("add_init_year Error {}", &e)
                }
            },
            "add_next_year" => match security_api::add_next_year().await {
                Ok(_) => event!(target: "security_api", Level::INFO, "add_next_year Done"),
                Err(e) => {
                    event!(target: "security_api",  Level::ERROR,"add_next_year {}", &e);
                    panic!("add_next_year Error {}", &e)
                }
            },
            "add_daily_task" => match security_api::add_daily_task().await {
                Ok(_) => event!(target: "security_api", Level::INFO, "add_daily_task Done"),
                Err(e) => {
                    event!(target: "security_api",  Level::ERROR,"add_daily_task {}", &e);
                    panic!("add_daily_task Error {}", &e)
                }
            },
            "run_daily_task" => match security_api::run_daily_task(false).await {
                Ok(_) => event!(target: "security_api", Level::INFO, "run_daily_task Done"),
                Err(e) => {
                    event!(target: "security_api", Level::ERROR, "run_daily_task {}", &e);
                    panic!("run_daily_task Error {}", &e)
                }
            },
            "run_price_task" => match security_api::run_price_task(false).await {
                Ok(_) => event!(target: "security_api", Level::INFO, "run_price_task Done"),
                Err(e) => {
                    event!(target: "security_api", Level::ERROR,"run_price_task {}", &e);
                    panic!("run_price_task Error {}", &e)
                }
            },
            "daily_task" => {
                match security_api::add_daily_task().await {
                    Ok(_) => event!(target: "security_api",Level::INFO,  "add_daily_task Done"),
                    Err(e) => {
                        event!(target: "security_api", Level::ERROR, "add_daily_task {}", &e);
                        panic!("add_daily_task Error {}", &e)
                    }
                };
                match security_api::run_daily_task(false).await {
                    Ok(_) => event!(target: "security_api", Level::INFO, "run_daily_task Done"),
                    Err(e) => {
                        event!(target: "security_api", Level::ERROR,"run_daily_task {}", &e);
                        panic!("run_daily_task Error {}", &e)
                    }
                };
                match security_api::run_price_task(false).await {
                    Ok(_) => event!(target: "security_api", Level::INFO, "run_price_task Done"),
                    Err(e) => {
                        event!(target: "security_api", Level::ERROR, "run_price_task {}", &e);
                        panic!("run_price_task Error {}", &e)
                    }
                };
            }
            "rerun_daily_task" => match security_api::run_daily_task(true).await {
                Ok(_) => event!(target: "security_api", Level::INFO, "run_daily_task Done"),
                Err(e) => {
                    event!(target: "security_api", Level::ERROR, "run_daily_task {}", &e);
                    panic!("run_daily_task Error {}", &e)
                }
            },
            "rerun_price_task" => match security_api::run_price_task(true).await {
                Ok(_) => event!(target: "security_api", Level::INFO, "run_price_task Done"),
                Err(e) => {
                    event!(target: "security_api", Level::ERROR, "run_price_task {}", &e);
                    panic!("run_price_task Error {}", &e)
                }
            },
            _ => event!(target: "security_api", Level::INFO, "{:?}", args[1]),
        }
    } else {
        match security_api::add_daily_task().await {
            Ok(_) => event!(target: "security_api", Level::INFO, "add_daily_task Done"),
            Err(e) => {
                event!(target: "security_api", Level::ERROR, "add_daily_task {}", &e);
                panic!("add_daily_task Error {}", &e)
            }
        };
        match security_api::run_daily_task(true).await {
            Ok(_) => event!(target: "security_api", Level::INFO, "run_daily_task Done"),
            Err(e) => {
                event!(target: "security_api", Level::ERROR, "run_daily_task {}", &e);
                panic!("run_daily_task Error {}", &e)
            }
        };
        match security_api::run_price_task(true).await {
            Ok(_) => event!(target: "security_api", Level::INFO, "run_price_task Done"),
            Err(e) => {
                event!(target: "security_api", Level::ERROR, "run_price_task {}", &e);
                panic!("run_price_task Error {}", &e)
            }
        };
    }
}
