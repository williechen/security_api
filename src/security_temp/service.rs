#![warn(clippy::all, clippy::pedantic)]

use std::collections::HashMap;

use chrono::Local;
use diesel::{Connection, PgConnection};
use log::{debug, info};
use scraper::{Html, Selector};

use crate::{
    daily_task::model::DailyTask, repository::Repository, response_data,
    security_error::SecurityError, security_temp::model::NewSecurityTemp,
};

use super::dao;

pub fn delete_temp() -> Result<(), SecurityError> {
    info!(target: "security_api", "call daily_task.delete_temp");

    dao::remove_all()?;

    Ok(())
}

pub fn get_security_to_temp(task: &DailyTask) -> Result<(), SecurityError> {
    info!(target: "security_api", "call daily_task.get_security_to_temp");
    let dao = Repository::new();
    let mut conn = dao.connection;

    let q_year = task.clone().open_date_year;
    let q_month = task.clone().open_date_month;
    let q_day = task.clone().open_date_day;
    let q_exec_code = "security".to_string();

    let data = response_data::dao::find_one(q_year, q_month, q_day, q_exec_code);
    if data.is_some() {
        let data_content = data.unwrap().data_content;
        conn.transaction::<_, SecurityError, _>(|trax_conn| {
            insert_temp_data(trax_conn, data_content, &task)
        })?;
    }

    Ok(())
}

fn get_new_security_temp(content: HashMap<String, String>, task: &DailyTask) -> NewSecurityTemp {
    NewSecurityTemp {
        open_date_year: task.open_date_year.clone(),
        open_date_month: task.open_date_month.clone(),
        open_date_day: task.open_date_day.clone(),
        international_code: content.get("1").cloned().unwrap_or(String::new()),
        security_code: content.get("2").cloned().unwrap_or(String::new()),
        security_name: content.get("3").cloned().unwrap_or(String::new()),
        market_type: content.get("4").cloned().unwrap_or(String::new()),
        security_type: content.get("5").cloned().unwrap_or(String::new()),
        industry_type: content.get("6").cloned().unwrap_or(String::new()),
        issue_date: content.get("7").cloned().unwrap_or(String::new()),
        cfi_code: content.get("8").cloned().unwrap_or(String::new()),
        remark: content.get("9").cloned().unwrap_or(String::new()),
        created_date: Local::now().naive_local(),
        updated_date: Local::now().naive_local(),
    }
}

fn check_data_exists(temp: &NewSecurityTemp) -> bool {
    let q_year = temp.open_date_year.clone();
    let q_month = temp.open_date_month.clone();
    let q_day = temp.open_date_day.clone();
    let q_security_code = temp.security_code.clone();
    let q_market_type = temp.market_type.clone();
    let q_issue_date = temp.issue_date.clone();

    dao::find_one(
        q_year,
        q_month,
        q_day,
        q_security_code,
        q_market_type,
        q_issue_date,
    )
    .is_some()
}

fn insert_temp_data(
    transaction: &mut PgConnection,
    data_content: String,
    task: &DailyTask,
) -> Result<(), SecurityError> {
    let mut security_temps = Vec::<NewSecurityTemp>::new();

    let rows = parse_table_data(data_content).unwrap();
    for row in rows {
        debug!(target: "security_api", "ROW: {:?}", &row);
        security_temps.push(get_new_security_temp(row, task));
    }

    for security_temp in security_temps {
        if !check_data_exists(&security_temp) {
            dao::create(transaction, security_temp)?;
        }
    }

    Ok(())
}

fn html_decode(input: &str) -> String {
    input
        .replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&apos;", "'")
        .replace("*", "")
        .replace("＊", "")
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
            let a = html_decode(&td_content.inner_html());
            cells.insert(index.to_string(), a.trim().to_string());

            index = index + 1;
        }

        rows.push(cells);
    }
    rows.remove(0);

    Ok(rows)
}
