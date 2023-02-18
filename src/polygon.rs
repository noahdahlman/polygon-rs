use async_trait::async_trait;
use eyre::Result;
use reqwest::{self, StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::error::Error;

use url::Url;

pub fn get_polygon_api_key() -> String {
    std::env::var("POLYGON_API_KEY").expect("POLYGON_API_KEY must be set")
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ResponseObject<T> {
    count: i32,
    next_url: Option<String>,
    request_id: String,
    results: Vec<T>,
    status: String,
}

// fn save_response<T>(response: ResponseObject<T>, filename: &str) -> Result<()> {
//     let data: serde_json::Value = match std::path::Path::exists(filename) {
//         true => {
//             let file = fs::read_to_string(filename)?;
//             serde_json::from_str(&file)?
//         }
//         false => {
//             json!({})
//         }
//     };
//     let mut data_map = data.as_object_mut().unwrap();

//     std::fs::write(filename, response)?;
//     Ok(())
// }

const base_url: &str = "https://api.polygon.io/v3/";
const tickers: &str = "reference/tickers";

pub struct Polygon {
    api_key: String,
    client: reqwest::Client,
}

#[async_trait]
pub trait API<T> {
    async fn connect(api_key: String) -> Result<T>;
    async fn fetch(&self, url: Url) -> Result<ResponseObject<Value>>;
}

#[async_trait]
impl API<Polygon> for Polygon {
    async fn connect(api_key: String) -> Result<Polygon> {
        Ok(Self {
            api_key,
            client: reqwest::Client::new(),
        })
    }

    async fn fetch(&self, url: Url) -> Result<ResponseObject<Value>> {
        let response = self
            .client
            .get(url.clone())
            .header("Authorization", format!("Bearer {}", self.api_key.clone()))
            .send()
            .await?;
        match response.status() {
            StatusCode::OK => Ok(response.json::<ResponseObject<Value>>().await?),
            _ => Err(eyre::eyre!(
                "Error fetching {} with HTTP status code {}: {}",
                url.clone(),
                response.status(),
                response.text().await?
            )),
        }
    }
}

impl Polygon {
    pub async fn markets(&self) -> Result<ResponseObject<Value>, Box<dyn Error>> {
        let url: Url = format!("{}{}?market=stocks&active=true", base_url, tickers).parse()?;
        let response = self.fetch(url).await?;
        println!("{:?}", response);
        Ok(response)
    }
}

mod test {
    use super::*;
    use dotenv::dotenv;

    #[tokio::test]
    async fn test_polygon_api() {
        dotenv().ok();
        let api_key = get_polygon_api_key();
        let polygon = Polygon::connect(api_key).await.unwrap();
        let response = polygon.markets().await.unwrap();
        assert_eq!(response.status, "OK");
    }
}
