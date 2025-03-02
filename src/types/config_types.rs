use std::collections::HashMap;

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

    // OAuth instance of client for requesting .MPD because roux itself seemed to not allow this
    pub(crate) client: reqwest::Client,

    // params for the OAuth client
    pub(crate) params: HashMap<String, String>,

    // access token that is received after authorization
    pub(crate) token: String,

    // user agent in the standard format!
    pub(crate) user_agent: String,
}
