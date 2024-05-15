use std::collections::HashMap;

use chrono::Local;
use scraper::{Html, Selector};
use tracing::{event, Level};

use super::{dao, model::SecurityTemp};

pub async fn insert_temp_data(
    pool: &sqlx::PgPool,
    data_content: &String,
) -> Result<(), Box<dyn std::error::Error>> {
    let rows = parse_table_data(data_content)?;
    for row in rows {
        let mut transaction_loop = pool.begin().await?;

        let now = Local::now().naive_local();

        let mut query_security_temp = SecurityTemp::new();
        query_security_temp.version_code = Some(now.format("%Y%m%d").to_string());
        query_security_temp.security_code = match row.get("2") {
            Some(t) => Some(t.to_owned()),
            None => None,
        };

        let data_list = dao::read_all(&mut transaction_loop, &query_security_temp).await?;
        if data_list.0 <= 0 {
            let security_temp = SecurityTemp {
                row_id: None,
                version_code: Some(now.format("%Y%m%d").to_string()),
                international_code: match row.get("1") {
                    Some(t) => Some(t.to_owned()),
                    None => None,
                },
                security_code: match row.get("2") {
                    Some(t) => Some(t.to_owned()),
                    None => None,
                },
                security_name: match row.get("3") {
                    Some(t) => Some(t.to_owned()),
                    None => None,
                },
                market_type: match row.get("4") {
                    Some(t) => Some(t.to_owned()),
                    None => None,
                },
                security_type: match row.get("5") {
                    Some(t) => Some(t.to_owned()),
                    None => None,
                },
                industry_type: match row.get("6") {
                    Some(t) => Some(t.to_owned()),
                    None => None,
                },
                issue_date: match row.get("7") {
                    Some(t) => Some(t.to_owned()),
                    None => None,
                },
                cfi_code: match row.get("8") {
                    Some(t) => Some(t.to_owned()),
                    None => None,
                },
                remark: match row.get("9") {
                    Some(t) => Some(t.to_owned()),
                    None => None,
                },
            };

            match dao::create(&mut transaction_loop, security_temp).await {
                Ok(_) => transaction_loop.commit().await?,
                Err(e) => {
                    transaction_loop.rollback().await?;
                    event!(target: "security_api", Level::ERROR, "{:?}", e);
                }
            };
        }
    }

    Ok(())
}

fn parse_table_data(
    table: &String,
) -> Result<Vec<HashMap<String, String>>, Box<dyn std::error::Error>> {
    let mut rows: Vec<HashMap<String, String>> = vec![];

    let fragment = Html::parse_fragment(table);

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
