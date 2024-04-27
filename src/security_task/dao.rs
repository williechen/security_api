use sqlx::{postgres::PgRow, Row};
use tracing::{event, Level};

use super::model::SecurityTask;

pub async fn read_all(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    data: &SecurityTask,
) -> Result<(usize, Vec<SecurityTask>), sqlx::Error> {
    let mut select_str = r#" 
        SELECT row_id
             , market_type
             , security_code
             , issue_date
             , twse_date
             , tpex_date
             , security_seed
             , is_enabled
             , sort_no
             , retry_count
             , created_date
             , updated_date
          FROM security_task
    "#
    .to_string();

    let mut is_where = false;
    let mut index = 1;
    if data.market_type.is_some() {
        if !is_where {
            select_str.push_str(" WHERE ");
        } else {
            select_str.push_str(" AND ");
        }
        select_str.push_str(&format!(" market_type = ${}", index));

        index = index + 1;
        is_where = true;
    }
    if data.security_code.is_some() {
        if !is_where {
            select_str.push_str(" WHERE ");
        } else {
            select_str.push_str(" AND ");
        }
        select_str.push_str(&format!(" security_code = ${}", index));
        index = index + 1;
        is_where = true;
    }
    if data.issue_date.is_some() {
        if !is_where {
            select_str.push_str(" WHERE ");
        } else {
            select_str.push_str(" AND ");
        }
        select_str.push_str(&format!(" issue_date <= ${}", index));
        index = index + 1;
        is_where = true;
    }
    if data.twse_date.is_some() {
        if !is_where {
            select_str.push_str(" WHERE ");
        } else {
            select_str.push_str(" AND ");
        }
        select_str.push_str(&format!(" twse_date = ${}", index));
        index = index + 1;
        is_where = true;
    }
    if data.is_enabled.is_some() {
        if !is_where {
            select_str.push_str(" WHERE ");
        } else {
            select_str.push_str(" AND ");
        }
        select_str.push_str(&format!(" is_enabled = ${}", index));
        index = index + 1;
        is_where = true;
    }

    let mut query = sqlx::query(&select_str);

    if data.market_type.is_some() {
        query = query.bind(&data.market_type);
    }
    if data.security_code.is_some() {
        query = query.bind(&data.security_code);
    }
    if data.issue_date.is_some() {
        query = query.bind(&data.issue_date);
    }
    if data.twse_date.is_some() {
        query = query.bind(&data.twse_date);
    }
    if data.is_enabled.is_some() {
        query = query.bind(data.is_enabled);
    }

    match query
        .map(|row: PgRow| SecurityTask {
            row_id: row.get("row_id"),
            market_type: row.get("market_type"),
            security_code: row.get("security_code"),
            issue_date: row.get("issue_date"),
            twse_date: row.get("twse_date"),
            tpex_date: row.get("tpex_date"),
            security_seed: row.get("security_seed"),
            is_enabled: row.get("is_enabled"),
            sort_no: row.get("sort_no"),
            retry_count: row.get("retry_count"),
            created_date: row.get("created_date"),
            updated_date: row.get("updated_date"),
        })
        .fetch_all(&mut **transaction)
        .await
    {
        Ok(rows) => Ok((rows.len(), rows)),
        Err(_) => Ok((0, vec![])),
    }
}

pub async fn read_all_by_sql(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    sql: &str,
) -> Result<(usize, Vec<SecurityTask>), sqlx::Error> {
    match sqlx::query(sql)
        .map(|row: PgRow| SecurityTask {
            row_id: row.get("row_id"),
            market_type: row.get("market_type"),
            security_code: row.get("security_code"),
            issue_date: row.get("issue_date"),
            twse_date: row.get("twse_date"),
            tpex_date: row.get("tpex_date"),
            security_seed: row.get("security_seed"),
            is_enabled: row.get("is_enabled"),
            sort_no: row.get("sort_no"),
            retry_count: row.get("retry_count"),
            created_date: row.get("created_date"),
            updated_date: row.get("updated_date"),
        })
        .fetch_all(&mut **transaction)
        .await
    {
        Ok(rows) => Ok((rows.len(), rows)),
        Err(e) => {
            event!(target: "my_api", Level::ERROR, "{:?}", e);
            Ok((0, vec![]))
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
             , market_type
             , security_code
             , issue_date
             , twse_date
             , tpex_date
             , security_seed
             , is_enabled
             , sort_no
             , retry_count
             , created_date
             , updated_date
          FROM security_task
         WHERE row_id = $1 "#,
    )
    .bind(row_id)
    .map(|row: PgRow| SecurityTask {
        row_id: row.get("row_id"),
        market_type: row.get("market_type"),
        security_code: row.get("security_code"),
        issue_date: row.get("issue_date"),
        twse_date: row.get("twse_date"),
        tpex_date: row.get("tpex_date"),
        security_seed: row.get("security_seed"),
        is_enabled: row.get("is_enabled"),
        sort_no: row.get("sort_no"),
        retry_count: row.get("retry_count"),
        created_date: row.get("created_date"),
        updated_date: row.get("updated_date"),
    })
    .fetch_one(&mut **transaction)
    .await
    {
        Ok(row) => Ok(Some(row)),
        Err(_) => Ok(None),
    }
}

pub async fn create(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    data: SecurityTask,
) -> Result<u64, sqlx::Error> {
    match sqlx::query(
        r#" INSERT INTO security_task(
               market_type
             , security_code
             , issue_date
             , twse_date
             , tpex_date
             , security_seed
             , is_enabled
             , sort_no
             , retry_count
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)  "#,
    )
    .bind(data.market_type)
    .bind(data.security_code)
    .bind(data.issue_date)
    .bind(data.twse_date)
    .bind(data.tpex_date)
    .bind(data.security_seed)
    .bind(data.is_enabled)
    .bind(data.sort_no)
    .bind(data.retry_count)
    .execute(&mut **transaction)
    .await
    {
        Ok(row) => Ok(row.rows_affected()),
        Err(_) => Ok(0),
    }
}

pub async fn update(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    data: SecurityTask,
) -> Result<u64, sqlx::Error> {
    match sqlx::query(
        r#" UPDATE security_task
            SET market_type= $1
              , security_code= $2
              , issue_date= $3
              , twse_date= $4
              , tpex_date= $5
              , security_seed= $6
              , is_enabled= $7
              , sort_no= $8
              , retry_count = $9
              , updated_date = $10
            WHERE row_id = $11
          "#,
    )
    .bind(data.market_type)
    .bind(data.security_code)
    .bind(data.issue_date)
    .bind(data.twse_date)
    .bind(data.tpex_date)
    .bind(data.security_seed)
    .bind(data.is_enabled)
    .bind(data.sort_no)
    .bind(data.retry_count)
    .bind(data.updated_date)
    .bind(data.row_id)
    .execute(&mut **transaction)
    .await
    {
        Ok(row) => Ok(row.rows_affected()),
        Err(_) => Ok(0),
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
        Err(_) => Ok(0),
    }
}
