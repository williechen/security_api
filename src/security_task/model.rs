#[derive(Debug, Clone)]
pub struct SecurityTask {
    pub row_id: Option<String>,
    pub version_code: Option<String>,
    pub security_code: Option<String>,
    pub market_type: Option<String>,
    pub issue_date: Option<String>,
    pub security_date: Option<String>,
    pub security_seed: Option<String>,
    pub exec_count: Option<i32>,
    pub is_enabled: Option<i32>,
    pub sort_no: Option<i32>,
}

impl SecurityTask {
    pub fn new() -> Self {
        SecurityTask {
            row_id: None,
            version_code: None,
            security_code: None,
            market_type: None,
            issue_date: None,
            security_date: None,
            security_seed: None,
            exec_count: None,
            is_enabled: None,
            sort_no: None,
        }
    }
}

impl std::fmt::Display for SecurityTask {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let row_id = self.row_id.clone().unwrap_or(String::from(""));
        let version_code = self.version_code.clone().unwrap_or(String::from(""));
        let security_code = self.security_code.clone().unwrap_or(String::from(""));
        let market_type = self.market_type.clone().unwrap_or(String::from(""));
        let issue_date = self.issue_date.clone().unwrap_or(String::from(""));
        let security_date = self.security_date.clone().unwrap_or(String::from(""));
        let security_seed = self.security_seed.clone().unwrap_or(String::from(""));
        let exec_count = self.exec_count.unwrap_or(0);
        let is_enabled = self.is_enabled.unwrap_or(0);
        let sort_no = self.sort_no.unwrap_or(0);

        write!(
            f,
            r#"{}, 
            version_code: {}, 
            security_code: {}, 
            market_type: {}, 
            issue_date: {}, 
            security_date: {}, 
            security_seed: {}, 
            exec_count: {}, 
            is_enabled: {}, 
            sort_no: {}
            "#,
            row_id,
            version_code,
            security_code,
            market_type,
            issue_date,
            security_date,
            security_seed,
            exec_count,
            is_enabled,
            sort_no
        )
    }
}
