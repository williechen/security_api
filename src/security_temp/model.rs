#![warn(clippy::all, clippy::pedantic)]

use chrono::NaiveDateTime;
use diesel::prelude::{AsChangeset, Insertable, Queryable, QueryableByName};

use crate::schema::security_temp;

#[derive(Debug, Clone, Queryable, QueryableByName, AsChangeset)]
#[diesel(table_name=security_temp)]
#[diesel(primary_key(row_id))]
pub struct SecurityTemp {
    pub row_id: String,
    pub open_date_year: String,
    pub open_date_month: String,
    pub open_date_day: String,
    pub international_code: String,
    pub security_code: String,
    pub security_name: String,
    pub market_type: String,
    pub security_type: String,
    pub industry_type: String,
    pub issue_date: String,
    pub cfi_code: String,
    pub remark: String,
    pub created_date: NaiveDateTime,
    pub updated_date: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name=security_temp)]
pub struct NewSecurityTemp {
    pub open_date_year: String,
    pub open_date_month: String,
    pub open_date_day: String,
    pub international_code: String,
    pub security_code: String,
    pub security_name: String,
    pub market_type: String,
    pub security_type: String,
    pub industry_type: String,
    pub issue_date: String,
    pub cfi_code: String,
    pub remark: String,
    pub created_date: NaiveDateTime,
    pub updated_date: NaiveDateTime,
}

impl std::fmt::Display for SecurityTemp {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let row_id = self.row_id.clone();
        let open_date_year = self.open_date_year.clone();
        let open_date_month = self.open_date_month.clone();
        let open_date_day = self.open_date_day.clone();
        let international_code = self.international_code.clone();
        let security_code = self.security_code.clone();
        let security_name = self.security_name.clone();
        let market_type = self.market_type.clone();
        let security_type = self.security_type.clone();
        let industry_type = self.industry_type.clone();
        let issue_date = self.issue_date.clone();
        let cfi_code = self.cfi_code.clone();
        let remark = self.remark.clone();

        write!(
            f,
            r#"{0}, 
            open_date: {1}{2}{3}, 
            international_code: {4}, 
            security_code: {5}, 
            security_name: {6}, 
            market_type: {7}, 
            security_type: {8}, 
            industry_type: {9}, 
            issue_date: {10}, 
            cfi_code: {11}, 
            remark: {12}, 
            "#,
            row_id,
            open_date_year,
            open_date_month,
            open_date_day,
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
