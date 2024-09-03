#![warn(clippy::all, clippy::pedantic)]

use chrono::Local;
use sqlx::{postgres::PgRow, Row};
use tracing::{event, Level};

use crate::repository::Repository;

use super::model::CalendarData;

pub async fn create(data: CalendarData) -> Result<u64, sqlx::Error> {
    let dao = Repository::new().await;
    let conn = dao.connection;

    match sqlx::query(
        r"
        INSERT INTO calendar_data(
            ce_year, ce_month, ce_day, data_status, group_task, created_date, updated_date
        ) VALUES ( $1, $2, $3, $4, $5, $6, $7 )
    ",
    )
    .bind(data.ce_year)
    .bind(data.ce_month)
    .bind(data.ce_day)
    .bind(data.date_status)
    .bind(data.group_task)
    .bind(Local::now())
    .bind(Local::now())
    .execute(&conn)
    .await
    {
        Ok(cnt) => Ok(cnt.rows_affected()),
        Err(e) => Err(e),
    }
}

pub async fn modify(data: CalendarData) -> Result<u64, sqlx::Error> {
    let dao = Repository::new().await;
    let conn = dao.connection;

    match sqlx::query(
        r"
        UPDATE calendar_data 
           SET ce_year = $1
             , ce_month = $2
             , ce_day = $3
             , data_status = $4
             , group_task = $5
             , updated_date = $6
         WHERE row_id = $7
    ",
    )
    .bind(data.ce_year)
    .bind(data.ce_month)
    .bind(data.ce_day)
    .bind(data.date_status)
    .bind(data.group_task)
    .bind(Local::now())
    .bind(data.row_id)
    .execute(&conn)
    .await
    {
        Ok(cnt) => Ok(cnt.rows_affected()),
        Err(e) => Err(e),
    }
}

pub async fn find_one(q_year: String, q_month: String, q_day: String) -> Option<CalendarData> {
    let dao = Repository::new().await;
    let conn = dao.connection;

    match sqlx::query(
        r"
        SELECT row_id
             , ce_year
             , ce_month
             , ce_day
             , date_status
             , group_task
          FROM calendar_data
         WHERE ce_year = $1
           AND ce_month = $2
           AND ce_day = $3
    ",
    )
    .bind(q_year)
    .bind(q_month)
    .bind(q_day)
    .map(|row: PgRow| CalendarData {
        row_id: row.get("row_id"),
        ce_year: row.get("ce_year"),
        ce_month: row.get("ce_month"),
        ce_day: row.get("ce_day"),
        date_status: row.get("date_status"),
        group_task: row.get("group_task"),
    })
    .fetch_optional(&conn)
    .await
    {
        Ok(row) => row,
        Err(e) => {
            event!(target: "security_api", Level::ERROR, "calendar_data.find_one: {}", &e);
            None
        }
    }
}

pub async fn find_one_by_work_day_first(q_year: String, q_month: String) -> Option<CalendarData> {
    let dao = Repository::new().await;
    let conn = dao.connection;

    match sqlx::query(
        r"
        SELECT cd.row_id
             , cd.ce_year
             , cd.ce_month
             , cd.ce_day
             , cd.date_status
             , cd.group_task
          FROM calendar_data cd
         WHERE cd.ce_year = $1
           AND cd.ce_month = $2
           AND cd.date_status = 'O'
           AND NOT EXISTS (
               SELECT 1 
                 FROM calendar_data cd1 
                WHERE cd1.ce_year = cd.ce_year
                  AND cd1.ce_month = cd.ce_month
                  AND cd.date_status = 'O'
                  AND cd1.group_task IN ('FIRST', 'FIRST_INIT')
           )
         ORDER BY Concat(cd.ce_year, cd.ce_month, cd.ce_day)
         LIMIT 1
           ",
    )
    .bind(q_year)
    .bind(q_month)
    .map(|row: PgRow| CalendarData {
        row_id: row.get("row_id"),
        ce_year: row.get("ce_year"),
        ce_month: row.get("ce_month"),
        ce_day: row.get("ce_day"),
        date_status: row.get("date_status"),
        group_task: row.get("group_task"),
    })
    .fetch_optional(&conn)
    .await
    {
        Ok(row) => row,
        Err(e) => {
            event!(target: "security_api", Level::ERROR, "calendar_data.find_one_by_work_day_first: {}", &e);
            None
        }
    }
}
