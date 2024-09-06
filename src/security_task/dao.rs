#![warn(clippy::all, clippy::pedantic)]

use chrono::Local;
use sqlx::{postgres::PgRow, Row};
use tracing::{event, Level};

use crate::{daily_task::model::DailyTask, repository::Repository};

use super::model::SecurityTask;

pub async fn create(data: SecurityTask) -> Result<u64, sqlx::Error> {
    let dao = Repository::new().await;
    let conn = dao.connection;

    match sqlx::query(
        r"
        INSERT INTO security_task(
            open_date_year
          , open_date_month
          , open_date_day
          , security_code 
          , security_name 
          , market_type 
          , issue_date 
          , exec_seed 
          , exec_count 
          , is_enabled 
          , sort_no 
          , created_date
          , updated_date
        ) VALUES ( $1, $2, $3, $4, $5, $6, $7, 
         $8, $9, $10, $11, $12, $13 )
    ",
    )
    .bind(data.open_date_year)
    .bind(data.open_date_month)
    .bind(data.open_date_day)
    .bind(data.security_code)
    .bind(data.security_name)
    .bind(data.market_type)
    .bind(data.issue_date)
    .bind(data.exec_seed)
    .bind(data.exec_count)
    .bind(data.is_enabled)
    .bind(data.sort_no)
    .bind(Local::now())
    .bind(Local::now())
    .execute(&conn)
    .await
    {
        Ok(cnt) => Ok(cnt.rows_affected()),
        Err(e) => Err(e),
    }
}

pub async fn modify(data: SecurityTask) -> Result<u64, sqlx::Error> {
    let dao = Repository::new().await;
    let conn = dao.connection;

    match sqlx::query(
        r"
        UPDATE security_task 
           SET open_date_year = $1
             , open_date_month = $2
             , open_date_day = $3
             , security_code = $4
             , security_name = $5
             , market_type = $6
             , issue_date = $7
             , exec_seed = $8
             , exec_count = $9
             , is_enabled = $10
             , sort_no = $11
             , updated_date = $12
         WHERE row_id = $13
    ",
    )
    .bind(data.open_date_year)
    .bind(data.open_date_month)
    .bind(data.open_date_day)
    .bind(data.security_code)
    .bind(data.security_name)
    .bind(data.market_type)
    .bind(data.issue_date)
    .bind(data.exec_seed)
    .bind(data.exec_count)
    .bind(data.is_enabled)
    .bind(data.sort_no)
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
    q_security_code: String,
    q_market_type: String,
    q_issue_date: String,
) -> Option<SecurityTask> {
    let dao = Repository::new().await;
    let conn = dao.connection;

    match sqlx::query(
        r"
        SELECT row_id
             , open_date_year
             , open_date_month
             , open_date_day
             , security_code 
             , security_name 
             , market_type 
             , issue_date 
             , exec_seed 
             , exec_count 
             , is_enabled 
             , sort_no 
             , created_date
             , updated_date
          FROM security_task
         WHERE open_date_year = $1
           AND open_date_month = $2
           AND open_date_day = $3
           AND security_code = $4
           AND market_type = $5
           AND issue_date = $6
          ",
    )
    .bind(q_year)
    .bind(q_month)
    .bind(q_day)
    .bind(q_security_code)
    .bind(q_market_type)
    .bind(q_issue_date)
    .map(|row: PgRow| SecurityTask {
        row_id: row.get("row_id"),
        open_date_year: row.get("open_date_year"),
        open_date_month: row.get("open_date_month"),
        open_date_day: row.get("open_date_day"),
        security_code: row.get("security_code"),
        security_name: row.get("security_name"),
        market_type: row.get("market_type"),
        issue_date: row.get("issue_date"),
        exec_seed: row.get("exec_seed"),
        exec_count: row.get("exec_count"),
        is_enabled: row.get("is_enabled"),
        sort_no: row.get("sort_no"),
    })
    .fetch_optional(&conn)
    .await
    {
        Ok(row) => row,
        Err(e) => {
            event!(target: "security_api", Level::ERROR, "security_task.find_one: {}", &e);
            None
        }
    }
}

pub async fn find_all_by_twse(task: &DailyTask) -> Vec<SecurityTask> {
    let dao = Repository::new().await;
    let conn = dao.connection;

    let q_year = task.clone().open_date_year;
    let q_month = task.clone().open_date_month;
    let q_day = task.clone().open_date_day;

    match sqlx::query(
        r"
        SELECT row_id
             , open_date_year
             , open_date_month
             , open_date_day
             , security_code
             , security_name
             , market_type
             , issue_date
             , exec_seed
             , exec_count
             , is_enabled
             , sort_no
          FROM security_task 
         WHERE open_date_year = $1
           AND open_date_month = $2
           AND open_date_day = $3
           AND market_type in ('上市')
         ORDER BY security_code, issue_date, market_type
          ",
    )
    .bind(q_year)
    .bind(q_month)
    .bind(q_day)
    .map(|row: PgRow| SecurityTask {
        row_id: row.get("row_id"),
        open_date_year: row.get("open_date_year"),
        open_date_month: row.get("open_date_month"),
        open_date_day: row.get("open_date_day"),
        security_code: row.get("security_code"),
        security_name: row.get("security_name"),
        market_type: row.get("market_type"),
        issue_date: row.get("issue_date"),
        exec_seed: row.get("exec_seed"),
        exec_count: row.get("exec_count"),
        is_enabled: row.get("is_enabled"),
        sort_no: row.get("sort_no"),
    })
    .fetch_all(&conn)
    .await
    {
        Ok(rows) => rows,
        Err(e) => {
            event!(target: "security_api", Level::ERROR, "security_task.find_all_by_twse: {}", &e);
            Vec::new()
        }
    }
}

pub async fn find_all_by_tpex(task: &DailyTask) -> Vec<SecurityTask> {
    let dao = Repository::new().await;
    let conn = dao.connection;

    let q_year = task.clone().open_date_year;
    let q_month = task.clone().open_date_month;
    let q_day = task.clone().open_date_day;

    match sqlx::query(
        r"
        SELECT row_id
             , open_date_year
             , open_date_month
             , open_date_day
             , security_code
             , security_name
             , market_type
             , issue_date
             , exec_seed
             , exec_count
             , is_enabled
             , sort_no
          FROM security_task 
         WHERE open_date_year = $1
           AND open_date_month = $2
           AND open_date_day = $3
           AND market_type in ('上櫃', '興櫃')
         ORDER BY security_code, issue_date, market_type
          ",
    )
    .bind(q_year)
    .bind(q_month)
    .bind(q_day)
    .map(|row: PgRow| SecurityTask {
        row_id: row.get("row_id"),
        open_date_year: row.get("open_date_year"),
        open_date_month: row.get("open_date_month"),
        open_date_day: row.get("open_date_day"),
        security_code: row.get("security_code"),
        security_name: row.get("security_name"),
        market_type: row.get("market_type"),
        issue_date: row.get("issue_date"),
        exec_seed: row.get("exec_seed"),
        exec_count: row.get("exec_count"),
        is_enabled: row.get("is_enabled"),
        sort_no: row.get("sort_no"),
    })
    .fetch_all(&conn)
    .await
    {
        Ok(rows) => rows,
        Err(e) => {
            event!(target: "security_api", Level::ERROR, "security_task.find_all_by_tpex: {}", &e);
            Vec::new()
        }
    }
}

pub async fn find_all_by_times(q_year: String, q_month: String, q_day: String) -> Vec<SecurityTask> {
    let dao = Repository::new().await;
    let conn = dao.connection;

    match sqlx::query(
        r"
        SELECT row_id
             , open_date_year
             , open_date_month
             , open_date_day
             , security_code
             , security_name
             , market_type
             , issue_date
             , exec_seed
             , exec_count
             , is_enabled
             , sort_no
          FROM security_task 
         WHERE open_date_year = $1
           AND open_date_month = $2
           AND open_date_day = $3
           AND exec_count <= 3
           AND is_enabled = 1
         ORDER BY sort_no
          ",
    )
    .bind(q_year)
    .bind(q_month)
    .bind(q_day)
    .map(|row: PgRow| SecurityTask {
        row_id: row.get("row_id"),
        open_date_year: row.get("open_date_year"),
        open_date_month: row.get("open_date_month"),
        open_date_day: row.get("open_date_day"),
        security_code: row.get("security_code"),
        security_name: row.get("security_name"),
        market_type: row.get("market_type"),
        issue_date: row.get("issue_date"),
        exec_seed: row.get("exec_seed"),
        exec_count: row.get("exec_count"),
        is_enabled: row.get("is_enabled"),
        sort_no: row.get("sort_no"),
    })
    .fetch_all(&conn)
    .await
    {
        Ok(rows) => rows,
        Err(e) => {
            event!(target: "security_api", Level::ERROR, "security_task.find_all_by_times: {}", &e);
            Vec::new()
        }
    }
}