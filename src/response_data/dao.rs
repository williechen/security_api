use sqlx::{postgres::PgRow, Row};

use super::model::ResponseData;

pub async fn read_all(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    data: &ResponseData,
) -> Result<(usize, Vec<ResponseData>), sqlx::Error> {
    let mut select_str = r#" 
        SELECT row_id
             , data_content
             , data_code
             , read_date
             , created_date
             , updated_date
          FROM response_data
    "#
    .to_string();

    let mut is_where = false;
    let mut index = 1;
    if data.data_content.is_some() {
        if !is_where {
            select_str.push_str(" WHERE ");
        } else {
            select_str.push_str(" AND ");
        }
        select_str.push_str(&format!(" data_content like ${}", index));

        index = index + 1;
        is_where = true;
    }
    if data.data_code.is_some() {
        if !is_where {
            select_str.push_str(" WHERE ");
        } else {
            select_str.push_str(" AND ");
        }
        select_str.push_str(&format!(" data_code = ${}", index));
        index = index + 1;
        is_where = true;
    }
    if data.read_date.is_some() {
        if !is_where {
            select_str.push_str(" WHERE ");
        } else {
            select_str.push_str(" AND ");
        }
        select_str.push_str(&format!(" read_date = ${}", index));
        index = index + 1;
        is_where = true;
    }

    let mut query = sqlx::query(&select_str);

    if data.data_content.is_some() {
        query = query.bind(&data.data_content);
    }
    if data.data_code.is_some() {
        query = query.bind(&data.data_code);
    }
    if data.read_date.is_some() {
        query = query.bind(&data.read_date);
    }

    match query
        .map(|row: PgRow| ResponseData {
            row_id: row.get("row_id"),
            data_content: row.get("data_content"),
            data_code: row.get("data_code"),
            read_date: row.get("read_date"),
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
) -> Result<(usize, Vec<ResponseData>), sqlx::Error> {
    match sqlx::query(sql)
        .map(|row: PgRow| ResponseData {
            row_id: row.try_get("row_id").unwrap_or(None),
            data_content: row.get("data_content"),
            data_code: row.get("data_code"),
            read_date: row.get("read_date"),
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
) -> Result<Option<ResponseData>, sqlx::Error> {
    match sqlx::query(
        r#"
        SELECT row_id
             , data_content
             , data_code
             , read_date
             , created_date
             , updated_date
          FROM response_data
         WHERE row_id = $1 "#,
    )
    .bind(row_id)
    .map(|row: PgRow| ResponseData {
        row_id: row.get("row_id"),
        data_content: row.get("data_content"),
        data_code: row.get("data_code"),
        read_date: row.get("read_date"),
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
    data: ResponseData,
) -> Result<u64, sqlx::Error> {
    match sqlx::query(
        r#" INSERT INTO response_data(
            data_content,
            data_code,
            read_date
        ) VALUES ($1, $2, $3)  "#,
    )
    .bind(data.data_content)
    .bind(data.data_code)
    .bind(data.read_date)
    .execute(&mut **transaction)
    .await
    {
        Ok(row) => Ok(row.rows_affected()),
        Err(_) => Ok(0),
    }
}

pub async fn update(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    data: ResponseData,
) -> Result<u64, sqlx::Error> {
    match sqlx::query(
        r#" UPDATE response_data
            SET data_content = $1
              , data_code = $2
              , read_date = $3
              , updated_date = $4
            WHERE row_id = $5
          "#,
    )
    .bind(data.data_content)
    .bind(data.data_code)
    .bind(data.read_date)
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
    data: ResponseData,
) -> Result<u64, sqlx::Error> {
    match sqlx::query(r#" DELETE FROM response_data WHERE row_id = $1 "#)
        .bind(data.row_id)
        .execute(&mut **transaction)
        .await
    {
        Ok(row) => Ok(row.rows_affected()),
        Err(_) => Ok(0),
    }
}
