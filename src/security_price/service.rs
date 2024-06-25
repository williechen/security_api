#![warn(clippy::all, clippy::pedantic)]

use std::ops::{Add, Div};
use std::str::FromStr;

use bigdecimal::{BigDecimal, RoundingMode, Zero};
use regex::Regex;
use sqlx::PgConnection;
use tracing::{event, Level};

use crate::{daily_task::model::DailyTaskInfo, repository::Repository, security_price::dao};

use super::model::{
    ResposePrice, SecurityPrice, SecurityPriceTpex1, SecurityPriceTpex2, SecurityPriceTwse,
};

pub async fn get_security_to_price(
    db_url: &str,
    task_info: &DailyTaskInfo,
) -> Result<(), Box<dyn std::error::Error>> {
    event!(target: "security_api", Level::INFO, "call daily_task.get_security_to_price");
    let pool = Repository::new(db_url).await;
    let mut transaction = pool.connection.acquire().await?;

    let open_date = task_info.open_date.clone().unwrap();

    let res_prices = dao::read_all_by_res(
        &mut *transaction,
        &open_date[0..4],
        &open_date[4..6],
        &open_date[6..8],
    )
    .await?;
    for price in res_prices {
        event!(target: "security_api", Level::DEBUG, "ResposePrice: {:?}", &price);
        let pool = Repository::new(db_url).await;
        let mut transaction = pool.connection.begin().await?;
        match loop_data_res(&mut *transaction, price).await {
            Ok(_) => transaction.commit().await?,
            Err(_) => transaction.rollback().await?,
        };
    }

    Ok(())
}

async fn loop_data_res(
    transaction: &mut PgConnection,
    data: ResposePrice,
) -> Result<(), Box<dyn std::error::Error>> {
    let re = Regex::new(r"[0-9-.]+").unwrap();

    let market_type = data.market_type.clone().unwrap();
    let data_content = data.data_content.clone().unwrap();

    match market_type.as_str() {
        "上市" => {
            let obj_data = serde_json::from_str::<SecurityPriceTwse>(&data_content)?;
            for row in obj_data.data {
                if re.is_match(&row[1]) {
                    let price_code = BigDecimal::from_str(&row[1])?;
                    if price_code > BigDecimal::default() {
                        let price = SecurityPrice {
                            row_id: None,
                            open_date: data.open_date.clone(),
                            security_code: data.security_code.clone(),
                            security_name: data.security_name.clone(),
                            price_date: Some(row[0].clone()),
                            price_close: Some(price_code),
                            price_avg: Some(BigDecimal::zero()),
                            price_hight: Some(BigDecimal::zero()),
                            price_hight_avg: Some(BigDecimal::zero()),
                            price_lowest: Some(BigDecimal::zero()),
                            price_lowest_avg: Some(BigDecimal::zero()),
                        };
                        dao::create(transaction, price).await?;
                    }
                }
            }
        }
        "上櫃" => {
            let obj_data = serde_json::from_str::<SecurityPriceTpex1>(&data_content)?;
            for row in obj_data.aa_data {
                if re.is_match(&row[6]) {
                    let price_code = BigDecimal::from_str(&row[6])?;
                    if price_code > BigDecimal::default() {
                        let price = SecurityPrice {
                            row_id: None,
                            open_date: data.open_date.clone(),
                            security_code: data.security_code.clone(),
                            security_name: data.security_name.clone(),
                            price_date: Some(row[0].clone()),
                            price_close: Some(BigDecimal::from_str(&row[6])?),
                            price_avg: Some(BigDecimal::default()),
                            price_hight: Some(BigDecimal::default()),
                            price_hight_avg: Some(BigDecimal::default()),
                            price_lowest: Some(BigDecimal::default()),
                            price_lowest_avg: Some(BigDecimal::default()),
                        };
                        dao::create(transaction, price).await?;
                    }
                }
            }
        }
        "興櫃" => {
            let obj_data = serde_json::from_str::<SecurityPriceTpex2>(&data_content)?;
            for row in obj_data.aa_data {
                if re.is_match(&row[5]) {
                    let price_code = BigDecimal::from_str(&row[5])?;
                    if price_code > BigDecimal::default() {
                        let price = SecurityPrice {
                            row_id: None,
                            open_date: data.open_date.clone(),
                            security_code: data.security_code.clone(),
                            security_name: data.security_name.clone(),
                            price_date: Some(row[0].clone()),
                            price_close: Some(BigDecimal::from_str(&row[5])?),
                            price_avg: Some(BigDecimal::default()),
                            price_hight: Some(BigDecimal::default()),
                            price_hight_avg: Some(BigDecimal::default()),
                            price_lowest: Some(BigDecimal::default()),
                            price_lowest_avg: Some(BigDecimal::default()),
                        };
                        dao::create(transaction, price).await?;
                    }
                }
            }
        }
        _ => (),
    }

    Ok(())
}

pub async fn get_calculator_to_price(
    db_url: &str,
    task_info: &DailyTaskInfo,
) -> Result<(), Box<dyn std::error::Error>> {
    event!(target: "security_api", Level::INFO, "call daily_task.get_calculator_to_price");
    let pool = Repository::new(db_url).await;
    let mut transaction = pool.connection.acquire().await?;

    let open_date = task_info.open_date.clone().unwrap();

    let res_prices = dao::read_all_by_date(&mut *transaction, &open_date).await?;
    for price in res_prices {
        event!(target: "security_api", Level::DEBUG, "SecurityPrice: {:?}", &price);
        let pool = Repository::new(db_url).await;
        let mut transaction = pool.connection.begin().await?;
        match loop_data_calculator(&mut *transaction, &open_date, price).await {
            Ok(_) => transaction.commit().await?,
            Err(_) => transaction.rollback().await?,
        };
    }

    Ok(())
}

async fn loop_data_calculator(
    transaction: &mut PgConnection,
    open_date: &str,
    data: SecurityPrice,
) -> Result<(), Box<dyn std::error::Error>> {
    let security_code = data.security_code.clone().unwrap();

    let mut sum_count = BigDecimal::from(0);
    let mut sum_price = BigDecimal::from(0);

    let res_prices = dao::read_all_by_code(&mut *transaction, &open_date, &security_code).await?;
    for price in res_prices {
        sum_count = sum_count.add(BigDecimal::from(1));
        sum_price = sum_price.add(price.price_close.clone().unwrap());
    }

    // 總平均數
    let price_avg = to_big_decimal_round(sum_price.div(sum_count));

    let mut max_price_closes = Vec::new();
    let mut sum_max_count = BigDecimal::from(0);
    let mut sum_max_price = BigDecimal::from(0);

    let mut min_price_closes = Vec::new();
    let mut sum_min_count = bigdecimal::BigDecimal::from(0);
    let mut sum_min_price = BigDecimal::from(0);

    let res_prices = dao::read_all_by_code(&mut *transaction, &open_date, &security_code).await?;
    for price in res_prices {
        let price_close = to_big_decimal_round(price.price_close.clone().unwrap());
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
    new_price.price_avg = Some(price_avg);

    if max_price_closes.len() > 0 {
        let max_price = max_price_closes.iter().max();
        new_price.price_hight = Some(to_big_decimal_round(max_price.unwrap().clone()));
        new_price.price_hight_avg = Some(to_big_decimal_round(sum_max_price.div(sum_max_count)));
    } else {
        new_price.price_hight = Some(to_big_decimal_round(data.price_close.clone().unwrap()));
        new_price.price_hight_avg = Some(to_big_decimal_round(data.price_close.clone().unwrap()));
    }

    if min_price_closes.len() > 0 {
        let min_price = min_price_closes.iter().min();
        new_price.price_lowest = Some(to_big_decimal_round(min_price.unwrap().clone()));
        new_price.price_lowest_avg = Some(to_big_decimal_round(sum_min_price.div(sum_min_count)));
    } else {
        new_price.price_lowest = Some(to_big_decimal_round(data.price_close.clone().unwrap()));
        new_price.price_lowest_avg = Some(to_big_decimal_round(data.price_close.clone().unwrap()));
    }

    dao::update(transaction, new_price).await?;

    Ok(())
}

fn to_big_decimal_round(val: bigdecimal::BigDecimal) -> bigdecimal::BigDecimal {
    val.with_scale_round(4, RoundingMode::Up)
}
