use mrkdwn2markdown::Block;
use serde::{Deserialize, Serialize};

use crate::{
    conversations::ConversationsQuery,
    request::Request,
    response::{Response, ResponseMetadata},
};

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
    /// Paginate through collections of data by setting the cursor parameter to a next_cursor
    /// attribute returned by a previous request's response_metadata. Default value fetches the
    /// first "page" of the collection. See pagination for more detail.
    pub cursor: Option<String>,
}

impl ConversationsQuery for History<'_> {}

impl Request for History<'_> {
    type Response = Conversations;

    fn path(&self) -> &'static str {
        "conversations.history"
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct Conversations {
    pub ok: bool,
    pub messages: Option<Vec<Message>>,
    pub response_metadata: Option<ResponseMetadata>,
}

impl Response for Conversations {
    fn is_ok(&self) -> bool {
        self.ok
    }

    fn next_cursor(&self) -> Option<String> {
        self.response_metadata.as_ref().and_then(|m| {
            if m.next_cursor.is_empty() {
                return None;
            }
            Some(m.next_cursor.clone())
        })
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
