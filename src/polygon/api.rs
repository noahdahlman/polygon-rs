
use super::common::{ResponseObject, Ticker};
use super::constants::*;
use async_trait::async_trait;
use eyre::Result;
use reqwest::{self, StatusCode};
use std::env;
use std::sync::Arc;
use url::Url;

#[async_trait]
pub trait API {
    async fn connect() -> Result<reqwest::Client>;
    async fn fetch(&self, url: Url) -> Result<ResponseObject>;
}

#[async_trait]
impl API for Polygon {
    async fn connect() -> Result<reqwest::Client> {
        Ok(reqwest::Client::new())
    }

    async fn fetch(&self, url: Url) -> Result<ResponseObject> {
        let response = self
            .client
            .get(url.clone())
            .header("Authorization", format!("Bearer {}", self.api_key()))
            .send()
            .await?;
        match response.status() {
            StatusCode::OK => Ok(response.json::<ResponseObject>().await?),
            _ => Err(eyre::eyre!(
                "Error fetching {} with HTTP status code {}: {}",
                url.clone(),
                response.status(),
                response.text().await?
            )),
        }
    }
}

pub struct Polygon {
    client: Arc<reqwest::Client>,
    base_url: String,
}

impl Polygon {
    pub async fn new() -> Result<Polygon> {
        const BASE_URL: &str = "https://api.polygon.io";

        Ok(Polygon {
            client: Arc::new(Self::connect().await?),
            base_url: BASE_URL.to_string(),
        })
    }
    #[cfg(test)]
    pub async fn new_with_base_url(base_url: String) -> Result<Polygon> {
        Ok(Polygon {
            client: Arc::new(Self::connect().await?),
            base_url,
        })
    }

    pub fn api_key(&self) -> String {
        dotenv::dotenv().ok();
        env::var("POLYGON_API_KEY").expect("Polygon API key not in env variables...")
    }

    pub async fn markets(&self) -> Result<ResponseObject> {
        let url: Url = format!("{}{}?market=stocks&active=true", self.base_url, TICKERS).parse()?;
        let response = self.fetch(url).await?;
        Ok(response)
    }
    pub async fn financials(&self, symbol: String) -> Result<ResponseObject> {
        self.fetch(match symbol.is_empty() {
            true => format!("{}{}", self.base_url, FINANCIALS).parse()?,
            false => format!("{}{}?ticker={}", self.base_url, FINANCIALS, symbol).parse()?,
        })
        .await
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use dotenv::dotenv;

    #[tokio::test]
    async fn test_polygon_markets() {
        dotenv().ok();
        let mut server = mockito::Server::new_async().await;
        let url = server.url();
        // http://localhost:8080/v3/reference/tickers?market=stocks&active=true
        let ticker_mock = server
            .mock(
                "GET",
                format!("{}{}", TICKERS, "?market=stocks&active=true").as_str(),
            )
            .with_body_from_file("tests/mocks/v3_reference_tickers.json")
            .create_async()
            .await;
        let polygon = Polygon::new_with_base_url(url)
            .await
            .expect("Failed to create Polygon API");

        let response = polygon
            .markets()
            .await
            .expect("Failed to fetch markets....");

        let ticker: Ticker = serde_json::from_value(response.results.first().unwrap().to_owned())
            .expect("Failed to deserialize Ticker...");
        assert_eq!(ticker.ticker, "A");
        assert_eq!(ticker.name, "Agilent Technologies Inc.");
        assert_eq!(ticker.market, "stocks");
        assert_eq!(ticker.locale, "us");
        assert_eq!(ticker.currency_name, "usd");
        assert_eq!(ticker.active, true);
        assert_eq!(ticker.primary_exchange, "XNYS");
        assert_eq!(ticker.type_, "CS");
        assert_eq!(response.status, "OK");

        ticker_mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_polygon_financials() {
        dotenv().ok();
        let mut server = mockito::Server::new_async().await;
        let url = server.url();

        let financials_mock = server
            .mock("GET", "/vX/reference/financials")
            .with_body_from_file("tests/mocks/vx_reference_financials.json")
            .create_async()
            .await;

        let appl_financials_mock = server
            .mock("GET", format!("{}{}", FINANCIALS, "?ticker=AAPL").as_str())
            .with_body_from_file("tests/mocks/vx_reference_financials_appl.json")
            .create_async()
            .await;

        let polygon = Polygon::new_with_base_url(url).await.unwrap();
        let aapl_response = polygon.financials("AAPL".to_string()).await.unwrap();
        let financials_response = polygon.financials("".to_string()).await.unwrap();

        assert_eq!(aapl_response.status, "OK");
        assert_eq!(financials_response.status, "OK");
        financials_mock.assert_async().await;
        appl_financials_mock.assert_async().await;
    }
}
