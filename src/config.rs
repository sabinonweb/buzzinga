use crate::args::Args;
use anyhow::{Context, Error};
use roux::Reddit;

pub(crate) async fn config_client(args: &Args) -> anyhow::Result<roux::me::Me> {
    dotenv::from_path(&args.env).context("Error while reading file path of dotenv file!")?;

    macro_rules! env_variable {
        ($key:ident) => {
            std::env::var(stringify!($key))
                .context(concat!(stringify!($key), " not found in dotenv config."))?
        };
    }

    let client_username = env_variable!(CLIENt_USERNAME);
    let client_id = env_variable!(CLIENT_ID);
    let client_secret = env_variable!(CLIENT_SECRET);
    let user_agent = env_variable!(USER_AGENT);

    // Equivalent to: Not exactly but just a basic idea of it
    // reqwest::post("https://www.reddit.com/api/v1/access_token").content_type('application/x-www-form-urlencoded')
    // .header(user_agent).auth(username, password);
    match Reddit::new(&user_agent, &client_id, &client_secret)
        .username(&client_username)
        .password(&client_secret)
        .login()
        .await
    {
        Ok(me) => Ok(me),
        Err(err) => {
            log::error!("Error occured while sending an auth request: {}", err);
            Err(Error::new(err).context("Error occured while sending an auth request!"))
        }
    }
}
