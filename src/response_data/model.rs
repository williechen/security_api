#[derive(Debug, Clone)]
pub struct ResponseData {
    pub row_id: Option<String>,
    pub version_code: Option<String>,
    pub exec_code: Option<String>,
    pub data_content: Option<String>,
}

impl ResponseData {
    pub fn new() -> Self {
        ResponseData {
            row_id: None,
            version_code: None,
            exec_code: None,
            data_content: None,
        }
    }
}

impl std::fmt::Display for ResponseData {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let row_id = self.row_id.clone().unwrap_or(String::from(""));
        let version_code = self.version_code.clone().unwrap_or(String::from(""));
        let exec_code = self.exec_code.clone().unwrap_or(String::from(""));
        let data_content = self.data_content.clone().unwrap_or(String::from(""));

        write!(
            f,
            r#"{}, 
            version_code: {}, 
            exec_code: {}, 
            data_content: {}
            "#,
            row_id, version_code, exec_code, data_content
        )
    }
}
