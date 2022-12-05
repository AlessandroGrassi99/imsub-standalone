use super::super::BotType;
use crate::locale::LocaleManager;
use std::error::Error;
use teloxide::prelude::*;

pub(crate) async fn start(
    message: Message,
    bot: BotType,
    mut locale: LocaleManager,
) -> Result<(), Box<dyn Error + Send + Sync>> {

    bot.send_message(message.chat.id, format!("/start message test"))
        .await?;

    Ok(())
}
