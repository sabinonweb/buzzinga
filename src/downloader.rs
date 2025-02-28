use crate::types::reddit_types::RedditContent;
use anyhow::Ok;
use log::warn;
use tempfile::{tempdir, tempfile, Builder};

pub fn downloader(reddit_content: Vec<RedditContent>) -> anyhow::Result<()> {
    let tempdir = Builder::new().prefix("scrapped_content").tempdir()?;
    log::info!("Temporary directory created: {:?}", tempdir);

    for content in reddit_content {}

    Ok(())
}
