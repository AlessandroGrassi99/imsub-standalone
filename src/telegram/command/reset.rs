use super::super::BotType;
use super::Command;
use std::error::Error;
use teloxide::prelude::*;

pub(crate) async fn reset(
    message: Message,
    bot: BotType,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    bot.send_message(message.chat.id, "Reset".to_string())
        .await?;

    Ok(())
}
