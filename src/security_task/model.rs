#![warn(clippy::all, clippy::pedantic)]

#[derive(Debug, Clone)]
pub struct SecurityTask {
    pub row_id: String,
    pub open_date_year: String,
    pub open_date_month: String,
    pub open_date_day: String,
    pub security_code: String,
    pub security_name: String,
    pub market_type: String,
    pub issue_date: String,
    pub exec_seed: String,
    pub exec_count: i32,
    pub is_enabled: i32,
    pub sort_no: i32,
}

impl std::fmt::Display for SecurityTask {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(
            f,
            r#"{0}, 
            open_date: {1}{2}{3}, 
            security_code: {4}, 
            security_name: {5}, 
            market_type: {6}, 
            issue_date: {7}, 
            exec_seed: {8}, 
            exec_count: {9}, 
            is_enabled: {10}, 
            sort_no: {11}
            "#,
            self.row_id,
            self.open_date_year,
            self.open_date_month,
            self.open_date_day,
            self.security_code,
            self.security_name,
            self.market_type,
            self.issue_date,
            self.exec_seed,
            self.exec_count,
            self.is_enabled,
            self.sort_no
        )
    }
}
