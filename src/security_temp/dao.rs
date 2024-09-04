#![warn(clippy::all, clippy::pedantic)]

use chrono::Local;
use sqlx::{postgres::PgRow, PgConnection, Row};
use tracing::{event, Level};

use crate::{daily_task::model::DailyTask, repository::Repository};

use super::model::SecurityTemp;

pub async fn create(trax_conn: &mut PgConnection, data: SecurityTemp) -> Result<u64, sqlx::Error> {
    match sqlx::query(
        r"
        INSERT INTO security_temp(
            open_date_year
          , open_date_month
          , open_date_day
          , international_code 
          , security_code 
          , security_name 
          , market_type 
          , security_type 
          , industry_type 
          , issue_date 
          , cfi_code 
          , remark 
          , created_date
          , updated_date
        ) VALUES ( $1, $2, $3, $4, $5, $6, $7, 
         $8, $9, $10, $11, $12, $13, $14 )
    ",
    )
    .bind(data.open_date_year)
    .bind(data.open_date_month)
    .bind(data.open_date_day)
    .bind(data.international_code)
    .bind(data.security_code)
    .bind(data.security_name)
    .bind(data.market_type)
    .bind(data.security_type)
    .bind(data.industry_type)
    .bind(data.issue_date)
    .bind(data.cfi_code)
    .bind(data.remark)
    .bind(Local::now())
    .bind(Local::now())
    .execute(trax_conn)
    .await
    {
        Ok(cnt) => Ok(cnt.rows_affected()),
        Err(e) => Err(e),
    }
}

pub async fn remove_all() -> Result<u64, sqlx::Error> {
    let dao = Repository::new().await;
    let conn = dao.connection;

    match sqlx::query(
        r"
        DELETE FROM security_temp 
         WHERE 1=1
    ",
    )
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
) -> Option<SecurityTemp> {
    let dao = Repository::new().await;
    let conn = dao.connection;

    match sqlx::query(
        r"
        SELECT row_id
             , open_date_year
             , open_date_month
             , open_date_day
             , international_code 
             , security_code 
             , security_name 
             , market_type 
             , security_type 
             , industry_type 
             , issue_date 
             , cfi_code 
             , remark 
          FROM security_temp st
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
    .map(|row: PgRow| SecurityTemp {
        row_id: row.get("row_id"),
        open_date_year: row.get("open_date_year"),
        open_date_month: row.get("open_date_month"),
        open_date_day: row.get("open_date_day"),
        international_code: row.get("international_code"),
        security_code: row.get("security_code"),
        security_name: row.get("security_name"),
        market_type: row.get("market_type"),
        security_type: row.get("security_type"),
        industry_type: row.get("industry_type"),
        issue_date: row.get("issue_date"),
        cfi_code: row.get("cfi_code"),
        remark: row.get("remark"),
    })
    .fetch_optional(&conn)
    .await
    {
        Ok(row) => row,
        Err(e) => {
            event!(target: "security_api", Level::ERROR, "security_temp.find_one: {}", &e);
            None
        }
    }
}

pub async fn find_all_by_twse(task: DailyTask) -> Vec<SecurityTemp> {
    let dao = Repository::new().await;
    let conn = dao.connection;

    let q_year = task.clone().open_date_year;
    let q_month = task.clone().open_date_month;
    let q_day = task.clone().open_date_day;
    let q_open_date = format!("{0}{1}{2}", q_year, q_month, q_day);
    let q_issue_date = format!("{0}/{1}/{2}", q_year, q_month, q_day);

    match sqlx::query(r"
        SELECT row_id
             , open_date_year
             , open_date_month
             , open_date_day
             , international_code
             , security_code
             , security_name
             , market_type
             , security_type
             , industry_type
             , issue_date
             , cfi_code
             , remark
          FROM security_temp 
         WHERE CONCAT(open_date_year, open_date_month, open_date_day) >= $1
           AND issue_date <= $2
           AND market_type in ('上市')
           AND security_type in ('ETF', 'ETN', '股票', '特別股')
         ORDER BY security_code, issue_date, market_type, security_type
          ",
    )
    .bind(q_open_date)
    .bind(q_issue_date)
    .map(|row: PgRow| SecurityTemp {
        row_id: row.get("row_id"),
        open_date_year: row.get("open_date_year"),
        open_date_month: row.get("open_date_month"),
        open_date_day: row.get("open_date_day"),
        international_code: row.get("international_code"),
        security_code: row.get("security_code"),
        security_name: row.get("security_name"),
        market_type: row.get("market_type"),
        security_type: row.get("security_type"),
        industry_type: row.get("industry_type"),
        issue_date: row.get("issue_date"),
        cfi_code: row.get("cfi_code"),
        remark: row.get("remark"),
    })
    .fetch_all(&conn)
    .await
    {
        Ok(rows) => rows,
        Err(e) => {
            event!(target: "security_api", Level::ERROR, "security_temp.find_one: {}", &e);
            Vec::new()
        }
    }
}

pub async fn find_all_by_tpex(task: DailyTask) -> Vec<SecurityTemp> {
    let dao = Repository::new().await;
    let conn = dao.connection;

    let q_year = task.clone().open_date_year;
    let q_month = task.clone().open_date_month;
    let q_day = task.clone().open_date_day;
    let q_open_date = format!("{0}{1}{2}", q_year, q_month, q_day);
    let q_issue_date = format!("{0}/{1}/{2}", q_year, q_month, q_day);

    match sqlx::query(r"
        SELECT row_id
             , open_date_year
             , open_date_month
             , open_date_day
             , international_code
             , security_code
             , security_name
             , market_type
             , security_type
             , industry_type
             , issue_date
             , cfi_code
             , remark
          FROM security_temp 
         WHERE CONCAT(open_date_year, open_date_month, open_date_day) >= $1
           AND issue_date <= $2
           AND market_type in ('上櫃', '興櫃')
           AND security_type in ('ETF', 'ETN', '股票', '特別股')
         ORDER BY security_code, issue_date, market_type, security_type
          ",
    )
    .bind(q_open_date)
    .bind(q_issue_date)
    .map(|row: PgRow| SecurityTemp {
        row_id: row.get("row_id"),
        open_date_year: row.get("open_date_year"),
        open_date_month: row.get("open_date_month"),
        open_date_day: row.get("open_date_day"),
        international_code: row.get("international_code"),
        security_code: row.get("security_code"),
        security_name: row.get("security_name"),
        market_type: row.get("market_type"),
        security_type: row.get("security_type"),
        industry_type: row.get("industry_type"),
        issue_date: row.get("issue_date"),
        cfi_code: row.get("cfi_code"),
        remark: row.get("remark"),
    })
    .fetch_all(&conn)
    .await
    {
        Ok(rows) => rows,
        Err(e) => {
            event!(target: "security_api", Level::ERROR, "security_temp.find_one: {}", &e);
            Vec::new()
        }
    }
}