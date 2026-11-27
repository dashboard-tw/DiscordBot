mod admin;
mod datasets;
mod environment;
mod living;
mod ping;
mod info;
mod presentation;
mod society;
mod transport;
mod weather;

use crate::bot::{Data, Error};

pub fn all() -> Vec<poise::Command<Data, Error>> {
    vec![
        ping::ping(),
        info::info(),
        admin::refresh(),
        weather::command(),
        environment::command(),
        living::command(),
        transport::command(),
        society::command(),
    ]
}
