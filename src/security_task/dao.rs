use chrono::Local;
use sqlx::{postgres::PgRow, Row};
use tracing::{event, Level};

use super::model::SecurityTask;

pub async fn read_all(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    data: &SecurityTask,
) -> Result<(usize, Vec<SecurityTask>), sqlx::Error> {
    let mut select_str = r#" 
        SELECT row_id
             , open_date
             , security_code
             , market_type
             , issue_date
             , security_date
             , security_seed
             , exec_count
             , is_enabled
             , sort_no
             , created_date
             , updated_date
          FROM security_task
    "#
    .to_string();

    let mut index = 0;
    if data.open_date.is_some() {
        select_str.push_str(&where_append("open_date", "=", &mut index));
    }
    if data.security_code.is_some() {
        select_str.push_str(&where_append("security_code", "=", &mut index));
    }
    if data.market_type.is_some() {
        select_str.push_str(&where_append("market_type", "=", &mut index));
    }
    if data.issue_date.is_some() {
        select_str.push_str(&where_append("issue_date", ">=", &mut index));
    }
    if data.security_date.is_some() {
        select_str.push_str(&where_append("security_date", ">=", &mut index));
    }
    if data.exec_count.is_some() {
        select_str.push_str(&where_append("exec_count", "=", &mut index));
    }
    if data.is_enabled.is_some() {
        select_str.push_str(&where_append("is_enabled", "=", &mut index));
    }
    if data.sort_no.is_some() {
        select_str.push_str(&where_append("sort_no", "=", &mut index));
    }

    select_str.push_str("ORDER BY open_date desc, sort_no");

    let mut query = sqlx::query(&select_str);

    if data.open_date.is_some() {
        query = query.bind(data.open_date.clone());
    }
    if data.security_code.is_some() {
        query = query.bind(data.security_code.clone());
    }
    if data.market_type.is_some() {
        query = query.bind(data.market_type.clone());
    }
    if data.issue_date.is_some() {
        query = query.bind(data.issue_date.clone());
    }
    if data.security_date.is_some() {
        query = query.bind(data.security_date.clone());
    }
    if data.exec_count.is_some() {
        query = query.bind(data.exec_count.clone());
    }
    if data.is_enabled.is_some() {
        query = query.bind(data.is_enabled.clone());
    }
    if data.sort_no.is_some() {
        query = query.bind(data.sort_no.clone());
    }

    match query
        .map(|row: PgRow| SecurityTask {
            row_id: row.get("row_id"),
            open_date: row.get("open_date"),
            security_code: row.get("security_code"),
            market_type: row.get("market_type"),
            issue_date: row.get("issue_date"),
            security_date: row.get("security_date"),
            security_seed: row.get("security_seed"),
            exec_count: row.get("exec_count"),
            is_enabled: row.get("is_enabled"),
            sort_no: row.get("sort_no"),
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
) -> Result<(usize, Vec<SecurityTask>), sqlx::Error> {
    match sqlx::query(sql)
        .map(|row: PgRow| SecurityTask {
            row_id: row.get("row_id"),
            open_date: row.get("open_date"),
            security_code: row.get("security_code"),
            market_type: row.get("market_type"),
            issue_date: row.get("issue_date"),
            security_date: row.get("security_date"),
            security_seed: row.get("security_seed"),
            exec_count: row.get("exec_count"),
            is_enabled: row.get("is_enabled"),
            sort_no: row.get("sort_no"),
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
) -> Result<Option<SecurityTask>, sqlx::Error> {
    match sqlx::query(
        r#"
        SELECT row_id
             , open_date
             , security_code
             , market_type
             , issue_date
             , security_date
             , security_seed
             , exec_count
             , is_enabled
             , sort_no
             , created_date
             , updated_date
          FROM security_task
         WHERE row_id = $1 "#,
    )
    .bind(row_id)
    .map(|row: PgRow| SecurityTask {
        row_id: row.get("row_id"),
        open_date: row.get("open_date"),
        security_code: row.get("security_code"),
        market_type: row.get("market_type"),
        issue_date: row.get("issue_date"),
        security_date: row.get("security_date"),
        security_seed: row.get("security_seed"),
        exec_count: row.get("exec_count"),
        is_enabled: row.get("is_enabled"),
        sort_no: row.get("sort_no"),
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
    data: SecurityTask,
) -> Result<u64, sqlx::Error> {
    match sqlx::query(
        r#" 
        INSERT INTO security_task(open_date
            , security_code
            , market_type
            , issue_date
            , security_date
            , security_seed
            , exec_count
            , is_enabled
            , sort_no
            , created_date
            , updated_date
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)  "#,
    )
    .bind(data.open_date)
    .bind(data.security_code)
    .bind(data.market_type)
    .bind(data.issue_date)
    .bind(data.security_date)
    .bind(data.security_seed)
    .bind(data.exec_count)
    .bind(data.is_enabled)
    .bind(data.sort_no)
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
    data: SecurityTask,
) -> Result<u64, sqlx::Error> {
    match sqlx::query(
        r#" UPDATE security_task
            SET open_date= $1
              , security_code = $2
              , market_type = $3
              , issue_date = $4
              , security_date = $5
              , security_seed = $6
              , exec_count = $7
              , is_enabled = $8
              , sort_no = $9
              , updated_date = $10
            WHERE row_id = $11
          "#,
    )
    .bind(data.open_date)
    .bind(data.security_code)
    .bind(data.market_type)
    .bind(data.issue_date)
    .bind(data.security_date)
    .bind(data.security_seed)
    .bind(data.exec_count)
    .bind(data.is_enabled)
    .bind(data.sort_no)
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
    data: SecurityTask,
) -> Result<u64, sqlx::Error> {
    match sqlx::query(r#" DELETE FROM security_task WHERE row_id = $1 "#)
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
