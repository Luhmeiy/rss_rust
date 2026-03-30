use std::error::Error;

mod feed;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    feed::run().await
}
