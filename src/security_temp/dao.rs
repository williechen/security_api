use chrono::Local;
use sqlx::{postgres::PgRow, Row};
use tracing::{event, Level};

use super::model::SecurityTemp;

pub async fn read_all(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    data: &SecurityTemp,
) -> Result<(usize, Vec<SecurityTemp>), sqlx::Error> {
    let mut select_str = r#" 
        SELECT row_id
             , open_date
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
          FROM security_temp
    "#
    .to_string();

    let mut index = 0;
    if data.open_date.is_some() {
        select_str.push_str(&where_append("open_date", "=", &mut index));
    }
    if data.security_code.is_some() {
        select_str.push_str(&where_append("security_code", "=", &mut index));
    }
    if data.security_name.is_some() {
        select_str.push_str(&where_append("security_name", "like", &mut index));
    }
    if data.market_type.is_some() {
        select_str.push_str(&where_append("market_type", "like", &mut index));
    }
    if data.security_type.is_some() {
        select_str.push_str(&where_append("security_type", "like", &mut index));
    }
    if data.issue_date.is_some() {
        select_str.push_str(&where_append("issue_date", ">=", &mut index));
    }

    let mut query = sqlx::query(&select_str);

    if data.open_date.is_some() {
        query = query.bind(data.open_date.clone());
    }
    if data.security_code.is_some() {
        query = query.bind(data.security_code.clone());
    }
    if data.security_name.is_some() {
        query = query.bind(data.security_name.clone());
    }
    if data.market_type.is_some() {
        query = query.bind(data.market_type.clone());
    }
    if data.security_type.is_some() {
        query = query.bind(data.security_type.clone());
    }
    if data.issue_date.is_some() {
        query = query.bind(data.issue_date.clone());
    }

    match query
        .map(|row: PgRow| SecurityTemp {
            row_id: row.get("row_id"),
            open_date: row.get("open_date"),
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
        .fetch_all(&mut **transaction)
        .await
    {
        Ok(rows) => Ok((rows.len(), rows)),
        Err(e) => {
            event!(target: "security_api", Level::ERROR, "read_all {}", &e);
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
) -> Result<(usize, Vec<SecurityTemp>), sqlx::Error> {
    match sqlx::query(sql)
        .map(|row: PgRow| SecurityTemp {
            row_id: row.get("row_id"),
            open_date: row.get("open_date"),
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
        .fetch_all(&mut **transaction)
        .await
    {
        Ok(rows) => Ok((rows.len(), rows)),
        Err(e) => {
            event!(target: "security_api", Level::ERROR, "read_all_by_sql {}", &e);
            Err(e)
        }
    }
}

pub async fn read(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    row_id: &str,
) -> Result<Option<SecurityTemp>, sqlx::Error> {
    match sqlx::query(
        r#"
        SELECT row_id
             , open_date
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
          FROM security_temp
         WHERE row_id = $1 "#,
    )
    .bind(row_id)
    .map(|row: PgRow| SecurityTemp {
        row_id: row.get("row_id"),
        open_date: row.get("open_date"),
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
    .fetch_one(&mut **transaction)
    .await
    {
        Ok(row) => Ok(Some(row)),
        Err(e) => {
            event!(target: "security_api", Level::ERROR, "read {}", &e);
            Err(e)
        }
    }
}

pub async fn create(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    data: SecurityTemp,
) -> Result<u64, sqlx::Error> {
    match sqlx::query(
        r#" 
        INSERT INTO security_temp(open_date
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
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)  "#,
    )
    .bind(data.open_date)
    .bind(data.international_code)
    .bind(data.security_code)
    .bind(data.security_name)
    .bind(data.market_type)
    .bind(data.security_type)
    .bind(data.industry_type)
    .bind(data.issue_date)
    .bind(data.cfi_code)
    .bind(data.remark)
    .bind(Local::now().naive_local())
    .bind(Local::now().naive_local())
    .execute(&mut **transaction)
    .await
    {
        Ok(row) => Ok(row.rows_affected()),
        Err(e) => {
            event!(target: "security_api", Level::ERROR, "create {}", &e);
            Err(e)
        }
    }
}

pub async fn update(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    data: SecurityTemp,
) -> Result<u64, sqlx::Error> {
    match sqlx::query(
        r#" UPDATE security_temp
            SET open_date= $1
            , international_code = $2
            , security_code = $3
            , security_name = $4
            , market_type = $5 
            , security_type = $6
            , industry_type = $7
            , issue_date = $8
            , cfi_code = $9
            , remark = $10
            , updated_date = $11
            WHERE row_id = $12
          "#,
    )
    .bind(data.open_date)
    .bind(data.international_code)
    .bind(data.security_code)
    .bind(data.security_name)
    .bind(data.market_type)
    .bind(data.security_type)
    .bind(data.industry_type)
    .bind(data.issue_date)
    .bind(data.cfi_code)
    .bind(data.remark)
    .bind(Local::now().naive_local())
    .bind(data.row_id)
    .execute(&mut **transaction)
    .await
    {
        Ok(row) => Ok(row.rows_affected()),
        Err(e) => {
            event!(target: "security_api", Level::ERROR, "update {}", &e);
            Err(e)
        }
    }
}

pub async fn delete(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    data: SecurityTemp,
) -> Result<u64, sqlx::Error> {
    match sqlx::query(r#" DELETE FROM security_temp WHERE row_id = $1 "#)
        .bind(data.row_id)
        .execute(&mut **transaction)
        .await
    {
        Ok(row) => Ok(row.rows_affected()),
        Err(e) => {
            event!(target: "security_api", Level::ERROR, "delete {}", &e);
            Err(e)
        }
    }
}

pub async fn truncate(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
) -> Result<u64, sqlx::Error> {
    match sqlx::query(r#" TRUNCATE TABLE security_temp CONTINUE IDENTITY RESTRICT; "#)
        .execute(&mut **transaction)
        .await
    {
        Ok(row) => Ok(row.rows_affected()),
        Err(e) => {
            event!(target: "security_api", Level::ERROR, "truncate {}", &e);
            Err(e)
        }
    }
}
