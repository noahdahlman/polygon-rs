mod polygon;
use polygon::{Polygon, API};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    Polygon::connect(std::env::var("POLYGON_API_KEY")?)
        .await?
        .markets()
        .await?;
    Ok(())
}
