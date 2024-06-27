#![warn(clippy::all, clippy::pedantic)]

use crate::repository::Repository;

use super::{dao, model::ListenFlow};

pub async fn read_flow_data(
    db_url: &str,
    pid: i64,
    flow_code: &str,
    flow_param1: &str,
    flow_param2: &str,
) -> Vec<ListenFlow> {
    let pool = Repository::new(db_url).await;
    let mut transaction = pool.connection.acquire().await.unwrap();

    let mut listen_flow = ListenFlow::new();
    listen_flow.pid = Some(pid);
    listen_flow.flow_code = Some(flow_code.to_string());
    listen_flow.flow_param1 = Some(flow_param1.to_string());
    listen_flow.flow_param2 = Some(flow_param2.to_string());

    dao::read_all(&mut transaction, &listen_flow).await.unwrap()
}

pub async fn delete_flow_data(db_url: &str, flow_code: &str) {
    let pool = Repository::new(db_url).await;
    let mut transaction = pool.connection.acquire().await.unwrap();

    dao::delete(&mut *transaction, flow_code).await.unwrap();
}

pub async fn insert_flow_data1(db_url: &str, pid: i64, flow_code: &str, flow_param1: &str) {
    let pool = Repository::new(db_url).await;
    let mut transaction = pool.connection.acquire().await.unwrap();

    let mut listen_flow = ListenFlow::new();
    listen_flow.pid = Some(pid);
    listen_flow.flow_code = Some(flow_code.to_string());
    listen_flow.flow_param1 = Some(flow_param1.to_string());

    let cnt = dao::read_all(&mut transaction, &listen_flow).await.unwrap();
    if cnt.len() <= 0 {
        dao::create(&mut *transaction, listen_flow).await.unwrap();
    }
}

pub async fn insert_flow_data2(
    db_url: &str,
    pid: i64,
    flow_code: &str,
    flow_param1: &str,
    flow_param2: &str,
) {
    let pool = Repository::new(db_url).await;
    let mut transaction = pool.connection.acquire().await.unwrap();

    let mut listen_flow = ListenFlow::new();
    listen_flow.pid = Some(pid);
    listen_flow.flow_code = Some(flow_code.to_string());
    listen_flow.flow_param1 = Some(flow_param1.to_string());
    listen_flow.flow_param2 = Some(flow_param2.to_string());

    let cnt = dao::read_all(&mut transaction, &listen_flow).await.unwrap();
    if cnt.len() <= 0 {
        dao::create(&mut *transaction, listen_flow).await.unwrap();
    }
}

pub async fn insert_flow_data3(
    db_url: &str,
    pid: i64,
    flow_code: &str,
    flow_param1: &str,
    flow_param2: &str,
    flow_param3: &str,
) {
    let pool = Repository::new(db_url).await;
    let mut transaction: sqlx::pool::PoolConnection<sqlx::Postgres> =
        pool.connection.acquire().await.unwrap();

    let mut listen_flow = ListenFlow::new();
    listen_flow.pid = Some(pid);
    listen_flow.flow_code = Some(flow_code.to_string());
    listen_flow.flow_param1 = Some(flow_param1.to_string());
    listen_flow.flow_param2 = Some(flow_param2.to_string());
    listen_flow.flow_param3 = Some(flow_param3.to_string());

    let cnt = dao::read_all(&mut transaction, &listen_flow).await.unwrap();
    if cnt.len() <= 0 {
        dao::create(&mut *transaction, listen_flow).await.unwrap();
    }
}

pub async fn insert_flow_data4(
    db_url: &str,
    pid: i64,
    flow_code: &str,
    flow_param1: &str,
    flow_param2: &str,
    flow_param3: &str,
    flow_param4: &str,
) {
    let pool = Repository::new(db_url).await;
    let mut transaction: sqlx::pool::PoolConnection<sqlx::Postgres> =
        pool.connection.acquire().await.unwrap();

    let mut listen_flow = ListenFlow::new();
    listen_flow.pid = Some(pid);
    listen_flow.flow_code = Some(flow_code.to_string());
    listen_flow.flow_param1 = Some(flow_param1.to_string());
    listen_flow.flow_param2 = Some(flow_param2.to_string());
    listen_flow.flow_param3 = Some(flow_param3.to_string());
    listen_flow.flow_param4 = Some(flow_param4.to_string());

    let cnt = dao::read_all(&mut transaction, &listen_flow).await.unwrap();
    if cnt.len() <= 0 {
        dao::create(&mut *transaction, listen_flow).await.unwrap();
    }
}

pub async fn insert_flow_data5(
    db_url: &str,
    pid: i64,
    flow_code: &str,
    flow_param1: &str,
    flow_param2: &str,
    flow_param3: &str,
    flow_param4: &str,
    flow_param5: &str,
) {
    let pool = Repository::new(db_url).await;
    let mut transaction = pool.connection.acquire().await.unwrap();

    let mut listen_flow = ListenFlow::new();
    listen_flow.pid = Some(pid);
    listen_flow.flow_code = Some(flow_code.to_string());
    listen_flow.flow_param1 = Some(flow_param1.to_string());
    listen_flow.flow_param2 = Some(flow_param2.to_string());
    listen_flow.flow_param3 = Some(flow_param3.to_string());
    listen_flow.flow_param4 = Some(flow_param4.to_string());
    listen_flow.flow_param5 = Some(flow_param5.to_string());

    let cnt = dao::read_all(&mut transaction, &listen_flow).await.unwrap();
    if cnt.len() <= 0 {
        dao::create(&mut *transaction, listen_flow).await.unwrap();
    }
}
