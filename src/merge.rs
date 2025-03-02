use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, Write},
    net::TcpStream,
    sync::{Arc, Mutex},
};

use base64::write;
use bytes::Bytes;
use ffmpeg_next::{self as ffmpeg, codec::context};
use reqwest::Client;
use tokio::io::{AsyncRead, BufWriter};

use crate::types::{config_types::RedditClient, reddit_types::RedditContent};

pub fn merge_content(reddit_content: Vec<RedditContent>) -> anyhow::Result<()> {
    ffmpeg::init().unwrap();

    match ffmpeg::format::input("https://v.redd.it/six0gvyv6xge1") {
        Ok(context) => {
            for (k, v) in context.metadata().iter() {
                println!("{}: {}", k, v);
            }

            if let Some(stream) = context.streams().best(ffmpeg::media::Type::Video) {
                println!("Best video stream index: {}", stream.index());
            }

            if let Some(stream) = context.streams().best(ffmpeg::media::Type::Audio) {
                println!("Best audio stream index: {}", stream.index());
            }

            if let Some(stream) = context.streams().best(ffmpeg::media::Type::Subtitle) {
                println!("Best subtitle stream index: {}", stream.index());
            }
        }

        Err(error) => println!("error: {}", error),
    }

    Ok(())
}

pub async fn read_from_source(
    reddit_content1: String,
    reddit_content2: String,
    reddit_client: Arc<Mutex<RedditClient>>,
) {
    println!("Here we are\n");
    let byte_stream = reddit_client
        .lock()
        .unwrap()
        .me
        .client
        .get(reddit_content1)
        .send()
        .await
        .expect("GET request for content! failed");

    // let pretty_json = serde_json::to_string_pretty(&byte_stream)
    // .expect("Error converting the byte_stream to pretty json!");
    // println!("ByteStream: {:?}", byte_stream);
    //     .bytes()
    //     .await
    //     .expect("GET request for content! failed");
    // let mut f = File::create("../../teamm.mpeg").expect("Failed to create teamm");
    // f.write_all(&byte_stream).unwrap();
    // let byte_stream_2 = Client::new()
    //     .get(reddit_content2)
    //     .send()
    //     .await
    //     .expect("GET request for content! failed")
    //     .bytes()
    //     .await
    //     .expect("GET request for content! failed");
    // let mut fu = File::create("../../hello.mp4").expect("Failed to create Hello");
    // fu.write_all(&byte_stream_2).unwrap();
    //
    // let mut file = File::create("./videoo.mp4").expect("Failed to create Video");
    // let write = file.write_all(&byte_stream).unwrap();
    // let w2 = file.write_all(&byte_stream_2).unwrap();
}
