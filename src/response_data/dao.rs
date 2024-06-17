use chrono::Local;
use sqlx::{postgres::PgRow, PgConnection, Row};
use tracing::{event, Level};

use super::model::ResponseData;

pub async fn read_all(
    transaction: &mut PgConnection,
    data: ResponseData,
) -> Result<(usize, Vec<ResponseData>), sqlx::Error> {
    let mut select_str = r#" 
        SELECT row_id
             , open_date
             , exec_code
             , data_content
             , created_date
             , updated_date
          FROM response_data
    "#
    .to_string();

    let mut index = 0;
    if data.open_date.is_some() {
        select_str.push_str(&where_append("open_date", "=", &mut index));
    }
    if data.exec_code.is_some() {
        select_str.push_str(&where_append("exec_code", "=", &mut index));
    }
    if data.data_content.is_some() {
        select_str.push_str(&where_append("data_content", "like", &mut index));
    }

    let mut query = sqlx::query(&select_str);

    if data.open_date.is_some() {
        query = query.bind(data.open_date.clone());
    }
    if data.exec_code.is_some() {
        query = query.bind(data.exec_code.clone());
    }
    if data.data_content.is_some() {
        query = query.bind(data.data_content.clone());
    }

    match query
        .map(|row: PgRow| ResponseData {
            row_id: row.get("row_id"),
            open_date: row.get("open_date"),
            exec_code: row.get("exec_code"),
            data_content: row.get("data_content"),
        })
        .fetch_all(transaction)
        .await
    {
        Ok(rows) => Ok((rows.len(), rows)),
        Err(e) => {
            event!(target: "security_api", Level::ERROR, "response_data.read_all: {}", &e);
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
) -> Result<(usize, Vec<ResponseData>), sqlx::Error> {
    match sqlx::query(sql)
        .map(|row: PgRow| ResponseData {
            row_id: row.get("row_id"),
            open_date: row.get("open_date"),
            exec_code: row.get("exec_code"),
            data_content: row.get("data_content"),
        })
        .fetch_all(transaction)
        .await
    {
        Ok(rows) => Ok((rows.len(), rows)),
        Err(e) => {
            event!(target: "security_api", Level::ERROR, "response_data.read_all_by_sql: {}", &e);
            Err(e)
        }
    }
}

pub async fn read(
    transaction: &mut PgConnection,
    row_id: &str,
) -> Result<Option<ResponseData>, sqlx::Error> {
    match sqlx::query(
        r#"
        SELECT row_id
             , open_date
             , exec_code
             , data_content
             , created_date
             , updated_date
          FROM response_data
         WHERE row_id = $1 "#,
    )
    .bind(row_id)
    .map(|row: PgRow| ResponseData {
        row_id: row.get("row_id"),
        open_date: row.get("open_date"),
        exec_code: row.get("exec_code"),
        data_content: row.get("data_content"),
    })
    .fetch_one(transaction)
    .await
    {
        Ok(row) => Ok(Some(row)),
        Err(e) => {
            event!(target: "security_api", Level::ERROR, "response_data.read: {}", &e);
            Err(e)
        }
    }
}

pub async fn create(
    transaction: &mut PgConnection,
    data: ResponseData,
) -> Result<u64, sqlx::Error> {
    match sqlx::query(
        r#" 
        INSERT INTO response_data(open_date
             , exec_code
             , data_content
             , created_date
             , updated_date
        ) VALUES ($1, $2, $3, $4, $5)  "#,
    )
    .bind(data.open_date)
    .bind(data.exec_code)
    .bind(data.data_content)
    .bind(Local::now().naive_local())
    .bind(Local::now().naive_local())
    .execute(transaction)
    .await
    {
        Ok(row) => Ok(row.rows_affected()),
        Err(e) => {
            event!(target: "security_api", Level::ERROR, "response_data.create: {}", &e);
            Err(e)
        }
    }
}

pub async fn update(
    transaction: &mut PgConnection,
    data: ResponseData,
) -> Result<u64, sqlx::Error> {
    match sqlx::query(
        r#" UPDATE response_data
            SET open_date= $1
              , exec_code = $2
              , data_content = $3
              , updated_date = $4
            WHERE row_id = $5
          "#,
    )
    .bind(data.open_date)
    .bind(data.exec_code)
    .bind(data.data_content)
    .bind(Local::now().naive_local())
    .bind(data.row_id)
    .execute(transaction)
    .await
    {
        Ok(row) => Ok(row.rows_affected()),
        Err(e) => {
            event!(target: "security_api", Level::ERROR, "response_data.update: {}", &e);
            Err(e)
        }
    }
}

pub async fn delete(
    transaction: &mut PgConnection,
    data: ResponseData,
) -> Result<u64, sqlx::Error> {
    match sqlx::query(r#" DELETE FROM response_data WHERE row_id = $1 "#)
        .bind(data.row_id)
        .execute(transaction)
        .await
    {
        Ok(row) => Ok(row.rows_affected()),
        Err(e) => {
            event!(target: "security_api", Level::ERROR, "response_data.delete: {}", &e);
            Err(e)
        }
    }
}

pub async fn read_by_max_day(
    transaction: &mut PgConnection,
    security_code: &str,
    year_str: &str,
    month_str: &str,
    day_str: &str,
) -> Result<Option<ResponseData>, sqlx::Error> {
    match sqlx::query(
        r#"
        SELECT rd.row_id
             , rd.open_date
             , rd.exec_code
             , rd.data_content
             , rd.created_date
             , rd.updated_date 
          FROM response_data rd 
          JOIN calendar_data cd 
            ON rd.open_date = CONCAT(cd.ce_year, cd.ce_month, cd.ce_day)
         WHERE rd.exec_code = $1
           AND cd.ce_year = $2
           AND cd.ce_month = $3
           AND cd.ce_day >= $4
         "#,
    )
    .bind(security_code)
    .bind(year_str)
    .bind(month_str)
    .bind(day_str)
    .map(|row: PgRow| ResponseData {
        row_id: row.get("row_id"),
        open_date: row.get("open_date"),
        exec_code: row.get("exec_code"),
        data_content: row.get("data_content"),
    })
    .fetch_all(transaction)
    .await
    {
        Ok(row) => {
            if row.is_empty() {
                Ok(None)
            } else {
                Ok(row.get(0).cloned())
            }
        }
        Err(e) => {
            event!(target: "security_api", Level::ERROR, "response_data.read_by_max_day: {}", &e);
            Err(e)
        }
    }
}

pub async fn update_by_max_day(
    transaction: &mut PgConnection,
    data: ResponseData,
    security_code: &str,
    year_str: &str,
    month_str: &str,
) -> Result<u64, sqlx::Error> {
    match sqlx::query(
        r#" UPDATE response_data
            SET open_date= $1
              , exec_code = $2
              , data_content = $3
              , updated_date = $4
            WHERE exec_code = $5
              AND open_date in (
                SELECT CONCAT(cd.ce_year, cd.ce_month, cd.ce_day) AS open_date
                  FROM calendar_data cd 
                 WHERE cd.ce_year = $6
                   AND cd.ce_month = $7
            )
          "#,
    )
    .bind(data.open_date)
    .bind(data.exec_code)
    .bind(data.data_content)
    .bind(Local::now().naive_local())
    .bind(security_code)
    .bind(year_str)
    .bind(month_str)
    .execute(transaction)
    .await
    {
        Ok(row) => Ok(row.rows_affected()),
        Err(e) => {
            event!(target: "security_api", Level::ERROR, "response_data.update_by_max_day: {}", &e);
            Err(e)
        }
    }
}
