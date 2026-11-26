use twdash_bot::{bot, cache, config::Config, http::HttpClient, logger};

#[tokio::main]
async fn main() -> Result<(), bot::Error> {
    let config = Config::init()?;
    logger::init();
    cache::init();
    HttpClient::init();
    bot::run(config).await
}
