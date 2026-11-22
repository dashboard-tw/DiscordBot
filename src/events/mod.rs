mod guild;
mod ready;

use poise::serenity_prelude as serenity;

use crate::bot::{Data, Error};

pub async fn event_handler(
    _: poise::FrameworkContext<'_, Data, Error>,
    event: &serenity::FullEvent,
) -> Result<(), Error> {
    match event {
        serenity::FullEvent::Ready { data_about_bot, .. } => ready::handle(data_about_bot).await,
        serenity::FullEvent::GuildCreate { .. } | serenity::FullEvent::GuildDelete { .. } => {
            guild::handle(event).await;
        }
        _ => {}
    }

    Ok(())
}
