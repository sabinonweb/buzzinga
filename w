use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct RedditReponse {
    pub data: PostInformation,

    pub kind: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct PostInformation {
    pub id: String,

    pub author: String,

    #[serde(rename = "category")]
    pub content_category: Option<String>,

    pub discussion_type: Option<String>,

    pub is_original_content: bool,

    pub is_video: bool,

    pub secure_media: Option<SecureMedia>,

    #[serde(rename = "selftext")]
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
    reddit_video: Option<RedditVideo>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RedditVideo {
    pub bitrate_kbps: Option<i32>,
    pub dash_url: Option<String>,
    pub duration: Option<f64>,
    pub has_audio: Option<bool>,
    pub height: Option<i32>,
    pub hls_url: Option<String>,
    pub is_gif: Option<bool>,
    pub width: Option<i32>,
    pub scrubber_media_url: Option<String>,}
