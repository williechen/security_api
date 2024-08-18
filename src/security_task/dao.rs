#![warn(clippy::all, clippy::pedantic)]

use diesel::query_dsl::methods::FilterDsl;
use diesel::query_dsl::methods::OrderDsl;
use diesel::sql_types::VarChar;
use diesel::{insert_into, update, ExpressionMethods,sql_query, RunQueryDsl, OptionalExtension};
use log::debug;
use log::error;

use crate::daily_task::model::DailyTask;
use crate::repository::Repository;
use crate::schema::security_task::dsl::security_task as table;
use crate::schema::security_task::{
    exec_count, is_enabled, issue_date, market_type, open_date_day, open_date_month,
    open_date_year, row_id, security_code, sort_no,
};
use crate::security_error::SecurityError;

use super::model::{NewSecurityTask, SecurityTask};

pub fn find_all_by_twse(task: &DailyTask) -> Vec<SecurityTask> {
    let dao = Repository::new();
    let mut conn = dao.connection;

    let q_year = task.clone().open_date_year;
    let q_month = task.clone().open_date_month;
    let q_day = task.clone().open_date_day;

    let query = sql_query(
        r#" SELECT row_id
                     , open_date_year
                     , open_date_month
                     , open_date_day
                     , security_code
                     , security_name
                     , market_type
                     , issue_date
                     , exec_seed
                     , exec_count
                     , is_enabled
                     , sort_no
                     , created_date
                     , updated_date
                   FROM security_task 
                  WHERE open_date_year = $1
                    AND open_date_month = $2
                    AND open_date_day = $3
                    AND market_type in ('上市')
                  ORDER BY security_code, issue_date, market_type
                  "#,
    )
    .bind::<VarChar, _>(q_year)
    .bind::<VarChar, _>(q_month)
    .bind::<VarChar, _>(q_day);

    debug!("{}", diesel::debug_query::<diesel::pg::Pg, _>(&query));

    match query.load::<SecurityTask>(&mut conn){
        Ok(rows) => rows,
        Err(e) =>{
            error!("{}", SecurityError::SQLError(e));
            Vec::new()
        }
    }
}

pub fn find_all_by_tpex(task: &DailyTask) -> Vec<SecurityTask> {
    let dao = Repository::new();
    let mut conn = dao.connection;

    let q_year = task.clone().open_date_year;
    let q_month = task.clone().open_date_month;
    let q_day = task.clone().open_date_day;

    let query = sql_query(
        r#" SELECT row_id
                     , open_date_year
                     , open_date_month
                     , open_date_day
                     , security_code
                     , security_name
                     , market_type
                     , issue_date
                     , exec_seed
                     , exec_count
                     , is_enabled
                     , sort_no
                     , created_date
                     , updated_date
                   FROM security_task 
                  WHERE open_date_year = $1
                    AND open_date_month = $2
                    AND open_date_day = $3 
                    AND market_type in ('上櫃', '興櫃')
                  ORDER BY security_code, issue_date, market_type
            "#,
    )
    .bind::<VarChar, _>(q_year)
    .bind::<VarChar, _>(q_month)
    .bind::<VarChar, _>(q_day);

    debug!("{}", diesel::debug_query::<diesel::pg::Pg, _>(&query));

    match query.load::<SecurityTask>(&mut conn){
        Ok(rows)=> rows,
        Err(e) =>{
            error!("{}", SecurityError::SQLError(e));
            Vec::new()
        }
    }
}

pub fn find_one(
    q_year: String,
    q_month: String,
    q_day: String,
    q_security_code: String,
    q_market_type: String,
    q_issue_date: String,
) -> Option<SecurityTask> {
    let dao = Repository::new();
    let mut conn = dao.connection;

    let query = table
        .filter(open_date_year.eq(q_year))
        .filter(open_date_month.eq(q_month))
        .filter(open_date_day.eq(q_day))
        .filter(security_code.eq(q_security_code))
        .filter(market_type.eq(q_market_type))
        .filter(issue_date.eq(q_issue_date));

    debug!("{}", diesel::debug_query::<diesel::pg::Pg, _>(&query));

    match query.first::<SecurityTask>(&mut conn).optional() {
        Ok(row) => row,
        Err(e) => {
            error!("{}", SecurityError::SQLError(e));
            None
        }
    }
}

pub fn find_all_by_times(q_year: String, q_month: String, q_day: String) -> Vec<SecurityTask> {
    let dao = Repository::new();
    let mut conn = dao.connection;

    let query = table
        .filter(open_date_year.eq(q_year))
        .filter(open_date_month.eq(q_month))
        .filter(open_date_day.eq(q_day))
        .filter(exec_count.eq(0))
        .filter(is_enabled.eq(1))
        .order(sort_no.asc());

    debug!("{}", diesel::debug_query::<diesel::pg::Pg, _>(&query));

    match query.load::<SecurityTask>(&mut conn) {
        Ok(rows) => rows,
        Err(e) => {
            error!("{}", SecurityError::SQLError(e));
            Vec::new()
        }
    }
}

pub fn create(data: NewSecurityTask) -> Result<usize, SecurityError> {
    let dao = Repository::new();
    let mut conn = dao.connection;

    match insert_into(table).values(data).execute(&mut conn){
        Ok(cnt) => Ok(cnt),
        Err(e) => Err(SecurityError::SQLError(e))
    }
}

pub fn modify(data: SecurityTask) -> Result<usize, SecurityError> {
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
