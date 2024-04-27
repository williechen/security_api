use chrono::NaiveDateTime;

#[derive(Debug, Clone)]
pub struct SecurityTask {
    pub row_id: Option<String>,
    pub market_type: Option<String>,
    pub security_code: Option<String>,
    pub issue_date: Option<String>,
    pub twse_date: Option<String>,
    pub tpex_date: Option<String>,
    pub security_seed: Option<String>,
    pub is_enabled: Option<i32>,
    pub sort_no: Option<i32>,
    pub retry_count: Option<i32>,
    pub created_date: Option<NaiveDateTime>,
    pub updated_date: Option<NaiveDateTime>,
}

impl SecurityTask {
    pub fn new() -> Self {
        SecurityTask {
            row_id: None,
            market_type: None,
            security_code: None,
            issue_date: None,
            twse_date: None,
            tpex_date: None,
            security_seed: None,
            is_enabled: None,
            sort_no: None,
            retry_count: None,
            created_date: None,
            updated_date: None,
        }
    }
}

impl std::fmt::Display for SecurityTask {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(
            f,
            r#"{:?}, 
            market_type: {:?}, 
            security_code: {:?}, 
            issue_date: {:?}, 
            twse_date: {:?}, 
            tpex_date: {:?}, 
            security_seed: {:?}, 
            is_enabled: {:?}, 
            sort_no: {:?}, 
            retry_count: {:?},
            created_date: {:?}, 
            updated_date: {:?}
            "#,
            self.row_id,
            self.market_type,
            self.security_code,
            self.issue_date,
            self.twse_date,
            self.tpex_date,
            self.security_seed,
            self.is_enabled,
            self.sort_no,
            self.retry_count,
            self.created_date,
            self.updated_date
        )
    }
}
