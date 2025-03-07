use roux::{response::BasicThing, submission::SubmissionData};
use serde::{Deserialize, Serialize};

use crate::{data::IMAGE_FORMATS, types::args::RedditConfig};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RedditContent {
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

impl From<BasicThing<SubmissionData>> for RedditContent {
    fn from(value: BasicThing<SubmissionData>) -> Self {
        Self {
            subreddit_name: Some(value.data.subreddit.clone()),
            link_flair_text: value.data.link_flair_text.clone(),
            post_id: value.data.id.clone(),
            over_18: value.data.over_18.clone(),
            kind_of_reddit_entity: value.data.name.clone(),
            url_of_the_post: value.data.url.clone(),
            title_of_the_post: value.data.title.clone(),
            upvote_ratio: value.data.upvote_ratio.clone(),
            number_of_comments: value.data.num_comments.clone(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RedditResponse {
    pub reddit_content: RedditContent,

    pub dash_url: Option<String>,

    pub video_url: Option<String>,

    pub audio_url: Option<String>,
}

pub struct AbsurdContent;
pub struct GamingContent;
pub struct GeneralMemes;
pub struct NicheMemes;
pub struct FailContent;
pub struct PerfectlyTimed;
pub struct SportsContent;
pub struct Miscellaneous;

pub trait Filtration {
    fn filter_content(response_collection: Vec<BasicThing<SubmissionData>>) -> Vec<RedditContent> {
        let mut reddit_videos: Vec<RedditContent> = Vec::new();

        for mut response in response_collection.into_iter() {
            response.data.url = Some(Self::build_url(&response));
            if Self::filter_image_formats(&response)
                || Self::filter_gallery_formatted_urls(&response)
            {
                continue;
            }

            let video = RedditContent::from(response);
            println!("{:?}\n", video);

            reddit_videos.push(video);
        }

        reddit_videos
    }

    fn filter_image_formats(response: &BasicThing<SubmissionData>) -> bool {
        IMAGE_FORMATS
            .iter()
            .any(|img_format| response.data.url.clone().unwrap().ends_with(img_format))
    }

    fn filter_gallery_formatted_urls(response: &BasicThing<SubmissionData>) -> bool {
        response
            .data
            .url
            .clone()
            .unwrap()
            .split("/")
            .collect::<Vec<&str>>()[3]
            == "gallery"
    }

    fn filter_comment_urls(response: &BasicThing<SubmissionData>) -> bool {
        let comment_url = response.data.url.as_ref().unwrap();

        let split_url = comment_url.split("/").collect::<Vec<&str>>();

        if split_url.len() > 5 {
            split_url[5] == "comments"
        } else {
            false
        }
    }

    fn build_url(response: &BasicThing<SubmissionData>) -> String {
        format!(
            "https://www.reddit.com/r/{}/comments/{}/.json",
            response.data.subreddit, response.data.id
        )
    }
}

impl Filtration for AbsurdContent {}
impl Filtration for GamingContent {}
impl Filtration for FailContent {}
impl Filtration for PerfectlyTimed {}
impl Filtration for SportsContent {}

#[derive(Clone, Debug)]
pub enum RedditContentType {
    AbsurdContent,

    GamingContent,

    // Content
    GeneralMemes,

    // Content
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
