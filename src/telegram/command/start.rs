use super::super::BotType;
use crate::locale::LocaleManager;
use std::error::Error;
use teloxide::prelude::*;

pub(crate) async fn start(
    message: Message,
    bot: BotType,
    locale: LocaleManager,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let text = locale.get_message(
        "start",
        "hello",
        Some(vec![("userName", message.chat.first_name().unwrap())]),
    ).unwrap();

    bot.send_message(message.chat.id, text).await?;

    Ok(())
}
