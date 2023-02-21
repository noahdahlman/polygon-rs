use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;


#[derive(Debug, Deserialize, Serialize)]
pub struct ResponseObject {
    pub count: i32,
    pub next_url: Option<String>,
    pub request_id: String,
    pub results: Vec<serde_json::Value>,
    pub status: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Ticker {
    pub ticker: String,
    pub name: String,
    pub market: String,
    pub locale: String,
    pub currency_name: String,
    pub active: bool,
    pub cik: String,
    pub primary_exchange: String,
    pub share_class_figi: String,
    pub composite_figi: String,
    #[serde(rename = "type")]
    pub type_: String,
    //TODO: Figure out how to deserialize this
    pub last_updated_utc: String,
    pub delisted_utc: Option<String>,
}

impl Into<String> for ResponseObject {
    fn into(self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

impl ToString for ResponseObject {
    fn to_string(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

#[cfg(test)]
impl ResponseObject {
    pub fn save(&self) {
        let response_str: String = self.to_string();
        let mut file = std::fs::File::create("response.json").unwrap();
        write!(file, "{}", response_str).unwrap();
    }
}
