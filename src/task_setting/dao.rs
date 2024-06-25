#![warn(clippy::all, clippy::pedantic)]

use chrono::Local;
use sqlx::{postgres::PgRow, Row};
use tracing::{event, Level};

use super::model::TaskSetting;

pub async fn read_all(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    data: &TaskSetting,
) -> Result<(usize, Vec<TaskSetting>), sqlx::Error> {
    let mut select_str = r#" 
        SELECT row_id
             , group_code
             , job_code
             , wait_type
             , wait_number
             , is_enabled
             , sort_no
             , created_date
             , updated_date
          FROM task_setting
    "#
    .to_string();

    let mut index = 0;
    if data.group_code.is_some() {
        select_str.push_str(&where_append("group_code", "=", &mut index));
    }
    if data.job_code.is_some() {
        select_str.push_str(&where_append("job_code", "=", &mut index));
    }
    if data.wait_type.is_some() {
        select_str.push_str(&where_append("wait_type", "=", &mut index));
    }
    if data.wait_number.is_some() {
        select_str.push_str(&where_append("wait_number", "=", &mut index));
    }
    if data.is_enabled.is_some() {
        select_str.push_str(&where_append("is_enabled", "=", &mut index));
    }

    let mut query = sqlx::query(&select_str);

    if data.group_code.is_some() {
        query = query.bind(data.group_code.clone());
    }
    if data.job_code.is_some() {
        query = query.bind(data.job_code.clone());
    }
    if data.wait_type.is_some() {
        query = query.bind(data.wait_type.clone());
    }
    if data.wait_number.is_some() {
        query = query.bind(data.wait_number.clone());
    }
    if data.is_enabled.is_some() {
        query = query.bind(data.is_enabled.clone());
    }

    match query
        .map(|row: PgRow| TaskSetting {
            row_id: row.get("row_id"),
            group_code: row.get("group_code"),
            job_code: row.get("job_code"),
            wait_type: row.get("wait_type"),
            wait_number: row.get("wait_number"),
            is_enabled: row.get("is_enabled"),
            sort_no: row.get("sort_no"),
        })
        .fetch_all(&mut **transaction)
        .await
    {
        Ok(rows) => Ok((rows.len(), rows)),
        Err(e) => {
            event!(target: "security_api", Level::ERROR, "task_setting.read_all: {}", &e);
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
) -> Result<(usize, Vec<TaskSetting>), sqlx::Error> {
    match sqlx::query(sql)
        .map(|row: PgRow| TaskSetting {
            row_id: row.get("row_id"),
            group_code: row.get("group_code"),
            job_code: row.get("job_code"),
            wait_type: row.get("wait_type"),
            wait_number: row.get("wait_number"),
            is_enabled: row.get("is_enabled"),
            sort_no: row.get("sort_no"),
        })
        .fetch_all(&mut **transaction)
        .await
    {
        Ok(rows) => Ok((rows.len(), rows)),
        Err(e) => {
            event!(target: "security_api", Level::ERROR, "task_setting.read_all_by_sql: {}", &e);
            Err(e)
        }
    }
}

pub async fn read(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    row_id: &str,
) -> Result<Option<TaskSetting>, sqlx::Error> {
    match sqlx::query(
        r#"
        SELECT row_id
             , group_code
             , job_code
             , wait_type
             , wait_number
             , is_enabled
             , sort_no
             , created_date
             , updated_date
          FROM task_setting
         WHERE row_id = $1 "#,
    )
    .bind(row_id)
    .map(|row: PgRow| TaskSetting {
        row_id: row.get("row_id"),
        group_code: row.get("group_code"),
        job_code: row.get("job_code"),
        wait_type: row.get("wait_type"),
        wait_number: row.get("wait_number"),
        is_enabled: row.get("is_enabled"),
        sort_no: row.get("sort_no"),
    })
    .fetch_one(&mut **transaction)
    .await
    {
        Ok(row) => Ok(Some(row)),
        Err(e) => {
            event!(target: "security_api", Level::ERROR, "task_setting.read: {}", &e);
            Err(e)
        }
    }
}

pub async fn create(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    data: TaskSetting,
) -> Result<u64, sqlx::Error> {
    match sqlx::query(
        r#" 
        INSERT INTO task_setting(group_code
            , job_code
            , wait_type
            , wait_number
            , is_enabled
            , sort_no
            , created_date
            , updated_date
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)  "#,
    )
    .bind(data.group_code)
    .bind(data.job_code)
    .bind(data.wait_type)
    .bind(data.wait_number)
    .bind(data.is_enabled)
    .bind(data.sort_no)
    .bind(Local::now().naive_local())
    .bind(Local::now().naive_local())
    .execute(&mut **transaction)
    .await
    {
        Ok(row) => Ok(row.rows_affected()),
        Err(e) => {
            event!(target: "security_api", Level::ERROR, "task_setting.create: {}", &e);
            Err(e)
        }
    }
}

pub async fn update(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    data: TaskSetting,
) -> Result<u64, sqlx::Error> {
    match sqlx::query(
        r#" UPDATE task_setting
            SET group_code = $1
              , job_code = $2
              , wait_type = $3
              , wait_number = $4
              , is_enabled = $5
              , sort_no = $6
              , updated_date = $7
            WHERE row_id = $8
          "#,
    )
    .bind(data.group_code)
    .bind(data.job_code)
    .bind(data.wait_type)
    .bind(data.wait_number)
    .bind(data.is_enabled)
    .bind(data.sort_no)
    .bind(Local::now().naive_local())
    .bind(data.row_id)
    .execute(&mut **transaction)
    .await
    {
        Ok(row) => Ok(row.rows_affected()),
        Err(e) => {
            event!(target: "security_api", Level::ERROR, "task_setting.update: {}", &e);
            Err(e)
        }
    }
}

pub async fn delete(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    data: TaskSetting,
) -> Result<u64, sqlx::Error> {
    match sqlx::query(r#" DELETE FROM task_setting WHERE row_id = $1 "#)
        .bind(data.row_id)
        .execute(&mut **transaction)
        .await
    {
        Ok(row) => Ok(row.rows_affected()),
        Err(e) => {
            event!(target: "security_api", Level::ERROR, "task_setting.delete: {}", &e);
            Err(e)
        }
    }
}
