use chrono::{Datelike, Local, NaiveDate};
use sqlx::PgConnection;

use crate::repository::Repository;

use super::{dao, model::CalendarData};

pub async fn init_calendar_data(db_url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let pool = Repository::new(db_url).await;
    let mut connection = pool.connection.acquire().await?;

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
                    loop_date_calendar(&mut connection, y, m, d).await?;
                }
            }

            let first_date = dao::read_by_work_day_first(
                &mut *connection,
                format!("{:04}", y).as_str(),
                format!("{:02}", m).as_str(),
            )
            .await?;
            update_first_date(&mut *connection, &first_date).await;
        }
    }

    Ok(())
}

pub async fn insert_calendar_data(
    db_url: &str,
    open_next_year: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let pool = Repository::new(db_url).await;
    let mut connection = pool.connection.acquire().await?;

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
            let cal_list = dao::read_all(&mut *connection, &query_cal).await?;
            if cal_list.0 <= 0 {
                loop_date_calendar(&mut *connection, year, m, d).await?;
            }
        }

        let first_date = dao::read_by_work_day_first(
            &mut *connection,
            format!("{:04}", year).as_str(),
            format!("{:02}", m).as_str(),
        )
        .await?;
        update_first_date(&mut *connection, &first_date).await;
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
    transaction: &mut PgConnection,
    year: i32,
    month: u32,
    day: u32,
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

async fn update_first_date(transaction: &mut PgConnection, first_date: &Option<CalendarData>) {
    // 當前日期
    let now = Local::now().date_naive();
    // 指定日期

    if first_date.is_some() {
        let mut new_first_date = first_date.clone().unwrap();

        let this_date = NaiveDate::parse_from_str(
            &format!(
                "{}{}{}",
                new_first_date.ce_year.clone().unwrap(),
                new_first_date.ce_month.clone().unwrap(),
                new_first_date.ce_day.clone().unwrap()
            ),
            "%Y%m%d",
        )
        .unwrap();

        if now <= this_date {
            new_first_date.group_task = Some("FIRST".to_string());
        } else {
            new_first_date.group_task = Some("FIRST_INIT".to_string());
        }

        dao::update(transaction, new_first_date).await.unwrap();
    }
}
