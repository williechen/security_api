use std::collections::HashMap;

use scraper::{Html, Selector};
use tracing::{event, instrument, Level};

use crate::daily_task::model::DailyTaskInfo;

use super::{dao, model::SecurityTemp};

#[instrument]
pub async fn insert_temp_data(
    pool: sqlx::PgPool,
    data_content: String,
    task_info: DailyTaskInfo,
) -> Result<(), Box<dyn std::error::Error>> {
    let rows = parse_table_data(data_content)?;
    for row in rows {
        let mut transaction = pool.clone().begin().await?;

        let open_date = task_info.open_date.clone().unwrap();

        let mut query_security_temp = SecurityTemp::new();
        query_security_temp.open_date = Some(open_date.clone());
        query_security_temp.security_code = row.get("2").cloned();

        let data_list = dao::read_all(&mut transaction, query_security_temp).await?;
        if data_list.0 <= 0 {
            let security_temp = SecurityTemp {
                row_id: None,
                open_date: Some(open_date.clone()),
                international_code: row.get("1").cloned(),
                security_code: row.get("2").cloned(),
                security_name: row.get("3").cloned(),
                market_type: row.get("4").cloned(),
                security_type: row.get("5").cloned(),
                industry_type: row.get("6").cloned(),
                issue_date: row.get("7").cloned(),
                cfi_code: row.get("8").cloned(),
                remark: row.get("9").cloned(),
            };

            match dao::create(&mut transaction, security_temp).await {
                Ok(_) => transaction.commit().await?,
                Err(e) => {
                    transaction.rollback().await?;
                    event!(target: "security_api", Level::ERROR, "security_temp.insert_temp_data: {}", &e);
                    panic!("security_temp.insert_temp_data Error {}", &e)
                }
            };
        }
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
