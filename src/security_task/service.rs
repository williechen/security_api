use chrono::{Datelike, Local, NaiveDate};
use rand::{thread_rng, Rng};
use tracing::{event, Level};

use super::{dao, model::SecurityTask};
use crate::security_temp::{self, model::SecurityTemp};

pub async fn insert_task_data(pool: &sqlx::PgPool) -> Result<(), Box<dyn std::error::Error>> {
    let mut transaction = pool.begin().await?;

    let twse_list = select_temp_to_twse(&mut transaction, Local::now().date_naive()).await?;
    event!(target: "my_api", Level::DEBUG, "{:?}", twse_list);
    loop_date_temp_data(pool, &twse_list, 1).await?;

    let tpex_list = select_temp_to_tpex(&mut transaction, Local::now().date_naive()).await?;
    event!(target: "my_api", Level::DEBUG, "{:?}", tpex_list);
    loop_date_temp_data(pool, &tpex_list, 2).await?;

    Ok(())
}

async fn loop_date_temp_data(
    pool: &sqlx::PgPool,
    data_list: &Vec<SecurityTemp>,
    index: i32,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut item_index = index;

    for data in data_list {
        let now = Local::now().naive_local();
        let issue_date = match &data.issue_date {
            Some(t) => NaiveDate::parse_from_str(t, "%Y/%m/%d")?,
            None => Local::now().date_naive(),
        };

        for y in (issue_date.year()..=now.year()).rev() {
            let mut me = (1..=12).rev();
            if issue_date.year() == y && issue_date.year() == now.year() {
                me = (1..=now.month0()).rev();
            } else if now.year() == y {
                me = (1..=now.month0()).rev();
            } else if issue_date.year() == y {
                me = (issue_date.month()..=12).rev();
            }

            for m in me {
                let mut transaction = pool.begin().await?;

                let mut rng = thread_rng();
                let seed: i64 = rng.gen_range(1000000000000..=9999999999999);

                let twse_date = format!("{:04}{:02}01", y, m);
                let tpex_date = format!("{:03}/{:02}", y - 1911, m);

                let mut query_security_task = SecurityTask::new();
                query_security_task.security_code = match &data.security_code {
                    Some(t) => Some(t.to_string()),
                    None => None,
                };
                query_security_task.twse_date = Some(twse_date.clone());

                let task_list = dao::read_all(&mut transaction, query_security_task).await?;
                if task_list.0 <= 0 {
                    let security_seed = seed.to_string();
                    let sort_no = item_index;

                    let security_task = SecurityTask {
                        row_id: None,
                        market_type: match &data.market_type {
                            Some(t) => Some(t.to_string()),
                            None => None,
                        },
                        security_code: match &data.security_code {
                            Some(t) => Some(t.to_string()),
                            None => None,
                        },
                        issue_date: match &data.issue_date {
                            Some(t) => Some(t.to_string()),
                            None => None,
                        },
                        twse_date: Some(twse_date),
                        tpex_date: Some(tpex_date),
                        security_seed: Some(security_seed),
                        is_enabled: Some(1),
                        sort_no: Some(sort_no),
                        created_date: Some(now),
                        updated_date: Some(now),
                    };

                    match dao::create(&mut transaction, security_task).await {
                        Ok(_) => transaction.commit().await?,
                        Err(_) => transaction.rollback().await?,
                    };
                }
            }
        }
        item_index = item_index + 2;
    }

    Ok(())
}

async fn select_temp_to_twse(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    date: NaiveDate,
) -> Result<Vec<SecurityTemp>, Box<dyn std::error::Error>> {
    match security_temp::dao::read_all_by_sql(transaction,
        &format!(r#" SELECT row_id
                      , version_code 
                      , international_code
                      , security_code
                      , security_name
                      , market_type
                      , security_type
                      , industry_type
                      , issue_date
                      , cfi_code
                      , remark
                      , is_enabled
                      , created_date
                      , updated_date
                   FROM security_temp 
                  WHERE version_code='{}' 
                    AND market_type in ('上市')
                    AND security_type in ('ETF', 'ETN', '股票', '特別股', '轉換公司債', '交換公司債')
                  ORDER BY market_type, security_type, security_code, issue_date
            "#, date.format("%Y%m%d"))).await{
            Ok(rows) => Ok(rows.1),
            Err(e) => Ok(vec![])
                    }
}

async fn select_temp_to_tpex(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    date: NaiveDate,
) -> Result<Vec<SecurityTemp>, Box<dyn std::error::Error>> {
    match security_temp::dao::read_all_by_sql(transaction,
        &format!(r#" SELECT row_id
        , version_code 
        , international_code
        , security_code
        , security_name
        , market_type
        , security_type
        , industry_type
        , issue_date
        , cfi_code
        , remark
        , is_enabled
        , created_date
        , updated_date
                   FROM security_temp 
                  WHERE version_code='{}' 
                    AND market_type in ('上櫃', '興櫃')
                    AND security_type in ('ETF', 'ETN', '股票', '特別股', '轉換公司債', '交換公司債')
                    ORDER BY market_type, security_type, security_code, issue_date
            "#,date.format("%Y%m%d"))).await{
                Ok(rows) => Ok(rows.1),
                Err(e) => Ok(vec![])
                        }
}


