use poise::serenity_prelude as serenity;

use crate::bot::{Data, Error};

pub async fn event_handler(
    _: poise::FrameworkContext<'_, Data, Error>,
    event: &serenity::FullEvent,
) -> Result<(), Error> {
    if let serenity::FullEvent::Ready { data_about_bot, .. } = event {
        println!("Logged in as {}", data_about_bot.user.name);
    }

    Ok(())
}
