use chrono::NaiveDateTime;

#[derive(Debug, Clone)]
pub struct ResponseData {
    pub row_id: Option<String>,
    pub data_content: Option<String>,
    pub data_code: Option<String>,
    pub read_date: Option<String>,
    pub created_date: Option<NaiveDateTime>,
    pub updated_date: Option<NaiveDateTime>,
}

impl ResponseData {
    pub fn new() -> Self {
        ResponseData {
            row_id: None,
            data_content: None,
            data_code: None,
            read_date: None,
            created_date: None,
            updated_date: None,
        }
    }
}

impl std::fmt::Display for ResponseData {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(
            f,
            r#"{:?}, 
            data_content: {:?}, 
            data_code: {:?}, 
            read_date: {:?}, 
            created_date: {:?}, 
            updated_date: {:?}
            "#,
            self.row_id,
            self.data_content,
            self.data_code,
            self.read_date,
            self.created_date,
            self.updated_date
        )
    }
}
