#![warn(clippy::all, clippy::pedantic)]

use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Queryable, QueryableByName, AsChangeset)]
#[diesel(table_name=security_task)]
#[diesel(primary_key(row_id))]
pub struct SecurityPrice {
    pub row_id: String,
    pub open_date_year: String,
    pub open_date_month: String,
    pub open_date_day: String,
    pub security_code: String,
    pub security_name: String,
    pub price_date: String,
    pub price_close: BigDecimal,
    pub price_avg: BigDecimal,
    pub price_hight: BigDecimal,
    pub price_hight_avg: BigDecimal,
    pub price_lowest: BigDecimal,
    pub price_lowest_avg: BigDecimal,
    pub created_date: NaiveDateTime,
    pub updated_date: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name=security_task)]
pub struct NewSecurityTask {
    pub open_date_year: String,
    pub open_date_month: String,
    pub open_date_day: String,
    pub security_code: String,
    pub security_name: String,
    pub price_date: String,
    pub price_close: BigDecimal,
    pub price_avg: BigDecimal,
    pub price_hight: BigDecimal,
    pub price_hight_avg: BigDecimal,
    pub price_lowest: BigDecimal,
    pub price_lowest_avg: BigDecimal,
    pub created_date: NaiveDateTime,
    pub updated_date: NaiveDateTime,
}

impl std::fmt::Display for SecurityPrice {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let row_id = self.row_id.clone();
        let open_date_year = self.open_date_year.clone();
        let open_date_month = self.open_date_month.clone();
        let open_date_day = self.open_date_day.clone();
        let security_code = self.security_code.clone();
        let security_name = self.security_name.clone();
        let price_date = self.price_date.clone();
        let price_close = self.price_close.clone();
        let price_avg = self.price_avg.clone();
        let price_hight = self.price_hight.clone();
        let price_hight_avg = self
            .price_hight_avg
            .clone();
        let price_lowest = self.price_lowest.clone();
        let price_lowest_avg = self
            .price_lowest_avg
            .clone();

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
            open_date_year,
            open_date_month,
            open_date_day,
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
