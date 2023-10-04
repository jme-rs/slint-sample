use std::error::Error;

use chrono::{DateTime, Datelike, Local, Timelike};
use reqwest::Client;
use serde_json::Value;
use url::Url;

use crate::webapi::locale::Locale;

const API_KEY: &str = "d57755c02ca442f5b4b145148231608";
const API_URL: &str = "https://api.weatherapi.com/v1/forecast.json";

#[derive(Debug)]
pub struct Weather {
    pub date: DateTime<Local>,
    pub wether: String,
    pub temperature: f64,
    pub humidity: f64,
    pub pressure: f64,
}

#[derive(Debug)]
pub struct Forecast {
    pub locale: Locale,
    pub weekly_weathers: Vec<Weather>,
    pub hourly_weathers: Vec<Weather>,
}

impl Forecast {
    pub fn new(locale: Locale) -> Self {
        Self {
            locale,
            weekly_weathers: Vec::new(),
            hourly_weathers: Vec::new(),
        }
    }

    pub async fn fetch_json(&self) -> Result<Value, Box<dyn Error>> {
        let lat_lon = format!("{},{}", self.locale.lat, self.locale.lon);
        let queries = vec![
            ("key", API_KEY.to_string()),
            ("q", lat_lon),
            ("lang", "ja".to_string()),
        ];

        let mut url = Url::parse(API_URL).unwrap();
        url.query_pairs_mut().extend_pairs(queries);
        dbg!(url.to_string());

        let client = Client::new();
        let response = client.get(url.to_string()).send().await?;
        let res_text = response.text().await?;

        Ok(serde_json::from_str(&res_text)?)
    }

    // pub async fn update(self) -> Self {

    // }
}

pub fn compare_date(date1: &DateTime<Local>, date2: &DateTime<Local>) -> bool {
    date1.year() == date2.year()
        && date1.month() == date2.month()
        && date1.day() == date2.day()
        && date1.hour() == date2.hour()
}
