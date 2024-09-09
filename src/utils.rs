use crate::{data::IMAGE_FORMATS, types::reddit_types::RedditVideo};
use roux::{response::BasicThing, submission::SubmissionData};

pub(crate) fn filter_content(
    response_collection: &Vec<BasicThing<SubmissionData>>,
) -> Vec<RedditVideo> {
    let mut reddit_videos: Vec<RedditVideo> = Vec::new();

    for response in response_collection {
        if filter_image_formats(response) {
            continue;
        }

        if filter_gallery_formatted_urls(response) {
            continue;
        }

        let video = RedditVideo::from(response.clone());
        println!("{:?}\n", video);

        reddit_videos.push(video);
    }

    reddit_videos
}

pub(crate) fn filter_image_formats(response: &BasicThing<SubmissionData>) -> bool {
    IMAGE_FORMATS
        .iter()
        .any(|img_format| response.clone().data.url.unwrap().ends_with(img_format))
}

pub(crate) fn filter_gallery_formatted_urls(response: &BasicThing<SubmissionData>) -> bool {
    response
        .clone()
        .data
        .url
        .unwrap()
        .split("/")
        .collect::<Vec<&str>>()[3]
        == "gallery"
}
