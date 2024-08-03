use args::Args;
use clap::Parser;
use config::RedditClient;
use reddit::reddit;

mod args;
mod config;
mod reddit;
mod types;
mod utils;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    log4rs::init_file(&args.log_config, Default::default())?;
    log::info!("Fetching the environment variables!");

    let reddit_client = RedditClient::new(&args).await?;

    let response = reddit_client
        .reddit
        .subreddit("PublicFreakOut")
        .await?
        .hot(25, None)
        .await?;

    println!("response: {:?}", response);

    Ok(())
}
