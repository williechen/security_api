use chrono::{Local, NaiveDate};
use tracing::{event, Level};

use super::{dao, model::DailyTask};

pub async fn insert_task_data(pool: &sqlx::PgPool) -> Result<(), Box<dyn std::error::Error>> {
    let mut transaction = pool.begin().await?;

    let task_list = select_task(&mut transaction, Local::now().date_naive()).await?;
    event!(target: "security_api", Level::DEBUG, "{:?}", &task_list);
    loop_date_task_data(pool, &task_list).await?;

    Ok(())
}

async fn loop_date_task_data(
    pool: &sqlx::PgPool,
    data_list: &Vec<DailyTask>,
) -> Result<(), Box<dyn std::error::Error>> {
    for data in data_list {
        let mut transaction = pool.begin().await?;

        let query_daily_task = DailyTask {
            row_id: None,
            version_code: None,
            open_date: data.open_date.clone(),
            job_code: data.job_code.clone(),
            exec_status: None,
        };

        let task_list = dao::read_all(&mut transaction, &query_daily_task).await?;
        if task_list.0 <= 0 {
            match dao::create(&mut transaction, data.clone()).await {
                Ok(_) => transaction.commit().await?,
                Err(e) => {
                    transaction.rollback().await?;
                    event!(target: "security_api", Level::ERROR, "{:?}", &e);
                }
            };
        } else {
            match dao::update(&mut transaction, data.clone()).await {
                Ok(_) => transaction.commit().await?,
                Err(e) => {
                    transaction.rollback().await?;
                    event!(target: "security_api", Level::ERROR, "{:?}", &e);
                }
            };
        }
    }

    Ok(())
}

async fn select_task(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    date: NaiveDate,
) -> Result<Vec<DailyTask>, Box<dyn std::error::Error>> {
    match dao::read_all_by_sql(
        transaction,
        &format!(
            r#"
            SELECT '' AS row_id
                 , '{0}' AS version_code 
                 , CONCAT(cd.ce_year, cd.ce_month, cd.ce_day) AS open_date
                 , ts.job_code 
                 , 'WAIT' AS exec_status
              FROM calendar_data cd
              JOIN task_setting ts
                ON cd.group_task = ts.group_code
              WHERE NOT EXISTS (
                SELECT 1 
                  FROM daily_task dt
                 WHERE dt.open_date = CONCAT(cd.ce_year, cd.ce_month, cd.ce_day)
                   AND dt.job_code = ts.job_code
                   AND dt.exec_status = 'EXIT'
              )
                AND CONCAT(cd.ce_year, cd.ce_month, cd.ce_day) <= '{0}'
                AND cd.date_status = 'O'
             ORDER BY cd.ce_month desc, cd.ce_day desc, cd.ce_year desc, ts.sort_no  
            "#,
            date.format("%Y%m%d")
        ),
    )
    .await
    {
        Ok(rows) => Ok(rows.1),
        Err(e) => {
            event!(target: "security_api", Level::ERROR, "{:?}", &e);
            Ok(vec![])
        }
    }
}
