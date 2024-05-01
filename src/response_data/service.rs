use regex::Regex;
use reqwest::Client;
use scraper::{Html, Selector};
use serde_json::json;
use tracing::{event, Level};

use crate::security_task::model::SecurityTask;

pub async fn get_web_security_data() -> Result<String, Box<dyn std::error::Error>> {
    let client = Client::new();

    let res = client
        .get("https://isin.twse.com.tw/isin/class_main.jsp")
        .send()
        .await?;

    let big5_text = res.bytes().await?;
    let utf8_text = encoding_rs::BIG5.decode(&big5_text);

    let result_html = parse_web_security_data(&utf8_text.0.to_string())?;

    Ok(result_html)
}

fn parse_web_security_data(table: &String) -> Result<String, Box<dyn std::error::Error>> {
    let document = Html::parse_document(table);

    let table_select = Selector::parse("table.h4").unwrap();

    let table_content = document.select(&table_select).next().unwrap();
    let table_html = table_content.html();

    let re = Regex::new(">\n\\s+<").unwrap();
    let result = re.replace_all(&table_html, "><");

    Ok(result.to_string())
}

pub async fn get_twse_json(task: &SecurityTask) -> Result<String, Box<dyn std::error::Error>> {
    let client = Client::new();

    let res = client
        .get("https://www.twse.com.tw/rwd/zh/afterTrading/STOCK_DAY")
        .query(&[("date", &task.twse_date)])
        .query(&[("stockNo", &task.security_code)])
        .query(&[("response", "json")])
        .query(&[("_", &task.security_seed)])
        .send()
        .await?;

    let json = res.text().await?;
    event!(target: "my_api", Level::DEBUG, "{:?}", &json);

    Ok(json)
}

pub async fn get_tpex1_json(task: &SecurityTask) -> Result<String, Box<dyn std::error::Error>> {
    let client = Client::new();

    let res = client
        .get("https://www.tpex.org.tw/web/stock/aftertrading/daily_trading_info/st43_result.php")
        .query(&[("d", &task.tpex_date)])
        .query(&[("stkno", &task.security_code)])
        .query(&[("_", &task.security_seed)])
        .send()
        .await?;

    let json = res.text().await?;
    event!(target: "my_api", Level::DEBUG, "{:?}", &json);

    let parse_text = decode_unicode_escape(&json);

    event!(target: "my_api", Level::DEBUG, "{:?}", &parse_text);

    Ok(parse_text)
}

fn decode_unicode_escape(s: &str) -> String {
    let mut result = String::new();
    let mut chars = s.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '\\' {
            let c1 = chars.next();
            if let Some('u') = c1 {
                let mut codepoint = String::new();
                for _ in 0..4 {
                    if let Some(digit) = chars.next() {
                        codepoint.push(digit);
                    } else {
                        break;
                    }
                }
                if let Ok(code) = u32::from_str_radix(&codepoint, 16) {
                    if let Some(unicode_char) = std::char::from_u32(code) {
                        result.push(unicode_char);
                    }
                }
            } else {
                result.push(c1.unwrap());
            }
        } else {
            result.push(c);
        }
    }

    result
}

pub async fn get_tpex2_html(task: &SecurityTask) -> Result<String, Box<dyn std::error::Error>> {
    let params = [
        ("input_month", &task.tpex_date),
        ("input_emgstk_code", &task.security_code),
        ("ajax", &Some("true".to_string())),
    ];

    let client = Client::new();

    let res = client
        .post("https://www.tpex.org.tw/web/emergingstock/single_historical/result.php")
        .form(&params)
        .send()
        .await?;

    let text = res.text().await?;
    event!(target: "my_api", Level::DEBUG, "{:?}", &text);

    let json_text = parse_web_tpex2_data(&text)?;
    event!(target: "my_api", Level::DEBUG, "{:?}", &json_text);

    Ok(json_text)
}

fn parse_web_tpex2_data(document: &String) -> Result<String, Box<dyn std::error::Error>> {
    let mut data_map = serde_json::Map::new();

    let html_content = Html::parse_document(document);

    let input_selector = Selector::parse("div.v-pnl form input").unwrap();
    let input_content = html_content.select(&input_selector);
    for input in input_content {
        data_map.insert(
            input.attr("name").unwrap().to_string(),
            serde_json::Value::String(input.attr("value").unwrap().to_string()),
        );
    }

    let mut data_row = Vec::<serde_json::Value>::new();

    let tr_selector = Selector::parse("div.v-pnl table tr").unwrap();
    let td_selector = Selector::parse("td").unwrap();

    let tr_content = html_content.select(&tr_selector);
    for tr in tr_content {
        let mut row = Vec::<String>::new();

        let td_content = tr.select(&td_selector);
        for td in td_content {
            row.push(td.inner_html());
        }
        data_row.push(json!(row));
    }
    data_map.insert("data_row".to_string(), serde_json::Value::Array(data_row));

    Ok(serde_json::to_string(&data_map)?)
}
