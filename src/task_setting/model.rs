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
        write!(
            f,
            r#"{0:#?}, 
            group_code: {1:#?}, 
            job_code: {2:#?}, 
            wait_type: {3:#?}, 
            wait_number: {4:#?}, 
            is_enabled: {5:#?},
            sort_no: {6:#?}
            "#,
            self.row_id,
            self.group_code,
            self.job_code,
            self.wait_type,
            self.wait_number,
            self.is_enabled,
            self.sort_no,
        )
    }
}
