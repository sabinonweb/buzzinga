use crate::args::{AppConfig, Args, RedditConfig};
use anyhow::{Context, Error, Ok};
use reqwest::{
    header::{HeaderMap, AUTHORIZATION, CONTENT_TYPE, USER_AGENT},
    Client, StatusCode,
};
use serde::Deserialize;
use std::{collections::HashMap, fs::File, io::Read};

const REDDIT_TOKEN_URL: &str = "https://www.reddit.com/api/v1/access_token";

macro_rules! env_variable {
    ($key:ident) => {
        std::env::var(stringify!($key))
            .context(concat!(stringify!($key), " not found in dotenv config."))?
    };
}

#[derive(Debug, Clone, Deserialize)]
pub struct AccessToken {
    access_token: String,
}

/// Redis OAuth client for interacting with reddit API
pub struct RedditClient {
    // reqwest client for interacting with the reddit API
    client: Client,

    // access token that is received after authorization
    token: AccessToken,

    // user agent in the standard format!
    user_agent: String,
}

impl RedditClient {
    /// For creation of oauth client, `Resource Owner Password Credentials Grant` is used
    pub(crate) async fn new(args: &Args) -> anyhow::Result<RedditClient> {
        let reddit_config = config(&args)?;

        let token = get_token(&reddit_config).await?;
        let mut headers = HeaderMap::new();
        headers.insert(AUTHORIZATION, token.access_token.parse()?);
        headers.insert(USER_AGENT, reddit_config.user_agent.parse()?);

        let client = Client::builder().default_headers(headers).build()?;

        Ok(RedditClient {
            client,
            token,
            user_agent: reddit_config.user_agent,
        })
    }
}

async fn get_token(reddit_config: &RedditConfig) -> anyhow::Result<AccessToken> {
    let basic_header = format!(
        "basic {}, {}",
        &reddit_config.client_id, &reddit_config.client_secret,
    );

    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION, basic_header.parse()?);
    headers.insert(USER_AGENT, reddit_config.user_agent.parse()?);
    headers.insert(CONTENT_TYPE, "application/x-www-form-urlencoded".parse()?);

    let mut params = HashMap::new();
    params.insert("grant_type", "password");
    params.insert("username", &reddit_config.client_username);
    params.insert("password", &reddit_config.client_password);

    let response = Client::new()
        .post(REDDIT_TOKEN_URL)
        .headers(headers)
        .form(&params)
        .send()
        .await?;

    match response.status() {
        StatusCode::OK => Ok(response.json::<AccessToken>().await?),
        _ => {
            log::error!("Response couldn't succeed! Error while reading response json!");
            Err(Error::msg("Status: Internal Server Error")
                .context("Response couldn't succeed! Error while reading response json!"))
        }
    }
}

pub(crate) fn config(args: &Args) -> anyhow::Result<RedditConfig> {
    dotenv::from_path(&args.dotenv).context("Error while reading file path of dotenv file!")?;

    let app_config: AppConfig = {
        let file_path = &args.config;
        let mut config_file =
            File::open(file_path).context("File path for the config file not found!")?;
        let mut file_content = String::new();

        config_file
            .read_to_string(&mut file_content)
            .context("Error occured while reading file contents!")?;

        toml::from_str(&file_content.as_str())?
    };

    let client_username = env_variable!(CLIENT_USERNAME);
    let client_id = env_variable!(CLIENT_ID);
    let client_secret = env_variable!(CLIENT_SECRET);
    let client_password = env_variable!(CLIENT_PASSWORD);
    let user_agent = format!(
        "{}:{}:v.{} (by {client_username})",
        &app_config.environment, &app_config.application_name, &app_config.version
    );

    Ok(RedditConfig {
        client_id,
        client_username,
        client_password,
        client_secret,
        user_agent,
    })
}

pub(crate) async fn config_client(args: &Args) -> anyhow::Result<()> {
    // Equivalent to: Not exactly but just a basic idea of it
    // reqwest::post("https://www.reddit.com/api/v1/access_token").content_type('application/x-www-form-urlencoded')
    // .header(user_agent).auth(username, password);
    // match Reddit::new(&user_agent, &client_id, &client_secret)
    //     .username(&client_username)
    //     .password(&client_password)
    //     .login()
    //     .await
    // {
    //     Ok(me) => Ok(me),
    //     Err(err) => {
    //         log::error!("Error occured while sending an auth request: {}", err);
    //         Err(Error::new(err).context("Error occured while sending an auth request!"))
    //     }
    // }

    Ok(())
}
