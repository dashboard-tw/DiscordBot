mod client;

use std::sync::atomic::AtomicU32;

pub use client::run;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

#[derive(Default)]
pub struct Data {
    pub datasets: crate::http::datasets::DatasetClient,
    #[allow(dead_code)]
    poise_mentions: AtomicU32,
}
