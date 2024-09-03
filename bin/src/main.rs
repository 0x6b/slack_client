use anyhow::Result;
use clap::Parser;
use slack_client::{
    request::{usergroups, users},
    response::Response,
};

#[derive(Parser)]
pub struct Args {
    /// Slack API token.
    #[arg(long, env = "SLACK_TOKEN")]
    pub token: String,

    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Parser)]
pub enum Command {
    /// Dump a list of Slack channels as JSON
    Channels {
        /// Only list channels created by a specified username
        #[arg(long)]
        creator: Option<String>,
    },

    /// Dump last message of a given channel as JSON
    ChannelLastMessage {
        /// Channel ID
        #[arg(required = true)]
        channel: String,
    },

    /// Dump a list of Slack user groups as JSON
    Usergroups,

    /// Dump a list of user IDs for a given user group as JSON
    UsergroupUsers {
        /// User group ID
        #[arg(required = true)]
        usergroup: String,
    },

    /// Dump users
    Users,
}

#[tokio::main]
async fn main() -> Result<()> {
    let Args { token, command } = Args::parse();
    let client = slack_client::Client::new(&token)?;

    match command {
        Command::Users => {
            let mut results = vec![];
            let mut request = users::List { cursor: None, limit: Some(1000) };

            loop {
                let users = client.users(&request).await?;
                let cursor = users.next_cursor();

                if let Some(members) = users.members {
                    results.extend(members)
                }

                if cursor.is_some() {
                    request.cursor = cursor;
                } else {
                    break;
                }
            }
            println!("{:#?}", results);
        }
        Command::Usergroups => {
            let usergroups = client.usergroups(&usergroups::List {}).await?;
            println!("{:#?}", usergroups);
        }
        _ => unimplemented!(),
    }
    Ok(())
}
