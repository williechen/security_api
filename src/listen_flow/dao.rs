#![warn(clippy::all, clippy::pedantic)]

use sqlx::{postgres::PgRow, PgConnection, Row};
use tracing::{event, Level};

use super::model::ListenFlow;

pub async fn read_all(
    transaction: &mut PgConnection,
    data: &ListenFlow,
) -> Result<i32, sqlx::Error> {
    let mut select_str = r#" 
        SELECT count(flow_code) AS cnt
          FROM listen_flow
    "#
    .to_string();

    let mut index = 0;
    if data.flow_code.is_some() {
        select_str.push_str(&where_append("flow_code", "=", &mut index));
    }
    if data.flow_param1.is_some() {
        select_str.push_str(&where_append("flow_param1", "=", &mut index));
    }
    if data.flow_param2.is_some() {
        select_str.push_str(&where_append("flow_param2", "=", &mut index));
    }
    if data.flow_param3.is_some() {
        select_str.push_str(&where_append("flow_param3", "=", &mut index));
    }
    if data.flow_param4.is_some() {
        select_str.push_str(&where_append("flow_param4", "=", &mut index));
    }
    if data.flow_param5.is_some() {
        select_str.push_str(&where_append("flow_param5", "=", &mut index));
    }

    let mut query = sqlx::query(&select_str);

    if data.flow_code.is_some() {
        query = query.bind(data.flow_code.clone());
    }
    if data.flow_param1.is_some() {
        query = query.bind(data.flow_param1.clone());
    }
    if data.flow_param2.is_some() {
        query = query.bind(data.flow_param2.clone());
    }
    if data.flow_param3.is_some() {
        query = query.bind(data.flow_param3.clone());
    }
    if data.flow_param4.is_some() {
        query = query.bind(data.flow_param4.clone());
    }
    if data.flow_param5.is_some() {
        query = query.bind(data.flow_param5.clone());
    }

    match query
        .map(|row: PgRow| row.get("cnt"))
        .fetch_one(transaction)
        .await
    {
        Ok(rows) => Ok(rows),
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

pub async fn create(transaction: &mut PgConnection, data: ListenFlow) -> Result<u64, sqlx::Error> {
    match sqlx::query(
        r#" 
        INSERT INTO listen_flow(flow_code
             , flow_param1
             , flow_param2
             , flow_param3
             , flow_param4
             , flow_param5
        ) VALUES ($1, $2, $3, $4, $5, $6)  "#,
    )
    .bind(data.flow_code)
    .bind(data.flow_param1)
    .bind(data.flow_param2)
    .bind(data.flow_param3)
    .bind(data.flow_param4)
    .bind(data.flow_param5)
    .execute(transaction)
    .await
    {
        Ok(row) => Ok(row.rows_affected()),
        Err(e) => {
            event!(target: "security_api", Level::ERROR, "listen_flow.create: {}", &e);
            Err(e)
        }
    }
}

pub async fn delete(transaction: &mut PgConnection, flow_code: &str) -> Result<u64, sqlx::Error> {
    match sqlx::query(r#" DELETE FROM listen_flow WHERE flow_code = $1 "#)
        .bind(flow_code)
        .execute(transaction)
        .await
    {
        Ok(row) => Ok(row.rows_affected()),
        Err(e) => {
            event!(target: "security_api", Level::ERROR, "listen_flow.delete: {}", &e);
            Err(e)
        }
    }
}
