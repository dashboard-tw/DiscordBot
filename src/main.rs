use twdash_bot::{bot, cache, config::Config, http::HttpClient};

#[tokio::main]
async fn main() -> Result<(), bot::Error> {
    let config = Config::init()?;
    cache::init();
    HttpClient::init();
    bot::run(config).await
}
