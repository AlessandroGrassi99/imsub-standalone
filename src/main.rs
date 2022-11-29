use clap::Parser;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

mod config;
mod database;
mod locale;

use config::Config;

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

}

async fn config_from_file(file_path: String) -> Config {
    let mut file = File::open(file_path).await.expect("open config file");
    let mut contents = vec![];
    file.read_to_end(&mut contents).await.expect("read file");

    toml::from_slice(contents.as_slice()).expect("parse config file to toml")
}