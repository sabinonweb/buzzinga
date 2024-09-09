use roux::{response::BasicThing, submission::SubmissionData};
use serde::{Deserialize, Serialize};

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

    pub upvote_ratio: f64,

    #[serde(rename = "num_comments")]
    pub number_of_comments: u64,
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
            upvote_ratio: value.data.upvote_ratio,
            number_of_comments: value.data.num_comments,
        }
    }
}

#[derive(Clone, Debug)]
pub enum RedditContentType {
    AbsurdContent,

    GamingContent,

    GeneralMemes,

    NicheMemes,

    FailContent,

    PerfectlyTimed,

    SportsContent,

    Miscellaneous,
}

impl<T> From<T> for RedditContentType
where
    T: AsRef<str> + ToString,
{
    fn from(value: T) -> Self {
        let content_type = value.as_ref();

        match content_type {
            "absurd_content" => RedditContentType::AbsurdContent,
            "gaming_content" => RedditContentType::GamingContent,
            "general_memes" => RedditContentType::GeneralMemes,
            "niche_memes" => RedditContentType::NicheMemes,
            "fail_content" => RedditContentType::FailContent,
            "perfectly_timed" => RedditContentType::PerfectlyTimed,
            "sports_content" => RedditContentType::SportsContent,
            _ => RedditContentType::Miscellaneous,
        }
    }
}
