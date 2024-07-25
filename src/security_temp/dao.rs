#![warn(clippy::all, clippy::pedantic)]

use diesel::query_dsl::methods::FilterDsl;
use diesel::sql_types::VarChar;
use diesel::{delete, insert_into, sql_query, ExpressionMethods, PgConnection, RunQueryDsl};
use log::debug;

use crate::daily_task::model::DailyTask;
use crate::repository::Repository;
use crate::schema::security_temp::dsl::security_temp as table;
use crate::schema::security_temp::{
    issue_date, market_type, open_date_day, open_date_month, open_date_year, security_code,
};

use super::model::{NewSecurityTemp, SecurityTemp};

pub fn find_all_by_twse(task: &DailyTask) -> Vec<SecurityTemp> {
    let dao = Repository::new();
    let mut conn = dao.connection.get().unwrap();

    let q_year = task.clone().open_date_year;
    let q_month = task.clone().open_date_month;
    let q_day = task.clone().open_date_day;
    let q_issue_date = format!("{0}/{1}/{2}", q_year, q_month, q_day);

    let query = sql_query(
        r#" SELECT row_id
                      , open_date_year
                      , open_date_month
                      , open_date_day
                      , international_code
                      , security_code
                      , security_name
                      , market_type
                      , security_type
                      , industry_type
                      , issue_date
                      , cfi_code
                      , remark
                      , created_date
                      , updated_date
                   FROM security_temp 
                  WHERE open_date_year = $1
                    AND open_date_month = $2
                    AND open_date_day = $3 
                    AND issue_date <= $4
                    AND market_type in ('上市')
                    AND security_type in ('ETF', 'ETN', '股票', '特別股')
                  ORDER BY security_code, issue_date, market_type, security_type
            "#,
    )
    .bind::<VarChar, _>(q_year)
    .bind::<VarChar, _>(q_month)
    .bind::<VarChar, _>(q_day)
    .bind::<VarChar, _>(q_issue_date);

    debug!("{}", diesel::debug_query::<diesel::pg::Pg, _>(&query));

    query.load::<SecurityTemp>(&mut conn).unwrap()
}

pub fn find_all_by_tpex(task: &DailyTask) -> Vec<SecurityTemp> {
    let dao = Repository::new();
    let mut conn = dao.connection.get().unwrap();

    let q_year = task.clone().open_date_year;
    let q_month = task.clone().open_date_month;
    let q_day = task.clone().open_date_day;
    let q_issue_date = format!("{0}/{1}/{2}", q_year, q_month, q_day);

    let query = sql_query(
        r#" SELECT row_id
                     , open_date_year
                     , open_date_month
                     , open_date_day
                     , international_code
                     , security_code
                     , security_name
                     , market_type
                     , security_type
                     , industry_type
                     , issue_date
                     , cfi_code
                     , remark
                     , created_date
                     , updated_date
                  FROM security_temp 
                 WHERE open_date_year = $1
                   AND open_date_month = $2
                   AND open_date_day = $3 
                   AND issue_date <= $4
                   AND market_type in ('上櫃', '興櫃')
                   AND security_type in ('ETF', 'ETN', '股票', '特別股')
                 ORDER BY security_code, issue_date, market_type, security_type
            "#,
    )
    .bind::<VarChar, _>(q_year)
    .bind::<VarChar, _>(q_month)
    .bind::<VarChar, _>(q_day)
    .bind::<VarChar, _>(q_issue_date);

    debug!("{}", diesel::debug_query::<diesel::pg::Pg, _>(&query));

    query.load::<SecurityTemp>(&mut conn).unwrap()
}

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

pub fn remove_all() -> Result<usize, diesel::result::Error> {
    let dao = Repository::new();
    let mut conn = dao.connection.get().unwrap();

    delete(table).execute(&mut conn)
}
