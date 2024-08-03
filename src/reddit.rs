use crate::config::RedditClient;
use crate::types::response::{PostInformation, RedditReponse};
use std::fs::File;

pub(crate) async fn reddit(reddit_client: RedditClient) -> anyhow::Result<()> {
    let limit = 50;
    // let info = content["data"]["children"]
    //     .clone()
    //     .as_array()
    //     .unwrap()
    //     .to_owned();
    // println!("\n\n\ninfo: {:?}\n\n", info);

    // for post in info {
    //     println!("post as str: {:?}\n", post);
    //     let post_info: RedditReponse = serde_json::from_str(post.as_str().unwrap())?;
    //
    //     println!("\npost: {:#?}\n", post_info);
    // }

    // println!("\n\ndoc: {:?}\n\n", doc);

    Ok(())
}
