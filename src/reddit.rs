use crate::{
    data::{
        ABSURD_CONTENT_SUBREDDITS, BEST_FAILS, GAMING_CONTENT_SUBREDDITS,
        GENERAL_MEMES_CONTENT_SUBREDDITS, LIMIT, NICHE_MEMES_CONTENT_SUBREDDITS,
        PERFECTLY_TIMED_CONTENT_SUBREDDITS,
    },
    types::{
        config_types::RedditClient,
        reddit_types::{
            AbsurdContent, FailContent, Filtration, GamingContent, GeneralMemes, NicheMemes,
            PerfectlyTimed, RedditContent, RedditContentType, RedditDashJsonResponse,
        },
    },
};
use anyhow::anyhow;
use roux::{response::BasicThing, submission::SubmissionData};
use std::sync::{Arc, Mutex};

pub(crate) async fn get_videos_collection<T>(
    reddit_client: Arc<Mutex<RedditClient>>,
    subreddit_collection_type: Vec<&str>,
    _content_type: T,
) -> anyhow::Result<Vec<RedditContent>>
where
    T: Filtration,
{
    log::info!("Fetching videos from subreddit!");

    log::info!("{:?}", subreddit_collection_type);
    let response_collection =
        get_reddit_response(reddit_client.clone(), subreddit_collection_type).await?;
    // filtering content for we want to post videos only

    let filtered_videos = T::filter_content(&response_collection);
    get_content_json(reddit_client, filtered_videos.clone()).await;

    Ok(filtered_videos)
}

pub(crate) async fn get_content_collection(
    reddit_client: Arc<Mutex<RedditClient>>,
    subreddit_collection_type: Vec<&str>,
) -> anyhow::Result<Vec<RedditContent>> {
    log::info!("Fetching videos from subreddit!");

    log::info!("{:?}", subreddit_collection_type);
    let _response_collection =
        get_reddit_response(reddit_client.clone(), subreddit_collection_type).await?;

    let reddit_content: Vec<RedditContent> = Vec::new();

    Ok(reddit_content)
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

    // println!(
    //     "Response Collecrion: {:?}",
    //     serde_json::to_string_pretty(&response_collection)
    // );
    Ok(response_collection)
}

pub(crate) async fn get_content_json(
    reddit_client: Arc<Mutex<RedditClient>>,
    reddit_content: Vec<RedditContent>,
) {
    let reddit_client = reddit_client.lock().unwrap();

    /* for content in reddit_content { */
    let res = reddit_client
        .client
        .get("https://www.reddit.com/r/PublicFreakout/comments/1j17pct/senator_marshall_rks_flees_his_own_town_hall.json")
        .form(&reddit_client.params)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    let json_response: Result<Vec<RedditDashJsonResponse>, _> = serde_json::from_str(&res);

    match json_response {
        Ok(ref json_res) => {
            println!("Video_details: {:?}\n", json_res);
        }

        Err(ref e) => {
            log::error!("Deserialization Error for RedditDashJsonResponse: {:?}", e);
            println!("Deserialization Error for RedditDashJsonResponse: {:?}", e);
        }
    }

    // }
}

pub(crate) async fn scrape_for_content(
    reddit_client: Arc<Mutex<RedditClient>>,
    video_domain: String,
) -> anyhow::Result<Vec<RedditContent>> {
    match video_domain.as_str().into() {
        RedditContentType::AbsurdContent => {
            get_videos_collection(
                reddit_client,
                ABSURD_CONTENT_SUBREDDITS.to_vec(),
                AbsurdContent,
            )
            .await
        }
        RedditContentType::GamingContent => {
            get_videos_collection(
                reddit_client,
                GAMING_CONTENT_SUBREDDITS.to_vec(),
                GamingContent,
            )
            .await
        }

        RedditContentType::GeneralMemes => {
            get_content_collection(reddit_client, GENERAL_MEMES_CONTENT_SUBREDDITS.to_vec()).await
        }

        RedditContentType::NicheMemes => {
            get_content_collection(reddit_client, NICHE_MEMES_CONTENT_SUBREDDITS.to_vec()).await
        }

        RedditContentType::FailContent => {
            get_videos_collection(reddit_client, BEST_FAILS.to_vec(), FailContent).await
        }

        RedditContentType::PerfectlyTimed => {
            get_videos_collection(
                reddit_client,
                PERFECTLY_TIMED_CONTENT_SUBREDDITS.to_vec(),
                PerfectlyTimed,
            )
            .await
        }

        RedditContentType::SportsContent => {
            get_videos_collection(
                reddit_client,
                GAMING_CONTENT_SUBREDDITS.to_vec(),
                GamingContent,
            )
            .await
        }

        RedditContentType::Miscellaneous => Err(anyhow!("The content type is not supported yet!")),
    }
}
