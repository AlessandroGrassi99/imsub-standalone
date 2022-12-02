use clap::Parser;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

mod config;
mod database;
mod locale;
mod telegram;

use config::Config;
use teloxide::{prelude::*, adaptors::throttle::Limits};
use crate::locale::LocaleManager;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = String::from("config.toml"))]
    file_path: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let config = config_from_file(args.file_path).await;

    let locale = LocaleManager::new(config.locale.path.as_str(), config.locale.default_lang.as_str()).await
        .expect("unable to create the locale manager");

    let bot = Bot::new(config.telegram.token.as_str())
        .throttle(Limits::default())
        .parse_mode(teloxide::types::ParseMode::Html);

    let handler = dptree::entry()
        .branch(Update::filter_message().endpoint(telegram::handler::message_handler));

    Dispatcher::builder(bot, handler)
        .dependencies(dptree::deps![locale])
        .default_handler(|upd| async move {
            println!("Unhandled update: {:?}", upd);
        })
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}

async fn config_from_file(file_path: String) -> Config {
    let mut file = File::open(file_path).await.expect("open config file");
    let mut contents = vec![];
    file.read_to_end(&mut contents).await.expect("read file");

    toml::from_slice(contents.as_slice()).expect("parse config file to toml")
}