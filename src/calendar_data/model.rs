#![warn(clippy::all, clippy::pedantic)]

use crate::schema::calendar_data;
use chrono::NaiveDateTime;
use diesel::prelude::{AsChangeset, Insertable, Queryable, QueryableByName};

#[derive(Debug, Clone, Queryable, QueryableByName, AsChangeset)]
#[diesel(table_name=calendar_data)]
#[diesel(primary_key(row_id))]
pub struct CalendarData {
    pub row_id: String,
    pub ce_year: String,
    pub ce_month: String,
    pub ce_day: String,
    pub date_status: String,
    pub group_task: String,
    pub created_date: NaiveDateTime,
    pub updated_date: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name=calendar_data)]
pub struct NewCalendarData {
    pub ce_year: String,
    pub ce_month: String,
    pub ce_day: String,
    pub date_status: String,
    pub group_task: String,
    pub created_date: NaiveDateTime,
    pub updated_date: NaiveDateTime,
}

impl std::fmt::Display for CalendarData {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let row_id = self.row_id.clone();
        let ce_year = self.ce_year.clone();
        let ce_month = self.ce_month.clone();
        let ce_day = self.ce_day.clone();
        let date_status = self.date_status.clone();
        let group_task = self.group_task.clone();

        write!(
            f,
            r#"{0}, 
            ce_date: {1}/{3}/{4}, 
            tw_date: {2}/{3}/{4}, 
            date_status: {5},
            group_task: {6}
            "#,
            row_id,
            ce_year,
            (ce_year.parse::<i32>().unwrap() - 1911),
            ce_month,
            ce_day,
            date_status,
            group_task
        )
    }
}
