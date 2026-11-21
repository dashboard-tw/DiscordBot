use twdash_bot::{bot, cache, http::HttpClient, config::Config};

#[tokio::main]
async fn main() -> Result<(), bot::Error> {
    let config = Config::from_env()?;
    cache::init();
    HttpClient::init();
    bot::run(config).await
}
