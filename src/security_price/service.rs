use std::collections::HashMap;

use scraper::{Html, Selector};
use sqlx::PgConnection;
use tracing::{event, Level};

use crate::{
    daily_task::model::DailyTaskInfo,
    repository::Repository,
    response_data::{self, model::ResponseData},
};

use super::{dao, model::SecurityTemp};

pub async fn get_security_to_price(
    db_url: &str,
    task_info: &DailyTaskInfo,
) -> Result<(), Box<dyn std::error::Error>> {
    event!(target: "security_api", Level::INFO, "call daily_task.get_security_to_price");
    let pool = Repository::new(db_url).await;
    let mut transaction = pool.connection.acquire().await?;

    let query_response_data = ResponseData {
        row_id: None,
        open_date: task_info.open_date.clone(),
        exec_code: Some("security".to_string()),
        data_content: None,
    };

    let data_list = response_data::dao::read_all(&mut *transaction, query_response_data).await?;
    if data_list.0 > 0 {
        let first_data = data_list.1.get(0);
        let response_data = first_data.clone().unwrap();
        let data_content = response_data.data_content.clone().unwrap();

        let mut transaction = pool.connection.begin().await?;
        match insert_price_data(&mut *transaction, data_content, task_info.clone()).await {
            Ok(_) => transaction.commit().await?,
            Err(_) => transaction.rollback().await?,
        }
    }

    Ok(())
}

pub async fn insert_price_data(
    transaction: &mut PgConnection,
    data_content: String,
    task_info: DailyTaskInfo,
) -> Result<(), Box<dyn std::error::Error>> {
    let rows = parse_table_data(data_content)?;
    for row in rows {
        loop_data_temp(&mut *transaction, row, &task_info).await?;
    }

    Ok(())
}

async fn loop_data_temp(
    transaction: &mut PgConnection,
    content: HashMap<String, String>,
    task_info: &DailyTaskInfo,
) -> Result<(), Box<dyn std::error::Error>> {
    let open_date = task_info.open_date.clone().unwrap();

    let mut query_security_temp = SecurityTemp::new();
    query_security_temp.open_date = Some(open_date.clone());
    query_security_temp.security_code = content.get("2").cloned();

    let data_list = dao::read_all(transaction, query_security_temp).await?;
    if data_list.0 <= 0 {
        let security_temp = SecurityTemp {
            row_id: None,
            open_date: Some(open_date.clone()),
            international_code: content.get("1").cloned(),
            security_code: content.get("2").cloned(),
            security_name: content.get("3").cloned(),
            market_type: content.get("4").cloned(),
            security_type: content.get("5").cloned(),
            industry_type: content.get("6").cloned(),
            issue_date: content.get("7").cloned(),
            cfi_code: content.get("8").cloned(),
            remark: content.get("9").cloned(),
        };
        dao::create(transaction, security_temp).await?;
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
