#![warn(clippy::all, clippy::pedantic)]

use std::{str::FromStr, time::Duration};

use bigdecimal::Zero;
use regex::Regex;
use reqwest::Client;
use scraper::{Html, Selector};
use tokio_retry::{strategy::ExponentialBackoff, Retry};
use tracing::{event, Level};

use crate::{
    daily_task::model::DailyTask,
    response_data::{
        dao,
        model::{ResponseData, SecurityPriceTpex, SecurityPriceTwse},
    },
    security_task::model::SecurityTask,
};

use super::model::MonthlyPrice;

fn html_decode(input: &str) -> String {
    input
        .replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&apos;", "'")
        .replace("*", "")
        .replace("＊", "")
}

pub async fn get_security_all_code(task: &DailyTask) -> Result<(), Box<dyn std::error::Error>> {
    event!(target: "security_api", Level:: INFO, "call daily_task.get_security_all_code");

    let q_year = task.clone().open_date_year;
    let q_month = task.clone().open_date_month;
    let q_day = task.clone().open_date_day;
    let q_exec_code = "security".to_string();

    // 重試設定
    let retry_strategy = ExponentialBackoff::from_millis(2000)
        .max_delay(Duration::from_secs(10))
        .take(5);

    let data = dao::find_one(q_year, q_month, q_day, q_exec_code).await;
    if data.is_none() {
        match Retry::spawn(retry_strategy.clone(), || async {
            get_web_security_data().await
        })
        .await
        {
            Ok(res) => {
                let new_response_data = ResponseData {
                    row_id: String::new(),
                    exec_code: "security".to_string(),
                    data_content: res,
                    open_date_year: task.clone().open_date_year,
                    open_date_month: task.clone().open_date_month,
                    open_date_day: task.clone().open_date_day,
                };

                dao::create(new_response_data).await?;
                return Ok(());
            }
            Err(e) => return Err(e),
        }
    }

    Ok(())
}

async fn get_web_security_data() -> Result<String, Box<dyn std::error::Error>> {
    let client = Client::new();

    let res = client
        .get("https://isin.twse.com.tw/isin/class_main.jsp")
        .timeout(Duration::from_secs(20))
        .send()
        .await?;
    event!(target: "security_api", Level::INFO, "{:?}", &res.url().to_string());

    let big5_text = res.bytes().await?;
    let utf8_text = encoding_rs::BIG5.decode(&big5_text);

    let result_html = parse_web_security_data(&utf8_text.0.to_string())?;
    event!(target: "security_api", Level::DEBUG, "{:?}", &result_html);

    Ok(html_decode(&result_html))
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

pub async fn get_twse_avg_json(
    task: &SecurityTask,
) -> Result<String, Box<dyn std::error::Error + 'static + Send + Sync>> {
    run_task_log(task);

    let y = &task.open_date_year;
    let m = &task.open_date_month;
    let d = &task.open_date_day;
    let open_date = format!("{0}{1}{2}", y, m, d);
    let tw_ym = format!("{0}/{1}", y.parse::<u32>().unwrap() - 1911, m);

    let client = Client::new();

    let res = client
        .get("https://www.twse.com.tw/rwd/zh/afterTrading/STOCK_DAY_AVG")
        .query(&[("date", &open_date)])
        .query(&[("stockNo", &task.security_code)])
        .query(&[("response", "json")])
        .query(&[("_", &task.exec_seed)])
        .timeout(Duration::from_secs(4))
        .send()
        .await?;
    event!(target: "security_api", Level::DEBUG,  "{:?}", &res.url().to_string());

    let json = res.json::<SecurityPriceTwse>().await?;
    event!(target: "security_api", Level::DEBUG,  "{:?}", &json);

    let json_str = get_twse_price(json, tw_ym, 0, 1);
    event!(target: "security_api", Level::DEBUG,  "{0}", &json_str);

    Ok(html_decode(&json_str))
}

fn get_twse_price(
    twse_json: SecurityPriceTwse,
    tw_ym: String,
    date_index: usize,
    price_index: usize,
) -> String {
    let status = if "OK" == twse_json.stat {
        "Y".to_string()
    } else {
        "N".to_string()
    };
    let title = twse_json.title.unwrap_or("".to_string());
    let date = twse_json.date.unwrap_or("".to_string());
    let fields = twse_json.fields.unwrap_or(Vec::<String>::new());
    let data = get_close_price(
        twse_json.data.unwrap_or(Vec::<Vec<String>>::new()),
        tw_ym,
        date_index,
        price_index,
    );

    if "Y" == status && !data.is_empty() {
        return serde_json::to_string(&MonthlyPrice {
            status,
            title,
            date,
            fields,
            data,
        })
        .unwrap_or("".to_string());
    } else {
        return "".to_string();
    }
}

pub async fn get_tpex1_json(
    task: &SecurityTask,
) -> Result<String, Box<dyn std::error::Error + 'static + Send + Sync>> {
    run_task_log(task);

    let y = &task.open_date_year;
    let m = &task.open_date_month;
    let d = &task.open_date_day;
    let open_date = format!("{0}/{1}/{2}", y, m, d);
    let tw_ym = format!("{0}/{1}", y.parse::<u32>().unwrap() - 1911, m);

    let client = Client::new();

    let params = [
        ("code", &task.security_code),
        ("date", &open_date),
        ("id", &"".to_string()),
        ("response", &"json".to_string()),
    ];

    let res = client
        .post("https://www.tpex.org.tw/www/zh-tw/afterTrading/tradingStock")
        .form(&params)
        .timeout(Duration::from_secs(4))
        .send()
        .await?;
    event!(target: "security_api", Level::DEBUG, "{:?}", &res.url().to_string());

    let json = res.json::<SecurityPriceTpex>().await?;
    event!(target: "security_api", Level::DEBUG,  "{:?}", &json);

    let json_str = get_tpex_price(json, tw_ym, 0, 6);
    event!(target: "security_api", Level::DEBUG,  "{0}", &json_str);

    Ok(html_decode(&json_str))
}

pub async fn get_tpex2_json(
    task: &SecurityTask,
) -> Result<String, Box<dyn std::error::Error + 'static + Send + Sync>> {
    run_task_log(task);

    let y = &task.open_date_year;
    let m = &task.open_date_month;
    let d = &task.open_date_day;
    let open_date = format!("{0}/{1}/{2}", y, m, d);
    let tw_ym = format!("{0}/{1}", y.parse::<u32>().unwrap() - 1911, m);

    let params = [
        ("type", &"Monthly".to_string()),
        ("date", &open_date),
        ("code", &task.security_code),
        ("id", &"".to_string()),
        ("response", &"json".to_string()),
    ];

    let client = Client::new();

    let res = client
        .post("https://www.tpex.org.tw/www/zh-tw/emerging/historical")
        .form(&params)
        .timeout(Duration::from_secs(4))
        .send()
        .await?;
    event!(target: "security_api", Level::DEBUG, "{:?}", &res.url().to_string());

    let json = res.json::<SecurityPriceTpex>().await?;
    event!(target: "security_api", Level::DEBUG, "{:?}", &json);

    let json_str = get_tpex_price(json, tw_ym, 0, 5);
    event!(target: "security_api", Level::DEBUG,  "{0}", &json_str);

    Ok(html_decode(&json_str))
}

fn get_tpex_price(
    tpex_json: SecurityPriceTpex,
    tw_ym: String,
    date_index: usize,
    price_index: usize,
) -> String {
    if tpex_json.tables.first().is_some() {
        let table = tpex_json.tables.first().unwrap();

        let status = if table.total_count > 0 {
            "Y".to_string()
        } else {
            "N".to_string()
        };
        let title = table.subtitle.clone();
        let date = table.date.clone();
        let fields = table.fields.clone();
        let data = get_close_price(table.data.clone(), tw_ym, date_index, price_index);

        if "Y" == status && !data.is_empty() {
            return serde_json::to_string(&MonthlyPrice {
                status,
                title,
                date,
                fields,
                data,
            })
            .unwrap_or("".to_string());
        } else {
            return "".to_string();
        }
    } else {
        return "".to_string();
    }
}

fn get_close_price(
    data: Vec<Vec<String>>,
    tw_ym: String,
    date_index: usize,
    price_index: usize,
) -> Vec<Vec<String>> {
    data.iter()
        .filter(|x| x[date_index].trim().starts_with(&tw_ym))
        .filter(|x| {
            bigdecimal::BigDecimal::from_str(x[price_index].replace(",", "").as_str())
                .unwrap_or(bigdecimal::BigDecimal::zero())
                > bigdecimal::BigDecimal::zero()
        })
        .map(|x| {
            vec![
                x[date_index as usize].clone(),
                x[price_index as usize].replace(",", "").clone(),
            ]
        })
        .collect()
}

fn run_task_log(task: &SecurityTask) {
    let security_code = &task.security_code;
    let market_type = &task.market_type;

    let y = &task.open_date_year;
    let m = &task.open_date_month;
    let d = &task.open_date_day;
    let open_date = format!("{0}{1}{2}", y, m, d);

    event!(target: "security_api", Level::INFO, "send [ {0}: {1}({2}) ]", open_date, market_type, security_code);
}
