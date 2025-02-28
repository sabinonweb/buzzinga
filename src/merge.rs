use ffmpeg_next as ffmpeg;

use crate::types::reddit_types::RedditContent;

pub fn merge_content(reddit_content: Vec<RedditContent>) -> anyhow::Result<()> {
    ffmpeg::init().unwrap();

    match ffmpeg::format::input("https://v.redd.it/b8d3zshxbjrd1") {
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
