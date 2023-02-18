use dotenv;
use tokio;
mod polygon;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    Ok(())
}
