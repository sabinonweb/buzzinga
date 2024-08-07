use crate::types::utils_types::LinkType;

pub(crate) fn post_type(url: String) -> anyhow::Result<LinkType> {
    let url = url.split("/").collect::<Vec<&str>>();
    println!("url: {:?}", url);

    match url[2] {
        "i.redd.it" => Ok(LinkType::ReddIt(url[3].to_string())),
        "v.redd.it" => {
            Ok(differentiate_video_and_images(LinkType::ReddIt(url[3].to_string())).unwrap())
        }
        "www.reddit.com" => {
            if reddit_com(&url[3])? {
                Ok(LinkType::Gallery)
            } else {
                Ok(LinkType::RedditCom)
            }
        }

        _ => return Ok(LinkType::None),
    }
}

fn differentiate_video_and_images(link_type: LinkType) -> Option<LinkType> {
    match link_type {
        LinkType::ReddIt(url_segment) => {
            if url_segment.ends_with(".jpeg") {
                Some(LinkType::Gallery)
            } else {
                Some(LinkType::Video)
            }
        }
        _ => None,
    }
}

fn reddit_com(url_segment: &str) -> anyhow::Result<bool> {
    match url_segment {
        "gallery" => Ok(true),
        _ => Ok(false),
    }
}
