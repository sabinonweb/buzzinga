use crate::{
    data::{ABSURD_CONTENT_SUBREDDITS, GAMING_CONTENT_SUBREDDITS, LIMIT},
    types::{config_types::RedditClient, reddit_types::RedditVideo},
    utils::filter_content,
};
use anyhow::Ok;
use roux::{response::BasicThing, submission::SubmissionData};
use std::sync::{Arc, Mutex};

pub(crate) async fn get_videos_collection(
    reddit_client: Arc<Mutex<RedditClient>>,
    subreddit_collection_type: Vec<&str>,
) -> anyhow::Result<Vec<RedditVideo>> {
    log::info!("Fetching videos from subreddit!");

    let response_collection =
        get_reddit_response(reddit_client.clone(), subreddit_collection_type).await?;

    // filtering content for we want to post videos only for ABSURD_CONTENT
    let filtered_videos = filter_content(&response_collection);

    Ok(filtered_videos)
}

async fn get_reddit_response(
    reddit_client: Arc<Mutex<RedditClient>>,
    subreddit_collection: Vec<&str>,
) -> anyhow::Result<Vec<BasicThing<SubmissionData>>> {
    let mut handles = Vec::with_capacity(LIMIT as usize * subreddit_collection.len());

    for &subreddit in &subreddit_collection {
        let reddit_client = Arc::clone(&reddit_client);
        let reddit_client = reddit_client.lock().unwrap();
        let subreddit_client = reddit_client.clone().reddit.subreddit(subreddit).await?;

        let handle = tokio::spawn(async move { subreddit_client.hot(LIMIT as u32, None).await });
        handles.push(handle);
    }

    let mut response_collection: Vec<BasicThing<SubmissionData>> = Vec::new();

    for handle in handles {
        let response = handle.await??.data.children;

        for response_data in response {
            response_collection.push(response_data);
        }
    }

    Ok(response_collection)
}

pub(crate) async fn scrape_for_videos(
    reddit_client: Arc<Mutex<RedditClient>>,
    video_domain: String,
) -> anyhow::Result<Vec<RedditVideo>> {
    match video_domain.as_str() {
        "absurd" => get_videos_collection(reddit_client, ABSURD_CONTENT_SUBREDDITS.to_vec()).await,
        "gaming" => get_videos_collection(reddit_client, GAMING_CONTENT_SUBREDDITS.to_vec()).await,
    }
}
