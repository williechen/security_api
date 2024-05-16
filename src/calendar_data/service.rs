use chrono::{Datelike, Local, NaiveDate};
use tracing::{event, Level};

use super::{dao, model::CalendarData};

pub async fn init_calendar_data(pool: &sqlx::PgPool) -> Result<(), Box<dyn std::error::Error>> {
    let max_date = Local::now().date_naive();
    let min_date = NaiveDate::from_ymd_opt(1962, 2, 9).unwrap();

    let max_date_str = max_date.format("%Y%m%d").to_string();
    let min_date_str = min_date.format("%Y%m%d").to_string();

    for y in min_date.year()..=max_date.year() {
        for m in 1..=12 {
            for d in 1..=last_day_in_month(y, m).day() {
                let this_date_str = format!("{:04}{:02}{:02}", y, m, d);

                if (max_date_str > this_date_str) && (min_date_str <= this_date_str) {
                    loop_date_calendar(pool, y, m, d).await?;
                }
            }
        }
    }

    Ok(())
}

pub async fn insert_calendar_data(pool: &sqlx::PgPool) -> Result<(), Box<dyn std::error::Error>> {
    let now = Local::now().date_naive();

    let year = now.year();
    for m in 1..=now.month() {
        for d in 1..=last_day_in_month(year, m).day() {
            let mut transaction = pool.begin().await?;

            let query_cal = CalendarData {
                row_id: None,
                ce_year: Some(format!("{:04}", year)),
                tw_year: Some(format!("{:03}", year - 1911)),
                ce_month: Some(format!("{:02}", m)),
                ce_day: Some(format!("{:02}", d)),
                date_status: None,
                group_task: None,
            };
            let cal_list = dao::read_all(&mut transaction, &query_cal).await?;
            if cal_list.0 <= 0 {
                loop_date_calendar(pool, year, m, d).await?;
            }
        }
    }
    Ok(())
}

fn last_day_in_month(year: i32, month: u32) -> NaiveDate {
    let (y, m) = if month == 12 {
        (year + 1, 1)
    } else {
        (year, month + 1)
    };
    NaiveDate::from_ymd_opt(y, m, 1)
        .unwrap()
        .pred_opt()
        .unwrap()
}

async fn loop_date_calendar(
    pool: &sqlx::PgPool,
    year: i32,
    month: u32,
    day: u32,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut transaction = pool.begin().await?;

    let this_date = NaiveDate::from_ymd_opt(year, month, day).unwrap();
    if this_date.weekday().number_from_monday() == 6
        || this_date.weekday().number_from_monday() == 7
    {
        let calendar_data = CalendarData {
            row_id: None,
            ce_year: Some(format!("{:04}", year)),
            tw_year: Some(format!("{:03}", year - 1911)),
            ce_month: Some(format!("{:02}", month)),
            ce_day: Some(format!("{:02}", day)),
            date_status: Some("S".to_string()),
            group_task: Some("STOP".to_string()),
        };

        match dao::create(&mut transaction, calendar_data).await {
            Ok(_) => transaction.commit().await?,
            Err(e) => {
                transaction.rollback().await?;
                event!(target: "security_api", Level::ERROR, "{:?}", &e);
            }
        };
    } else {
        let calendar_data = CalendarData {
            row_id: None,
            ce_year: Some(format!("{:04}", year)),
            tw_year: Some(format!("{:03}", year - 1911)),
            ce_month: Some(format!("{:02}", month)),
            ce_day: Some(format!("{:02}", day)),
            date_status: Some("O".to_string()),
            group_task: Some("SECURITY".to_string()),
        };

        match dao::create(&mut transaction, calendar_data).await {
            Ok(_) => transaction.commit().await?,
            Err(e) => {
                transaction.rollback().await?;
                event!(target: "security_api", Level::ERROR, "{:?}", &e);
            }
        };
    }

    Ok(())
}