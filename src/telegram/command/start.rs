use super::super::BotType;
use std::error::Error;
use teloxide::prelude::*;
use crate::locale::Locale;

pub(crate) async fn start(
    message: Message,
    bot: BotType,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    bot.send_message(message.chat.id, format!("/start message test"))
        .await?;

    Ok(())
}
