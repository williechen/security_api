#![warn(clippy::all, clippy::pedantic)]

#[derive(Debug, Clone)]
pub struct SecurityTemp {
    pub row_id: String,
    pub open_date_year: String,
    pub open_date_month: String,
    pub open_date_day: String,
    pub international_code: String,
    pub security_code: String,
    pub security_name: String,
    pub market_type: String,
    pub security_type: String,
    pub industry_type: String,
    pub issue_date: String,
    pub cfi_code: String,
    pub remark: String,
}

impl std::fmt::Display for SecurityTemp {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(
            f,
            r#"{0}, 
            open_date: {1}{2}{3}, 
            international_code: {4}, 
            security_code: {5}, 
            security_name: {6}, 
            market_type: {7}, 
            security_type: {8}, 
            industry_type: {9}, 
            issue_date: {10}, 
            cfi_code: {11}, 
            remark: {12}, 
            "#,
            self.row_id,
            self.open_date_year,
            self.open_date_month,
            self.open_date_day,
            self.international_code,
            self.security_code,
            self.security_name,
            self.market_type,
            self.security_type,
            self.industry_type,
            self.issue_date,
            self.cfi_code,
            self.remark,
        )
    }
}
