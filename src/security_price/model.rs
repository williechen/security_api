#![warn(clippy::all, clippy::pedantic)]

use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use diesel::{
    prelude::{AsChangeset, Insertable, Queryable, QueryableByName},
    sql_types,
};
use serde::{Deserialize, Serialize};

use crate::schema::security_price;

#[derive(Debug, Clone, Queryable, QueryableByName, AsChangeset)]
#[diesel(table_name=security_price)]
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
#[diesel(table_name=security_price)]
pub struct NewSecurityPrice {
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
        let price_hight_avg = self.price_hight_avg.clone();
        let price_lowest = self.price_lowest.clone();
        let price_lowest_avg = self.price_lowest_avg.clone();

        write!(
            f,
            r#"{0}, 
            open_date: {1}{2}{3}, 
            security_code: {4}, 
            security_name: {5}, 
            price_date: {6}, 
            price_close: {7}, 
            price_avg: {8},
            price_hight: {9},
            price_hight_avg: {10},
            price_lowest: {11},
            price_lowest_avg: {12},
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

#[derive(Debug, Clone, QueryableByName)]
pub struct ResposePrice {
    #[diesel(sql_type = sql_types::VarChar)]
    pub open_date_year: String,
    #[diesel(sql_type = sql_types::VarChar)]
    pub open_date_month: String,
    #[diesel(sql_type = sql_types::VarChar)]
    pub open_date_day: String,
    #[diesel(sql_type = sql_types::VarChar)]
    pub security_code: String,
    #[diesel(sql_type = sql_types::VarChar)]
    pub security_name: String,
    #[diesel(sql_type = sql_types::VarChar)]
    pub market_type: String,
    #[diesel(sql_type = sql_types::VarChar)]
    pub data_content: String,
}

#[derive(Debug, Clone, QueryableByName)]
pub struct MaxPriceDate {
    #[diesel(sql_type = sql_types::VarChar)]
    pub price_date: String,
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
