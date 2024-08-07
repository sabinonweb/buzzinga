use crate::types::{
    args::{AppConfig, Args, RedditConfig},
    config_types::{AccessToken, RedditClient},
};
use anyhow::{Context, Error, Ok};
use base64::{engine::general_purpose, Engine};
use reqwest::{
    header::{HeaderMap, AUTHORIZATION, CONTENT_TYPE, USER_AGENT},
    Client,
};
use std::{collections::HashMap, fs::File, io::Read};

const REDDIT_TOKEN_URL: &str = "https://www.reddit.com/api/v1/access_token";

macro_rules! env_variable {
    ($key:ident) => {
        std::env::var(stringify!($key))
            .context(concat!(stringify!($key), " not found in dotenv config."))?
    };
}

impl RedditClient {
    /// For creation of oauth client, `Resource Owner Password Credentials Grant` is used
    pub(crate) async fn new(args: &Args) -> anyhow::Result<RedditClient> {
        let reddit_config = config(&args)?;

        let token = get_token(&reddit_config).await?;

        let reddit_instance = roux::Reddit::new(
            &reddit_config.user_agent,
            &reddit_config.client_id,
            &reddit_config.client_secret,
        )
        .username(&reddit_config.client_username)
        .password(&reddit_config.client_password);

        Ok(RedditClient {
            reddit: reddit_instance,
            token: token.access_token,
            user_agent: reddit_config.user_agent,
        })
    }
}

async fn get_token(reddit_config: &RedditConfig) -> anyhow::Result<AccessToken> {
    let basic_header = format!(
        "Basic {}",
        general_purpose::STANDARD.encode(format!(
            "{}:{}",
            &reddit_config.client_id, &reddit_config.client_secret
        ))
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
        .form(&[
            ("grant_type", "password"),
            ("username", &reddit_config.client_username),
            ("password", &reddit_config.client_password),
        ])
        .send()
        .await
        .context("Error while sending a authorization request for Password grant!")?;

    if response.status().is_success() {
        let token: AccessToken = serde_json::from_str(&response.text().await?)?;
        Ok(token)
    } else {
        Err(Error::msg(format!(
            "Failed to fetch access token. Status: {}",
            response.status()
        )))
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

        let config: AppConfig = toml::from_str(&file_content.as_str())
            .context("Error occurred while deserializing TOML content into AppConfig")?;

        config
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
