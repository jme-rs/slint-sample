use std::error::Error;

use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use url::Url;

const APP_ID: &str = "dj00aiZpPVN3WlVGcUVBRHR5ZyZzPWNvbnN1bWVyc2VjcmV0Jng9YWU-";
const API_URL: &str = "https://map.yahooapis.jp/geocode/cont/V1/contentsGeoCoder";

#[derive(Debug)]
pub struct Locale {
    pub lat: f64,
    pub lon: f64,
    pub address: String,
}

impl Locale {
    pub fn new() -> Self {
        Self {
            lat: 0.0,
            lon: 0.0,
            address: String::new(),
        }
    }

    pub async fn get(&self, address: String) -> Result<Self, Box<dyn Error>> {
        let serialized: Value = Self::fetch_json(address).await?;

        if serialized["Feature"].is_null() {
            return Err("Feature is missing".into());
        }
        let point = serialized["Feature"][0]["Geometry"]["Coordinates"]
            .as_str()
            .ok_or("Coordinates are missing or not a string")?
            .split(',')
            .collect::<Vec<_>>();
        let address = serialized["Feature"][0]["Name"]
            .as_str()
            .ok_or("Address name is missing or not a string")?
            .to_string();

        Ok(Self {
            lat: point[0].parse().unwrap(),
            lon: point[1].parse().unwrap(),
            address,
        })
    }

    pub async fn fetch_json(address: String) -> Result<Value, Box<dyn Error>> {
        let queries = vec![
            ("appid", APP_ID.to_string()),
            ("query", address),
            ("output", "json".to_string()),
        ];

        let mut url = Url::parse(API_URL).unwrap();
        url.query_pairs_mut().extend_pairs(queries);
        dbg!(url.to_string());

        let client = Client::new();
        let response = client.get(url.to_string()).send().await?;
        let res_text = response.text().await?;

        Ok(serde_json::from_str(&res_text)?)
    }
}
