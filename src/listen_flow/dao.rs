#![warn(clippy::all, clippy::pedantic)]

use chrono::Local;
use sqlx::{postgres::PgRow, Row};
use tracing::{event, Level};

use crate::repository::Repository;

use super::model::ListenFlow;

pub async fn create(data: ListenFlow) -> Result<u64, sqlx::Error> {
    let dao = Repository::new().await;
    let conn = dao.connection;

    match sqlx::query(
        r"
        INSERT INTO listen_flow(
            flow_code
          , flow_param1
          , flow_param2
          , flow_param3
          , flow_param4
          , flow_param5
          , pid
          , pstatus
          , created_date
          , updated_date
        ) VALUES ( $1, $2, $3, $4, $5, $6, $7, $8 )
    ",
    )
    .bind(data.flow_code)
    .bind(data.flow_param1)
    .bind(data.flow_param2)
    .bind(data.flow_param3)
    .bind(data.flow_param4)
    .bind(data.flow_param5)
    .bind(data.pid)
    .bind(data.pstatus)
    .bind(Local::now())
    .bind(Local::now())
    .execute(&conn)
    .await
    {
        Ok(cnt) => Ok(cnt.rows_affected()),
        Err(e) => Err(e),
    }
}

pub async fn modify(data: ListenFlow) -> Result<u64, sqlx::Error> {
    let dao = Repository::new().await;
    let conn = dao.connection;

    match sqlx::query(
        r"
        UPDATE listen_flow 
           SET flow_code = $1
             , flow_param1 = $2
             , flow_param2 = $3
             , flow_param3 = $4
             , flow_param4 = $5
             , flow_param5 = $6
             , pid = $7
             , pstatus = $8
             , updated_date = $9
         WHERE row_id = $10
    ",
    )
    .bind(data.flow_code)
    .bind(data.flow_param1)
    .bind(data.flow_param2)
    .bind(data.flow_param3)
    .bind(data.flow_param4)
    .bind(data.flow_param5)
    .bind(data.pid)
    .bind(data.pstatus)
    .bind(Local::now())
    .bind(data.row_id)
    .execute(&conn)
    .await
    {
        Ok(cnt) => Ok(cnt.rows_affected()),
        Err(e) => Err(e),
    }
}

pub async fn remove_all(q_flow_code: &str) -> Result<u64, sqlx::Error> {
    let dao = Repository::new().await;
    let conn = dao.connection;

    match sqlx::query(
        r"
        DELETE FROM listen_flow 
         WHERE flow_code = $1
    ",
    )
    .bind(q_flow_code)
    .execute(&conn)
    .await
    {
        Ok(cnt) => Ok(cnt.rows_affected()),
        Err(e) => Err(e),
    }
}

pub async fn find_all(data: ListenFlow) -> Vec<ListenFlow> {
    let dao = Repository::new().await;
    let conn = dao.connection;

    let mut select_str = r#" 
        SELECT row_id
             , flow_code
             , flow_param1
             , flow_param2
             , flow_param3
             , flow_param4
             , flow_param5
             , pid
             , pstatus
          FROM listen_flow
    "#
    .to_string();

    let mut index = 0;
    if !data.flow_code.is_empty() {
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
    if data.pid > 0 {
        select_str.push_str(&where_append("pid", "=", &mut index));
    }

    select_str.push_str("ORDER BY pid, created_date");

    let mut query = sqlx::query(&select_str);

    if !data.flow_code.is_empty() {
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
    if data.pid > 0 {
        query = query.bind(data.pid.clone());
    }

    match query
        .map(|row: PgRow| ListenFlow {
            row_id: row.get("row_id"),
            flow_code: row.get("flow_code"),
            flow_param1: row.get("flow_param1"),
            flow_param2: row.get("flow_param2"),
            flow_param3: row.get("flow_param3"),
            flow_param4: row.get("flow_param4"),
            flow_param5: row.get("flow_param5"),
            pid: row.get("pid"),
            pstatus: row.get("pstatus"),
        })
        .fetch_all(&conn)
        .await
    {
        Ok(rows) => rows,
        Err(e) => {
            event!(target: "security_api", Level::ERROR, "daily_task.read_all: {}", &e);
            Vec::new()
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
