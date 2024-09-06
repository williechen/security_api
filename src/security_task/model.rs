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
        let row_id = self.row_id.clone();
        let open_date_year = self.open_date_year.clone();
        let open_date_month = self.open_date_month.clone();
        let open_date_day = self.open_date_day.clone();
        let security_code = self.security_code.clone();
        let security_name = self.security_name.clone();
        let market_type = self.market_type.clone();
        let issue_date = self.issue_date.clone();
        let exec_seed = self.exec_seed.clone();
        let exec_count = self.exec_count;
        let is_enabled = self.is_enabled;
        let sort_no = self.sort_no;
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
            row_id,
            open_date_year,
            open_date_month,
            open_date_day,
            security_code,
            security_name,
            market_type,
            issue_date,
            exec_seed,
            exec_count,
            is_enabled,
            sort_no
        )
    }
}
