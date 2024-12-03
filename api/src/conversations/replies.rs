use serde::Serialize;

use crate::{
    conversations::{history::Conversations, ConversationsQuery},
    request::Request,
};

/// A request for `conversations.replies` API.
///
/// See: https://api.slack.com/methods/conversations.replies
#[derive(Serialize, Debug, Clone)]
pub struct Replies<'a> {
    /// Conversation ID to fetch thread from.
    pub channel: &'a str,
    /// Unique identifier of either a thread’s parent message or a message in the thread. `ts` must
    /// be the timestamp of an existing message with 0 or more replies. If there are no replies
    /// then just the single message referenced by `ts` will return - it is just an ordinary,
    /// unthreaded message.
    pub ts: f64,
    /// Only messages before this Unix timestamp will be included in results.
    pub latest: f64,
    /// Only messages after this Unix timestamp will be included in results.
    pub oldest: f64,
    /// The maximum number of items to return. Fewer than the requested number of items may be
    /// returned, even if the end of the users list hasn't been reached.
    pub limit: u64,
    /// Include messages with `oldest` or `latest` timestamps in results. Ignored unless either
    /// timestamp is specified.
    pub inclusive: bool,
}

impl ConversationsQuery for Replies<'_> {}

impl Request for Replies<'_> {
    type Response = Conversations;

    fn path(&self) -> &'static str {
        "conversations.replies"
    }
}
