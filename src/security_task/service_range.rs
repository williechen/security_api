#![warn(clippy::all, clippy::pedantic)]

use std::cmp::max;

use sqlx::PgConnection;
use tracing::{event, Level};

use super::{dao, model::SecurityTask};
use crate::{daily_task::model::DailyTaskInfo, repository::Repository};

pub async fn update_task_data(
    db_url: &str,
    task_info: &DailyTaskInfo,
) -> Result<(), Box<dyn std::error::Error>> {
    event!(target: "security_api", Level::INFO, "call daily_task.task_range");
    let pool = Repository::new(db_url).await;
    let mut transaction = pool.connection.acquire().await?;

    let open_date = task_info.open_date.clone().unwrap();

    let twse_list = select_task_to_twse(&mut *transaction, open_date.clone()).await?;
    let tpex_list = select_task_to_tpex(&mut *transaction, open_date.clone()).await?;

    let max_count = max(twse_list.len(), tpex_list.len());

    let mut sort_num = 0;
    for i in 0..max_count {
        if i < twse_list.len() {
            sort_num = sort_num + 1;

            let twse_data = &twse_list[i];
            let mut transaction = pool.connection.acquire().await?;
            loop_data_task_data(&mut *transaction, twse_data.clone(), sort_num).await?;
        }
        if i < tpex_list.len() {
            sort_num = sort_num + 1;

            let tpex_data = &tpex_list[i];
            let mut transaction = pool.connection.acquire().await?;
            loop_data_task_data(&mut *transaction, tpex_data.clone(), sort_num).await?;
        }
    }

    Ok(())
}

async fn loop_data_task_data(
    transaction: &mut PgConnection,
    data: SecurityTask,
    item_index: i32,
) -> Result<(), Box<dyn std::error::Error>> {
    if data.sort_no != Some(item_index) {
        let mut new_data = data.clone();
        new_data.sort_no = Some(item_index);
        dao::update(transaction, new_data).await?;
    }

    Ok(())
}

async fn select_task_to_twse(
    transaction: &mut PgConnection,
    open_date: String,
) -> Result<Vec<SecurityTask>, Box<dyn std::error::Error>> {
    match dao::read_all_by_sql(
        transaction,
        &format!(
            r#" SELECT row_id
                     , open_date
                     , security_code
                     , security_name
                     , market_type
                     , issue_date
                     , security_date
                     , security_seed
                     , exec_count
                     , is_enabled
                     , sort_no
                     , created_date
                     , updated_date
                   FROM security_task 
                  WHERE open_date = '{0}' 
                    AND market_type in ('上市')
                  ORDER BY security_code, issue_date, market_type
            "#,
            open_date
        ),
    )
    .await
    {
        Ok(rows) => Ok(rows.1),
        Err(e) => {
            event!(target: "security_api", Level::ERROR, "security_task.select_temp_to_twse: {}", &e);
            panic!("security_task.select_temp_to_twse Error {}", &e)
        }
    }
}

async fn select_task_to_tpex(
    transaction: &mut PgConnection,
    open_date: String,
) -> Result<Vec<SecurityTask>, Box<dyn std::error::Error>> {
    match dao::read_all_by_sql(
        transaction,
        &format!(
            r#" SELECT row_id
                     , open_date
                     , security_code
                     , security_name
                     , market_type
                     , issue_date
                     , security_date
                     , security_seed
                     , exec_count
                     , is_enabled
                     , sort_no
                     , created_date
                     , updated_date
                   FROM security_task 
                 WHERE open_date = '{0}' 
                   AND market_type in ('上櫃', '興櫃')
                 ORDER BY security_code, issue_date, market_type
            "#,
            open_date
        ),
    )
    .await
    {
        Ok(rows) => Ok(rows.1),
        Err(e) => {
            event!(target: "security_api", Level::ERROR, "security_task.select_temp_to_tpex: {}", &e);
            panic!("security_task.select_temp_to_tpex Error {}", &e)
        }
    }
}
