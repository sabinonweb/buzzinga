use std::sync::{Arc, Mutex};

use clap::Parser;
use reddit::scrape_for_videos;
use types::{args::Args, config_types::RedditClient};

mod config;
mod data;
mod reddit;
mod types;
mod utils;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    log4rs::init_file(&args.log_config, Default::default())?;
    log::info!("Fetching the environment variables!");

    let reddit_client = Arc::new(Mutex::new(RedditClient::new(&args).await?));

    // let response = reddit_client
    //     .clone()
    //     .reddit
    //     .subreddit("PublicFreakout")
    //     .await?
    //     .hot(25, None)
    //     .await?;
    //
    // for r in response.data.children {
    //     let post_type = post_type(r.data.url.unwrap())?;
    //     println!("post_type: {:?}", post_type);
    // }

    scrape_for_videos(reddit_client).await?;

    Ok(())
}
