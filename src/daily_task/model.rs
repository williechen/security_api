#[derive(Debug, Clone)]
pub struct DailyTask {
    pub row_id: Option<String>,
    pub version_code: Option<String>,
    pub open_date: Option<String>,
    pub job_code: Option<String>,
    pub exec_status: Option<String>,
}

impl DailyTask {
    pub fn new() -> Self {
        DailyTask {
            row_id: None,
            version_code: None,
            open_date: None,
            job_code: None,
            exec_status: None,
        }
    }
}

impl std::fmt::Display for DailyTask {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let row_id = self.row_id.clone().unwrap_or(String::from(""));
        let version_code = self.version_code.clone().unwrap_or(String::from(""));
        let open_date = self.open_date.clone().unwrap_or(String::from(""));
        let job_code = self.job_code.clone().unwrap_or(String::from(""));
        let exec_status = self.exec_status.clone().unwrap_or(String::from(""));

        write!(
            f,
            r#"{0}, 
            version_code: {1}, 
            open_date: {2},
            job_code: {3}, 
            exec_status: {4}
            "#,
            row_id, version_code, open_date, job_code, exec_status
        )
    }
}
