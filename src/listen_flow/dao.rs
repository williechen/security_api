#![warn(clippy::all, clippy::pedantic)]

use bigdecimal::Zero;
use diesel::{delete, insert_into, update, ExpressionMethods, QueryDsl, RunQueryDsl};
use log::debug;

use crate::schema::listen_flow::dsl::listen_flow as table;
use crate::schema::listen_flow::{
    flow_code, flow_param1, flow_param2, flow_param3, flow_param4, flow_param5, pid,
};
use crate::{repository::Repository, schema::listen_flow::row_id};

use super::model::{ListenFlow, NewListenFlow};

pub fn find_all(data: ListenFlow) -> Vec<ListenFlow> {
    let dao = Repository::new();
    let mut conn = dao.connection.get().unwrap();

    let mut query = table.into_boxed();

    query = query.filter(flow_code.eq(data.flow_code));
    if !data.pid.is_zero() {
        query = query.filter(pid.eq(data.pid));
    }
    if data.flow_param1.is_some() {
        query = query.filter(flow_param1.eq(data.flow_param1));
    }
    if data.flow_param2.is_some() {
        query = query.filter(flow_param2.eq(data.flow_param2));
    }
    if data.flow_param3.is_some() {
        query = query.filter(flow_param3.eq(data.flow_param3));
    }
    if data.flow_param4.is_some() {
        query = query.filter(flow_param4.eq(data.flow_param4));
    }
    if data.flow_param5.is_some() {
        query = query.filter(flow_param5.eq(data.flow_param5));
    }

    debug!("{}", diesel::debug_query::<diesel::pg::Pg, _>(&query));

    match query.load::<ListenFlow>(&mut conn) {
        Ok(rows) => rows,
        Err(e) => {
            debug!("read_by_exec {}", e);
            Vec::new()
        }
    }
}

pub fn create(data: NewListenFlow) -> Result<usize, diesel::result::Error> {
    let dao = Repository::new();
    let mut conn = dao.connection.get().unwrap();

    insert_into(table).values(data).execute(&mut conn)
}

pub fn modify(data: ListenFlow) -> Result<usize, diesel::result::Error> {
    let dao = Repository::new();
    let mut conn = dao.connection.get().unwrap();

    update(table)
        .filter(row_id.eq(data.row_id.clone()))
        .set(data)
        .execute(&mut conn)
}

pub fn remove_all(q_flow_code: &str) -> Result<usize, diesel::result::Error> {
    let dao = Repository::new();
    let mut conn = dao.connection.get().unwrap();

    delete(table)
        .filter(flow_code.eq(q_flow_code))
        .execute(&mut conn)
}
