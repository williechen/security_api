#![warn(clippy::all, clippy::pedantic)]

#[derive(Debug, Clone)]
pub struct DailyTask {
    pub row_id: String,
    pub open_date_year: String,
    pub open_date_month: String,
    pub open_date_day: String,
    pub job_code: String,
    pub exec_status: String,
}

impl std::fmt::Display for DailyTask {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{0}, open_date: {1}{2}{3}, job_code: {4}, exec_status: {5}",
            self.row_id,
            self.open_date_year,
            self.open_date_month,
            self.open_date_day,
            self.job_code,
            self.exec_status
        )
    }
}
