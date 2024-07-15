use clap::Parser;
use serde::Deserialize;

/// Parser takes the cli arguments and parses them to Args type
#[derive(Debug, Parser, Clone)]
#[command(about, version)]
pub struct Args {
    /// The path of Config.toml for default application configs
    #[arg(short, long, default_value = "Config.toml")]
    pub config: String,

    /// The path of the environment variable file
    #[arg(short, long, default_value = ".env")]
    pub dotenv: String,

    /// The path of the log file
    #[arg(short, long, default_value = "log_config.yml")]
    pub log_config: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct AppConfig {
    pub application_name: String,

    pub environment: String,

    pub version: String,
}

// let client_username = env_variable!(CLIENT_USERNAME);
//   let client_id = env_variable!(CLIENT_ID);
//   let client_secret = env_variable!(CLIENT_SECRET);
//   let client_password = env_variable!(CLIENT_PASSWORD);
//   let user_agent = format!

#[derive(Debug, Clone)]
pub struct RedditConfig {
    // The id of client retrieved from .env
    pub client_id: String,

    // The username of client retrieved from .env
    pub client_username: String,

    // The password of client retrieved from .env
    pub client_password: String,

    // The secret of the applcation/script
    pub client_secret: String,

    // User agent in the format {environment}:{application}:v.{version} (by {client_username}
    pub user_agent: String,
}
