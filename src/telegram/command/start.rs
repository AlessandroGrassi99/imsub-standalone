use super::super::BotType;
use crate::locale::LocaleManager;
use std::error::Error;
use fluent::fluent_args;
use teloxide::prelude::*;

pub(crate) async fn start(
    message: Message,
    bot: BotType,
    locale: LocaleManager,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let loading_text = locale.get_message("start", "loading", fluent_args![])?;
    let msg = bot.send_message(message.chat.id, loading_text).await?;

    let text = locale.get_message(
        "start",
        "hello",
        fluent_args!["userName" => message.chat.first_name().unwrap()],
    )?;
    bot.edit_message_text(msg.chat.id, msg.id, text).await?;
    Ok(())
}
