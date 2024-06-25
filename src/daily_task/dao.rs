#![warn(clippy::all, clippy::pedantic)]

use chrono::{Local, NaiveDate};
use sqlx::{postgres::PgRow, PgConnection, Row};
use tracing::{event, Level};

use super::model::{DailyTask, DailyTaskInfo};

pub async fn read_all(
    transaction: &mut PgConnection,
    data: DailyTask,
) -> Result<(usize, Vec<DailyTask>), sqlx::Error> {
    let mut select_str = r#" 
        SELECT row_id
             , open_date
             , job_code
             , exec_status
             , created_date
             , updated_date
          FROM daily_task
    "#
    .to_string();

    let mut index = 0;
    if data.open_date.is_some() {
        select_str.push_str(&where_append("open_date", "=", &mut index));
    }
    if data.job_code.is_some() {
        select_str.push_str(&where_append("job_code", "=", &mut index));
    }
    if data.exec_status.is_some() {
        select_str.push_str(&where_append("exec_status", "=", &mut index));
    }

    let mut query = sqlx::query(&select_str);

    if data.open_date.is_some() {
        query = query.bind(data.open_date.clone());
    }
    if data.job_code.is_some() {
        query = query.bind(data.job_code.clone());
    }
    if data.exec_status.is_some() {
        query = query.bind(data.exec_status.clone());
    }

    match query
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
            event!(target: "security_api", Level::ERROR, "daily_task.read_all: {}", &e);
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
    date: NaiveDate,
) -> Result<Vec<DailyTaskInfo>, sqlx::Error> {
    match sqlx::query(
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
         WHERE dt.open_date <= $1 
           AND dt.exec_status in ('WAIT', 'OPEN', 'EXEC')
         ORDER BY dt.open_date desc, ts.sort_no
         "#,
    )
    .bind(date.format("%Y%m%d").to_string())
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

pub async fn read_all_by_daily1(
    transaction: &mut PgConnection,
    date: NaiveDate,
) -> Result<Vec<DailyTaskInfo>, sqlx::Error> {
    match sqlx::query(
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
         WHERE dt.open_date <= $1 
           AND dt.exec_status in ('WAIT', 'OPEN', 'EXEC')
         ORDER BY dt.open_date, ts.sort_no
         "#,
    )
    .bind(date.format("%Y%m%d").to_string())
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
