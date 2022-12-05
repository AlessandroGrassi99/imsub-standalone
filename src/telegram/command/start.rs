use super::super::BotType;
use crate::locale::{locale_args, LocaleManager};
use std::error::Error;
use teloxide::prelude::*;

pub(crate) async fn start_new_user(
    message: Message,
    bot: BotType,
    locale: LocaleManager,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let loading_text = locale
        .get_message("start", "loading", locale_args![])
        .ok_or("Locale message not found")?;
    let msg = bot.send_message(message.chat.id, loading_text).await?;

    let text = locale
        .get_message(
            "start",
            "hello",
            locale_args![("userName", message.chat.first_name().unwrap())],
        )
        .unwrap();
    bot.edit_message_text(msg.chat.id, msg.id, text).await?;
    Ok(())
}
