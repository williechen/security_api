#![warn(clippy::all, clippy::pedantic)]

use diesel::{
    insert_into, query_dsl::methods::FilterDsl, sql_query, sql_types::VarChar, update,
    ExpressionMethods, RunQueryDsl,OptionalExtension
};
use log::{debug, error};

use crate::{
    repository::Repository,
    schema::calendar_data::{ce_day, ce_month, ce_year, dsl::calendar_data as table, row_id},
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

    match insert_into(table).values(data).execute(&mut conn){
        Ok(cnt) => Ok(cnt),
        Err(e) => Err(SecurityError::SQLError(e))
    }
}

pub fn modify(data: CalendarData) -> Result<usize, SecurityError> {
    let dao = Repository::new();
    let mut conn = dao.connection;

    match update(table)
        .filter(row_id.eq(data.row_id.clone()))
        .set(data)
        .execute(&mut conn){
            Ok(cnt) => Ok(cnt),
            Err(e) => Err(SecurityError::SQLError(e))
        }
}

pub fn find_one_by_work_day_first(q_year: String, q_month: String) -> Option<CalendarData> {
    let dao = Repository::new();
    let mut conn = dao.connection;

    let query = sql_query(
        "SELECT cd.row_id
             , cd.ce_year
             , cd.ce_month
             , cd.ce_day
             , cd.date_status
             , cd.group_task
             , cd.created_date
             , cd.updated_date
          FROM calendar_data cd
         WHERE cd.ce_year = $1
           AND cd.ce_month = $2
           AND cd.date_status = 'O'
           AND NOT EXISTS (
               SELECT 1 
                 FROM calendar_data cd1 
                WHERE cd1.ce_year = cd.ce_year
                  AND cd1.ce_month = cd.ce_month
                  AND cd1.date_status = 'O'
                  AND cd1.group_task IN ('FIRST', 'FIRST_INIT')
           )
         ORDER BY Concat(cd.ce_year, cd.ce_month, cd.ce_day)
         Limit 1
           ",
    )
    .bind::<VarChar, _>(q_year)
    .bind::<VarChar, _>(q_month);

    debug!("{}", diesel::debug_query::<diesel::pg::Pg, _>(&query));

    match query.get_result::<CalendarData>(&mut conn).optional() {
        Ok(row) => row,
        Err(e) => {
            error!("{}", SecurityError::SQLError(e));
            None
        }
    }
}
