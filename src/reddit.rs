use crate::{
    data::{ABSURDVIDEOSSUBREDDIT, LIMIT, MEMES},
    types::{config_types::RedditClient, response::RedditVideo},
};
use anyhow::Ok;
use reqwest::Client;
use roux::{response::BasicThing, submission::SubmissionData, subreddit};
use std::{
    fs::File,
    io::Write,
    sync::{Arc, Mutex},
};
use tokio::task::JoinHandle;

pub(crate) async fn scrape_for_videos(
    reddit_client: Arc<Mutex<RedditClient>>,
) -> anyhow::Result<()> {
    // Future is a Dynamically Sized Type, so it is put behind a pointer
    // dyn refers that trait implements dynamic dispatch i.e. selects implementation according to
    // object at runtime
    // let mut handles: Vec<
    //     tokio::task::JoinHandle<Box<dyn Future<Output = Result<Submissions, RouxError>>>>,
    // > = vec![];
    get_absurd_videos_collection(reddit_client.clone()).await?;
    // let mut handles = Vec::with_capacity(25);
    //
    // for &subreddit in &MEMES {
    //     let reddit_client = Arc::clone(&reddit_client);
    //     let reddit_client = reddit_client.lock().unwrap();
    //     let subreddit_client = reddit_client.reddit.clone().subreddit(subreddit).await?;
    //
    //     let response = tokio::spawn(async move { subreddit_client.hot(25, None).await });
    //     handles.push(response);
    // }
    //
    // println!("Here i am ");
    // for handle in handles {
    //     let res = handle.await??.data.children;
    //
    //     for data in res {
    //         println!("\nres: {:?}\n", data.data.url);
    //         let mut file = File::create_new(format!("{:?}.jpeg", data.data.name))?;
    //         let res = reddit_client
    //             .lock()
    //             .unwrap()
    //             .me
    //             .client
    //             .get(data.data.url.unwrap())
    //             .send()
    //             .await?;
    //         let r = res.text().await?;
    //         println!("Res: {:?}\n", r);
    //         std::io::copy(&mut r.as_bytes(), &mut file)?;
    //     }
    // }

    Ok(())
}

pub(crate) async fn get_absurd_videos_collection(
    reddit_client: Arc<Mutex<RedditClient>>,
) -> anyhow::Result<()> {
    let mut reddit_videos: Vec<RedditVideo> = Vec::new();

    let response_collection =
        get_reddit_response(reddit_client.clone(), ABSURDVIDEOSSUBREDDIT.to_vec()).await?;

    let mut reddit_videos: Vec<RedditVideo> = Vec::new();

    for response in response_collection {
        let video = RedditVideo::from(response.clone());
        println!("\n{:?}\n", video);

        reddit_videos.push(RedditVideo::from(response));
    }

    Ok(())
}

async fn get_reddit_response(
    reddit_client: Arc<Mutex<RedditClient>>,
    subreddit_collection: Vec<&str>,
) -> anyhow::Result<Vec<BasicThing<SubmissionData>>> {
    let mut handles = Vec::with_capacity(LIMIT as usize * ABSURDVIDEOSSUBREDDIT.len());

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

// async fn get_data_in_chunks(subreddit_client: Subreddit) -> anyhow::Result<Vec<JoinHandle<Result<BasicThing<SubmissionData>>>>> {
//     let mut chunks_of_joinhandles = Vec::with_capacity(5);
//
//     for chunk in subreddit.skip(10) {
//
//     }
// }
