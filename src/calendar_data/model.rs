#![warn(clippy::all, clippy::pedantic)]

#[derive(Debug, Clone)]
pub struct CalendarData {
    pub row_id: String,
    pub ce_year: String,
    pub ce_month: String,
    pub ce_day: String,
    pub week_index: i32,
    pub date_status: String,
    pub group_task: String,
}

impl std::fmt::Display for CalendarData {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(
            f,
            r#"{0}, 
            ce_date: {1}/{3}/{4}, 
            tw_date: {2}/{3}/{4}, 
            date_status: {5},
            group_task: {6}
            "#,
            self.row_id,
            self.ce_year,
            self.ce_year.parse::<i32>().unwrap() - 1911,
            self.ce_month,
            self.ce_day,
            self.date_status,
            self.group_task
        )
    }
}
