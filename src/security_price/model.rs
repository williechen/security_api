#![warn(clippy::all, clippy::pedantic)]

use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct SecurityPrice {
    pub row_id: Option<String>,
    pub open_date: Option<String>,
    pub security_code: Option<String>,
    pub security_name: Option<String>,
    pub price_date: Option<String>,
    pub price_close: Option<BigDecimal>,
    pub price_avg: Option<BigDecimal>,
    pub price_hight: Option<BigDecimal>,
    pub price_hight_avg: Option<BigDecimal>,
    pub price_lowest: Option<BigDecimal>,
    pub price_lowest_avg: Option<BigDecimal>,
}

impl SecurityPrice {
    pub fn new() -> Self {
        SecurityPrice {
            row_id: None,
            open_date: None,
            security_code: None,
            security_name: None,
            price_date: None,
            price_close: None,
            price_avg: None,
            price_hight: None,
            price_hight_avg: None,
            price_lowest: None,
            price_lowest_avg: None,
        }
    }
}

impl std::fmt::Display for SecurityPrice {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let row_id = self.row_id.clone().unwrap_or(String::from(""));
        let open_date = self.open_date.clone().unwrap_or(String::from(""));
        let security_code = self.security_code.clone().unwrap_or(String::from(""));
        let security_name = self.security_name.clone().unwrap_or(String::from(""));
        let price_date = self.price_date.clone().unwrap_or(String::from(""));
        let price_close = self.price_close.clone().unwrap_or(BigDecimal::default());
        let price_avg = self.price_avg.clone().unwrap_or(BigDecimal::default());
        let price_hight = self.price_hight.clone().unwrap_or(BigDecimal::default());
        let price_hight_avg = self
            .price_hight_avg
            .clone()
            .unwrap_or(BigDecimal::default());
        let price_lowest = self.price_lowest.clone().unwrap_or(BigDecimal::default());
        let price_lowest_avg = self
            .price_lowest_avg
            .clone()
            .unwrap_or(BigDecimal::default());

        write!(
            f,
            r#"{}, 
            open_date: {}, 
            security_code: {}, 
            security_name: {}, 
            price_date: {}, 
            price_close: {}, 
            price_avg: {},
            price_hight: {},
            price_hight_avg: {},
            price_lowest: {},
            price_lowest_avg: {},
            "#,
            row_id,
            open_date,
            security_code,
            security_name,
            price_date,
            price_close,
            price_avg,
            price_hight,
            price_hight_avg,
            price_lowest,
            price_lowest_avg
        )
    }
}

#[derive(Debug, Clone)]
pub struct ResposePrice {
    pub open_date: Option<String>,
    pub security_code: Option<String>,
    pub security_name: Option<String>,
    pub market_type: Option<String>,
    pub data_content: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityPriceTwse {
    pub stat: String,
    pub date: String,
    pub title: String,
    pub fields: Vec<String>,
    pub data: Vec<Vec<String>>,
    pub notes: Vec<String>,
    pub hints: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityPriceTpex1 {
    #[serde(alias = "stkNo")]
    pub stk_no: String,
    #[serde(alias = "stkName")]
    pub stk_name: String,
    #[serde(alias = "showListPriceNote")]
    pub show_list_price_note: bool,
    #[serde(alias = "showListPriceLink")]
    pub show_list_price_link: bool,
    #[serde(alias = "reportDate")]
    pub report_date: String,
    #[serde(alias = "iTotalRecords")]
    pub i_total_records: i32,
    #[serde(alias = "aaData")]
    pub aa_data: Vec<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityPriceTpex2 {
    #[serde(alias = "stkno")]
    pub stk_no: String,
    #[serde(alias = "stkname")]
    pub stk_name: String,
    #[serde(alias = "iTotalRecords")]
    pub i_total_records: i32,
    #[serde(alias = "aaData")]
    pub aa_data: Vec<Vec<String>>,
    pub fields: Vec<Vec<String>>,
    pub lang: String,
    pub year: String,
    pub month: String,
}
