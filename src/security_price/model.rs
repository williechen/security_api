use sqlx::types::BigDecimal;

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
