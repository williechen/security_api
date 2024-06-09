use chrono::{Datelike, Local, NaiveDate};
use sqlx::{Postgres, Transaction};
use tracing::{event, Level};

use super::{
    dao::{self, CalendarDataDao},
    model::CalendarData,
};

pub async fn init_calendar_data(db_url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let pool = CalendarDataDao::new(db_url).await;
    let mut transaction = pool.connection.begin().await?;

    match loop_data_calendar_init(&mut transaction).await {
        Ok(_) => transaction.commit().await?,
        Err(e) => {
            transaction.rollback().await?;
            event!(target: "security_api", Level::ERROR, "calendar_data.insert_calendar_data: {}", &e);
            panic!("calendar_data.insert_calendar_data Error {}", &e)
        }
    };
    Ok(())
}

async fn loop_data_calendar_init(
    transaction: &mut Transaction<'static, Postgres>,
) -> Result<(), Box<dyn std::error::Error>> {
    let max_date = Local::now().date_naive();
    let min_date = NaiveDate::from_ymd_opt(1962, 2, 9).unwrap();
    let max_date_str = max_date.format("%Y%m%d").to_string();
    let min_date_str = min_date.format("%Y%m%d").to_string();

    for y in min_date.year()..=max_date.year() {
        for m in 1..=12 {
            let last_day = last_day_in_month(y, m).day();
            for d in 1..=last_day {
                let this_date_str = format!("{:04}{:02}{:02}", y, m, d);
                if (max_date_str > this_date_str) && (min_date_str <= this_date_str) {
                    loop_date_calendar(transaction, y, m, d, last_day).await?;
                }
            }
        }
    }

    Ok(())
}

pub async fn insert_calendar_data(
    db_url: &str,
    open_next_year: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let pool = CalendarDataDao::new(db_url).await;
    let mut transaction = pool.connection.begin().await?;
    match loop_data_calendar_year(&mut transaction, open_next_year).await {
        Ok(_) => transaction.commit().await?,
        Err(e) => {
            transaction.rollback().await?;
            event!(target: "security_api", Level::ERROR, "calendar_data.insert_calendar_data: {}", &e);
            panic!("calendar_data.insert_calendar_data Error {}", &e)
        }
    };
    Ok(())
}

async fn loop_data_calendar_year(
    transaction: &mut Transaction<'static, Postgres>,
    open_next_year: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let now = Local::now().date_naive();
    let year = if open_next_year {
        now.year() + 1
    } else {
        now.year()
    };
    for m in 1..=12 {
        let last_day = last_day_in_month(year, m).day();
        for d in 1..=last_day {
            let query_cal = CalendarData {
                row_id: None,
                ce_year: Some(format!("{:04}", year)),
                tw_year: Some(format!("{:03}", year - 1911)),
                ce_month: Some(format!("{:02}", m)),
                ce_day: Some(format!("{:02}", d)),
                date_status: None,
                group_task: None,
            };
            let cal_list = dao::read_all(transaction, &query_cal).await?;
            if cal_list.0 <= 0 {
                loop_date_calendar(transaction, year, m, d, last_day).await?;
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
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    year: i32,
    month: u32,
    day: u32,
    last_day: u32,
) -> Result<(), Box<dyn std::error::Error>> {
    // 當前日期
    let now = Local::now().date_naive();
    // 指定日期
    let this_date = NaiveDate::from_ymd_opt(year, month, day).unwrap();

    // 如果是假日
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

        dao::create(transaction, calendar_data).await?;

    // 如果是初始
    } else if this_date < now {
        let calendar_data = CalendarData {
            row_id: None,
            ce_year: Some(format!("{:04}", year)),
            tw_year: Some(format!("{:03}", year - 1911)),
            ce_month: Some(format!("{:02}", month)),
            ce_day: Some(format!("{:02}", day)),
            date_status: Some("O".to_string()),
            group_task: Some("INIT".to_string()),
        };

        dao::create(transaction, calendar_data).await?;

    // 如果是最後一日
    } else if day == last_day {
        let calendar_data = CalendarData {
            row_id: None,
            ce_year: Some(format!("{:04}", year)),
            tw_year: Some(format!("{:03}", year - 1911)),
            ce_month: Some(format!("{:02}", month)),
            ce_day: Some(format!("{:02}", day)),
            date_status: Some("O".to_string()),
            group_task: Some("SECURITY".to_string()),
        };

        dao::create(transaction, calendar_data).await?;
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

        dao::create(transaction, calendar_data).await?;
    }

    Ok(())
}
