use sqlx::{postgres::PgRow, Row};

use super::model::SecurityTemp;

pub async fn read_all(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    data: SecurityTemp,
) -> Result<(usize, Vec<SecurityTemp>), sqlx::Error> {
    let mut select_str = r#" 
        SELECT row_id
             , version_code
             , international_code
             , security_code
             , security_name
             , market_type
             , security_type
             , industry_type
             , issue_date
             , cfi_code
             , remark
             , is_enabled
             , created_date
             , updated_date
          FROM security_temp
    "#
    .to_string();

    let mut is_where = false;
    let mut index: i32 = 1;
    if data.version_code.is_some() {
        if !is_where {
            select_str.push_str(" WHERE ");
        } else {
            select_str.push_str(" AND ");
        }
        select_str.push_str(&format!(" version_code = ${}", index));

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
    if data.security_type.is_some() {
        if !is_where {
            select_str.push_str(" WHERE ");
        } else {
            select_str.push_str(" AND ");
        }
        select_str.push_str(&format!(" security_type = ${}", index));
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

    if data.version_code.is_some() {
        query = query.bind(data.version_code);
    }
    if data.security_code.is_some() {
        query = query.bind(data.security_code);
    }
    if data.market_type.is_some() {
        query = query.bind(data.market_type);
    }
    if data.security_type.is_some() {
        query = query.bind(data.security_type);
    }
    if data.issue_date.is_some() {
        query = query.bind(data.issue_date);
    }
    if data.is_enabled.is_some() {
        query = query.bind(data.is_enabled);
    }

    match query
        .map(|row: PgRow| SecurityTemp {
            row_id: row.get("row_id"),
            version_code: row.get("version_code"),
            international_code: row.get("international_code"),
            security_code: row.get("security_code"),
            security_name: row.get("security_name"),
            market_type: row.get("market_type"),
            security_type: row.get("security_type"),
            industry_type: row.get("industry_type"),
            issue_date: row.get("issue_date"),
            cfi_code: row.get("cfi_code"),
            remark: row.get("remark"),
            is_enabled: row.get("is_enabled"),
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
) -> Result<(usize, Vec<SecurityTemp>), sqlx::Error> {
    match sqlx::query(sql)
        .map(|row: PgRow| SecurityTemp {
            row_id: row.get("row_id"),
            version_code: row.get("version_code"),
            international_code: row.get("international_code"),
            security_code: row.get("security_code"),
            security_name: row.get("security_name"),
            market_type: row.get("market_type"),
            security_type: row.get("security_type"),
            industry_type: row.get("industry_type"),
            issue_date: row.get("issue_date"),
            cfi_code: row.get("cfi_code"),
            remark: row.get("remark"),
            is_enabled: row.get("is_enabled"),
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

pub async fn read(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    row_id: &str,
) -> Result<Option<SecurityTemp>, sqlx::Error> {
    match sqlx::query(
        r#"
        SELECT row_id
             , version_code
             , international_code
             , security_code
             , security_name
             , market_type
             , security_type
             , industry_type
             , issue_date
             , cfi_code
             , remark
             , is_enabled
             , created_date
             , updated_date
          FROM security_temp
         WHERE row_id = $1 "#,
    )
    .bind(row_id)
    .map(|row: PgRow| SecurityTemp {
        row_id: row.get("row_id"),
        version_code: row.get("version_code"),
        international_code: row.get("international_code"),
        security_code: row.get("security_code"),
        security_name: row.get("security_name"),
        market_type: row.get("market_type"),
        security_type: row.get("security_type"),
        industry_type: row.get("industry_type"),
        issue_date: row.get("issue_date"),
        cfi_code: row.get("cfi_code"),
        remark: row.get("remark"),
        is_enabled: row.get("is_enabled"),
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
    data: SecurityTemp,
) -> Result<u64, sqlx::Error> {
    match sqlx::query(
        r#" INSERT INTO security_temp(
               version_code
             , international_code
             , security_code
             , security_name
             , market_type
             , security_type
             , industry_type
             , issue_date
             , cfi_code
             , remark
             , is_enabled
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)  "#,
    )
    .bind(data.version_code)
    .bind(data.international_code)
    .bind(data.security_code)
    .bind(data.security_name)
    .bind(data.market_type)
    .bind(data.security_type)
    .bind(data.industry_type)
    .bind(data.issue_date)
    .bind(data.cfi_code)
    .bind(data.remark)
    .bind(data.is_enabled)
    .execute(&mut **transaction)
    .await
    {
        Ok(row) => Ok(row.rows_affected()),
        Err(_) => Ok(0),
    }
}

pub async fn update(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    data: SecurityTemp,
) -> Result<u64, sqlx::Error> {
    match sqlx::query(
        r#" UPDATE security_temp
            SET version_code = $1
              , international_code= $2
              , security_code= $3
              , security_name= $4
              , market_type= $5
              , security_type= $6
              , industry_type= $7
              , issue_date= $8
              , cfi_code= $9
              , remark= $10
              , is_enabled= $11
              , updated_date = $12
            WHERE row_id = $13
          "#,
    )
    .bind(data.version_code)
    .bind(data.international_code)
    .bind(data.security_code)
    .bind(data.security_name)
    .bind(data.market_type)
    .bind(data.security_type)
    .bind(data.industry_type)
    .bind(data.issue_date)
    .bind(data.cfi_code)
    .bind(data.remark)
    .bind(data.is_enabled)
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
    data: SecurityTemp,
) -> Result<u64, sqlx::Error> {
    match sqlx::query(r#" DELETE FROM security_temp WHERE row_id = $1 "#)
        .bind(data.row_id)
        .execute(&mut **transaction)
        .await
    {
        Ok(row) => Ok(row.rows_affected()),
        Err(_) => Ok(0),
    }
}
