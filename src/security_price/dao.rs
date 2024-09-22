#![warn(clippy::all, clippy::pedantic)]

use crate::repository::Repository;
use crate::schema::security_price::dsl::security_price as table;
use crate::schema::security_price::{open_date_month, open_date_year, row_id, security_code};
use crate::security_error::SecurityError;
use diesel::{
    delete, insert_into, update, ExpressionMethods, OptionalExtension, PgConnection, QueryDsl,
};
use diesel::{sql_query, sql_types::VarChar, RunQueryDsl};
use log::{debug, error};

use super::model::{MaxPriceDate, NewSecurityPrice, ResposePrice, SecurityPrice};

pub fn find_all(q_year: String, q_month: String, q_security_code: String) -> Vec<SecurityPrice> {
    let dao = Repository::new();
    let mut conn = dao.connection;

    let query = table
        .filter(open_date_year.eq(q_year))
        .filter(open_date_month.eq(q_month))
        .filter(security_code.eq(q_security_code));

    debug!("{}", diesel::debug_query::<diesel::pg::Pg, _>(&query));

    match query.load::<SecurityPrice>(&mut conn) {
        Ok(rows) => rows,
        Err(e) => {
            error!("{}", SecurityError::SQLError(e));
            Vec::new()
        }
    }
}

pub fn find_all_by_code(
    q_open_date: String,
    q_price_date: String,
    q_security_code: String,
) -> Vec<SecurityPrice> {
    let dao = Repository::new();
    let mut conn = dao.connection;

    let query = sql_query(
        r#"
        SELECT sp.row_id
             , sp.open_date_year
             , sp.open_date_month
             , sp.open_date_day
             , sp.security_code
             , sp.security_name
             , sp.price_date
             , sp.price_close
             , sp.price_avg
             , sp.price_hight
             , sp.price_hight_avg
             , sp.price_lowest
             , sp.price_lowest_avg
             , sp.created_date
             , sp.updated_date
          FROM security_price sp
          WHERE sp.price_date <= $1
            AND sp.price_date !='月平均收盤價' 
            AND sp.security_code = $2
            AND concat(sp.open_date_year, sp.open_date_month, sp.open_date_day) <= $3
         ORDER BY sp.open_date_year, sp.open_date_month, sp.open_date_day, sp.price_date, sp.security_code
        "#,
    )
    .bind::<VarChar, _>(q_price_date)
    .bind::<VarChar, _>(q_security_code)
    .bind::<VarChar, _>(q_open_date);

    debug!("{}", diesel::debug_query::<diesel::pg::Pg, _>(&query));

    match query.load::<SecurityPrice>(&mut conn) {
        Ok(rows) => rows,
        Err(e) => {
            error!("{}", SecurityError::SQLError(e));
            Vec::new()
        }
    }
}

pub fn find_all_by_date(q_year: String, q_month: String, q_day: String) -> Vec<SecurityPrice> {
    let dao = Repository::new();
    let mut conn = dao.connection;

    let query = sql_query(
        r#"
        SELECT sp.row_id
             , sp.open_date_year
             , sp.open_date_month
             , sp.open_date_day
             , sp.security_code
             , sp.security_name
             , sp.price_date
             , sp.price_close
             , sp.price_avg
             , sp.price_hight
             , sp.price_hight_avg
             , sp.price_lowest
             , sp.price_lowest_avg
             , sp.created_date
             , sp.updated_date
          FROM security_price sp
          WHERE sp.price_date = $1
         ORDER BY sp.open_date_year, sp.open_date_month, sp.open_date_day, sp.price_date, sp.security_code
        "#,
    )
    .bind::<VarChar, _>(format!(
        "{0:04}/{1:02}/{2:02}",
        (q_year.parse::<i32>().unwrap() - 1911),
        q_month.parse::<i32>().unwrap(),
        q_day.parse::<i32>().unwrap()
    ));

    debug!("{}", diesel::debug_query::<diesel::pg::Pg, _>(&query));

    match query.load::<SecurityPrice>(&mut conn) {
        Ok(rows) => rows,
        Err(e) => {
            error!("{}", SecurityError::SQLError(e));
            Vec::new()
        }
    }
}

pub fn find_one_by_maxdate() -> Option<MaxPriceDate> {
    let dao = Repository::new();
    let mut conn = dao.connection;

    let query = sql_query(
        r#"
        SELECT COALESCE(MAX(concat(open_date_year, open_date_month, RIGHT(price_date, 2))), '19981231') AS price_date
          FROM security_price sp
        WHERE sp.price_date not like '%月平均收盤價%'
        "#,
    );

    debug!("{}", diesel::debug_query::<diesel::pg::Pg, _>(&query));

    match query.get_result::<MaxPriceDate>(&mut conn).optional() {
        Ok(row) => row,
        Err(e) => {
            error!("{}", SecurityError::SQLError(e));
            None
        }
    }
}

pub fn read_all_by_res(q_year: String, q_month: String, q_day: String) -> Vec<ResposePrice> {
    let dao = Repository::new();
    let mut conn = dao.connection;

    let query = sql_query(r#"
        SELECT rd.data_content
             , st.open_date_year
             , st.open_date_month
             , MAX(st.open_date_day) AS open_date_day 
             , st.security_code
             , st.security_name
             , st.market_type
          FROM response_data rd
          JOIN security_task st
            ON rd.exec_code = st.security_code
           AND rd.open_date_year = st.open_date_year
           AND rd.open_date_month = st.open_date_month
         WHERE rd.open_date_year = $1
           AND rd.open_date_month = $2
           AND rd.open_date_day >= $3
         GROUP BY rd.data_content, st.open_date_year, st.open_date_month, st.security_code, st.security_name , st.market_type
         ORDER BY st.open_date_year, st.open_date_month,  st.security_code
         "#,
    )
    .bind::<VarChar, _>(q_year)
    .bind::<VarChar, _>(q_month)
    .bind::<VarChar, _>(q_day);

    debug!("{}", diesel::debug_query::<diesel::pg::Pg, _>(&query));

    match query.load::<ResposePrice>(&mut conn) {
        Ok(rows) => rows,
        Err(e) => {
            error!("{}", SecurityError::SQLError(e));
            Vec::new()
        }
    }
}

pub fn create(conn: &mut PgConnection, data: NewSecurityPrice) -> Result<usize, SecurityError> {
    match insert_into(table).values(data).execute(conn) {
        Ok(cnt) => Ok(cnt),
        Err(e) => Err(SecurityError::SQLError(e)),
    }
}

pub fn modify(data: SecurityPrice) -> Result<usize, SecurityError> {
    let dao = Repository::new();
    let mut conn = dao.connection;

    match update(table)
        .filter(row_id.eq(data.row_id.clone()))
        .set(data)
        .execute(&mut conn)
    {
        Ok(cnt) => Ok(cnt),
        Err(e) => Err(SecurityError::SQLError(e)),
    }
}

pub fn remove(
    conn: &mut PgConnection,
    q_year: String,
    q_month: String,
    q_security_code: String,
) -> Result<usize, SecurityError> {
    match delete(table)
        .filter(open_date_year.eq(q_year))
        .filter(open_date_month.eq(q_month))
        .filter(security_code.eq(q_security_code))
        .execute(conn)
    {
        Ok(cnt) => Ok(cnt),
        Err(e) => Err(SecurityError::SQLError(e)),
    }
}
