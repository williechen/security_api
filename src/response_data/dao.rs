#![warn(clippy::all, clippy::pedantic)]

use chrono::Local;
use diesel::query_dsl::methods::FilterDsl;
use diesel::{insert_into, RunQueryDsl};
use diesel::{update, ExpressionMethods};
use log::debug;

use crate::repository::Repository;
use crate::schema::response_data::dsl::response_data as table;
use crate::schema::response_data::{
    exec_code, open_date_day, open_date_month, open_date_year, row_id,
};

use super::model::{NewResponseData, ResponseData};

pub fn find_one(
    q_year: String,
    q_month: String,
    q_day: String,
    q_exec_code: String,
) -> Option<ResponseData> {
    let dao = Repository::new();
    let mut conn = dao.connection.get().unwrap();

    let query = table
        .filter(open_date_year.eq(q_year))
        .filter(open_date_month.eq(q_month))
        .filter(open_date_day.eq(q_day))
        .filter(exec_code.eq(q_exec_code));

    debug!("{}", diesel::debug_query::<diesel::pg::Pg, _>(&query));

    match query.first::<ResponseData>(&mut conn) {
        Ok(row) => Some(row),
        Err(e) => {
            debug!("find_one {}", e);
            None
        }
    }
}

pub fn create(data: NewResponseData) -> Result<usize, diesel::result::Error> {
    let dao = Repository::new();
    let mut conn = dao.connection.get().unwrap();

    insert_into(table).values(data).execute(&mut conn)
}

pub fn modify(data: ResponseData) -> Result<usize, diesel::result::Error> {
    let dao = Repository::new();
    let mut conn = dao.connection.get().unwrap();

    update(table)
        .filter(row_id.eq(data.row_id.clone()))
        .set(data)
        .execute(&mut conn)
}
