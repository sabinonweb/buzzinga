use crate::types::{config_types::RedditClient, response::Media};
use ::log::{error, info};
use ffmpeg_next::log;
use std::{fs::File, io::Write};

pub async fn downloader(reddit_media: Media, reddit_client: RedditClient) {
    println!("Downloader");
    println!("Reddit Media URL: {:?}", reddit_media.clone().video_url);
    let response = match reddit_client
        .client
        .get(&reddit_media.video_url.clone()[..])
        .send()
        .await
    {
        Ok(response) => {
            info!("Video {} successfully downloaded!", reddit_media.video_url);
            Ok(response)
        }
        Err(e) => {
            info!(
                "Error while downloading the video: {}: {}",
                reddit_media.title, e
            );
            Err(e)
        }
    };
    let file_name = format!("{}.mp4", reddit_media.title);
    let mut file = match File::create(file_name.clone()) {
        Ok(file) => {
            info!("File {} successfully created!", file_name);
            file
        }
        Err(e) => {
            error!("Error while creating file {}: {}", file_name, e);
            return;
        }
    };

    let bytes = match response.unwrap().bytes().await {
        Ok(bytes) => bytes,
        Err(e) => {
            error!("Error reading response bytes: {}", e);
            return;
        }
    };

    match file.write_all(&bytes) {
        Ok(_) => info!("Bytes successfully written to the file"),
        Err(e) => error!("Error while writing to the file: {}", e),
    }
}
