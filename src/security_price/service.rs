#![warn(clippy::all, clippy::pedantic)]

use std::ops::{Add, Div};
use std::str::FromStr;

use bigdecimal::{BigDecimal, RoundingMode, Zero};
use chrono::Local;
use diesel::Connection;
use log::{debug, info};
use regex::Regex;

use crate::daily_task::model::DailyTask;
use crate::{repository::Repository, security_price::dao};

use super::model::{
    NewSecurityPrice, ResposePrice, SecurityPrice, SecurityPriceTpex1, SecurityPriceTpex2,
    SecurityPriceTwse,
};

pub fn get_security_to_price(task: &DailyTask) -> Result<(), Box<dyn std::error::Error>> {
    info!(target: "security_api", "call daily_task.get_security_to_price");

    let q_year = task.open_date_year.clone();
    let q_month = task.open_date_month.clone();
    let q_day = task.open_date_day.clone();

    let res_prices = dao::read_all_by_res(q_year, q_month, q_day);
    for price in res_prices {
        debug!(target: "security_api", "ResposePrice: {:?}", &price);
        loop_data_res(price)?;
    }

    Ok(())
}

fn loop_data_res(data: ResposePrice) -> Result<(), Box<dyn std::error::Error>> {
    let re = Regex::new(r"[0-9.]+").unwrap();

    let market_type = data.market_type.clone();
    let data_content = data.data_content.clone();

    let dao = Repository::new();
    let mut conn = dao.connection;

    conn.transaction(|trax_conn| {
        match market_type.as_str() {
            "上市" => {
                let obj_data = serde_json::from_str::<SecurityPriceTwse>(&data_content).unwrap();
                for row in obj_data.data {
                    if re.is_match(&row[1]) {
                        let price_code = BigDecimal::from_str(&row[1]).unwrap();
                        if price_code > BigDecimal::default() {
                            let price = NewSecurityPrice {
                                security_code: data.security_code.clone(),
                                security_name: data.security_name.clone(),
                                price_date: row[0].clone().trim().to_string(),
                                price_close: price_code,
                                price_avg: BigDecimal::zero(),
                                price_hight: BigDecimal::zero(),
                                price_hight_avg: BigDecimal::zero(),
                                price_lowest: BigDecimal::zero(),
                                price_lowest_avg: BigDecimal::zero(),
                                open_date_year: data.open_date_year.clone(),
                                open_date_month: data.open_date_month.clone(),
                                open_date_day: data.open_date_day.clone(),
                                created_date: Local::now().naive_local(),
                                updated_date: Local::now().naive_local(),
                            };
                            dao::create(trax_conn, price)?;
                        }
                    }
                }
            }
            "上櫃" => {
                let obj_data = serde_json::from_str::<SecurityPriceTpex1>(&data_content).unwrap();
                for row in obj_data.aa_data {
                    if re.is_match(&row[6]) {
                        let price_code = BigDecimal::from_str(&row[6]).unwrap();
                        if price_code > BigDecimal::default() {
                            let price = NewSecurityPrice {
                                security_code: data.security_code.clone(),
                                security_name: data.security_name.clone(),
                                price_date: row[0].clone().trim().to_string(),
                                price_close: BigDecimal::from_str(&row[6]).unwrap(),
                                price_avg: BigDecimal::zero(),
                                price_hight: BigDecimal::zero(),
                                price_hight_avg: BigDecimal::zero(),
                                price_lowest: BigDecimal::zero(),
                                price_lowest_avg: BigDecimal::zero(),
                                open_date_year: data.open_date_year.clone(),
                                open_date_month: data.open_date_month.clone(),
                                open_date_day: data.open_date_day.clone(),
                                created_date: Local::now().naive_local(),
                                updated_date: Local::now().naive_local(),
                            };
                            dao::create(trax_conn, price)?;
                        }
                    }
                }
            }
            "興櫃" => {
                let obj_data = serde_json::from_str::<SecurityPriceTpex2>(&data_content).unwrap();
                for row in obj_data.aa_data {
                    if re.is_match(&row[5]) {
                        let price_code = BigDecimal::from_str(&row[5]).unwrap();
                        if price_code > BigDecimal::default() {
                            let price = NewSecurityPrice {
                                security_code: data.security_code.clone(),
                                security_name: data.security_name.clone(),
                                price_date: row[0].clone().trim().to_string(),
                                price_close: BigDecimal::from_str(&row[5]).unwrap(),
                                price_avg: BigDecimal::zero(),
                                price_hight: BigDecimal::zero(),
                                price_hight_avg: BigDecimal::zero(),
                                price_lowest: BigDecimal::zero(),
                                price_lowest_avg: BigDecimal::zero(),
                                open_date_year: data.open_date_year.clone(),
                                open_date_month: data.open_date_month.clone(),
                                open_date_day: data.open_date_day.clone(),
                                created_date: Local::now().naive_local(),
                                updated_date: Local::now().naive_local(),
                            };
                            dao::create(trax_conn, price)?;
                        }
                    }
                }
            }
            _ => (),
        }
        Ok::<_, diesel::result::Error>(())
    })?;

    Ok(())
}

pub fn get_calculator_to_price(task: &DailyTask) -> Result<(), Box<dyn std::error::Error>> {
    info!(target: "security_api", "call daily_task.get_calculator_to_price");

    let q_year = task.open_date_year.clone();
    let q_month = task.open_date_month.clone();
    let q_day = task.open_date_day.clone();

    let res_prices = dao::find_all_by_date(q_year, q_month, q_day);
    for price in res_prices {
        debug!(target: "security_api", "SecurityPrice: {:?}", &price);
        loop_data_calculator(price)?;
    }

    Ok(())
}

fn loop_data_calculator(data: SecurityPrice) -> Result<(), Box<dyn std::error::Error>> {
    let q_security_code = data.security_code.clone();
    let q_price_date = data.price_date.clone();

    let mut sum_count = BigDecimal::from(0);
    let mut sum_price = BigDecimal::from(0);

    let res_prices = dao::find_all_by_code(q_price_date.clone(), q_security_code.clone());
    for price in res_prices {
        sum_count = sum_count.add(BigDecimal::from(1));
        sum_price = sum_price.add(price.price_close.clone());
    }

    // 總平均數
    let price_avg = to_big_decimal_round(sum_price.div(sum_count));

    let mut max_price_closes = Vec::new();
    let mut sum_max_count = BigDecimal::from(0);
    let mut sum_max_price = BigDecimal::from(0);

    let mut min_price_closes = Vec::new();
    let mut sum_min_count = BigDecimal::from(0);
    let mut sum_min_price = BigDecimal::from(0);

    let res_prices = dao::find_all_by_code(q_price_date.clone(), q_security_code.clone());
    for price in res_prices {
        let price_close = to_big_decimal_round(price.price_close.clone());
        if price_close > price_avg {
            max_price_closes.push(price_close.clone());
            sum_max_count = sum_max_count.add(BigDecimal::from(1));
            sum_max_price = sum_max_price.add(price_close.clone());
        } else if price_close < price_avg {
            min_price_closes.push(price_close.clone());
            sum_min_count = sum_min_count.add(BigDecimal::from(1));
            sum_min_price = sum_min_price.add(price_close.clone());
        }
    }

    let mut new_price = data.clone();
    new_price.price_avg = price_avg;
    new_price.updated_date = Local::now().naive_local();

    if max_price_closes.len() > 0 {
        let max_price = max_price_closes.iter().max();
        new_price.price_hight = to_big_decimal_round(max_price.unwrap().clone());
        new_price.price_hight_avg = to_big_decimal_round(sum_max_price.div(sum_max_count));
    } else {
        new_price.price_hight = to_big_decimal_round(data.price_close.clone());
        new_price.price_hight_avg = to_big_decimal_round(data.price_close.clone());
    }

    if min_price_closes.len() > 0 {
        let min_price = min_price_closes.iter().min();
        new_price.price_lowest = to_big_decimal_round(min_price.unwrap().clone());
        new_price.price_lowest_avg = to_big_decimal_round(sum_min_price.div(sum_min_count));
    } else {
        new_price.price_lowest = to_big_decimal_round(data.price_close.clone());
        new_price.price_lowest_avg = to_big_decimal_round(data.price_close.clone());
    }
    dao::modify(new_price)?;

    Ok(())
}

fn to_big_decimal_round(val: bigdecimal::BigDecimal) -> bigdecimal::BigDecimal {
    val.with_scale_round(4, RoundingMode::Up)
}
