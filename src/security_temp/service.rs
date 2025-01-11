#![warn(clippy::all, clippy::pedantic)]

use std::collections::HashMap;

use scraper::{Html, Selector};
use sqlx::PgConnection;
use tracing::{event, Level};

use crate::{daily_task::model::DailyTask, repository::Repository, response_data};

use super::{dao, model::SecurityTemp};

pub async fn delete_temp() -> Result<(), sqlx::Error> {
    event!(target: "security_api", Level::INFO, "call daily_task.delete_temp");

    dao::remove_all().await?;

    Ok(())
}

pub async fn get_security_to_temp(task: &DailyTask) -> Result<(), sqlx::Error> {
    event!(target: "security_api", Level::INFO, "call daily_task.get_security_to_temp");
    let dao = Repository::new().await;
    let mut conn = dao.connection.begin().await?;

    let q_year = &task.open_date_year;
    let q_month = &task.open_date_month;
    let q_day = &task.open_date_day;
    let q_exec_code = "security";

    let data = response_data::dao::find_one(q_year, q_month, q_day, q_exec_code).await;
    if data.is_some() {
        let data_content = data.unwrap().data_content;

        match insert_temp_data(&mut conn, &data_content, &task).await {
            Ok(_) => conn.commit().await?,
            Err(_) => conn.rollback().await?,
        }
    }

    Ok(())
}

async fn insert_temp_data(
    transaction: &mut PgConnection,
    data_content: &str,
    task: &DailyTask,
) -> Result<(), sqlx::Error> {
    let rows = parse_table_data(data_content).unwrap();
    for row in rows {
        event!(target: "security_api", Level::DEBUG, "ROW: {:?}", &row);
        loop_data_temp(transaction, &row, &task).await?;
    }

    Ok(())
}

async fn loop_data_temp(
    transaction: &mut PgConnection,
    content: &HashMap<String, String>,
    task: &DailyTask,
) -> Result<(), sqlx::Error> {
    let q_year = &task.open_date_year;
    let q_month = &task.open_date_month;
    let q_day = &task.open_date_day;
    let q_security_code = content.get("2").map_or("", |v| v);
    let q_market_type = content.get("4").map_or("", |v| v);
    let q_issue_date = content.get("7").map_or("", |v| v);

    let data = dao::find_one(
        q_year,
        q_month,
        q_day,
        q_security_code,
        q_market_type,
        q_issue_date,
    )
    .await;
    if data.is_none() {
        let security_temp = SecurityTemp {
            row_id: String::new(),
            open_date_year: task.open_date_year.clone(),
            open_date_month: task.open_date_month.clone(),
            open_date_day: task.open_date_day.clone(),
            international_code: content.get("1").map_or("", |v| v).to_string(),
            security_code: content.get("2").map_or("", |v| v).to_string(),
            security_name: content.get("3").map_or("", |v| v).to_string(),
            market_type: content.get("4").map_or("", |v| v).to_string(),
            security_type: content.get("5").map_or("", |v| v).to_string(),
            industry_type: content.get("6").map_or("", |v| v).to_string(),
            issue_date: content.get("7").map_or("", |v| v).to_string(),
            cfi_code: content.get("8").map_or("", |v| v).to_string(),
            remark: content.get("9").map_or("", |v| v).to_string(),
        };
        dao::create(transaction, security_temp).await?;
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
    table: &str,
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
