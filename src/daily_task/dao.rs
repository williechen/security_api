#![warn(clippy::all, clippy::pedantic)]
use chrono::{Datelike, Local};
use diesel::dsl::insert_into;
use diesel::sql_types::VarChar;
use diesel::{sql_query, update, ExpressionMethods, QueryDsl, RunQueryDsl};
use log::debug;

use crate::repository::Repository;
use crate::schema::daily_task::dsl::daily_task as table;
use crate::schema::daily_task::{job_code, open_date_day, open_date_month, open_date_year, row_id};

use super::model::{DailyTask, NewDailyTask};

pub fn find_all() -> Vec<DailyTask> {
    let dao = Repository::new();
    let mut conn = dao.connection.get().unwrap();

    let now = Local::now();

    let query = sql_query(
        r#"
            SELECT '' AS row_id
                 , cd.ce_year AS open_date_year
                 , cd.ce_month AS open_date_month
                 , cd.ce_day AS open_date_day
                 , ts.job_code 
                 , 'WAIT' AS exec_status
                 , now() AS created_date
                 , now() AS updated_date
              FROM calendar_data cd
              JOIN task_setting ts
                ON cd.group_task = ts.group_code
             WHERE NOT EXISTS (
                   SELECT 1 
                     FROM daily_task dt
                    WHERE dt.open_date_year = cd.ce_year 
                      AND dt.open_date_month = cd.ce_month
                      AND dt.open_date_day = cd.ce_day
                      AND dt.job_code = ts.job_code
             )
              AND cd.ce_year = $1
              AND cd.ce_month = $2
              AND cd.ce_day = $3
              AND cd.date_status = 'O'
            ORDER BY cd.ce_year desc, cd.ce_month desc, cd.ce_day desc, ts.sort_no  
            "#,
    )
    .bind::<VarChar, _>(format!("{:04}", now.year()))
    .bind::<VarChar, _>(format!("{:02}", now.month()))
    .bind::<VarChar, _>(format!("{:02}", now.day()));

    debug!("{}", diesel::debug_query::<diesel::pg::Pg, _>(&query));

    query.load::<DailyTask>(&mut conn).unwrap()
}

pub fn find_one(
    q_year: String,
    q_month: String,
    q_day: String,
    q_job_code: String,
) -> Option<DailyTask> {
    let dao = Repository::new();
    let mut conn = dao.connection.get().unwrap();

    let query = table
        .filter(open_date_year.eq(q_year))
        .filter(open_date_month.eq(q_month))
        .filter(open_date_day.eq(q_day))
        .filter(job_code.eq(q_job_code));

    debug!("{}", diesel::debug_query::<diesel::pg::Pg, _>(&query));

    match query.first::<DailyTask>(&mut conn) {
        Ok(row) => Some(row),
        Err(e) => {
            debug!("find_one {}", e);
            None
        }
    }
}

pub fn create(data: NewDailyTask) -> Result<usize, diesel::result::Error> {
    let dao = Repository::new();
    let mut conn = dao.connection.get().unwrap();

    insert_into(table).values(data).execute(&mut conn)
}

pub fn modify(data: DailyTask) -> Result<usize, diesel::result::Error> {
    let dao = Repository::new();
    let mut conn = dao.connection.get().unwrap();

    update(table)
        .filter(open_date_year.eq(data.open_date_year.clone()))
        .filter(open_date_month.eq(data.open_date_month.clone()))
        .filter(open_date_day.eq(data.open_date_day.clone()))
        .filter(job_code.eq(data.job_code.clone()))
        .set(data)
        .execute(&mut conn)
}

pub fn find_one_by_exec_asc(flow_code: String) -> Option<DailyTask> {
    let dao = Repository::new();
    let mut conn = dao.connection.get().unwrap();

    let query = sql_query(
        r#"
        SELECT distinct '' AS row_id
                 , '' AS job_code 
                 , '' AS exec_status
                 , now() AS created_date
                 , now() AS updated_date 
                 , dt.open_date_year
                 , dt.open_date_month
                 , dt.open_date_day
          FROM daily_task dt
         WHERE dt.exec_status in ('WAIT', 'OPEN', 'EXEC')
           AND NOT EXISTS (
               SELECT 1 
                 FROM listen_flow lf
                WHERE lf.flow_code = $1
                  AND lf.flow_param1 = cd.ce_year
                  AND lf.flow_param2 = cd.ce_month
            )
         ORDER BY dt.open_date_year, dt.open_date_month, dt.open_date_day
         Limit 1
         "#,
    )
    .bind::<VarChar, _>(flow_code);

    debug!("{}", diesel::debug_query::<diesel::pg::Pg, _>(&query));

    match query.get_result::<DailyTask>(&mut conn) {
        Ok(row) => Some(row),
        Err(e) => {
            debug!("find_one_by_exec_asc {}", e);
            None
        }
    }
}

pub fn find_one_by_exec_desc(flow_code: String) -> Option<DailyTask> {
    let dao = Repository::new();
    let mut conn = dao.connection.get().unwrap();

    let query = sql_query(
        r#"
        SELECT distinct '' AS row_id
                 , '' AS job_code 
                 , '' AS exec_status
                 , now() AS created_date
                 , now() AS updated_date 
                 , dt.open_date_year
                 , dt.open_date_month
                 , dt.open_date_day
          FROM daily_task dt
         WHERE dt.exec_status in ('WAIT', 'OPEN', 'EXEC')
           AND NOT EXISTS (
               SELECT 1 
                 FROM listen_flow lf
                WHERE lf.flow_code = $1
                  AND lf.flow_param1 = cd.ce_year
                  AND lf.flow_param2 = cd.ce_month
            )
         ORDER BY dt.open_date_year desc, dt.open_date_month desc, dt.open_date_day desc
         Limit 1
         "#,
    )
    .bind::<VarChar, _>(flow_code);

    debug!("{}", diesel::debug_query::<diesel::pg::Pg, _>(&query));

    match query.get_result::<DailyTask>(&mut conn) {
        Ok(row) => Some(row),
        Err(e) => {
            debug!("find_one_by_exec_desc {}", e);
            None
        }
    }
}

pub fn find_all_by_exec(q_year: String, q_month: String) -> Vec<DailyTask> {
    let dao = Repository::new();
    let mut conn = dao.connection.get().unwrap();

    let query = sql_query(
        r#"
        SELECT dt.row_id
             , dt.open_date_year
             , dt.open_date_month
             , dt.open_date_day
             , dt.job_code
             , dt.exec_status
             , dt.created_date
             , dt.updated_date
          FROM daily_task dt
          JOIN task_setting ts
            ON ts.group_code = cd.group_task 
           AND ts.job_code = dt.job_code
         WHERE dt.open_date_year = $1
           AND dt.open_date_month = $2
           AND dt.exec_status in ('WAIT', 'OPEN', 'EXEC')
         ORDER BY dt.open_date_year desc, dt.open_date_month desc, dt.open_date_day desc,ts.sort_no
         "#,
    )
    .bind::<VarChar, _>(q_year)
    .bind::<VarChar, _>(q_month);

    debug!("{}", diesel::debug_query::<diesel::pg::Pg, _>(&query));

    match query.get_results::<DailyTask>(&mut conn) {
        Ok(rows) => rows,
        Err(e) => {
            debug!("find_all_by_exec {}", e);
            Vec::new()
        }
    }
}
