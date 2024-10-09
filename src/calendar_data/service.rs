#![warn(clippy::all, clippy::pedantic)]

use chrono::{Datelike, Local, NaiveDate};

use crate::{security_error::SecurityError, security_price};

use super::{dao, model::NewCalendarData};

///
/// 取得每個月的最後一天
///
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

///
/// (年，月，日，開市第幾天)
///
fn get_open_stock_date(year: i32) -> Vec<(i32, u32, u32, i32)> {
    let mut open_stock_dates = Vec::<(i32, u32, u32, i32)>::new();

    let mut open_stock_index = 0;
    for month in 1..=12 {
        // 收盤價資料
        let price_data = security_price::dao::find_all_by_date(
            format!("{0:04}", year),
            format!("{0:02}", month),
        );

        let last_day = last_day_in_month(year, month).day();
        for day in 1..=last_day {
            let security_codes: Vec<String> = price_data
                .iter()
                .filter(|x| {
                    x.price_date == format!("{0:04}/{1:02}/{2:02}", year - 1911, month, day)
                })
                .map(|x| x.security_code.clone())
                .collect();
            if security_codes.len() > 0 {
                open_stock_dates.push((year, month, day, open_stock_index));
                open_stock_index = open_stock_index + 1;
            } else {
                open_stock_dates.push((year, month, day, -1));
            }
        }
    }

    open_stock_dates
}

///
/// 取得每天的星期
///
fn get_weekday(year: i32, month: u32, day: u32) -> i32 {
    let date = NaiveDate::from_ymd_opt(year, month, day).unwrap();
    let weekday = date.weekday().number_from_monday();
    weekday.try_into().unwrap()
}

///
/// 建立新增 Calendar Entity
///
fn get_new_calendar_date(
    year: i32,
    month: u32,
    day: u32,
    status: &str,
    task: &str,
) -> NewCalendarData {
    NewCalendarData {
        ce_year: format!("{0:04}", year),
        ce_month: format!("{0:02}", month),
        ce_day: format!("{0:02}", day),
        week_index: get_weekday(year, month, day),
        date_status: status.to_string(),
        group_task: task.to_string(),
        created_date: Local::now().naive_local(),
        updated_date: Local::now().naive_local(),
    }
}

fn check_data_exists(ce_year: String, ce_month: String, ce_day: String) -> bool {
    dao::find_one(ce_year, ce_month, ce_day).is_some()
}

pub fn init_calendar_data() -> Result<(), SecurityError> {
    let max_year = Local::now().year();
    let min_year = 1999;

    let mut calendar_datas = Vec::<NewCalendarData>::new();

    for y in min_year..=max_year {
        let open_stock_dates = get_open_stock_date(y);
        for open_stock_date in open_stock_dates {
            if open_stock_date.3 == -1 {
                calendar_datas.push(get_new_calendar_date(
                    open_stock_date.0,
                    open_stock_date.1,
                    open_stock_date.2,
                    "S",
                    "STOP",
                ));
            }
            if open_stock_date.3 == 0 {
                calendar_datas.push(get_new_calendar_date(
                    open_stock_date.0,
                    open_stock_date.1,
                    open_stock_date.2,
                    "O",
                    "FIRST_INIT",
                ));
            }
            if open_stock_date.3 > 0 {
                calendar_datas.push(get_new_calendar_date(
                    open_stock_date.0,
                    open_stock_date.1,
                    open_stock_date.2,
                    "O",
                    "INIT",
                ));
            }
        }
    }

    for calendar_data in calendar_datas {
        if check_data_exists(
            calendar_data.ce_year.clone(),
            calendar_data.ce_month.clone(),
            calendar_data.ce_day.clone(),
        ) {
            dao::create(calendar_data)?;
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

    let mut calendar_datas = Vec::<NewCalendarData>::new();

    let open_stock_dates = get_open_stock_date(year);
    for open_stock_date in open_stock_dates {
        if open_stock_date.3 == -1 {
            calendar_datas.push(get_new_calendar_date(
                open_stock_date.0,
                open_stock_date.1,
                open_stock_date.2,
                "S",
                "STOP",
            ));
        }
        if open_stock_date.3 == 0 {
            calendar_datas.push(get_new_calendar_date(
                open_stock_date.0,
                open_stock_date.1,
                open_stock_date.2,
                "O",
                "FRIST",
            ));
        }
        if open_stock_date.3 > 0 {
            calendar_datas.push(get_new_calendar_date(
                open_stock_date.0,
                open_stock_date.1,
                open_stock_date.2,
                "O",
                "SECURITY",
            ));
        }
    }

    for calendar_data in calendar_datas {
        if check_data_exists(
            calendar_data.ce_year.clone(),
            calendar_data.ce_month.clone(),
            calendar_data.ce_day.clone(),
        ) {
            dao::create(calendar_data)?;
        }
    }

    Ok(())
}
