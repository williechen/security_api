#![warn(clippy::all, clippy::pedantic)]

use chrono::Local;
use sqlx::{postgres::PgRow, PgConnection, Row};
use tracing::{event, Level};

use crate::repository::Repository;

use super::model::{ResposePrice, SecurityPrice};

pub async fn create(trax_conn: &mut PgConnection, data: SecurityPrice) -> Result<u64, sqlx::Error> {
    match sqlx::query(
        r"
        INSERT INTO security_price(
            open_date_year
          , open_date_month
          , open_date_day
          , security_code 
          , security_name 
          , price_date 
          , price_close 
          , price_avg 
          , price_hight 
          , price_hight_avg 
          , price_lowest 
          , price_lowest_avg 
          , created_date
          , updated_date
        ) VALUES ( $1, $2, $3, $4, $5, $6, $7, 
         $8, $9, $10, $11, $12, $13, $14 )
    ",
    )
    .bind(data.open_date_year)
    .bind(data.open_date_month)
    .bind(data.open_date_day)
    .bind(data.security_code)
    .bind(data.security_name)
    .bind(data.price_date)
    .bind(data.price_close)
    .bind(data.price_avg)
    .bind(data.price_hight)
    .bind(data.price_hight_avg)
    .bind(data.price_lowest)
    .bind(data.price_lowest_avg)
    .bind(Local::now())
    .bind(Local::now())
    .execute(trax_conn)
    .await
    {
        Ok(cnt) => Ok(cnt.rows_affected()),
        Err(e) => Err(e),
    }
}

pub async fn modify(data: SecurityPrice) -> Result<u64, sqlx::Error> {
    let dao = Repository::new().await;
    let conn = dao.connection;

    match sqlx::query(
        r"
        UPDATE security_price 
           SET open_date_year = $1
             , open_date_month = $2
             , open_date_day = $3
             , security_code = $4
             , security_name = $5
             , price_date = $6
             , price_close = $7
             , price_avg = $8
             , price_hight = $9
             , price_hight_avg = $10
             , price_lowest = $11
             , price_lowest_avg = $12
             , updated_date = $13
         WHERE row_id = $14
    ",
    )
    .bind(data.open_date_year)
    .bind(data.open_date_month)
    .bind(data.open_date_day)
    .bind(data.security_code)
    .bind(data.security_name)
    .bind(data.price_date)
    .bind(data.price_close)
    .bind(data.price_avg)
    .bind(data.price_hight)
    .bind(data.price_hight_avg)
    .bind(data.price_lowest)
    .bind(data.price_lowest_avg)
    .bind(Local::now())
    .bind(data.row_id)
    .execute(&conn)
    .await
    {
        Ok(cnt) => Ok(cnt.rows_affected()),
        Err(e) => Err(e),
    }
}

pub async fn remove(
    trax_conn: &mut PgConnection,
    q_year: String,
    q_month: String,
    q_security_code: String,
) -> Result<u64, sqlx::Error> {
    match sqlx::query(
        r"
        DELETE FROM security_price 
         WHERE open_date_year = $1
           AND open_date_month = $2
           AND security_code = $3
    ",
    )
    .bind(q_year)
    .bind(q_month)
    .bind(q_security_code)
    .execute(trax_conn)
    .await
    {
        Ok(cnt) => Ok(cnt.rows_affected()),
        Err(e) => Err(e),
    }
}

pub async fn read_all_by_res(q_year: String, q_month: String) -> Vec<ResposePrice> {
    let dao = Repository::new().await;
    let conn = dao.connection;

    match sqlx::query(
        r" 
        SELECT rd.data_content
             , st.open_date_year
             , st.open_date_month
             , st.open_date_day
             , st.security_code
             , st.security_name
             , st.market_type
          FROM response_data rd
          JOIN security_task st
            ON rd.exec_code = st.security_code
           AND rd.open_date_year = st.open_date_year
           AND rd.open_date_month = st.open_date_month
         WHERE st.open_date_year = $1
           AND st.open_date_month = $2
           AND st.open_date_day = (SELECT MAX(st2.open_date_day) 
                                      FROM security_task st2 
                                     WHERE st2.security_code = st.security_code 
                                       AND st2.open_date_year = st.open_date_year
                                       AND st2.open_date_month = st.open_date_month
                                   )
         ORDER BY st.open_date_year, st.open_date_month,  st.security_code
    ",
    )
    .bind(q_year)
    .bind(q_month)
    .map(|row: PgRow| ResposePrice {
        open_date_year: row.get("open_date_year"),
        open_date_month: row.get("open_date_month"),
        open_date_day: row.get("open_date_day"),
        security_code: row.get("security_code"),
        security_name: row.get("security_name"),
        market_type: row.get("market_type"),
        data_content: row.get("data_content"),
    })
    .fetch_all(&conn)
    .await
    {
        Ok(rows) => rows,
        Err(e) => {
            event!(target: "security_api", Level::ERROR, "security_price.read_all_by_res: {}", &e);
            Vec::new()
        }
    }
}

pub async fn find_all(
    q_year: String,
    q_month: String,
    q_security_code: String,
) -> Vec<SecurityPrice> {
    let dao = Repository::new().await;
    let conn = dao.connection;

    match sqlx::query(
        r" 
        SELECT sp.row_id
             , sp.open_date_year
             , sp.open_date_month
             , sp.open_date_day
             , sp.security_code 
             , sp.security_name 
             , sp.price_date 
             , sp.price_close 
             , sp.price_avg 
             , sp.price_hight 
             , sp.price_hight_avg 
             , sp.price_lowest 
             , sp.price_lowest_avg 
          FROM security_price sp
         WHERE sp.open_date_year = $1
           AND sp.open_date_month = $2
           AND sp.security_code = $3
    ",
    )
    .bind(q_year)
    .bind(q_month)
    .bind(q_security_code)
    .map(|row: PgRow| SecurityPrice {
        row_id: row.get("row_id"),
        open_date_year: row.get("open_date_year"),
        open_date_month: row.get("open_date_month"),
        open_date_day: row.get("open_date_day"),
        security_code: row.get("security_code"),
        security_name: row.get("security_name"),
        price_date: row.get("price_date"),
        price_close: row.get("price_close"),
        price_avg: row.get("price_avg"),
        price_hight: row.get("price_hight"),
        price_hight_avg: row.get("price_hight_avg"),
        price_lowest: row.get("price_lowest"),
        price_lowest_avg: row.get("price_lowest_avg"),
    })
    .fetch_all(&conn)
    .await
    {
        Ok(rows) => rows,
        Err(e) => {
            event!(target: "security_api", Level::ERROR, "security_price.find_all: {}", &e);
            Vec::new()
        }
    }
}

pub async fn find_all_by_code(
    q_open_date: String,
    q_price_date: String,
    q_security_code: String,
) -> Vec<SecurityPrice> {
    let dao = Repository::new().await;
    let conn = dao.connection;

    match sqlx::query(
        r" 
        SELECT sp.row_id
             , sp.open_date_year
             , sp.open_date_month
             , sp.open_date_day
             , sp.security_code
             , sp.security_name
             , sp.price_date
             , sp.price_close
             , sp.price_avg
             , sp.price_hight
             , sp.price_hight_avg
             , sp.price_lowest
             , sp.price_lowest_avg
             , sp.created_date
             , sp.updated_date
          FROM security_price sp
          WHERE sp.price_date <= $1
            AND sp.price_date !='月平均收盤價' 
            AND sp.security_code = $2
            AND concat(sp.open_date_year, sp.open_date_month, sp.open_date_day) <= $3
         ORDER BY sp.open_date_year, sp.open_date_month, sp.open_date_day, sp.price_date, sp.security_code
    ",
    )
    .bind(q_price_date)
    .bind(q_security_code)
    .bind(q_open_date)
    .map(|row: PgRow| SecurityPrice {
        row_id: row.get("row_id"),
        open_date_year: row.get("open_date_year"),
        open_date_month: row.get("open_date_month"),
        open_date_day: row.get("open_date_day"),
        security_code: row.get("security_code"),
        security_name: row.get("security_name"),
        price_date: row.get("price_date"),
        price_close: row.get("price_close"),
        price_avg: row.get("price_avg"),
        price_hight: row.get("price_hight"),
        price_hight_avg: row.get("price_hight_avg"),
        price_lowest: row.get("price_lowest"),
        price_lowest_avg: row.get("price_lowest_avg"),
    })
    .fetch_all(&conn)
    .await
    {
        Ok(rows) =>rows,
        Err(e) => {
            event!(target: "security_api", Level::ERROR, "security_price.find_all_by_code: {}", &e);
            Vec::new()
        }
    }
}

pub async fn find_all_by_date(q_year: String, q_month: String) -> Vec<SecurityPrice> {
    let dao = Repository::new().await;
    let conn = dao.connection;

    match sqlx::query(
        r" 
        SELECT sp.row_id
             , sp.open_date_year
             , sp.open_date_month
             , sp.open_date_day
             , sp.security_code
             , sp.security_name
             , sp.price_date
             , sp.price_close
             , sp.price_avg
             , sp.price_hight
             , sp.price_hight_avg
             , sp.price_lowest
             , sp.price_lowest_avg
             , sp.created_date
             , sp.updated_date
          FROM security_price sp
          WHERE sp.price_date LIKE $1
         ORDER BY sp.open_date_year, sp.open_date_month, sp.open_date_day, sp.price_date, sp.security_code
    ",
    )
    .bind(format!(
        "%{0:04}/{1:02}%",
        (q_year.parse::<i32>().unwrap() - 1911),
        q_month.parse::<i32>().unwrap()
    ))
    .map(|row: PgRow| SecurityPrice {
        row_id: row.get("row_id"),
        open_date_year: row.get("open_date_year"),
        open_date_month: row.get("open_date_month"),
        open_date_day: row.get("open_date_day"),
        security_code: row.get("security_code"),
        security_name: row.get("security_name"),
        price_date: row.get("price_date"),
        price_close: row.get("price_close"),
        price_avg: row.get("price_avg"),
        price_hight: row.get("price_hight"),
        price_hight_avg: row.get("price_hight_avg"),
        price_lowest: row.get("price_lowest"),
        price_lowest_avg: row.get("price_lowest_avg"),
    })
    .fetch_all(&conn)
    .await
    {
        Ok(rows) =>rows,
        Err(e) => {
            event!(target: "security_api", Level::ERROR, "security_price.find_all_by_code: {}", &e);
            Vec::new()
        }
    }
}

pub async fn find_one_by_maxdate() -> String {
    let dao = Repository::new().await;
    let conn = dao.connection;

    match sqlx::query(
        r" 
        SELECT COALESCE(MAX(concat(open_date_year, open_date_month, RIGHT(price_date, 2))), '19981231') AS price_date
          FROM security_price sp
        WHERE sp.price_date not like '%月平均收盤價%'
    ",
    )
    .fetch_one(&conn)
    .await
    {
        Ok(row) =>row.get("price_date"),
        Err(e) => {
            event!(target: "security_api", Level::ERROR, "security_price.find_one_by_maxdate: {}", &e);
            String::new()
        }
    }
}
