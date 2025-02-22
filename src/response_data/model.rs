#![warn(clippy::all, clippy::pedantic)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct ResponseData {
    pub row_id: String,
    pub open_date_year: String,
    pub open_date_month: String,
    pub open_date_day: String,
    pub exec_code: String,
    pub data_content: String,
}

impl std::fmt::Display for ResponseData {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let row_id = self.row_id.clone();
        let open_date_year = self.open_date_year.clone();
        let open_date_month = self.open_date_month.clone();
        let open_date_day = self.open_date_day.clone();
        let exec_code = self.exec_code.clone();
        let data_content = self.data_content.clone();

        write!(
            f,
            r#"{0}, 
            open_date: {1}{2}{3}, 
            exec_code: {4}, 
            data_content: {5}
            "#,
            row_id, open_date_year, open_date_month, open_date_day, exec_code, data_content
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityPriceTwse {
    pub stat: String,
    pub date: Option<String>,
    pub title: Option<String>,
    pub fields: Option<Vec<String>>,
    pub data: Option<Vec<Vec<String>>>,
    pub notes: Option<Vec<String>>,
    pub hints: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityPriceTpex {
    pub tables: Vec<SecurityPriceTpexTable>,
    pub stat: String,
    pub date: String,
    pub code: Option<String>,
    pub name: Option<String>,
    #[serde(alias = "showListPriceNote")]
    pub show_list_price_note: Option<bool>,
    #[serde(alias = "showListPriceLink")]
    pub show_list_price_link: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityPriceTpexTable {
    pub title: String,
    pub subtitle: String,
    pub data: Vec<Vec<String>>,
    pub date: String,
    #[serde(alias = "totalCount")]
    pub total_count: u32,
    pub fields: Vec<String>,
    pub notes: Vec<String>,
    pub summary: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonthlyPrice {
    pub status: String,
    pub title: String,
    pub date: String,
    pub fields: Vec<String>,
    pub data: Vec<Vec<String>>,
}
