#![warn(clippy::all, clippy::pedantic)]

use chrono::NaiveDateTime;
use diesel::prelude::{AsChangeset, Insertable, Queryable, QueryableByName};
use serde::{Deserialize, Serialize};

use crate::schema::response_data;

#[derive(Debug, Clone, Queryable, QueryableByName, AsChangeset)]
#[diesel(table_name=response_data)]
#[diesel(primary_key(row_id))]
pub struct ResponseData {
    pub row_id: String,
    pub open_date_year: String,
    pub open_date_month: String,
    pub open_date_day: String,
    pub exec_code: String,
    pub data_content: String,
    pub created_date: NaiveDateTime,
    pub updated_date: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name=response_data)]
pub struct NewResponseData {
    pub open_date_year: String,
    pub open_date_month: String,
    pub open_date_day: String,
    pub exec_code: String,
    pub data_content: String,
    pub created_date: NaiveDateTime,
    pub updated_date: NaiveDateTime,
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

