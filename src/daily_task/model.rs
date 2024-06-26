#![warn(clippy::all, clippy::pedantic)]

#[derive(Debug, Clone)]
pub struct DailyTask {
    pub row_id: Option<String>,
    pub open_date: Option<String>,
    pub job_code: Option<String>,
    pub exec_status: Option<String>,
}

impl DailyTask {
    pub fn new() -> Self {
        DailyTask {
            row_id: None,
            open_date: None,
            job_code: None,
            exec_status: None,
        }
    }
}

impl std::fmt::Display for DailyTask {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let row_id = self.row_id.clone().unwrap_or(String::from(""));
        let open_date = self.open_date.clone().unwrap_or(String::from(""));
        let job_code = self.job_code.clone().unwrap_or(String::from(""));
        let exec_status = self.exec_status.clone().unwrap_or(String::from(""));

        write!(
            f,
            r#"{0}, 
            open_date: {1},
            job_code: {2}, 
            exec_status: {3}
            "#,
            row_id, open_date, job_code, exec_status
        )
    }
}

#[derive(Debug, Clone)]
pub struct DailyTaskInfo {
    pub row_id: Option<String>,
    pub open_date: Option<String>,
    pub job_code: Option<String>,
    pub ce_date: Option<String>,
    pub tw_date: Option<String>,
    pub wait_type: Option<String>,
    pub wait_number: Option<i32>,
    pub exec_status: Option<String>,
}
