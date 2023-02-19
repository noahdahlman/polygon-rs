mod polygon;
use polygon::api::{Polygon, API};
use polygon::common::ResponseObject;
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let markets = Polygon::new().await?.markets().await?;
    let market_str: String = markets.into();
    println!("{}", market_str);
    Ok(())
}
