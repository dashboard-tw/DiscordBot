use twdash_bot::{bot, config::Config};

#[tokio::main]
async fn main() -> Result<(), bot::Error> {
    let config = Config::from_env()?;
    bot::run(config).await
}
