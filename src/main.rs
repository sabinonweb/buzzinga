use anyhow::Context;
use args::Args;
use clap::Parser;
use config::config_client;

mod args;
mod config;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    log4rs::init_file(&args.log_config, Default::default())?;
    log::info!("Fetching the environment variables!");

    let reddit_client = config_client(&args).await?;

    Ok(())
}
