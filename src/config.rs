use std::collections::HashSet;
use serde::Deserialize;
use toml::value::{Datetime, Time};

#[derive(Deserialize)]
pub(crate) struct Config {
    log: Log,
    schedule: Schedule,
    telegram: Telegram,
    twitch: Twitch,
    rabbitmq: RabbitMQ,
    database: Database,
}

#[derive(Deserialize)]
pub(crate) struct Log {
    level: String,
    path: String,
}

#[derive(Deserialize)]
pub(crate) struct Schedule {
    grace_days: Option<u32>,
    time: Option<Datetime>,
}

#[derive(Deserialize)]
pub(crate) struct Telegram {
    token: String,
    groups: HashSet<i64>,
    developers: HashSet<i64>,
}

#[derive(Deserialize)]
pub(crate) struct Twitch { 
    streamer: String,
    client_id: String,
    client_secret: String,
    redirect_uri: String,
}

#[derive(Deserialize)]
pub(crate) struct RabbitMQ {
    host: String,
    port: u16,
    ca_cert: Option<String>,
    client_cert: Option<String>,
    client_key: Option<String>,
}

#[derive(Deserialize)]
pub(crate) struct Database {
    host: String,
    port: u16,
    username: String,
    database: String,
    ca_cert: Option<String>,
    client_cert: Option<String>,
    client_key: Option<String>,
}



