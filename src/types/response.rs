use roux::{response::BasicThing, submission::SubmissionData};
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

    // pub id: i32,
    pub media_id: String,

    pub outbound_url: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SecureMedia {
    reddit_video: Option<RedditVideo>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RedditVideo {
    #[serde(rename = "subreddit")]
    pub subreddit_name: Option<String>,

    pub link_flair_text: Option<String>,

    #[serde(rename = "id")]
    pub post_id: String,

    pub over_18: bool,

    #[serde(rename = "name")]
    pub kind_of_reddit_entity: String,

    #[serde(rename = "url")]
    pub url_of_the_post: Option<String>,

    #[serde(rename = "title")]
    pub title_of_the_post: String,
}

impl From<BasicThing<SubmissionData>> for RedditVideo {
    fn from(value: BasicThing<SubmissionData>) -> Self {
        Self {
            subreddit_name: Some(value.data.subreddit),
            link_flair_text: value.data.link_flair_text,
            post_id: value.data.id,
            over_18: value.data.over_18,
            kind_of_reddit_entity: value.data.name,
            url_of_the_post: value.data.url,
            title_of_the_post: value.data.title,
        }
    }
}
