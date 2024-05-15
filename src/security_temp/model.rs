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
        }
    }
}

impl std::fmt::Display for SecurityTemp {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let row_id = self.row_id.clone().unwrap_or(String::from(""));
        let version_code = self.version_code.clone().unwrap_or(String::from(""));
        let international_code = self.international_code.clone().unwrap_or(String::from(""));
        let security_code = self.security_code.clone().unwrap_or(String::from(""));
        let security_name = self.security_name.clone().unwrap_or(String::from(""));
        let market_type = self.market_type.clone().unwrap_or(String::from(""));
        let security_type = self.security_type.clone().unwrap_or(String::from(""));
        let industry_type = self.industry_type.clone().unwrap_or(String::from(""));
        let issue_date = self.issue_date.clone().unwrap_or(String::from(""));
        let cfi_code = self.cfi_code.clone().unwrap_or(String::from(""));
        let remark = self.remark.clone().unwrap_or(String::from(""));

        write!(
            f,
            r#"{}, 
            version_code: {}, 
            international_code: {}, 
            security_code: {}, 
            security_name: {}, 
            market_type: {}, 
            security_type: {}, 
            industry_type: {}, 
            issue_date: {}, 
            cfi_code: {}, 
            remark: {}, 
            "#,
            row_id,
            version_code,
            international_code,
            security_code,
            security_name,
            market_type,
            security_type,
            industry_type,
            issue_date,
            cfi_code,
            remark,
        )
    }
}
