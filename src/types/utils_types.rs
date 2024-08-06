/// The types of posts that are posted in reddit
#[derive(Debug)]
pub(crate) enum PostType {
    // Image in jpeg format(mostly)
    Image,

    // Mostly question answers or opinions or confessions without visual aid
    Text,

    // Videos posts of the subreddit
    Video,
}

#[derive(Debug)]
pub(crate) enum LinkType {
    // https://www.reddit.com/r/ (Text posts mostly)
    RedditCom,

    // https://i/v.redd.it/ (Images and Videos)
    ReddIt(String),

    // https://www.reddit.com/gallery (Images)
    Gallery,

    None,
}
