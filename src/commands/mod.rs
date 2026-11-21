mod ping;

use crate::bot::{Data, Error};

pub fn all() -> Vec<poise::Command<Data, Error>> {
    vec![ping::ping()]
}