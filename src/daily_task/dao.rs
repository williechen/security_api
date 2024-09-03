#![warn(clippy::all, clippy::pedantic)]
use chrono::{Datelike, Local};
use sqlx::{postgres::PgRow, PgConnection, Row};
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
           SET open_date_year = $1
             , open_date_month = $2
             , open_date_day = $3
             , job_code = $4
             , exec_status = $5
             , updated_date = $6
         WHERE open_date_year = $7
           AND open_date_month = $8
           AND open_date_day = $9
           AND job_code = $10
    ",
    )
    .bind(&data.open_date_year)
    .bind(&data.open_date_month)
    .bind(&data.open_date_day)
    .bind(&data.job_code)
    .bind(data.exec_status)
    .bind(Local::now())
    .bind(&data.open_date_year)
    .bind(&data.open_date_month)
    .bind(&data.open_date_day)
    .bind(&data.job_code)
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
        "{:04}{:02}{:02}",
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

pub async fn read_all_by_sql(
    transaction: &mut PgConnection,
    sql: &str,
) -> Result<(usize, Vec<DailyTask>), sqlx::Error> {
    match sqlx::query(sql)
        .map(|row: PgRow| DailyTask {
            row_id: row.get("row_id"),
            open_date: row.get("open_date"),
            job_code: row.get("job_code"),
            exec_status: row.get("exec_status"),
        })
        .fetch_all(transaction)
        .await
    {
        Ok(rows) => Ok((rows.len(), rows)),
        Err(e) => {
            event!(target: "security_api", Level::ERROR, "daily_task.read_all_by_sql: {}", &e);
            Err(e)
        }
    }
}

pub async fn read(
    transaction: &mut PgConnection,
    row_id: &str,
) -> Result<Option<DailyTask>, sqlx::Error> {
    match sqlx::query(
        r#"
        SELECT row_id
             , open_date
             , job_code
             , exec_status
             , created_date
             , updated_date
          FROM daily_task
         WHERE row_id = $1 "#,
    )
    .bind(row_id)
    .map(|row: PgRow| DailyTask {
        row_id: row.get("row_id"),
        open_date: row.get("open_date"),
        job_code: row.get("job_code"),
        exec_status: row.get("exec_status"),
    })
    .fetch_one(transaction)
    .await
    {
        Ok(row) => Ok(Some(row)),
        Err(e) => {
            event!(target: "security_api", Level::ERROR, "daily_task.read: {}", &e);
            Err(e)
        }
    }
}

pub async fn create(transaction: &mut PgConnection, data: DailyTask) -> Result<u64, sqlx::Error> {
    match sqlx::query(
        r#" 
        INSERT INTO daily_task(open_date
             , job_code
             , exec_status
             , created_date
             , updated_date
        ) VALUES ($1, $2, $3, $4, $5)  "#,
    )
    .bind(data.open_date)
    .bind(data.job_code)
    .bind(data.exec_status)
    .bind(Local::now().naive_local())
    .bind(Local::now().naive_local())
    .execute(transaction)
    .await
    {
        Ok(row) => Ok(row.rows_affected()),
        Err(e) => {
            event!(target: "security_api", Level::ERROR, "daily_task.create: {}", &e);
            Err(e)
        }
    }
}

pub async fn update(transaction: &mut PgConnection, data: DailyTask) -> Result<u64, sqlx::Error> {
    match sqlx::query(
        r#" UPDATE daily_task
            SET open_date = $1
              , job_code = $2
              , exec_status = $3
              , updated_date = $4
            WHERE row_id = $5
          "#,
    )
    .bind(data.open_date)
    .bind(data.job_code)
    .bind(data.exec_status)
    .bind(Local::now().naive_local())
    .bind(data.row_id)
    .execute(transaction)
    .await
    {
        Ok(row) => Ok(row.rows_affected()),
        Err(e) => {
            event!(target: "security_api", Level::ERROR, "daily_task.update: {}", &e);
            Err(e)
        }
    }
}

pub async fn delete(transaction: &mut PgConnection, data: DailyTask) -> Result<u64, sqlx::Error> {
    match sqlx::query(r#" DELETE FROM daily_task WHERE row_id = $1 "#)
        .bind(data.row_id)
        .execute(transaction)
        .await
    {
        Ok(row) => Ok(row.rows_affected()),
        Err(e) => {
            event!(target: "security_api", Level::ERROR, "daily_task.delete: {}", &e);
            Err(e)
        }
    }
}

pub async fn read_all_by_daily(
    transaction: &mut PgConnection,
    ce_year: &str,
    ce_month: &str,
    orderby: &str,
) -> Result<Vec<DailyTaskInfo>, sqlx::Error> {
    match sqlx::query(&format!(
        r#"
        SELECT dt.row_id
             , dt.job_code
             , dt.open_date
             , CONCAT(cd.ce_year, '/', cd.ce_month, '/', cd.ce_day) AS ce_date
             , CONCAT(cd.tw_year, '/', cd.ce_month) AS tw_date
             , ts.wait_type
             , ts.wait_number
             , dt.exec_status
          FROM daily_task dt
          JOIN calendar_data cd
            ON dt.open_date = CONCAT(cd.ce_year, cd.ce_month, cd.ce_day)
          JOIN task_setting ts
            ON dt.job_code = ts.job_code
           AND cd.group_task = ts.group_code
         WHERE cd.ce_year = $1
           AND cd.ce_month = $2
           AND dt.exec_status in ('WAIT', 'OPEN', 'EXEC')
         ORDER BY {}, ts.sort_no
         "#,
        orderby
    ))
    .bind(ce_year)
    .bind(ce_month)
    .map(|row: PgRow| DailyTaskInfo {
        row_id: row.get("row_id"),
        job_code: row.get("job_code"),
        exec_status: row.get("exec_status"),
        open_date: row.get("open_date"),
        ce_date: row.get("ce_date"),
        tw_date: row.get("tw_date"),
        wait_type: row.get("wait_type"),
        wait_number: row.get("wait_number"),
    })
    .fetch_all(transaction)
    .await
    {
        Ok(row) => Ok(row),
        Err(e) => {
            event!(target: "security_api", Level::ERROR, "daily_task.read_all_by_daily: {}", &e);
            Err(e)
        }
    }
}

pub async fn read_by_exec(
    transaction: &mut PgConnection,
    flow_code: &str,
    orderby: &str,
) -> Result<Vec<String>, sqlx::Error> {
    match sqlx::query(&format!(
        r#"
        SELECT distinct dt.open_date
          FROM daily_task dt
          JOIN calendar_data cd
            ON dt.open_date = CONCAT(cd.ce_year, cd.ce_month, cd.ce_day)
         WHERE dt.exec_status in ('WAIT', 'OPEN', 'EXEC')
           AND NOT EXISTS (
               SELECT 1 
                 FROM listen_flow lf
                WHERE lf.flow_code = $1
                  AND lf.flow_param1 = cd.ce_year
                  AND lf.flow_param2 = cd.ce_month
            )
         ORDER BY {}
         "#,
        orderby
    ))
    .bind(flow_code)
    .map(|row: PgRow| row.get("open_date"))
    .fetch_all(transaction)
    .await
    {
        Ok(row) => Ok(row),
        Err(e) => {
            event!(target: "security_api", Level::ERROR, "daily_task.read_all_by_daily: {}", &e);
            Err(e)
        }
    }
}
