use super::super::BotType;
use crate::locale::LocaleManager;
use std::error::Error;
use teloxide::prelude::*;

pub(crate) async fn start(
    message: Message,
    bot: BotType,
    mut locale: LocaleManager
) -> Result<(), Box<dyn Error + Send + Sync>> {
    locale.set_chat_locale_from_message(&message);

    bot.send_message(message.chat.id, format!("/start message test")).await?;

    Ok(())
}
