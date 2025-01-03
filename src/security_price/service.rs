#![warn(clippy::all, clippy::pedantic)]

use std::str::FromStr;

use std::ops::{Add, Div};

use bigdecimal::{BigDecimal, RoundingMode, Zero};
use sqlx::PgConnection;
use tracing::{event, Level};

use crate::response_data::model::MonthlyPrice;
use crate::{daily_task::model::DailyTask, repository::Repository, security_price::dao};

use super::model::{ResposePrice, SecurityPrice};

pub async fn get_security_to_price(task: &DailyTask) -> Result<(), sqlx::Error> {
    event!(target: "security_api", Level::DEBUG, "call daily_task.get_security_to_price");

    let q_year = task.open_date_year.clone();
    let q_month = task.open_date_month.clone();

    let res_prices = dao::find_all_by_res(q_year, q_month).await;
    for price in res_prices {
        event!(target: "security_api", Level::DEBUG, "ResposePrice: {:?}", &price);

        let q_year = price.open_date_year.clone();
        let q_month = price.open_date_month.clone();
        let q_security_code = price.security_code.clone();

        let month_prices =
            dao::find_all(q_year.clone(), q_month.clone(), q_security_code.clone()).await;
        if month_prices.len() <= 0 {
            loop_data_res(price, Vec::new()).await?;
        } else {
            let price_dates: Vec<(String, BigDecimal)> = month_prices.iter().map(|x| (x.price_date.clone(), x.price_close.clone())).collect();
            loop_data_res(price, price_dates).await?;
        }
    }

    Ok(())
}

async fn loop_data_res(data: ResposePrice, price_dates: Vec<(String, BigDecimal)>) -> Result<(), sqlx::Error> {
    let data_content = data.data_content.clone();

    let dao = Repository::new().await;
    let conn = dao.connection;

    match serde_json::from_str::<MonthlyPrice>(&data_content) {
        Ok(data_row) => {
            if !data_row.data.is_empty() {
                for row in data_row.data {
                    let mut trax_conn = conn.begin().await?;

                    let price_date = row[0].trim().to_string();
                    let price_close = BigDecimal::from_str(&row[1]).unwrap();

                    let new_price_date = format!("{:0>10}", price_date);
                    if price_dates.contains(&(new_price_date, price_close.clone())) {
                        continue;
                    }

                    match loop_data_price(&mut trax_conn, price_date, price_close, data.clone())
                        .await
                    {
                        Ok(_) => {
                            trax_conn.commit().await?;
                        }
                        Err(e) => {
                            trax_conn.rollback().await?;
                            return Err(e.into());
                        }
                    }
                }
            }
        }
        Err(e) => return Err(sqlx::Error::Decode(Box::new(e))),
    }
    Ok(())
}

async fn loop_data_price(
    trax_conn: &mut PgConnection,
    price_date: String,
    price_close: BigDecimal,
    data: ResposePrice,
) -> Result<(), sqlx::Error> {
    

    if price_close > BigDecimal::zero() {
        let price = SecurityPrice {
            security_code: data.security_code.clone(),
            security_name: data.security_name.clone(),
            price_date: price_date,
            price_close: price_close,
            price_avg: BigDecimal::zero(),
            price_hight: BigDecimal::zero(),
            price_hight_avg: BigDecimal::zero(),
            price_lowest: BigDecimal::zero(),
            price_lowest_avg: BigDecimal::zero(),
            open_date_year: data.open_date_year.clone(),
            open_date_month: data.open_date_month.clone(),
            open_date_day: data.open_date_day.clone(),
            row_id: "".to_string(),
        };
        dao::create(trax_conn, price).await?;
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

    let resp_prices = dao::find_all_by_code(
        q_open_date.clone(),
        q_price_date.clone(),
        q_security_code.clone(),
    )
    .await;

    let price_avg =
        get_calculator_avg(&resp_prices.iter().map(|x| x.price_close.clone()).collect());

    let (price_max, price_avg_max) = get_calculator_max_avg(&price_avg, &resp_prices);
    let (price_min, price_avg_min) = get_calculator_min_avg(&price_avg, &resp_prices);

    let new_price = get_security_price(
        &data,
        price_avg,
        price_max,
        price_avg_max,
        price_min,
        price_avg_min,
    );

    dao::modify(new_price).await?;

    Ok(())
}

fn get_security_price(
    price: &SecurityPrice,
    price_avg: BigDecimal,
    price_max: BigDecimal,
    price_avg_max: BigDecimal,
    price_min: BigDecimal,
    price_avg_min: BigDecimal,
) -> SecurityPrice {
    let mut new_price = price.clone();
    new_price.price_avg = price_avg;
    new_price.price_hight = price_max;
    new_price.price_hight_avg = price_avg_max;
    new_price.price_lowest = price_min;
    new_price.price_lowest_avg = price_avg_min;

    new_price
}

fn get_calculator_avg(resp_prices: &Vec<BigDecimal>) -> BigDecimal {
    let mut sum_count = BigDecimal::from(0);
    let mut sum_price = BigDecimal::from(0);

    for price in resp_prices {
        sum_count = sum_count.add(BigDecimal::from(1));
        sum_price = sum_price.add(price);
    }

    get_round(sum_price, sum_count)
}

fn get_calculator_max_avg(
    price_avg: &BigDecimal,
    resp_prices: &Vec<SecurityPrice>,
) -> (BigDecimal, BigDecimal) {
    let price_list = resp_prices
        .iter()
        .filter(|x| x.price_close >= *price_avg)
        .map(|x| x.price_close.clone())
        .collect::<Vec<BigDecimal>>();

    let max_price = price_list.iter().max().unwrap().clone();
    let avg_price = get_calculator_avg(&price_list);

    (max_price, avg_price)
}

fn get_calculator_min_avg(
    price_avg: &BigDecimal,
    resp_prices: &Vec<SecurityPrice>,
) -> (BigDecimal, BigDecimal) {
    let price_list = resp_prices
        .iter()
        .filter(|x| x.price_close <= *price_avg)
        .map(|x| x.price_close.clone())
        .collect::<Vec<BigDecimal>>();

    let min_price = price_list.iter().min().unwrap().clone();
    let avg_price = get_calculator_avg(&price_list);

    (min_price, avg_price)
}

fn get_round(price: BigDecimal, count: BigDecimal) -> BigDecimal {
    if count == BigDecimal::zero() {
        to_big_decimal_round(price.div(1))
    } else {
        to_big_decimal_round(price.div(count))
    }
}

fn to_big_decimal_round(val: bigdecimal::BigDecimal) -> bigdecimal::BigDecimal {
    val.with_scale_round(4, RoundingMode::Up)
}
