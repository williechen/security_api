#![warn(clippy::all, clippy::pedantic)]

#[derive(Debug, Clone)]
pub struct TaskSetting {
    pub row_id: Option<String>,
    pub group_code: Option<String>,
    pub job_code: Option<String>,
    pub wait_type: Option<String>,
    pub wait_number: Option<i32>,
    pub is_enabled: Option<i32>,
    pub sort_no: Option<i32>,
}

impl std::fmt::Display for TaskSetting {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let row_id = self.row_id.clone().unwrap_or(String::from(""));
        let group_code = self.group_code.clone().unwrap_or(String::from(""));
        let job_code = self.job_code.clone().unwrap_or(String::from(""));
        let wait_type = self.wait_type.clone().unwrap_or(String::from(""));
        let wait_number = self.wait_number.unwrap_or(0);
        let is_enabled = self.is_enabled.unwrap_or(0);
        let sort_no = self.sort_no.unwrap_or(0);

        write!(
            f,
            r#"{}, 
            group_code: {}, 
            job_code: {}, 
            wait_type: {}, 
            wait_number: {}, 
            is_enabled: {},
            sort_no: {}
            "#,
            row_id, group_code, job_code, wait_type, wait_number, is_enabled, sort_no,
        )
    }
}
