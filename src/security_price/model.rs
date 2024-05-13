use rust_decimal::Decimal;

#[derive(Debug, Clone)]
pub struct SecurityPrice {
    pub row_id: Option<String>,
    pub version_code: Option<String>,
    pub security_code: Option<String>,
    pub price_date: Option<String>,
    pub price_close: Option<Decimal>,
    pub price_avg: Option<Decimal>,
    pub price_hight: Option<Decimal>,
    pub price_hight_avg: Option<Decimal>,
    pub price_lowest: Option<Decimal>,
    pub price_lowest_avg: Option<Decimal>,
}

impl SecurityPrice {
    pub fn new() -> Self {
        SecurityPrice {
            row_id: None,
            version_code: None,
            security_code: None,
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
        let version_code = self.version_code.clone().unwrap_or(String::from(""));
        let security_code = self.security_code.clone().unwrap_or(String::from(""));
        let price_date = self.price_date.clone().unwrap_or(String::from(""));
        let price_close = self.price_close.clone().unwrap_or(Decimal::ZERO);
        let price_avg = self.price_avg.clone().unwrap_or(Decimal::ZERO);
        let price_hight = self.price_hight.clone().unwrap_or(Decimal::ZERO);
        let price_hight_avg = self.price_hight_avg.clone().unwrap_or(Decimal::ZERO);
        let price_lowest = self.price_lowest.clone().unwrap_or(Decimal::ZERO);
        let price_lowest_avg = self.price_lowest_avg.clone().unwrap_or(Decimal::ZERO);

        write!(
            f,
            r#"{}, 
            version_code: {}, 
            security_code: {}, 
            price_date: {}, 
            price_close: {}, 
            price_avg: {},
            price_hight: {},
            price_hight_avg: {},
            price_lowest: {},
            price_lowest_avg: {},
            "#,
            row_id,
            version_code,
            security_code,
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
