use std::fs::File;
use std::io::Write;

use serde_json::Value;

use crate::config::RedditClient;
use crate::types::response::PostInformation;

pub(crate) async fn reddit(reddit_client: RedditClient) -> anyhow::Result<()> {
    let limit = 50;
    println!("\nheaders{:?}\n", reddit_client.client);
    let response = reddit_client
        .client
        .get("https://oauth.reddit.com/r/Guitar/hot.json")
        .query(&[("limit", limit.to_string())])
        .send()
        .await?;

    let mut file = File::create_new("./respone.json")?;
    let text = response.text().await?;
    // println!("text: {:#?}", text);
    let content: Value = serde_json::from_str(&text)?;
    std::io::copy(&mut text.as_bytes(), &mut file)?;
    // println!("\ncontent: {:#?}\n", content);
    let info = content["data"]["children"]
        .clone()
        .as_array()
        .unwrap()
        .to_owned();
    // println!("\n\n\ninfo: {:?}\n\n", info);

    for post in info {
        let post_info: PostInformation = serde_json::from_str(
            post.as_str()
                .expect("Error while fetching post information!"),
        )?;

        println!("\npost: {:#?}\n", post_info);
    }

    // println!("\n\ndoc: {:?}\n\n", doc);

    Ok(())
}
