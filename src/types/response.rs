use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RedditDashJsonResponse {
    pub(crate) kind: Option<String>,

    pub(crate) data: Option<RedditData>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RedditData {
    pub(crate) children: Vec<RedditChild>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub(crate) struct RedditChild {
    pub(crate) kind: Option<String>,

    pub(crate) data: RedditChildData,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub(crate) struct RedditChildData {
    pub(crate) title: Option<String>,

    pub(crate) name: Option<String>,

    pub(crate) subreddit_name_prefixed: Option<String>,

    pub(crate) secure_media: Option<SecureMedia>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub(crate) struct SecureMedia {
    pub(crate) reddit_video: Option<RedditVideo>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub(crate) struct RedditVideo {
    // URL of the video
    pub(crate) fallback_url: Option<String>,

    // URL of the audio
    pub(crate) dash_url: Option<String>,

    // URL of the m3u8 playlist file
    pub(crate) hls_url: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub(crate) struct Media {
    pub(crate) subreddit_name: String,

    pub(crate) title: String,

    pub(crate) video_url: String,

    pub(crate) audio_url: String,

    pub(crate) dash_url: String,

    pub(crate) hls_url: String,
}

impl Media {
    pub fn new(reddit_child: RedditChild, audio_url: String) -> Result<Media, String> {
        if let Some(reddit_video) = reddit_child
            .data
            .secure_media
            .expect("No value found while accessing secure media")
            .reddit_video
        {
            Ok(Media {
                subreddit_name: reddit_child
                    .data
                    .name
                    .expect("Subreddit name has NONE value!"),
                title: reddit_child
                    .data
                    .title
                    .expect("Subreddit title has NONE value!"),
                video_url: reddit_video
                    .fallback_url
                    .expect("Fallback URL doesn't exist"),
                audio_url,
                dash_url: reddit_video.dash_url.expect("Dash URL doesn't exist"),
                hls_url: reddit_video.hls_url.expect("HLS URL doesn't exist"),
            })
        } else {
            log::error!("Reddit Video has a NONE value!");
            Err(String::from("Reddit Video has a NONE value"))
        }
    }
}
