#![warn(clippy::all, clippy::pedantic)]

use std::{error::Error, str::FromStr};

use std::ops::{Add, Div};

use bigdecimal::{BigDecimal, RoundingMode, Zero};
use regex::Regex;
use sqlx::PgConnection;
use tracing::{event, Level};

use crate::{
    daily_task::model::DailyTask,
    repository::Repository,
    response_data::model::{SecurityPriceTpex1, SecurityPriceTpex2, SecurityPriceTwse},
    security_price::dao,
};

use super::model::{ResposePrice, SecurityPrice};

pub async fn get_security_to_price(task: &DailyTask) -> Result<(), Box<dyn Error>> {
    event!(target: "security_api", Level::INFO, "call daily_task.get_security_to_price");

    let q_year = task.open_date_year.clone();
    let q_month = task.open_date_month.clone();
    let q_day = task.open_date_day.clone();

    let res_prices = dao::read_all_by_res(q_year, q_month, q_day).await;
    for price in res_prices {
        event!(target: "security_api", Level::DEBUG, "ResposePrice: {:?}", &price);

        let q_year = price.open_date_year.clone();
        let q_month = price.open_date_month.clone();
        let q_security_code = price.security_code.clone();

        let month_prices =
            dao::find_all(q_year.clone(), q_month.clone(), q_security_code.clone()).await;
        if month_prices.len() <= 0 {
            loop_data_res(price).await?;
        } else {
            let dao = Repository::new().await;
            let mut conn = dao.connection.begin().await?;

            match dao::remove(&mut conn, q_year, q_month, q_security_code).await {
                Ok(_) => {
                    conn.commit().await?;
                    loop_data_res(price).await?;
                }
                Err(_) => conn.rollback().await?,
            };
        }
    }

    Ok(())
}

async fn loop_data_res(data: ResposePrice) -> Result<(), Box<dyn Error>> {
    let market_type = data.market_type.clone();
    let data_content = data.data_content.clone();

    let dao = Repository::new().await;

    match market_type.as_str() {
        "上市" => match serde_json::from_str::<SecurityPriceTwse>(&data_content) {
            Ok(data_row) => {
                if data_row.data.is_some() {
                    let mut conn = dao.connection.begin().await?;
                    match loop_data_code(data_row.data.unwrap(), 0, 1, &mut conn, data.clone())
                        .await
                    {
                        Ok(_) => conn.commit().await?,
                        Err(_) => conn.rollback().await?,
                    }
                }
            }
            Err(e) => return Err(Box::new(e)),
        },
        "上櫃" => match serde_json::from_str::<SecurityPriceTpex1>(&data_content) {
            Ok(data_row) => {
                let mut conn = dao.connection.begin().await?;
                match loop_data_code(data_row.aa_data, 0, 6, &mut conn, data.clone()).await {
                    Ok(_) => conn.commit().await?,
                    Err(_) => conn.rollback().await?,
                }
            }
            Err(e) => return Err(Box::new(e)),
        },
        "興櫃" => match serde_json::from_str::<SecurityPriceTpex2>(&data_content) {
            Ok(data_row) => {
                let mut conn = dao.connection.begin().await?;
                match loop_data_code(data_row.aa_data, 0, 5, &mut conn, data.clone()).await {
                    Ok(_) => conn.commit().await?,
                    Err(_) => conn.rollback().await?,
                }
            }
            Err(e) => return Err(Box::new(e)),
        },
        _ => (),
    }
    Ok(())
}

async fn loop_data_code(
    rows: Vec<Vec<String>>,
    date_index: usize,
    price_index: usize,
    trax_conn: &mut PgConnection,
    data: ResposePrice,
) -> Result<(), sqlx::Error> {
    let re = Regex::new(r"[0-9.,]+").unwrap();
    for row in rows {
        if re.is_match(&row[price_index]) {
            let price_close = BigDecimal::from_str(&row[price_index].replace(",", "")).unwrap();

            let mut price_date = row[date_index].trim().replace("＊", "");
            if "月平均收盤價" != price_date {
                price_date = format!("{:0>10}", price_date);
            }

            if price_close > BigDecimal::zero() {
                let price = SecurityPrice {
                    row_id: String::new(),
                    security_code: data.security_code.clone(),
                    security_name: data.security_name.clone(),
                    price_date,
                    price_close,
                    price_avg: BigDecimal::zero(),
                    price_hight: BigDecimal::zero(),
                    price_hight_avg: BigDecimal::zero(),
                    price_lowest: BigDecimal::zero(),
                    price_lowest_avg: BigDecimal::zero(),
                    open_date_year: data.open_date_year.clone(),
                    open_date_month: data.open_date_month.clone(),
                    open_date_day: data.open_date_day.clone(),
                };
                dao::create(trax_conn, price).await?;
            }
        }
    }
    Ok(())
}

pub async fn get_calculator_to_price(task: &DailyTask) -> Result<(), sqlx::Error> {
    event!(target: "security_api", Level::INFO, "call daily_task.get_calculator_to_price");

    let q_year = task.open_date_year.clone();
    let q_month = task.open_date_month.clone();
    let q_day = task.open_date_day.clone();

    let res_prices = dao::find_all_by_date(q_year, q_month, q_day).await;
    for price in res_prices {
        event!(target: "security_api", Level::DEBUG, "SecurityPrice: {:?}", &price);
        loop_data_calculator(price).await?;
    }

    Ok(())
}

async fn loop_data_calculator(data: SecurityPrice) -> Result<(), sqlx::Error> {
    let q_year = data.open_date_year.clone();
    let q_month = data.open_date_month.clone();
    let q_day = data.open_date_day.clone();
    let q_open_date = format!("{0}{1}{2}", q_year, q_month, q_day);
    let q_security_code = data.security_code.clone();
    let q_price_date = data.price_date.clone();

    let mut sum_count = BigDecimal::from(0);
    let mut sum_price = BigDecimal::from(0);

    let res_prices = dao::find_all_by_code(
        q_open_date.clone(),
        q_price_date.clone(),
        q_security_code.clone(),
    )
    .await;
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

    let res_prices = dao::find_all_by_code(
        q_open_date.clone(),
        q_price_date.clone(),
        q_security_code.clone(),
    )
    .await;
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
    dao::modify(new_price).await?;

    Ok(())
}

fn to_big_decimal_round(val: bigdecimal::BigDecimal) -> bigdecimal::BigDecimal {
    val.with_scale_round(4, RoundingMode::Up)
}
