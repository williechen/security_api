#![warn(clippy::all, clippy::pedantic)]

use chrono::Local;
use sqlx::{postgres::PgRow, Row};
use tracing::{event, Level};

use crate::{repository::Repository, security_task::model::SecurityTask};

use super::model::ResponseData;

pub async fn create(data: ResponseData) -> Result<u64, sqlx::Error> {
    let dao = Repository::new().await;
    let conn = dao.connection;

    match sqlx::query(
        r"
        INSERT INTO response_data(
            open_date_year
          , open_date_month
          , open_date_day
          , exec_code
          , data_content
          , created_date
          , updated_date
        ) VALUES ( $1, $2, $3, $4, $5, $6, $7 )
    ",
    )
    .bind(data.open_date_year)
    .bind(data.open_date_month)
    .bind(data.open_date_day)
    .bind(data.exec_code)
    .bind(data.data_content)
    .bind(Local::now())
    .bind(Local::now())
    .execute(&conn)
    .await
    {
        Ok(cnt) => Ok(cnt.rows_affected()),
        Err(e) => Err(e),
    }
}

pub async fn modify(data: ResponseData) -> Result<u64, sqlx::Error> {
    let dao = Repository::new().await;
    let conn = dao.connection;

    match sqlx::query(
        r"
        UPDATE response_data 
           SET open_date_year = $1
             , open_date_month = $2
             , open_date_day = $3
             , exec_code = $4
             , data_content = $5
             , updated_date = $6
         WHERE row_id = $7
    ",
    )
    .bind(data.open_date_year)
    .bind(data.open_date_month)
    .bind(data.open_date_day)
    .bind(data.exec_code)
    .bind(data.data_content)
    .bind(Local::now())
    .bind(data.row_id)
    .execute(&conn)
    .await
    {
        Ok(cnt) => Ok(cnt.rows_affected()),
        Err(e) => Err(e),
    }
}

pub async fn find_one(
    q_year: String,
    q_month: String,
    q_day: String,
    q_exec_code: String,
) -> Option<ResponseData> {
    let dao = Repository::new().await;
    let conn = dao.connection;

    match sqlx::query(
        r"
        SELECT row_id
             , open_date_year
             , open_date_month
             , open_date_day
             , exec_code
             , data_content
             , created_date
             , updated_date
          FROM response_data
         WHERE open_date_year = $1 
           AND open_date_month = $2
           AND open_date_day = $3
           AND exec_code = $4
         ",
    )
    .bind(q_year)
    .bind(q_month)
    .bind(q_day)
    .bind(q_exec_code)
    .map(|row: PgRow| ResponseData {
        row_id: row.get("row_id"),
        open_date_year: row.get("open_date_year"),
        open_date_month: row.get("open_date_month"),
        open_date_day: row.get("open_date_day"),
        exec_code: row.get("exec_code"),
        data_content: row.get("data_content"),
    })
    .fetch_optional(&conn)
    .await
    {
        Ok(row) => row,
        Err(e) => {
            event!(target: "security_api", Level::ERROR, "response_data.find_one: {}", &e);
            None
        }
    }
}

pub async fn find_one_by_max(task: &SecurityTask) -> Option<ResponseData> {
    let dao = Repository::new().await;
    let conn = dao.connection;

    match sqlx::query(
        r"
        SELECT row_id
             , open_date_year
             , open_date_month
             , open_date_day
             , exec_code
             , data_content
             , created_date
             , updated_date
          FROM response_data
         WHERE open_date_year = $1 
           AND open_date_month = $2
           AND open_date_day >= $3
           AND exec_code = $4
         ",
    )
    .bind(task.open_date_year.clone())
    .bind(task.open_date_month.clone())
    .bind(task.open_date_day.clone())
    .bind(task.security_code.clone())
    .map(|row: PgRow| ResponseData {
        row_id: row.get("row_id"),
        open_date_year: row.get("open_date_year"),
        open_date_month: row.get("open_date_month"),
        open_date_day: row.get("open_date_day"),
        exec_code: row.get("exec_code"),
        data_content: row.get("data_content"),
    })
    .fetch_optional(&conn)
    .await
    {
        Ok(row) => row,
        Err(e) => {
            event!(target: "security_api", Level::ERROR, "response_data.find_one_by_max: {}", &e);
            None
        }
    }
}

pub async fn find_one_by_min(task: &SecurityTask) -> Option<ResponseData> {
    let dao = Repository::new().await;
    let conn = dao.connection;

    match sqlx::query(
        r"
        SELECT row_id
             , open_date_year
             , open_date_month
             , open_date_day
             , exec_code
             , data_content
             , created_date
             , updated_date
          FROM response_data
         WHERE open_date_year = $1 
           AND open_date_month = $2
           AND open_date_day <= $3
           AND exec_code = $4
         ",
    )
    .bind(task.open_date_year.clone())
    .bind(task.open_date_month.clone())
    .bind(task.open_date_day.clone())
    .bind(task.security_code.clone())
    .map(|row: PgRow| ResponseData {
        row_id: row.get("row_id"),
        open_date_year: row.get("open_date_year"),
        open_date_month: row.get("open_date_month"),
        open_date_day: row.get("open_date_day"),
        exec_code: row.get("exec_code"),
        data_content: row.get("data_content"),
    })
    .fetch_optional(&conn)
    .await
    {
        Ok(row) => row,
        Err(e) => {
            event!(target: "security_api", Level::ERROR, "response_data.find_one_by_min: {}", &e);
            None
        }
    }
}
