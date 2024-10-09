#![warn(clippy::all, clippy::pedantic)]

use diesel::{
    insert_into, query_dsl::methods::FilterDsl, ExpressionMethods, OptionalExtension, RunQueryDsl,
};
use log::{debug, error};

use crate::{
    repository::Repository,
    schema::calendar_data::{ce_day, ce_month, ce_year, dsl::calendar_data as table},
    security_error::SecurityError,
};

use super::model::{CalendarData, NewCalendarData};

pub fn find_one(q_year: String, q_month: String, q_day: String) -> Option<CalendarData> {
    let dao = Repository::new();
    let mut conn = dao.connection;

    let query = table
        .filter(ce_year.eq(q_year))
        .filter(ce_month.eq(q_month))
        .filter(ce_day.eq(q_day));

    debug!("{}", diesel::debug_query::<diesel::pg::Pg, _>(&query));

    match query.first::<CalendarData>(&mut conn).optional() {
        Ok(row) => row,
        Err(e) => {
            error!("{}", SecurityError::SQLError(e));
            None
        }
    }
}

pub fn create(data: NewCalendarData) -> Result<usize, SecurityError> {
    let dao = Repository::new();
    let mut conn = dao.connection;

    match insert_into(table).values(data).execute(&mut conn) {
        Ok(cnt) => Ok(cnt),
        Err(e) => Err(SecurityError::SQLError(e)),
    }
}
