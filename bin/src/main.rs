use anyhow::Result;
use clap::Parser;
use jiff::{civil::Date, tz::TimeZone, Timestamp};
use slack_client::{conversations, usergroups, users, Response};

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

    /// Get messages
    Messages {
        /// Conversation ID to fetch history for.
        #[arg(required = true)]
        channel: String,
        /// Only messages after this YYYY-MM-DD will be fetched.
        #[arg(required = true)]
        oldest: String,
        /// Only messages before this YYYY-MM-DD will be fetched.
        #[arg(required = true)]
        latest: String,
        /// The IANA time zone database identifiers to use for the timest
        #[arg(long, default_value = "Asia/Tokyo")]
        time_zone: String,
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
    let client = slack_client::ApiClient::new(&token)?;

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
        Command::Messages { ref channel, ref oldest, ref latest, ref time_zone } => {
            let ymd_to_f64 = |s: &str| -> Result<f64> {
                let dt = Date::strptime("%Y-%m-%d", s)?.to_zoned(TimeZone::get(time_zone)?)?;
                Ok(Timestamp::from(dt).as_second() as f64)
            };

            let ts_to_datetime = |s: &str| -> Result<String> {
                let ts = (s.parse::<f64>()? * 1000000f64) as i64; // hacky
                let ts = Timestamp::from_microsecond(ts)?.to_zoned(TimeZone::get(time_zone)?);
                Ok(ts.strftime("%Y-%m-%d %H:%M:%S (%Z)").to_string())
            };

            let messages = client
                .conversations(&conversations::History {
                    channel,
                    oldest: ymd_to_f64(oldest)?,
                    latest: ymd_to_f64(latest)?,
                    limit: 1000,
                    inclusive: true,
                })
                .await?
                .messages;

            if let Some(messages) = messages {
                for m in messages {
                    println!("{}", ts_to_datetime(&m.ts)?);
                    match m.blocks {
                        None => {}
                        Some(blocks) => {
                            blocks.iter().for_each(|block| {
                                println!("{block}");
                            });
                        }
                    }
                }
            }
        }
        _ => unimplemented!(),
    }
    Ok(())
}
