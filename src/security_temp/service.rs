#![warn(clippy::all, clippy::pedantic)]

use std::collections::HashMap;

use chrono::Local;
use diesel::{Connection, PgConnection};
use log::{debug, info};
use scraper::{Html, Selector};

use crate::{
    daily_task::model::DailyTask, repository::Repository, response_data,
    security_temp::model::NewSecurityTemp,
};

use super::dao;

pub fn delete_temp() -> Result<(), Box<dyn std::error::Error>> {
    info!(target: "security_api", "call daily_task.delete_temp");

    dao::remove_all();

    Ok(())
}

pub fn get_security_to_temp(task: &DailyTask) -> Result<(), Box<dyn std::error::Error>> {
    info!(target: "security_api", "call daily_task.get_security_to_temp");
    let dao = Repository::new();
    let mut conn = dao.connection.get().unwrap();

    let q_year = task.clone().open_date_year;
    let q_month = task.clone().open_date_month;
    let q_day = task.clone().open_date_day;
    let q_exec_code = "security".to_string();

    let data = response_data::dao::find_one(q_year, q_month, q_day, q_exec_code);
    if data.is_none() {
        let data_content = data.unwrap().data_content;

        conn.transaction(|conn| insert_temp_data(conn, data_content, &task));
    }

    Ok(())
}

fn insert_temp_data(
    transaction: &mut PgConnection,
    data_content: String,
    task: &DailyTask,
) -> Result<(), diesel::result::Error> {
    let rows = parse_table_data(data_content).unwrap();
    for row in rows {
        debug!(target: "security_api", "ROW: {:?}", &row);
        loop_data_temp(&mut *transaction, row, &task)?;
    }

    Ok(())
}

fn loop_data_temp(
    transaction: &mut PgConnection,
    content: HashMap<String, String>,
    task: &DailyTask,
) -> Result<(), diesel::result::Error> {
    let q_year = task.clone().open_date_year;
    let q_month = task.clone().open_date_month;
    let q_day = task.clone().open_date_day;
    let q_security_code = content.get("2").cloned().unwrap();
    let q_market_type = content.get("4").cloned().unwrap();
    let q_issue_date = content.get("7").cloned().unwrap();

    let data = dao::find_one(
        q_year,
        q_month,
        q_day,
        q_security_code,
        q_market_type,
        q_issue_date,
    );
    if data.is_none() {
        let security_temp = NewSecurityTemp {
            open_date_year: task.clone().open_date_year,
            open_date_month: task.clone().open_date_month,
            open_date_day: task.clone().open_date_day,
            international_code: content.get("1").cloned().unwrap(),
            security_code: content.get("2").cloned().unwrap(),
            security_name: content.get("3").cloned().unwrap(),
            market_type: content.get("4").cloned().unwrap(),
            security_type: content.get("5").cloned().unwrap(),
            industry_type: content.get("6").cloned().unwrap(),
            issue_date: content.get("7").cloned().unwrap(),
            cfi_code: content.get("8").cloned().unwrap(),
            remark: content.get("9").cloned().unwrap(),
            created_date: Local::now().naive_local(),
            updated_date: Local::now().naive_local(),
        };
        dao::create(transaction, security_temp);
    }
    Ok(())
}

fn parse_table_data(
    table: String,
) -> Result<Vec<HashMap<String, String>>, Box<dyn std::error::Error>> {
    let mut rows: Vec<HashMap<String, String>> = vec![];

    let fragment = Html::parse_fragment(&table);

    let tr = Selector::parse("tr").unwrap();
    let td = Selector::parse("td").unwrap();

    let trs = fragment.select(&tr);
    for tr_content in trs {
        let mut index = 0;
        let mut cells = HashMap::<String, String>::new();

        let tds = tr_content.select(&td);
        for td_content in tds {
            let a = td_content.inner_html();
            cells.insert(index.to_string(), a.trim().to_string());

            index = index + 1;
        }

        rows.push(cells);
    }
    rows.remove(0);

    Ok(rows)
}
