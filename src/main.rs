#![allow(dead_code)]

use anyhow::{Context, Ok};
use chrono::Utc;
use clap::Parser;
use cron::Schedule;
use merge::{merge_content, read_from_source};
use reddit::scrape_for_content;
use std::{
    str::FromStr,
    sync::{Arc, Mutex},
    thread,
};
use types::{args::Args, config_types::RedditClient};

mod config;
mod data;
mod downloader;
mod merge;
mod reddit;
mod types;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    //sec  min   hour   day of month   month   day of week   year
    let expression = "0 0 9,12,15,18,21 * * *";
    let schedule = Schedule::from_str(expression).context("Failed to parse CRON expression")?;
    log::info!("Firing cronjob for buzzinga!");

    log4rs::init_file(&args.log_config, Default::default())?;
    log::info!("Fetching the environment variables!");
    //
    // // loop {
    // //     if let Some(datetime) = schedule.upcoming(Utc).take(1).next() {
    // //         let time_frame = datetime - Utc::now();
    // //         thread::sleep(time_frame.to_std().unwrap());
    // //         let reddit_client = Arc::new(Mutex::new(RedditClient::new(&args).await?));
    // //         scrape_for_content(reddit_client, "absurd_content".to_string()).await?;
    // //     }
    // // }
    let reddit_client = Arc::new(Mutex::new(RedditClient::new(&args).await?));
    let reddit_content =
        scrape_for_content(reddit_client.clone(), "absurd_content".to_string()).await?;
    // merge_content(reddit_content)?;
    let one = String::from("https://www.reddit.com/r/WTF/comments/1igo931/step_ladder/");
    let twp = String::from("https://v.redd.it/lfqpncoygple1");
    read_from_source(one, twp, reddit_client.clone()).await;

    Ok(())
}

// extern crate ffmpeg_next as ffmpeg;
//
// use std::env;
//
// fn main() {
//     ffmpeg::init().unwrap();
//
//     for arg in env::args().skip(1) {
//         if let Some(codec) = ffmpeg::decoder::find_by_name(&arg) {
//             println!("type: decoder");
//             println!("\t id: {:?}", codec.id());
//             println!("\t name: {}", codec.name());
//             println!("\t description: {}", codec.description());
//             println!("\t medium: {:?}", codec.medium());
//             println!("\t capabilities: {:?}", codec.capabilities());
//
//             if let Some(profiles) = codec.profiles() {
//                 println!("\t profiles: {:?}", profiles.collect::<Vec<_>>());
//             } else {
//                 println!("\t profiles: none");
//             }
//
//             if let Ok(video) = codec.video() {
//                 if let Some(rates) = video.rates() {
//                     println!("\t rates: {:?}", rates.collect::<Vec<_>>());
//                 } else {
//                     println!("\t rates: any");
//                 }
//
//                 if let Some(formats) = video.formats() {
//                     println!("\t formats: {:?}", formats.collect::<Vec<_>>());
//                 } else {
//                     println!("\t formats: any");
//                 }
//             }
//
//             if let Ok(audio) = codec.audio() {
//                 if let Some(rates) = audio.rates() {
//                     println!("\t audio_rates: {:?}", rates.collect::<Vec<_>>());
//                 } else {
//                     println!("\t audio_rates: any");
//                 }
//
//                 if let Some(formats) = audio.formats() {
//                     println!("\t audio_formats: {:?}", formats.collect::<Vec<_>>());
//                 } else {
//                     println!("\t audio_formats: any");
//                 }
//
//                 if let Some(layouts) = audio.channel_layouts() {
//                     println!("\t channel_layouts: {:?}", layouts.collect::<Vec<_>>());
//                 } else {
//                     println!("\t channel_layouts: any");
//                 }
//             }
//         }
//     }
// }
