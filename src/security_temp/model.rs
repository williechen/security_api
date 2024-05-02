use chrono::NaiveDateTime;

#[derive(Debug, Clone)]
pub struct SecurityTemp {
    pub row_id: Option<String>,
    pub version_code: Option<String>,
    pub international_code: Option<String>,
    pub security_code: Option<String>,
    pub security_name: Option<String>,
    pub market_type: Option<String>,
    pub security_type: Option<String>,
    pub industry_type: Option<String>,
    pub issue_date: Option<String>,
    pub cfi_code: Option<String>,
    pub remark: Option<String>,
    pub is_enabled: Option<i32>,
    pub created_date: Option<NaiveDateTime>,
    pub updated_date: Option<NaiveDateTime>,
}

impl SecurityTemp {
    pub fn new() -> Self {
        SecurityTemp {
            row_id: None,
            version_code: None,
            international_code: None,
            security_code: None,
            security_name: None,
            market_type: None,
            security_type: None,
            industry_type: None,
            issue_date: None,
            cfi_code: None,
            remark: None,
            is_enabled: None,
            created_date: None,
            updated_date: None,
        }
    }
}

impl std::fmt::Display for SecurityTemp {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(
            f,
            r#"{:?}, 
            version_code: {:?}, 
            international_code: {:?}, 
            security_code: {:?}, 
            security_name: {:?}, 
            market_type: {:?}, 
            security_type: {:?}, 
            industry_type: {:?}, 
            issue_date: {:?}, 
            cfi_code: {:?}, 
            remark: {:?}, 
            is_enabled: {:?}, 
            created_date: {:?}, 
            updated_date: {:?}
            "#,
            self.row_id,
            self.version_code,
            self.international_code,
            self.security_code,
            self.security_name,
            self.market_type,
            self.security_type,
            self.industry_type,
            self.issue_date,
            self.cfi_code,
            self.remark,
            self.is_enabled,
            self.created_date,
            self.updated_date
        )
    }
}
