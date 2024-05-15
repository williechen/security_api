use chrono::Local;
use sqlx::{postgres::PgRow, Row};
use tracing::{event, Level};

use super::model::DailyTask;

pub async fn read_all(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    data: &DailyTask,
) -> Result<(usize, Vec<DailyTask>), sqlx::Error> {
    let mut select_str = r#" 
        SELECT row_id
             , version_code
             , open_date
             , job_code
             , exec_status
             , created_date
             , updated_date
          FROM daily_task
    "#
    .to_string();

    let mut index = 0;
    if data.version_code.is_some() {
        select_str.push_str(&where_append("version_code", "=", &mut index));
    }
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

    if data.version_code.is_some() {
        query = query.bind(data.version_code.clone());
    }
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
            version_code: row.get("version_code"),
            open_date: row.get("open_date"),
            job_code: row.get("job_code"),
            exec_status: row.get("exec_status"),
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
) -> Result<(usize, Vec<DailyTask>), sqlx::Error> {
    match sqlx::query(sql)
        .map(|row: PgRow| DailyTask {
            row_id: row.get("row_id"),
            version_code: row.get("version_code"),
            open_date: row.get("open_date"),
            job_code: row.get("job_code"),
            exec_status: row.get("exec_status"),
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
) -> Result<Option<DailyTask>, sqlx::Error> {
    match sqlx::query(
        r#"
        SELECT row_id
             , version_code
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
        version_code: row.get("version_code"),
        open_date: row.get("open_date"),
        job_code: row.get("job_code"),
        exec_status: row.get("exec_status"),
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
    data: DailyTask,
) -> Result<u64, sqlx::Error> {
    match sqlx::query(
        r#" 
        INSERT INTO daily_task(version_code
             , open_date
             , job_code
             , exec_status
             , created_date
             , updated_date
        ) VALUES ($1, $2, $3, $4, $5, $6)  "#,
    )
    .bind(data.version_code)
    .bind(data.open_date)
    .bind(data.job_code)
    .bind(data.exec_status)
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
    data: DailyTask,
) -> Result<u64, sqlx::Error> {
    match sqlx::query(
        r#" UPDATE daily_task
            SET version_code = $1
              , open_date = $2
              , job_code = $3
              , exec_status = $4
              , updated_date = $5
            WHERE row_id = $6
          "#,
    )
    .bind(data.version_code)
    .bind(data.open_date)
    .bind(data.job_code)
    .bind(data.exec_status)
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
    data: DailyTask,
) -> Result<u64, sqlx::Error> {
    match sqlx::query(r#" DELETE FROM daily_task WHERE row_id = $1 "#)
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
