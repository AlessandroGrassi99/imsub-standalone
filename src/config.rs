use serde::Deserialize;
use std::collections::HashSet;
use toml::value::Datetime;

#[derive(Deserialize)]
pub(crate) struct Config {
    pub log: Log,
    pub schedule: Schedule,
    pub telegram: Telegram,
    pub twitch: Twitch,
    pub rabbitmq: RabbitMQ,
    pub database: Database,
    pub locale: Locale,
}

#[derive(Deserialize)]
pub(crate) struct Log {
    pub level: String,
    pub path: String,
}

#[derive(Deserialize)]
pub(crate) struct Schedule {
    pub grace_days: Option<u32>,
    pub time: Option<Datetime>,
}

#[derive(Deserialize)]
pub(crate) struct Telegram {
    pub token: String,
    pub groups: HashSet<i64>,
    pub developers: HashSet<i64>,
}

#[derive(Deserialize)]
pub(crate) struct Twitch {
    pub streamer: String,
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
}

#[derive(Deserialize)]
pub(crate) struct RabbitMQ {
    pub host: String,
    pub port: u16,
    pub ca_cert: Option<String>,
    pub client_cert: Option<String>,
    pub client_key: Option<String>,
}

#[derive(Deserialize)]
pub(crate) struct Database {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub database: String,
    pub ca_cert: Option<String>,
    pub client_cert: Option<String>,
    pub client_key: Option<String>,
}

#[derive(Deserialize)]
pub(crate) struct Locale {
    pub path: String,
    pub default_lang: String,
}
