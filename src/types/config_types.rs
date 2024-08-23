use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct AccessToken {
    pub(crate) access_token: String,

    pub(crate) token_type: String,

    pub(crate) expires_in: i64,

    pub(crate) scope: String,
}

/// Redis OAuth client for interacting with reddit API
#[derive(Clone)]
pub struct RedditClient {
    // reddit instance for interacting with reddit API
    pub(crate) reddit: roux::Reddit,

    // reddit client for interacting with the reddit API
    pub(crate) me: roux::Me,

    // access token that is received after authorization
    pub(crate) token: String,

    // user agent in the standard format!
    pub(crate) user_agent: String,
}
