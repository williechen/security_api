#![warn(clippy::all, clippy::pedantic)]

use chrono::{Datelike, Local, NaiveDate};

use crate::{security_error::SecurityError, security_price};

use super::{
    dao,
    model::{CalendarData, NewCalendarData},
};

pub fn init_calendar_data() -> Result<(), SecurityError> {
    let max_date = Local::now().date_naive();
    let min_date = NaiveDate::from_ymd_opt(1999, 1, 1).unwrap();
    let max_date_str = max_date.format("%Y%m%d").to_string();
    let min_date_str = min_date.format("%Y%m%d").to_string();

    for y in min_date.year()..=max_date.year() {
        for m in 1..=12 {
            let last_day = last_day_in_month(y, m).day();
            for d in 1..=last_day {
                let this_date_str = format!("{:04}{:02}{:02}", y, m, d);
                if (max_date_str > this_date_str) && (min_date_str <= this_date_str) {
                    loop_date_calendar(y, m, d)?;
                }
            }

            let first_date =
                dao::find_one_by_work_day_first(format!("{:04}", y), format!("{:02}", m));
            if first_date.is_some() {
                update_first_date(first_date.unwrap())?;
            }
        }
    }

    Ok(())
}

pub fn insert_calendar_data(open_next_year: bool) -> Result<(), SecurityError> {
    let now = Local::now().date_naive();
    let year = if open_next_year {
        now.year() + 1
    } else {
        now.year()
    };
    for m in 1..=12 {
        let last_day = last_day_in_month(year, m).day();
        for d in 1..=last_day {
            let q_year = format!("{:04}", year);
            let q_month = format!("{:02}", m);
            let q_day = format!("{:02}", d);

            let cal = dao::find_one(q_year, q_month, q_day);
            if cal.is_none() {
                loop_date_calendar(year, m, d)?;
            }
        }

        let first_date =
            dao::find_one_by_work_day_first(format!("{:04}", year), format!("{:02}", m));
        if first_date.is_some() {
            update_first_date(first_date.unwrap())?;
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

fn loop_date_calendar(year: i32, month: u32, day: u32) -> Result<(), SecurityError> {
    // 初始日期
    let now = NaiveDate::from_ymd_opt(2024, 5, 17).unwrap();
    // 指定日期
    let this_date = NaiveDate::from_ymd_opt(year, month, day).unwrap();
    let this_tw_month = format!("{0:04}{1:02}", year, month);
    // 收盤價資料
    let price_data =
        security_price::dao::find_all_by_date(year.to_string(), month.to_string(), day.to_string());
    let max_price_month = security_price::dao::find_one_by_maxdate().unwrap();

    // 如果是假日
    if (this_date.weekday().number_from_monday() == 6 && price_data.len() == 0)
        || (this_date.weekday().number_from_monday() == 7 && price_data.len() == 0)
        || (this_tw_month <= max_price_month.price_date && price_data.len() == 0)
    {
        let calendar_data = NewCalendarData {
            ce_year: format!("{:04}", year),
            ce_month: format!("{:02}", month),
            ce_day: format!("{:02}", day),
            date_status: "S".to_string(),
            group_task: "STOP".to_string(),
            created_date: Local::now().naive_local(),
            updated_date: Local::now().naive_local(),
        };

        dao::create(calendar_data)?;
    // 如果是初始
    } else if this_date < now {
        let calendar_data = NewCalendarData {
            ce_year: format!("{:04}", year),
            ce_month: format!("{:02}", month),
            ce_day: format!("{:02}", day),
            date_status: "O".to_string(),
            group_task: "INIT".to_string(),
            created_date: Local::now().naive_local(),
            updated_date: Local::now().naive_local(),
        };

        dao::create(calendar_data)?;
    } else {
        let calendar_data = NewCalendarData {
            ce_year: format!("{:04}", year),
            ce_month: format!("{:02}", month),
            ce_day: format!("{:02}", day),
            date_status: "O".to_string(),
            group_task: "SECURITY".to_string(),
            created_date: Local::now().naive_local(),
            updated_date: Local::now().naive_local(),
        };

        dao::create(calendar_data)?;
    }

    Ok(())
}

fn update_first_date(first_date: CalendarData) -> Result<(), SecurityError> {
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
    m_first_date.updated_date = Local::now().naive_local();

    dao::modify(m_first_date)?;

    Ok(())
}
