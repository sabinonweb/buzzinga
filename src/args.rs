use clap::Parser;

/// Parser takes the cli arguments and parses them to Args type
#[derive(Debug, Parser, Clone)]
#[command(about, version)]
pub struct Args {
    /// The path of the environment variable file
    #[arg(short, long, default_value = ".env")]
    pub env: String,

    /// The path of the log file
    #[arg(short, long, default_value = "log_config.yml")]
    pub log_config: String,
}
