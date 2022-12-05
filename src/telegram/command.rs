use std::error::Error;
use teloxide::{
    adaptors::{CacheMe, DefaultParseMode, Throttle},
    prelude::*,
    utils::command::BotCommands,
};

pub mod help;
pub mod reset;
pub mod start;

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
pub enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "verify user.", parse_with = "split")]
    Start,
    #[command(description = "delete all data.", parse_with = "split")]
    Reset,
}
