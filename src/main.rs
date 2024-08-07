use clap::Parser;
use types::{args::Args, config_types::RedditClient};
use utils::post_type;

mod config;
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
        .subreddit("PublicFreakout")
        .await?
        .hot(25, None)
        .await?;

    for r in response.data.children {
        let post_type = post_type(r.data.url.unwrap())?;
        println!("post_type: {:?}", post_type);
    }

    Ok(())
}
