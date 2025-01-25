#![warn(clippy::all, clippy::pedantic)]
use chrono::{Datelike, Local};
use sqlx::{postgres::PgRow, Row};
use tracing::{event, Level};

use crate::repository::Repository;

use super::model::DailyTask;

pub async fn create(data: DailyTask) -> Result<u64, sqlx::Error> {
    let dao = Repository::new().await;
    let conn = dao.connection;

    match sqlx::query(
        r"
        INSERT INTO daily_task(
            open_date_year
          , open_date_month
          , open_date_day
          , job_code
          , exec_status
          , created_date
          , updated_date
        ) VALUES ( $1, $2, $3, $4, $5, $6, $7 )
    ",
    )
    .bind(data.open_date_year)
    .bind(data.open_date_month)
    .bind(data.open_date_day)
    .bind(data.job_code)
    .bind(data.exec_status)
    .bind(Local::now())
    .bind(Local::now())
    .execute(&conn)
    .await
    {
        Ok(cnt) => Ok(cnt.rows_affected()),
        Err(e) => Err(e),
    }
}

pub async fn modify(data: DailyTask) -> Result<u64, sqlx::Error> {
    let dao = Repository::new().await;
    let conn = dao.connection;

    match sqlx::query(
        r"
        UPDATE daily_task 
           SET exec_status = $1
             , updated_date = $2
         WHERE open_date_year = $3
           AND open_date_month = $4
           AND open_date_day = $5
           AND job_code = $6
    ",
    )
    .bind(data.exec_status)
    .bind(Local::now())
    .bind(data.open_date_year)
    .bind(data.open_date_month)
    .bind(data.open_date_day)
    .bind(data.job_code)
    .execute(&conn)
    .await
    {
        Ok(cnt) => Ok(cnt.rows_affected()),
        Err(e) => Err(e),
    }
}

pub async fn find_all() -> Vec<DailyTask> {
    let dao = Repository::new().await;
    let conn = dao.connection;

    let now = Local::now();

    match sqlx::query(
        r"
        SELECT '' AS row_id
             , cd.ce_year AS open_date_year
             , cd.ce_month AS open_date_month
             , cd.ce_day AS open_date_day
             , ts.job_code 
             , 'WAIT' AS exec_status
          FROM calendar_data cd
          JOIN task_setting ts
            ON cd.group_task = ts.group_code
           AND ts.is_enabled = 1
         WHERE NOT EXISTS (
               SELECT 1 
                 FROM daily_task dt
                WHERE dt.open_date_year = cd.ce_year 
                  AND dt.open_date_month = cd.ce_month
                  AND dt.open_date_day = cd.ce_day
                  AND dt.job_code = ts.job_code
         )
           AND concat(cd.ce_year,cd.ce_month,cd.ce_day) = $1
           AND cd.date_status = 'O'
         ORDER BY cd.ce_year desc, cd.ce_month desc, cd.ce_day desc, ts.sort_no ",
    )
    .bind(format!(
        "{0:04}{1:02}{2:02}",
        now.year(),
        now.month(),
        now.day()
    ))
    .map(|row: PgRow| DailyTask {
        row_id: row.get("row_id"),
        open_date_year: row.get("open_date_year"),
        open_date_month: row.get("open_date_month"),
        open_date_day: row.get("open_date_day"),
        job_code: row.get("job_code"),
        exec_status: row.get("exec_status"),
    })
    .fetch_all(&conn)
    .await
    {
        Ok(rows) => rows,
        Err(e) => {
            event!(target: "security_api", Level::ERROR, "daily_task.find_all: {}", &e);
            Vec::new()
        }
    }
}

pub async fn find_one(
    q_year: &str,
    q_month: &str,
    q_day: &str,
    q_job_code: &str,
) -> Option<DailyTask> {
    let dao = Repository::new().await;
    let conn = dao.connection;

    match sqlx::query(
        r"
        SELECT row_id
             , open_date_year
             , open_date_month
             , open_date_day
             , job_code
             , exec_status
          FROM daily_task
         WHERE open_date_year = $1
           AND open_date_month = $2
           And open_date_day = $3
           And job_code = $4
    ",
    )
    .bind(q_year)
    .bind(q_month)
    .bind(q_day)
    .bind(q_job_code)
    .map(|row: PgRow| DailyTask {
        row_id: row.get("row_id"),
        job_code: row.get("job_code"),
        exec_status: row.get("exec_status"),
        open_date_year: row.get("exec_status"),
        open_date_month: row.get("exec_status"),
        open_date_day: row.get("exec_status"),
    })
    .fetch_optional(&conn)
    .await
    {
        Ok(row) => row,
        Err(e) => {
            event!(target: "security_api", Level::ERROR, "daily_task.find_one: {}", &e);
            None
        }
    }
}

pub async fn find_one_by_exec_asc(flow_code: &str) -> Option<DailyTask> {
    let dao = Repository::new().await;
    let conn = dao.connection;

    match sqlx::query(
        r"
        SELECT distinct '' AS row_id
                 , '' AS job_code 
                 , '' AS exec_status
                 , now() AS created_date
                 , now() AS updated_date 
                 , dt.open_date_year
                 , dt.open_date_month
                 , dt.open_date_day
          FROM daily_task dt
         WHERE dt.exec_status in ('WAIT', 'OPEN', 'EXEC')
           AND NOT EXISTS (
               SELECT 1 
                 FROM listen_flow lf
                WHERE lf.flow_code = $1
                  AND lf.flow_param1 = dt.open_date_year
                  AND lf.flow_param2 = dt.open_date_month
            )
         ORDER BY dt.open_date_year, dt.open_date_month, dt.open_date_day
         Limit 1
    ",
    )
    .bind(flow_code)
    .map(|row: PgRow| DailyTask {
        row_id: row.get("row_id"),
        open_date_year: row.get("open_date_year"),
        open_date_month: row.get("open_date_month"),
        open_date_day: row.get("open_date_day"),
        job_code: row.get("job_code"),
        exec_status: row.get("exec_status"),
    })
    .fetch_optional(&conn)
    .await
    {
        Ok(row) => row,
        Err(e) => {
            event!(target: "security_api", Level::ERROR, "daily_task.find_one_by_exec_asc: {}", &e);
            None
        }
    }
}

pub async fn find_one_by_exec_desc(flow_code: &str) -> Option<DailyTask> {
    let dao = Repository::new().await;
    let conn = dao.connection;

    match sqlx::query(
        r"
        SELECT distinct '' AS row_id
                 , '' AS job_code 
                 , '' AS exec_status
                 , now() AS created_date
                 , now() AS updated_date 
                 , dt.open_date_year
                 , dt.open_date_month
                 , dt.open_date_day
          FROM daily_task dt
         WHERE dt.exec_status in ('WAIT', 'OPEN', 'EXEC')
           AND NOT EXISTS (
               SELECT 1 
                 FROM listen_flow lf
                WHERE lf.flow_code = $1
                  AND lf.flow_param1 = dt.open_date_year
                  AND lf.flow_param2 = dt.open_date_month
            )
         ORDER BY dt.open_date_year desc, dt.open_date_month desc, dt.open_date_day desc
         Limit 1
    ",
    )
    .bind(flow_code)
    .map(|row: PgRow| DailyTask {
        row_id: row.get("row_id"),
        open_date_year: row.get("open_date_year"),
        open_date_month: row.get("open_date_month"),
        open_date_day: row.get("open_date_day"),
        job_code: row.get("job_code"),
        exec_status: row.get("exec_status"),
    })
    .fetch_optional(&conn)
    .await
    {
        Ok(row) => row,
        Err(e) => {
            event!(target: "security_api", Level::ERROR, "daily_task.find_one_by_exec_desc: {}", &e);
            None
        }
    }
}

pub async fn find_all_by_exec_asc(q_year: &str, q_month: &str) -> Vec<DailyTask> {
    let dao = Repository::new().await;
    let conn = dao.connection;

    match sqlx::query(
        r"
        SELECT dt.row_id
             , dt.open_date_year
             , dt.open_date_month
             , dt.open_date_day
             , dt.job_code
             , dt.exec_status
          FROM daily_task dt
          JOIN calendar_data cd
            ON dt.open_date_year = cd.ce_year
           AND dt.open_date_month = cd.ce_month
           AND dt.open_date_day = cd.ce_day
          JOIN task_setting ts
            ON ts.group_code = cd.group_task 
           AND ts.job_code = dt.job_code
           AND ts.is_enabled = 1
         WHERE dt.open_date_year = $1
           AND dt.open_date_month = $2
           AND dt.exec_status in ('WAIT', 'OPEN', 'EXEC')
         ORDER BY dt.open_date_year, dt.open_date_month, dt.open_date_day,ts.sort_no
    ",
    )
    .bind(q_year)
    .bind(q_month)
    .map(|row: PgRow| DailyTask {
        row_id: row.get("row_id"),
        open_date_year: row.get("open_date_year"),
        open_date_month: row.get("open_date_month"),
        open_date_day: row.get("open_date_day"),
        job_code: row.get("job_code"),
        exec_status: row.get("exec_status"),
    })
    .fetch_all(&conn)
    .await
    {
        Ok(rows) => rows,
        Err(e) => {
            event!(target: "security_api", Level::ERROR, "daily_task.find_all_by_exec_asc: {}", &e);
            Vec::new()
        }
    }
}

pub async fn find_all_by_exec_desc(q_year: &str, q_month: &str) -> Vec<DailyTask> {
    let dao = Repository::new().await;
    let conn = dao.connection;

    match sqlx::query(
        r"
        SELECT dt.row_id
             , dt.open_date_year
             , dt.open_date_month
             , dt.open_date_day
             , dt.job_code
             , dt.exec_status
          FROM daily_task dt
          JOIN calendar_data cd
            ON dt.open_date_year = cd.ce_year
           AND dt.open_date_month = cd.ce_month
           AND dt.open_date_day = cd.ce_day
          JOIN task_setting ts
            ON ts.group_code = cd.group_task 
           AND ts.job_code = dt.job_code
           AND ts.is_enabled = 1
         WHERE dt.open_date_year = $1
           AND dt.open_date_month = $2
           AND dt.exec_status in ('WAIT', 'OPEN', 'EXEC')
         ORDER BY dt.open_date_year desc, dt.open_date_month desc, dt.open_date_day desc,ts.sort_no
    ",
    )
    .bind(q_year)
    .bind(q_month)
    .map(|row: PgRow| DailyTask {
        row_id: row.get("row_id"),
        open_date_year: row.get("open_date_year"),
        open_date_month: row.get("open_date_month"),
        open_date_day: row.get("open_date_day"),
        job_code: row.get("job_code"),
        exec_status: row.get("exec_status"),
    })
    .fetch_all(&conn)
    .await
    {
        Ok(rows) => rows,
        Err(e) => {
            event!(target: "security_api", Level::ERROR, "daily_task.find_all_by_exec_desc: {}", &e);
            Vec::new()
        }
    }
}
