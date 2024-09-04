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
        let row_id = self.row_id.clone();
        let open_date_year = self.open_date_year.clone();
        let open_date_month = self.open_date_month.clone();
        let open_date_day = self.open_date_day.clone();
        let job_code = self.job_code.clone();
        let exec_status = self.exec_status.clone();

        write!(
            f,
            r#"{0}, 
            open_date: {1}{2}{3},
            job_code: {4}, 
            exec_status: {5}
            "#,
            row_id, open_date_year, open_date_month, open_date_day, job_code, exec_status
        )
    }
}
