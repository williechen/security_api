use chrono::Local;
use sqlx::{postgres::PgRow, Row};
use tracing::{event, instrument, Level};

use super::model::CalendarData;

#[instrument]
pub async fn read_all(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    data: &CalendarData,
) -> Result<(usize, Vec<CalendarData>), sqlx::Error> {
    let mut select_str = r#" 
        SELECT row_id
             , ce_year
             , tw_year
             , ce_month
             , ce_day
             , date_status
             , group_task
             , created_date
             , updated_date
          FROM calendar_data
    "#
    .to_string();

    let mut index = 0;
    if data.ce_year.is_some() {
        select_str.push_str(&where_append("ce_year", "=", &mut index));
    }
    if data.tw_year.is_some() {
        select_str.push_str(&where_append("tw_year", "=", &mut index));
    }
    if data.ce_month.is_some() {
        select_str.push_str(&where_append("ce_month", "=", &mut index));
    }
    if data.ce_day.is_some() {
        select_str.push_str(&where_append("ce_day", "=", &mut index));
    }
    if data.date_status.is_some() {
        select_str.push_str(&where_append("date_status", "=", &mut index));
    }
    if data.group_task.is_some() {
        select_str.push_str(&where_append("group_task", "=", &mut index));
    }

    let mut query = sqlx::query(&select_str);

    if data.ce_year.is_some() {
        query = query.bind(data.ce_year.clone());
    }
    if data.tw_year.is_some() {
        query = query.bind(data.tw_year.clone());
    }
    if data.ce_month.is_some() {
        query = query.bind(data.ce_month.clone());
    }
    if data.ce_day.is_some() {
        query = query.bind(data.ce_day.clone());
    }
    if data.date_status.is_some() {
        query = query.bind(data.date_status.clone());
    }
    if data.group_task.is_some() {
        query = query.bind(data.group_task.clone());
    }

    match query
        .map(|row: PgRow| CalendarData {
            row_id: row.get("row_id"),
            ce_year: row.get("ce_year"),
            tw_year: row.get("tw_year"),
            ce_month: row.get("ce_month"),
            ce_day: row.get("ce_day"),
            date_status: row.get("date_status"),
            group_task: row.get("group_task"),
        })
        .fetch_all(&mut **transaction)
        .await
    {
        Ok(rows) => Ok((rows.len(), rows)),
        Err(e) => {
            event!(target: "security_api", Level::ERROR, "calendar_data.read_all: {}", &e);
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
) -> Result<(usize, Vec<CalendarData>), sqlx::Error> {
    match sqlx::query(sql)
        .map(|row: PgRow| CalendarData {
            row_id: row.get("row_id"),
            ce_year: row.get("ce_year"),
            tw_year: row.get("tw_year"),
            ce_month: row.get("ce_month"),
            ce_day: row.get("ce_day"),
            date_status: row.get("date_status"),
            group_task: row.get("group_task"),
        })
        .fetch_all(&mut **transaction)
        .await
    {
        Ok(rows) => Ok((rows.len(), rows)),
        Err(e) => {
            event!(target: "security_api", Level::ERROR, "calendar_data.read_all_by_sql: {}", &e);
            Err(e)
        }
    }
}

pub async fn read(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    row_id: &str,
) -> Result<Option<CalendarData>, sqlx::Error> {
    match sqlx::query(
        r#"
        SELECT row_id
             , ce_year
             , tw_year
             , ce_month
             , ce_day
             , date_status
             , group_task
             , created_date
             , updated_date
          FROM calendar_data
         WHERE row_id = $1 "#,
    )
    .bind(row_id)
    .map(|row: PgRow| CalendarData {
        row_id: row.get("row_id"),
        ce_year: row.get("ce_year"),
        tw_year: row.get("tw_year"),
        ce_month: row.get("ce_month"),
        ce_day: row.get("ce_day"),
        date_status: row.get("date_status"),
        group_task: row.get("group_task"),
    })
    .fetch_one(&mut **transaction)
    .await
    {
        Ok(row) => Ok(Some(row)),
        Err(e) => {
            event!(target: "security_api", Level::ERROR, "calendar_data.read: {}", &e);
            Err(e)
        }
    }
}

#[instrument]
pub async fn create(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    data: CalendarData,
) -> Result<u64, sqlx::Error> {
    match sqlx::query(
        r#" 
        INSERT INTO calendar_data(ce_year
             , tw_year
             , ce_month
             , ce_day
             , date_status
             , group_task
             , created_date
             , updated_date
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)  "#,
    )
    .bind(data.ce_year)
    .bind(data.tw_year)
    .bind(data.ce_month)
    .bind(data.ce_day)
    .bind(data.date_status)
    .bind(data.group_task)
    .bind(Local::now().naive_local())
    .bind(Local::now().naive_local())
    .execute(&mut **transaction)
    .await
    {
        Ok(row) => Ok(row.rows_affected()),
        Err(e) => {
            event!(target: "security_api", Level::ERROR, "calendar_data.create: {}", &e);
            Err(e)
        }
    }
}

pub async fn update(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    data: CalendarData,
) -> Result<u64, sqlx::Error> {
    match sqlx::query(
        r#" UPDATE calendar_data
            SET ce_year = $1
              , tw_year = $2
              , ce_month = $3
              , ce_day = $4
              , date_status = $5
              , group_task = $6
              , updated_date = $7
            WHERE row_id = $8
          "#,
    )
    .bind(data.ce_year)
    .bind(data.tw_year)
    .bind(data.ce_month)
    .bind(data.ce_day)
    .bind(data.date_status)
    .bind(data.group_task)
    .bind(Local::now().naive_local())
    .bind(data.row_id)
    .execute(&mut **transaction)
    .await
    {
        Ok(row) => Ok(row.rows_affected()),
        Err(e) => {
            event!(target: "security_api", Level::ERROR, "calendar_data.update: {}", &e);
            Err(e)
        }
    }
}

pub async fn delete(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    data: CalendarData,
) -> Result<u64, sqlx::Error> {
    match sqlx::query(r#" DELETE FROM calendar_data WHERE row_id = $1 "#)
        .bind(data.row_id)
        .execute(&mut **transaction)
        .await
    {
        Ok(row) => Ok(row.rows_affected()),
        Err(e) => {
            event!(target: "security_api", Level::ERROR, "calendar_data.delete: {}", &e);
            Err(e)
        }
    }
}
