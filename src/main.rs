use std::{
    str::FromStr,
    sync::{Arc, Mutex},
    thread,
};

use anyhow::Context;
use chrono::Utc;
use clap::Parser;
use cron::Schedule;
use reddit::scrape_for_content;
use types::{args::Args, config_types::RedditClient};

mod config;
mod data;
mod reddit;
mod types;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    //sec  min   hour   day of month   month   day of week   year
    let expression = "0 0 9,12,15,18,21 * * *";
    let schedule = Schedule::from_str(expression).context("Failed to parse CRON expression")?;
    log::info!("Firing cronjob for buzzinga!");

    log4rs::init_file(&args.log_config, Default::default())?;
    log::info!("Fetching the environment variables!");

    loop {
        if let Some(datetime) = schedule.upcoming(Utc).take(1).next() {
            let time_frame = datetime - Utc::now();
            thread::sleep(time_frame.to_std().unwrap());
            let reddit_client = Arc::new(Mutex::new(RedditClient::new(&args).await?));
            scrape_for_content(reddit_client, "absurd_content".to_string()).await?;
        }
    }
}
