#![warn(clippy::all, clippy::pedantic)]

use std::env;

use log::{error, info};

fn main() {
    log4rs::init_file("log.yaml", Default::default()).unwrap();

    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let action_code = args[1].as_str();
        match action_code {
            "add_next_year" => match security_api::add_next_year() {
                Ok(_) => info!(target: "security_api",  "add_next_year Done"),
                Err(e) => {
                    error!(target: "security_api",  "add_next_year {}", &e);
                    panic!("add_next_year Error {}", &e)
                }
            },
            "add_daily_task" => match security_api::add_daily_task() {
                Ok(_) => info!(target: "security_api",  "add_daily_task Done"),
                Err(e) => {
                    error!(target: "security_api",  "add_daily_task {}", &e);
                    panic!("add_daily_task Error {}", &e)
                }
            },
            "run_daily_task" => match security_api::run_daily_task(false) {
                Ok(_) => info!(target: "security_api",  "run_daily_task Done"),
                Err(e) => {
                    error!(target: "security_api",  "run_daily_task {}", &e);
                    panic!("run_daily_task Error {}", &e)
                }
            },
            "run_price_task" => match security_api::run_price_task(false) {
                Ok(_) => info!(target: "security_api",  "run_price_task Done"),
                Err(e) => {
                    error!(target: "security_api", "run_price_task {}", &e);
                    panic!("run_price_task Error {}", &e)
                }
            },
            "daily_task" => {
                match security_api::add_daily_task() {
                    Ok(_) => info!(target: "security_api",  "add_daily_task Done"),
                    Err(e) => {
                        error!(target: "security_api",  "add_daily_task {}", &e);
                        panic!("add_daily_task Error {}", &e)
                    }
                };
                match security_api::run_daily_task(false) {
                    Ok(_) => info!(target: "security_api", "run_daily_task Done"),
                    Err(e) => {
                        error!(target: "security_api", "run_daily_task {}", &e);
                        panic!("run_daily_task Error {}", &e)
                    }
                };
                match security_api::run_price_task(false) {
                    Ok(_) => info!(target: "security_api",  "run_price_task Done"),
                    Err(e) => {
                        error!(target: "security_api",  "run_price_task {}", &e);
                        panic!("run_price_task Error {}", &e)
                    }
                };
            }
            "rerun_daily_task" => match security_api::run_daily_task(true) {
                Ok(_) => info!(target: "security_api",  "run_daily_task Done"),
                Err(e) => {
                    error!(target: "security_api",  "run_daily_task {}", &e);
                    panic!("run_daily_task Error {}", &e)
                }
            },
            "rerun_price_task" => match security_api::run_price_task(true) {
                Ok(_) => info!(target: "security_api",  "run_price_task Done"),
                Err(e) => {
                    error!(target: "security_api",  "run_price_task {}", &e);
                    panic!("run_price_task Error {}", &e)
                }
            },
            "redaily_task" => {
                match security_api::add_daily_task() {
                    Ok(_) => info!(target: "security_api",  "add_daily_task Done"),
                    Err(e) => {
                        error!(target: "security_api", "add_daily_task {}", &e);
                        panic!("add_daily_task Error {}", &e)
                    }
                };
                match security_api::run_daily_task(true) {
                    Ok(_) => info!(target: "security_api",  "run_daily_task Done"),
                    Err(e) => {
                        error!(target: "security_api", "run_daily_task {}", &e);
                        panic!("run_daily_task Error {}", &e)
                    }
                };
                match security_api::run_price_task(true) {
                    Ok(_) => info!(target: "security_api","run_price_task Done"),
                    Err(e) => {
                        error!(target: "security_api", "run_price_task {}", &e);
                        panic!("run_price_task Error {}", &e)
                    }
                };
            }
            _ => info!(target: "security_api", "{:?}", args[1]),
        }
    } else {
        match security_api::add_daily_task() {
            Ok(_) => info!(target: "security_api",  "add_daily_task Done"),
            Err(e) => {
                error!(target: "security_api", "add_daily_task {}", &e);
                panic!("add_daily_task Error {}", &e)
            }
        };
        match security_api::run_daily_task(true) {
            Ok(_) => info!(target: "security_api",  "run_daily_task Done"),
            Err(e) => {
                error!(target: "security_api", "run_daily_task {}", &e);
                panic!("run_daily_task Error {}", &e)
            }
        };
        match security_api::run_price_task(true) {
            Ok(_) => info!(target: "security_api","run_price_task Done"),
            Err(e) => {
                error!(target: "security_api", "run_price_task {}", &e);
                panic!("run_price_task Error {}", &e)
            }
        };
    }
}
