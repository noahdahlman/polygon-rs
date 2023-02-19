use serde::{Deserialize, Serialize};

pub fn get_polygon_api_key() -> String {
    std::env::var("POLYGON_API_KEY").expect("POLYGON_API_KEY must be set")
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ResponseObject<T> {
    pub count: i32,
    pub next_url: Option<String>,
    pub request_id: String,
    pub results: Vec<T>,
    pub status: String,
}
