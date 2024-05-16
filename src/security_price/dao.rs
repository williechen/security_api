use chrono::Local;
use sqlx::{postgres::PgRow, Row};
use tracing::{event, Level};

use super::model::SecurityPrice;

pub async fn read_all(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    data: &SecurityPrice,
) -> Result<(usize, Vec<SecurityPrice>), sqlx::Error> {
    let mut select_str = r#" 
        SELECT row_id
             , open_date
             , security_code
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
            price_date: row.get("price_date"),
            price_close: row.get("price_close"),
            price_avg: row.get("price_avg"),
            price_hight: row.get("price_hight"),
            price_hight_avg: row.get("price_hight_avg"),
            price_lowest: row.get("price_lowest"),
            price_lowest_avg: row.get("price_lowest_avg"),
        })
        .fetch_all(&mut **transaction)
        .await
    {
        Ok(rows) => Ok((rows.len(), rows)),
        Err(e) => {
            event!(target: "security_api", Level::ERROR, "{:?}", &e);
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
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    sql: &str,
) -> Result<(usize, Vec<SecurityPrice>), sqlx::Error> {
    match sqlx::query(sql)
        .map(|row: PgRow| SecurityPrice {
            row_id: row.get("row_id"),
            open_date: row.get("open_date"),
            security_code: row.get("security_code"),
            price_date: row.get("price_date"),
            price_close: row.get("price_close"),
            price_avg: row.get("price_avg"),
            price_hight: row.get("price_hight"),
            price_hight_avg: row.get("price_hight_avg"),
            price_lowest: row.get("price_lowest"),
            price_lowest_avg: row.get("price_lowest_avg"),
        })
        .fetch_all(&mut **transaction)
        .await
    {
        Ok(rows) => Ok((rows.len(), rows)),
        Err(e) => {
            event!(target: "security_api", Level::ERROR, "{:?}", &e);
            Err(e)
        }
    }
}

pub async fn read(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    row_id: &str,
) -> Result<Option<SecurityPrice>, sqlx::Error> {
    match sqlx::query(
        r#"
        SELECT row_id
             , open_date
             , security_code
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
        price_date: row.get("price_date"),
        price_close: row.get("price_close"),
        price_avg: row.get("price_avg"),
        price_hight: row.get("price_hight"),
        price_hight_avg: row.get("price_hight_avg"),
        price_lowest: row.get("price_lowest"),
        price_lowest_avg: row.get("price_lowest_avg"),
    })
    .fetch_one(&mut **transaction)
    .await
    {
        Ok(row) => Ok(Some(row)),
        Err(e) => {
            event!(target: "security_api", Level::ERROR, "{:?}", &e);
            Err(e)
        }
    }
}

pub async fn create(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    data: SecurityPrice,
) -> Result<u64, sqlx::Error> {
    match sqlx::query(
        r#" 
        INSERT INTO security_price(open_date
            , security_code
            , price_date
            , price_close
            , price_avg
            , price_hight
            , price_hight_avg
            , price_lowest
            , price_lowest_avg
            , created_date
            , updated_date
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)  "#,
    )
    .bind(data.open_date)
    .bind(data.security_code)
    .bind(data.price_date)
    .bind(data.price_close)
    .bind(data.price_avg)
    .bind(data.price_hight)
    .bind(data.price_hight_avg)
    .bind(data.price_lowest)
    .bind(data.price_lowest_avg)
    .bind(Local::now().naive_local())
    .bind(Local::now().naive_local())
    .execute(&mut **transaction)
    .await
    {
        Ok(row) => Ok(row.rows_affected()),
        Err(e) => {
            event!(target: "security_api", Level::ERROR, "{:?}", &e);
            Err(e)
        }
    }
}

pub async fn update(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    data: SecurityPrice,
) -> Result<u64, sqlx::Error> {
    match sqlx::query(
        r#" UPDATE security_price
            SET open_date= $1
              , security_code = $2
              , price_date = $3
              , price_close = $4
              , price_avg = $5
              , price_hight = $6
              , price_hight_avg = $7
              , price_lowest = $8
              , price_lowest_avg =$9
              , updated_date = $10
            WHERE row_id = $11
          "#,
    )
    .bind(data.open_date)
    .bind(data.security_code)
    .bind(data.price_date)
    .bind(data.price_close)
    .bind(data.price_avg)
    .bind(data.price_hight)
    .bind(data.price_hight_avg)
    .bind(data.price_lowest)
    .bind(data.price_lowest_avg)
    .bind(Local::now().naive_local())
    .bind(data.row_id)
    .execute(&mut **transaction)
    .await
    {
        Ok(row) => Ok(row.rows_affected()),
        Err(e) => {
            event!(target: "security_api", Level::ERROR, "{:?}", &e);
            Err(e)
        }
    }
}

pub async fn delete(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    data: SecurityPrice,
) -> Result<u64, sqlx::Error> {
    match sqlx::query(r#" DELETE FROM security_price WHERE row_id = $1 "#)
        .bind(data.row_id)
        .execute(&mut **transaction)
        .await
    {
        Ok(row) => Ok(row.rows_affected()),
        Err(e) => {
            event!(target: "security_api", Level::ERROR, "{:?}", &e);
            Err(e)
        }
    }
}
