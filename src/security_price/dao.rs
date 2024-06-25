#![warn(clippy::all, clippy::pedantic)]

use std::str::FromStr;

use chrono::Local;
use sqlx::{postgres::PgRow, PgConnection, Row};
use tracing::{event, Level};

use super::model::{ResposePrice, SecurityPrice};

pub async fn read_all(
    transaction: &mut PgConnection,
    data: &SecurityPrice,
) -> Result<(usize, Vec<SecurityPrice>), sqlx::Error> {
    let mut select_str = r#" 
        SELECT row_id
             , open_date
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
          FROM security_price
    "#
    .to_string();

    let mut index = 0;
    if data.open_date.is_some() {
        select_str.push_str(&where_append("open_date", "=", &mut index));
    }
    if data.security_code.is_some() {
        select_str.push_str(&where_append("security_code", "=", &mut index));
    }
    if data.price_date.is_some() {
        select_str.push_str(&where_append("price_date", "=", &mut index));
    }

    let mut query = sqlx::query(&select_str);

    if data.open_date.is_some() {
        query = query.bind(data.open_date.clone());
    }
    if data.security_code.is_some() {
        query = query.bind(data.security_code.clone());
    }
    if data.price_date.is_some() {
        query = query.bind(data.price_date.clone());
    }

    match query
        .map(|row: PgRow| SecurityPrice {
            row_id: row.get("row_id"),
            open_date: row.get("open_date"),
            security_code: row.get("security_code"),
            security_name: row.get("security_name"),
            price_date: row.get("price_date"),
            price_close: to_big_decimal(row.get("price_close")),
            price_avg: to_big_decimal(row.get("price_avg")),
            price_hight: to_big_decimal(row.get("price_hight")),
            price_hight_avg: to_big_decimal(row.get("price_hight_avg")),
            price_lowest: to_big_decimal(row.get("price_lowest")),
            price_lowest_avg: to_big_decimal(row.get("price_lowest_avg")),
        })
        .fetch_all(transaction)
        .await
    {
        Ok(rows) => Ok((rows.len(), rows)),
        Err(e) => {
            event!(target: "security_api", Level::ERROR, "security_price.read_all: {}", &e);
            Err(e)
        }
    }
}

fn where_append(field: &str, conditional: &str, index: &mut i32) -> String {
    let plus;
    if *index <= 0 {
        plus = " WHERE ";
    } else {
        plus = " AND ";
    }

    *index = *index + 1;

    format!(" {} {} {} ${} ", plus, field, conditional, index)
}

pub async fn read_all_by_sql(
    transaction: &mut PgConnection,
    sql: &str,
) -> Result<(usize, Vec<SecurityPrice>), sqlx::Error> {
    match sqlx::query(sql)
        .map(|row: PgRow| SecurityPrice {
            row_id: row.get("row_id"),
            open_date: row.get("open_date"),
            security_code: row.get("security_code"),
            security_name: row.get("security_name"),
            price_date: row.get("price_date"),
            price_close: to_big_decimal(row.get("price_close")),
            price_avg: to_big_decimal(row.get("price_avg")),
            price_hight: to_big_decimal(row.get("price_hight")),
            price_hight_avg: to_big_decimal(row.get("price_hight_avg")),
            price_lowest: to_big_decimal(row.get("price_lowest")),
            price_lowest_avg: to_big_decimal(row.get("price_lowest_avg")),
        })
        .fetch_all(transaction)
        .await
    {
        Ok(rows) => Ok((rows.len(), rows)),
        Err(e) => {
            event!(target: "security_api", Level::ERROR, "security_price.read_all_by_sql: {}", &e);
            Err(e)
        }
    }
}

pub async fn read(
    transaction: &mut PgConnection,
    row_id: &str,
) -> Result<Option<SecurityPrice>, sqlx::Error> {
    match sqlx::query(
        r#"
        SELECT row_id
             , open_date
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
          FROM security_price
         WHERE row_id = $1 "#,
    )
    .bind(row_id)
    .map(|row: PgRow| SecurityPrice {
        row_id: row.get("row_id"),
        open_date: row.get("open_date"),
        security_code: row.get("security_code"),
        security_name: row.get("security_name"),
        price_date: row.get("price_date"),
        price_close: to_big_decimal(row.get("price_close")),
        price_avg: to_big_decimal(row.get("price_avg")),
        price_hight: to_big_decimal(row.get("price_hight")),
        price_hight_avg: to_big_decimal(row.get("price_hight_avg")),
        price_lowest: to_big_decimal(row.get("price_lowest")),
        price_lowest_avg: to_big_decimal(row.get("price_lowest_avg")),
    })
    .fetch_one(transaction)
    .await
    {
        Ok(row) => Ok(Some(row)),
        Err(e) => {
            event!(target: "security_api", Level::ERROR, "security_price.read: {}", &e);
            Err(e)
        }
    }
}

pub async fn create(
    transaction: &mut PgConnection,
    data: SecurityPrice,
) -> Result<u64, sqlx::Error> {
    match sqlx::query(
        r#" 
        INSERT INTO security_price(open_date
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
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)  "#,
    )
    .bind(data.open_date)
    .bind(data.security_code)
    .bind(data.security_name)
    .bind(data.price_date)
    .bind(to_sql_big_decimal(data.price_close))
    .bind(to_sql_big_decimal(data.price_avg))
    .bind(to_sql_big_decimal(data.price_hight))
    .bind(to_sql_big_decimal(data.price_hight_avg))
    .bind(to_sql_big_decimal(data.price_lowest))
    .bind(to_sql_big_decimal(data.price_lowest_avg))
    .bind(Local::now().naive_local())
    .bind(Local::now().naive_local())
    .execute(transaction)
    .await
    {
        Ok(row) => Ok(row.rows_affected()),
        Err(e) => {
            event!(target: "security_api", Level::ERROR, "security_price.create: {}", &e);
            Err(e)
        }
    }
}

pub async fn update(
    transaction: &mut PgConnection,
    data: SecurityPrice,
) -> Result<u64, sqlx::Error> {
    match sqlx::query(
        r#" 
        UPDATE security_price
            SET open_date= $1
              , security_code = $2
              , security_name = $3
              , price_date = $4
              , price_close = $5
              , price_avg = $6
              , price_hight = $7
              , price_hight_avg = $8
              , price_lowest = $9
              , price_lowest_avg =$10
              , updated_date = $11
            WHERE row_id = $12
        "#,
    )
    .bind(data.open_date)
    .bind(data.security_code)
    .bind(data.security_name)
    .bind(data.price_date)
    .bind(to_sql_big_decimal(data.price_close))
    .bind(to_sql_big_decimal(data.price_avg))
    .bind(to_sql_big_decimal(data.price_hight))
    .bind(to_sql_big_decimal(data.price_hight_avg))
    .bind(to_sql_big_decimal(data.price_lowest))
    .bind(to_sql_big_decimal(data.price_lowest_avg))
    .bind(Local::now().naive_local())
    .bind(data.row_id)
    .execute(transaction)
    .await
    {
        Ok(row) => Ok(row.rows_affected()),
        Err(e) => {
            event!(target: "security_api", Level::ERROR, "security_price.update: {}", &e);
            Err(e)
        }
    }
}

pub async fn delete(
    transaction: &mut PgConnection,
    data: SecurityPrice,
) -> Result<u64, sqlx::Error> {
    match sqlx::query(r#" DELETE FROM security_price WHERE row_id = $1 "#)
        .bind(data.row_id)
        .execute(transaction)
        .await
    {
        Ok(row) => Ok(row.rows_affected()),
        Err(e) => {
            event!(target: "security_api", Level::ERROR, "security_price.delete: {}", &e);
            Err(e)
        }
    }
}

pub async fn read_all_by_res(
    transaction: &mut PgConnection,
    ce_year: &str,
    ce_month: &str,
    ce_day: &str,
) -> Result<Vec<ResposePrice>, sqlx::Error> {
    match sqlx::query(
        r#"
        SELECT rd.data_content
             , st.open_date
             , st.security_code
             , st.security_name
             , st.market_type
          FROM response_data rd
          JOIN security_task st
            ON rd.exec_code = st.security_code
           AND rd.open_date = st.open_date
          JOIN calendar_data cd
            ON CONCAT(cd.ce_year, cd.ce_month, cd.ce_day) = rd.open_date
         WHERE cd.ce_year = $1
           AND cd.ce_month = $2
           AND cd.ce_day >= $3 
         ORDER BY st.open_date, st.security_code
         "#,
    )
    .bind(ce_year)
    .bind(ce_month)
    .bind(ce_day)
    .map(|row: PgRow| ResposePrice {
        open_date: row.get("open_date"),
        security_code: row.get("security_code"),
        security_name: row.get("security_name"),
        market_type: row.get("market_type"),
        data_content: row.get("data_content"),
    })
    .fetch_all(transaction)
    .await
    {
        Ok(row) => Ok(row),
        Err(e) => {
            event!(target: "security_api", Level::ERROR, "security_price.read_all_by_res: {}", &e);
            Err(e)
        }
    }
}

pub async fn read_all_by_code(
    transaction: &mut PgConnection,
    open_date: &str,
    security_code: &str,
) -> Result<Vec<SecurityPrice>, sqlx::Error> {
    match sqlx::query(
        r#"
        SELECT sp.row_id
             , sp.open_date
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
          JOIN calendar_data cd
            ON CONCAT(cd.tw_year, '/', cd.ce_month, '/', cd.ce_day) = sp.price_date
         WHERE CONCAT(cd.ce_year, cd.ce_month, cd.ce_day) <= $1
           AND sp.security_code = $2
         ORDER BY sp.open_date, sp.security_code
        "#,
    )
    .bind(open_date)
    .bind(security_code)
    .map(|row: PgRow| SecurityPrice {
        row_id: row.get("row_id"),
        open_date: row.get("open_date"),
        security_code: row.get("security_code"),
        security_name: row.get("security_name"),
        price_date: row.get("price_date"),
        price_close: to_big_decimal(row.get("price_close")),
        price_avg: to_big_decimal(row.get("price_avg")),
        price_hight: to_big_decimal(row.get("price_hight")),
        price_hight_avg: to_big_decimal(row.get("price_hight_avg")),
        price_lowest: to_big_decimal(row.get("price_lowest")),
        price_lowest_avg: to_big_decimal(row.get("price_lowest_avg")),
    })
    .fetch_all(transaction)
    .await
    {
        Ok(row) => Ok(row),
        Err(e) => {
            event!(target: "security_api", Level::ERROR, "security_price.read_all_by_code: {}", &e);
            Err(e)
        }
    }
}

pub async fn read_all_by_date(
    transaction: &mut PgConnection,
    open_date: &str,
) -> Result<Vec<SecurityPrice>, sqlx::Error> {
    match sqlx::query(
        r#"
        SELECT sp.row_id
             , sp.open_date
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
          JOIN calendar_data cd
            ON CONCAT(cd.tw_year, '/', cd.ce_month, '/', cd.ce_day) = sp.price_date
         WHERE CONCAT(cd.ce_year, cd.ce_month, cd.ce_day) = $1
         ORDER BY sp.open_date, sp.security_code
        "#,
    )
    .bind(open_date)
    .map(|row: PgRow| SecurityPrice {
        row_id: row.get("row_id"),
        open_date: row.get("open_date"),
        security_code: row.get("security_code"),
        security_name: row.get("security_name"),
        price_date: row.get("price_date"),
        price_close: to_big_decimal(row.get("price_close")),
        price_avg: to_big_decimal(row.get("price_avg")),
        price_hight: to_big_decimal(row.get("price_hight")),
        price_hight_avg: to_big_decimal(row.get("price_hight_avg")),
        price_lowest: to_big_decimal(row.get("price_lowest")),
        price_lowest_avg: to_big_decimal(row.get("price_lowest_avg")),
    })
    .fetch_all(transaction)
    .await
    {
        Ok(row) => Ok(row),
        Err(e) => {
            event!(target: "security_api", Level::ERROR, "security_price.read: {}", &e);
            Err(e)
        }
    }
}

fn to_sql_big_decimal(val: Option<bigdecimal::BigDecimal>) -> sqlx::types::BigDecimal {
    sqlx::types::BigDecimal::from_str(&val.unwrap().to_string()).unwrap()
}

fn to_big_decimal(val: sqlx::types::BigDecimal) -> Option<bigdecimal::BigDecimal> {
    Some(bigdecimal::BigDecimal::from_str(&val.to_string()).unwrap())
}
