#[derive(Debug, Clone)]
pub struct CalendarData {
    pub row_id: Option<String>,
    pub ce_year: Option<String>,
    pub tw_year: Option<String>,
    pub ce_month: Option<String>,
    pub ce_day: Option<String>,
    pub date_status: Option<String>,
    pub group_task: Option<String>,
}

impl CalendarData {
    pub fn new() -> Self {
        CalendarData {
            row_id: None,
            ce_year: None,
            tw_year: None,
            ce_month: None,
            ce_day: None,
            date_status: None,
            group_task: None,
        }
    }
}

impl std::fmt::Display for CalendarData {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let row_id = self.row_id.clone().unwrap_or(String::from(""));
        let ce_year = self.ce_year.clone().unwrap_or(String::from(""));
        let tw_year = self.tw_year.clone().unwrap_or(String::from(""));
        let ce_month = self.ce_month.clone().unwrap_or(String::from(""));
        let ce_day = self.ce_day.clone().unwrap_or(String::from(""));
        let date_status = self.date_status.clone().unwrap_or(String::from(""));
        let group_task = self.group_task.clone().unwrap_or(String::from(""));

        write!(
            f,
            r#"{0}, 
            ce_date: {1}/{3}/{4}, 
            tw_date: {2}/{3}/{4}, 
            date_status: {5},
            group_task: {6}
            "#,
            row_id, ce_year, tw_year, ce_month, ce_day, date_status, group_task
        )
    }
}
