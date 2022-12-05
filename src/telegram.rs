use teloxide::dispatching::UpdateHandler;
use teloxide::dptree::case;
use teloxide::requests::Output;
use teloxide::{
    adaptors::{CacheMe, DefaultParseMode, Throttle},
    prelude::*,
    utils::command::BotCommands,
};

mod command;
mod handler;

use command::Command;

type BotType = DefaultParseMode<Throttle<CacheMe<Bot>>>;
type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

pub(crate) fn schema() -> UpdateHandler<Box<dyn std::error::Error + Send + Sync + 'static>> {
    let command_handler = teloxide::filter_command::<Command, _>()
        .branch(case![Command::Start].endpoint(command::start::start))
        .branch(case![Command::Help].endpoint(command::help::help));

    let message_handler = Update::filter_message().branch(command_handler);

    dptree::entry().branch(message_handler)
}
