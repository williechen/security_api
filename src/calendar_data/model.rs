use chrono::NaiveDateTime;

#[derive(Debug, Clone)]
pub struct CalendarData {
    pub row_id: Option<String>,
    pub ce_year: Option<String>,
    pub tw_year: Option<String>,
    pub ce_month: Option<String>,
    pub ce_day: Option<String>,
    pub date_status: Option<String>,
    pub group_task: Option<String>,
    pub created_date: Option<NaiveDateTime>,
    pub updated_date: Option<NaiveDateTime>,
}

impl CalendarData {
    pub fn new() -> Self {
        CalendarData {
            row_id: None,
            data_content: None,
            data_code: None,
            read_date: None,
            created_date: None,
            updated_date: None,
        }
    }
}

impl std::fmt::Display for CalendarData {
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
