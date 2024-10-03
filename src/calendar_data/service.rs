#![warn(clippy::all, clippy::pedantic)]

use chrono::{Datelike, Local, NaiveDate};

use crate::security_price;

use super::{dao, model::CalendarData};

pub async fn init_calendar_data() -> Result<(), sqlx::Error> {
    let max_date = Local::now().date_naive();
    let min_date = NaiveDate::from_ymd_opt(1999, 1, 1).unwrap();
    let max_date_str = max_date.format("%Y%m%d").to_string();
    let min_date_str = min_date.format("%Y%m%d").to_string();

    let max_price_date = security_price::dao::find_one_by_maxdate().await;

    for y in min_date.year()..=max_date.year() {
        for m in 1..=12 {
            let last_day = last_day_in_month(y, m).day();

            // 收盤價資料
            let price_data =
                security_price::dao::find_all_by_date(y.to_string(), m.to_string()).await;

            for d in 1..=last_day {
                let this_date_str = format!("{0:04}{1:02}{2:02}", y, m, d);
                if (max_date_str > this_date_str) && (min_date_str <= this_date_str) {
                    let dates: Vec<String> = price_data
                        .iter()
                        .filter(|x| x.price_date == format!("{0:04}/{1:02}/{2:02}", y-1911, m, d))
                        .map(|x| x.price_date.clone())
                        .collect();

                    loop_date_calendar(y, m, d, max_price_date.clone(), dates.len()).await?;
                }
            }

            let first_date =
                dao::find_one_by_work_day_first(format!("{0:04}", y), format!("{0:02}", m)).await;
            if first_date.is_some() {
                update_first_date(first_date.unwrap()).await?;
            }
        }
    }

    Ok(())
}

pub async fn insert_calendar_data(open_next_year: bool) -> Result<(), sqlx::Error> {
    let now = Local::now().date_naive();
    let year = if open_next_year {
        now.year() + 1
    } else {
        now.year()
    };

    let max_price_date = security_price::dao::find_one_by_maxdate().await;
    for m in 1..=12 {
        let last_day = last_day_in_month(year, m).day();

        // 收盤價資料
        let price_data = security_price::dao::find_all_by_date(year.to_string(), m.to_string()).await;

        for d in 1..=last_day {
            let q_year = format!("{0:04}", year);
            let q_month = format!("{0:02}", m);
            let q_day = format!("{0:02}", d);

            let dates: Vec<String> = price_data
                        .iter()
                        .filter(|x| x.price_date == format!("{0:04}/{1:02}/{2:02}", year-1911, m, d))
                        .map(|x| x.price_date.clone())
                        .collect();

            let cal = dao::find_one(q_year, q_month, q_day).await;
            if cal.is_none() {
                loop_date_calendar(year, m, d, max_price_date.clone(), dates.len()).await?;
            }
        }

        let first_date =
            dao::find_one_by_work_day_first(format!("{0:04}", year), format!("{0:02}", m)).await;
        if first_date.is_some() {
            update_first_date(first_date.unwrap()).await?;
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
    year: i32,
    month: u32,
    day: u32,
    price_date: String,
    price_count: usize,
) -> Result<(), sqlx::Error> {
    // 初始日期
    let now = NaiveDate::from_ymd_opt(2024, 5, 17).unwrap();
    // 指定日期
    let this_date = NaiveDate::from_ymd_opt(year, month, day).unwrap();
    let this_tw_date = format!("{0:04}{1:02}{2:02}", year, month, day);

    // 如果是假日
    if (this_date.weekday().number_from_monday() == 6 && price_count == 0)
        || (this_date.weekday().number_from_monday() == 7 && price_count == 0)
        || (this_tw_date <= price_date && price_count == 0)
    {
        let calendar_data = CalendarData {
            row_id: String::new(),
            ce_year: format!("{0:04}", year),
            ce_month: format!("{0:02}", month),
            ce_day: format!("{0:02}", day),
            week_index: this_date.weekday().number_from_monday() as i32,
            date_status: "S".to_string(),
            group_task: "STOP".to_string(),
        };

        dao::create(calendar_data).await?;
    // 如果是初始
    } else if this_date < now {
        let calendar_data = CalendarData {
            row_id: String::new(),
            ce_year: format!("{0:04}", year),
            ce_month: format!("{0:02}", month),
            ce_day: format!("{0:02}", day),
            week_index: this_date.weekday().number_from_monday() as i32,
            date_status: "O".to_string(),
            group_task: "INIT".to_string(),
        };

        dao::create(calendar_data).await?;
    } else {
        let calendar_data = CalendarData {
            row_id: String::new(),
            ce_year: format!("{0:04}", year),
            ce_month: format!("{0:02}", month),
            ce_day: format!("{0:02}", day),
            week_index: this_date.weekday().number_from_monday() as i32,
            date_status: "O".to_string(),
            group_task: "SECURITY".to_string(),
        };

        dao::create(calendar_data).await?;
    }

    Ok(())
}

async fn update_first_date(first_date: CalendarData) -> Result<(), sqlx::Error> {
    let mut m_first_date = first_date.clone();

    // 當前日期
    let now = NaiveDate::from_ymd_opt(2024, 5, 17).unwrap();
    // 指定日期
    let year = first_date.ce_year.parse().unwrap();
    let month = first_date.ce_month.parse().unwrap();
    let day = first_date.ce_day.parse().unwrap();
    let this_date = NaiveDate::from_ymd_opt(year, month, day).unwrap();

    if now <= this_date {
        m_first_date.group_task = "FIRST".to_string();
    } else {
        m_first_date.group_task = "FIRST_INIT".to_string();
    }

    dao::modify(m_first_date).await?;

    Ok(())
}
