#![warn(clippy::all, clippy::pedantic)]

use diesel::query_dsl::methods::FilterDsl;
use diesel::{delete, insert_into, update, ExpressionMethods, PgConnection, RunQueryDsl};
use log::debug;

use crate::schema::security_temp::dsl::security_temp as table;
use crate::schema::security_temp::{
    issue_date, market_type, open_date_day, open_date_month, open_date_year, security_code,
};
use crate::{repository::Repository, schema::security_temp::row_id};

use super::model::{NewSecurityTemp, SecurityTemp};

pub fn find_one(
    q_year: String,
    q_month: String,
    q_day: String,
    q_security_code: String,
    q_market_type: String,
    q_issue_date: String,
) -> Option<SecurityTemp> {
    let dao = Repository::new();
    let mut conn = dao.connection.get().unwrap();

    let query = table
        .filter(open_date_year.eq(q_year))
        .filter(open_date_month.eq(q_month))
        .filter(open_date_day.eq(q_day))
        .filter(security_code.eq(q_security_code))
        .filter(market_type.eq(q_market_type))
        .filter(issue_date.eq(q_issue_date));

    debug!("{}", diesel::debug_query::<diesel::pg::Pg, _>(&query));

    match query.first::<SecurityTemp>(&mut conn) {
        Ok(row) => Some(row),
        Err(e) => {
            debug!("find_one {}", e);
            None
        }
    }
}

pub fn create(
    conn: &mut PgConnection,
    data: NewSecurityTemp,
) -> Result<usize, diesel::result::Error> {
    insert_into(table).values(data).execute(conn)
}

pub fn modify(conn: &mut PgConnection, data: SecurityTemp) -> Result<usize, diesel::result::Error> {
    update(table)
        .filter(row_id.eq(data.row_id.clone()))
        .set(data)
        .execute(conn)
}

pub fn remove_all() -> Result<usize, diesel::result::Error> {
    let dao = Repository::new();
    let mut conn = dao.connection.get().unwrap();

    delete(table).execute(&mut conn)
}
