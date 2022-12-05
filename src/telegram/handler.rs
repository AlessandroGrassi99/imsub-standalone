use std::error::Error;
use teloxide::{
    adaptors::{CacheMe, DefaultParseMode, Throttle},
    prelude::*,
    utils::command::BotCommands,
};
// use crate::{LocaleManager, StorageManager, TwitchManager};
use super::{command, BotType};
use crate::locale::{Locale, LocaleManager};

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
