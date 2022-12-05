use super::super::BotType;
use crate::locale::LocaleManager;
use std::error::Error;
use teloxide::prelude::*;

pub(crate) async fn start(
    message: Message,
    bot: BotType,
    locale: LocaleManager,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    println!("{:#?}", locale.local_locale);

    let text = locale.get_message(
        "start",
        "hello",
        Some(vec![(
            "userName".to_string(),
            message.chat.first_name().unwrap().to_string(),
        )]),
    );

    bot.send_message(message.chat.id, text.unwrap())
        .await?;

    Ok(())
}
