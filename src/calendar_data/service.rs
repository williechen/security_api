#![warn(clippy::all, clippy::pedantic)]

use chrono::{Datelike, Local, NaiveDate};

use crate::{
    security_error::SecurityError,
    security_price::{self, model::SecurityPrice},
};

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
/// 取得每天的星期
///
fn get_weekday(year: i32, month: u32, day: u32) -> i32 {
    let date = NaiveDate::from_ymd_opt(year, month, day).unwrap();
    let weekday = date.weekday().number_from_monday();
    weekday.try_into().unwrap()
}

///
/// (年，月，日，開市第幾天)
///
fn get_open_stock_month(year: i32, last_price_date: &String) -> Vec<(i32, u32, u32, i32)> {
    let mut open_stock_dates = Vec::<(i32, u32, u32, i32)>::new();

    for month in 1..=12 {
        let str_ym = format!("{0:04}{1:02}", year, month);

        let price_data;
        if last_price_date.starts_with(&str_ym) {
            // 收盤價清單
            price_data = security_price::dao::find_all_by_date(
                format!("{0:04}", year),
                format!("{0:02}", month),
            );
        } else {
            price_data = Vec::<SecurityPrice>::new();
        }

        open_stock_dates.append(&mut get_open_stock_date(
            year,
            month,
            last_price_date,
            price_data,
        ));
    }
    open_stock_dates
}

fn get_open_stock_date(
    year: i32,
    month: u32,
    last_price_date: &String,
    price_data: Vec<SecurityPrice>,
) -> Vec<(i32, u32, u32, i32)> {
    let mut open_stock_dates = Vec::<(i32, u32, u32, i32)>::new();

    let mut open_stock_index = 0;
    let last_day = last_day_in_month(year, month).day();
    for day in 1..=last_day {
        if last_price_date >= &format!("{0:04}{1:02}{2:02}", year, month, day) {
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
        } else {
            let weekday = get_weekday(year, month, day);
            if weekday < 6 {
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

fn check_data_exists(data: &NewCalendarData) -> bool {
    dao::find_one(
        data.ce_year.clone(),
        data.ce_month.clone(),
        data.ce_day.clone(),
    )
    .is_some()
}

pub fn init_calendar_data() -> Result<(), SecurityError> {
    let max_year = Local::now().year();
    let min_year = 1999;

    let mut calendar_datas = Vec::<NewCalendarData>::new();

    let max_price_date = security_price::dao::find_one_by_maxdate().unwrap();
    let start_point = "20240517".to_string();

    for y in min_year..=max_year {
        let open_stock_dates = get_open_stock_month(y, &max_price_date.price_date);
        for open_stock_date in open_stock_dates {
            let point = format!(
                "{0:04}{1:02}{2:02}",
                open_stock_date.0, open_stock_date.1, open_stock_date.2
            );

            if open_stock_date.3 == -1 {
                calendar_datas.push(get_new_calendar_date(
                    open_stock_date.0,
                    open_stock_date.1,
                    open_stock_date.2,
                    "S",
                    "STOP",
                ));
            }

            if start_point > point {
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
            } else {
                if open_stock_date.3 == 0 {
                    calendar_datas.push(get_new_calendar_date(
                        open_stock_date.0,
                        open_stock_date.1,
                        open_stock_date.2,
                        "O",
                        "FIRST",
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
        }
    }

    for calendar_data in calendar_datas {
        if !check_data_exists(&calendar_data) {
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

    let max_price_date = security_price::dao::find_one_by_maxdate().unwrap();

    let open_stock_dates = get_open_stock_month(year, &max_price_date.price_date);
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
        if !check_data_exists(&calendar_data) {
            dao::create(calendar_data)?;
        }
    }

    Ok(())
}
