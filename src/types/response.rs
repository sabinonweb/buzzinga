use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct RedditReponse {
    pub data: Vec<PostInformation>,

    pub kind: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct PostInformation {
    pub id: i32,

    pub author: String,

    pub content_category: Option<String>,

    pub discussion_type: Option<String>,

    pub is_original_content: bool,

    pub is_video: bool,

    pub secure_media: Option<SecureMedia>,

    pub self_text: String,

    pub subreddit: String,

    pub title: String,

    pub url: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct GalleryData {
    items: Vec<GalleryItems>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GalleryItems {
    pub caption: Option<String>,

    pub id: i32,

    pub media_id: String,

    pub outbound_url: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SecureMedia {
    reddit_video: Vec<RedditVideo>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RedditVideo {
    pub bitrate_kbps: i32,

    pub dash_url: String,

    pub duration: std::time::Duration,

    pub has_audio: bool,

    pub height: i32,

    pub hls_url: String,

    pub is_gif: bool,

    pub width: i32,

    pub scrubber_media_url: String,
}
