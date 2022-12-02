use std::error::Error;
use teloxide::{
    prelude::*,
    utils::command::BotCommands,
    adaptors::{CacheMe, DefaultParseMode, Throttle}
};
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup}; 
use url::Url;

type BotType = DefaultParseMode<CacheMe<Throttle<Bot>>>;

#[derive(BotCommands)]
#[command(rename_rule = "lowercase", description = "These commands are supported:")]
pub enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "verify user.",  parse_with = "split")]
    Start,
    #[command(description = "delete all data.", parse_with = "split")]
    Reset,
}

pub async fn start(
    message: Message,
    bot: BotType
) -> Result<(), Box<dyn Error + Send + Sync>> {
    bot.send_message(message.chat.id, format!("/start message test")).await?;

    Ok(())
}