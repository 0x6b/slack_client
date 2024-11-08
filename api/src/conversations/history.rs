use mrkdwn2markdown::Block;
use serde::{Deserialize, Serialize};

use crate::{conversations::ConversationsQuery, request::Request, response::Response};

/// A request for `conversations.history` API.
///
/// See: https://api.slack.com/methods/conversations.history
#[derive(Serialize, Debug, Clone)]
pub struct History<'a> {
    /// Conversation ID to fetch history for.
    pub channel: &'a str,
    /// Only messages before this Unix timestamp will be included in results.
    pub latest: f64,
    /// Only messages after this Unix timestamp will be included in results.
    pub oldest: f64,
    /// The maximum number of items to return. Fewer than the requested number of items may be
    /// returned, even if the end of the conversation history hasn't been reached. Maximum of 999.
    pub limit: u64,
    /// Include messages with `oldest` or `latest` timestamps in results. Ignored unless either
    /// timestamp is specified.
    pub inclusive: bool,
}

impl<'a> ConversationsQuery for History<'a> {}

impl<'a> Request for History<'a> {
    type Response = Conversations;

    fn path(&self) -> &'static str {
        "conversations.history"
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct Conversations {
    pub ok: bool,
    pub messages: Option<Vec<Message>>,
}

impl Response for Conversations {
    fn is_ok(&self) -> bool {
        self.ok
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct Message {
    /// User ID of the author.
    pub user: Option<String>,
    /// bot ID of the author.
    pub bot_id: Option<String>,
    /// The text of the message.
    pub text: Option<String>,
    /// The Slack block kit blocks of the message.
    pub blocks: Option<Vec<Block>>,
    /// Timestamp of the message.
    pub ts: String,
}
