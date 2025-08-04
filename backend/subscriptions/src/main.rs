use subscriptions::Config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::load()?;
    subscriptions::run(config).await;
    Ok(())
}
