use std::sync::{Arc, Mutex};

use clap::Parser;
use reddit::scrape_for_content;
use types::{args::Args, config_types::RedditClient};

mod config;
mod data;
mod reddit;
mod types;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    log4rs::init_file(&args.log_config, Default::default())?;
    log::info!("Fetching the environment variables!");

    let reddit_client = Arc::new(Mutex::new(RedditClient::new(&args).await?));

    scrape_for_content(reddit_client, "general_memes".to_string()).await?;

    Ok(())
}
