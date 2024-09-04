#![warn(clippy::all, clippy::pedantic)]

use sqlx::types::BigDecimal;

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct ResposePrice {
    pub open_date_year: String,
    pub open_date_month: String,
    pub open_date_day: String,
    pub security_code: String,
    pub security_name: String,
    pub market_type: String,
    pub data_content: String,
}
