use crate::types::{config_types::RedditClient, response::Media};
use ::log::{error, info};
use roux::me::response;
use std::{fs::File, io::Write, process::Command};
use tokio::join;

pub async fn download_media(reddit_media: Media, reddit_client: RedditClient) {
    info!("Starting download for {}", reddit_media.title);
    let video_file_name = &format!(
        "/Users/sabinonweb/Documents/Projects/buzzinga/src/media/{}.mp4",
        reddit_media.title
    );
    let audio_file_name = &format!(
        "/Users/sabinonweb/Documents/Projects/buzzinga/src/media/{}-audio.mp4",
        reddit_media.title
    );
    println!(
        "\nCurrent directory: {:?}\n",
        std::env::current_dir().unwrap()
    );

    let (_video_response, _audio_reponse) = join!(
        downloader(
            reddit_media.video_url.clone(),
            reddit_client.clone(),
            video_file_name,
            reddit_media.title.clone()
        ),
        downloader(
            reddit_media.audio_url,
            reddit_client,
            audio_file_name,
            reddit_media.title
        )
    );
}

pub async fn download(dash_urls: Vec<String>) {
    let mut download_tasks = Vec::new();

    let mut j = 0;
    for (_i, url) in dash_urls.iter().enumerate() {
        log::info!("Number: {:?}", j);
        let url = url.clone();
        let file_path = format!(
            "/Users/sabinonweb/Documents/Projects/buzzinga/src/media/clip_{}.mp4",
            j
        );
        j = j + 1;

        download_tasks.push(tokio::task::spawn_blocking(move || {
            println!("Downloading {} -> {}", url, file_path);
            let status = Command::new("ffmpeg")
                .arg("-y")
                .arg("-i")
                .arg(url.clone())
                .arg("-c")
                .arg("copy")
                .arg(&file_path)
                .status()
                .expect("Failed to run ffmpeg");

            if !status.success() {
                eprintln!("Failed downloading {}", url);
            }

            file_path
        }));
    }

    let clip_paths: Vec<String> = futures::future::join_all(download_tasks)
        .await
        .into_iter()
        .filter_map(|res| res.ok())
        .collect();

    let list_path = "/Users/sabinonweb/Documents/Projects/buzzinga/src/media/list.txt";
    let mut list_file = File::create(list_path).unwrap();

    for path in clip_paths {
        writeln!(list_file, "file {}", path).unwrap();
    }

    let output_file = "/Users/sabinonweb/Documents/Projects/buzzinga/src/media/final.mp4";

    let status = Command::new("ffmpeg")
        .arg("-y")
        .arg("-f")
        .arg("concat")
        .arg("-safe")
        .arg("0")
        .arg("-i")
        .arg(list_path)
        .arg("-c")
        .arg("copy")
        .arg(output_file)
        .status()
        .expect("Failed to combine clips");

    if status.success() {
        log::info!("All clips combined into {}", output_file);
    } else {
        log::error!("Failed to combine clips");
    }
}

pub async fn downloader(
    reddit_media_url: String,
    reddit_client: RedditClient,
    file_name: &str,
    title: String,
) {
    println!("Downloader");
    let response = match reddit_client.client.get(&reddit_media_url).send().await {
        Ok(response) => {
            info!("Video {} successfully downloaded!", reddit_media_url);
            Ok(response)
        }
        Err(e) => {
            info!("Error while downloading the video: {}: {}", title, e);
            Err(e)
        }
    };

    let mut file = match File::create(file_name) {
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
