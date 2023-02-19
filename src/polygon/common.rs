use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ResponseObject {
    pub count: i32,
    pub next_url: Option<String>,
    pub request_id: String,
    pub results: Vec<serde_json::Value>,
    pub status: String,
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
