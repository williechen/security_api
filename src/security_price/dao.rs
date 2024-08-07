#![warn(clippy::all, clippy::pedantic)]

use crate::repository::Repository;
use crate::schema::security_price::dsl::security_price as table;
use crate::schema::security_price::row_id;
use diesel::{insert_into, update, ExpressionMethods, PgConnection};
use diesel::{sql_query, sql_types::VarChar, RunQueryDsl};
use log::{debug, info};

use super::model::{NewSecurityPrice, ResposePrice, SecurityPrice};

pub fn find_all_by_code(q_price_date: String, q_security_code: String) -> Vec<SecurityPrice> {
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
           AND sp.security_code = $2
         ORDER BY sp.open_date_year, sp.open_date_month, sp.open_date_day, sp.security_code
        "#,
    )
    .bind::<VarChar, _>(q_price_date)
    .bind::<VarChar, _>(q_security_code);

    debug!("{}", diesel::debug_query::<diesel::pg::Pg, _>(&query));

    match query.load::<SecurityPrice>(&mut conn) {
        Ok(rows) => rows,
        Err(e) => {
            debug!("find_all_by_code {}", e);
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
         ORDER BY sp.open_date_year, sp.open_date_month, sp.open_date_day, sp.security_code
        "#,
    )
    .bind::<VarChar, _>(format!(
        "{0}/{1}/{2}",
        (q_year.parse::<i32>().unwrap() - 1911),
        q_month,
        q_day
    ));

    debug!("{}", diesel::debug_query::<diesel::pg::Pg, _>(&query));

    match query.load::<SecurityPrice>(&mut conn) {
        Ok(rows) => rows,
        Err(e) => {
            debug!("find_one_by_date {}", e);
            Vec::new()
        }
    }
}

pub fn read_all_by_res(q_year: String, q_month: String, q_day: String) -> Vec<ResposePrice> {
    let dao = Repository::new();
    let mut conn = dao.connection;

    let query = sql_query(
        r#"
        SELECT rd.data_content
             , st.open_date_year
             , st.open_date_month
             , st.open_date_day
             , st.security_code
             , st.security_name
             , st.market_type
          FROM response_data rd
          JOIN security_task st
            ON rd.exec_code = st.security_code
           AND rd.open_date_year = st.open_date_year
           AND rd.open_date_month = st.open_date_month
           AND rd.open_date_day = st.open_date_day
         WHERE rd.open_date_year = $1
           AND rd.open_date_month = $2
           AND rd.open_date_day >= $3 
         ORDER BY rd.open_date_year, rd.open_date_month, rd.open_date_day, st.security_code
         "#,
    )
    .bind::<VarChar, _>(q_year)
    .bind::<VarChar, _>(q_month)
    .bind::<VarChar, _>(q_day);

    debug!("{}", diesel::debug_query::<diesel::pg::Pg, _>(&query));

    match query.load::<ResposePrice>(&mut conn) {
        Ok(rows) => rows,
        Err(e) => {
            debug!("read_all_by_res {}", e);
            Vec::new()
        }
    }
}

pub fn create(
    conn: &mut PgConnection,
    data: NewSecurityPrice,
) -> Result<usize, diesel::result::Error> {
    insert_into(table).values(data).execute(conn)
}

pub fn modify(data: SecurityPrice) -> Result<usize, diesel::result::Error> {
    let dao = Repository::new();
    let mut conn = dao.connection;

    update(table)
        .filter(row_id.eq(data.row_id.clone()))
        .set(data)
        .execute(&mut conn)
}
