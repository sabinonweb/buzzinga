use args::Args;
use clap::Parser;
use config::RedditClient;

mod args;
mod config;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    log4rs::init_file(&args.log_config, Default::default())?;
    log::info!("Fetching the environment variables!");

    let reddit_client = RedditClient::new(&args).await?;

    let response = reddit_client
        .client
        .get("https://reddit.com/api/v1/me")
        .send()
        .await?;

    Ok(())
}
