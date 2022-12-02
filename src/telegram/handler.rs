use std::error::Error;
use teloxide::{
    prelude::*,
    utils::command::BotCommands,
    adaptors::{CacheMe, DefaultParseMode, Throttle}
};
// use crate::{LocaleManager, StorageManager, TwitchManager};
use crate::locale::{Locale, LocaleManager};
use crate::telegram::command;

type BotType = DefaultParseMode<CacheMe<Throttle<Bot>>>;

pub async fn message_handler(
    message: Message,
    bot: BotType,
    locale: LocaleManager,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    if let Some(text) = message.text() {
        use crate::telegram::command::Command::*;
        use crate::telegram::command::Command;

        let mut locale = locale.clone();
        locale.load_current_locale(get_language(&message).await).await;


        match Command::parse(text, "buttons") {
            Ok(Help) => {
                bot.send_message(message.chat.id, Command::descriptions().to_string()).await?;
            },
            Ok(Start) => {
                command::start(message, bot).await?;
            },
            Ok(Reset) => {
                bot.send_message(message.chat.id, "Reset".to_string()).await?;
            },
            Err(_) => { }
        }
    };

    Ok(())
}

async fn get_language(message: &Message) -> Locale {
    let mut locale = Locale::default();
    if let Some(user) = message.from() {
        if let Some(lang) = &user.language_code {
            if let Ok(chat_locale) = Locale::try_from(lang.as_str()) {
                locale = chat_locale;
            }
        }
    }

    locale
}