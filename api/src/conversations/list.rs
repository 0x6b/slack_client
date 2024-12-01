use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::{
    conversations::{info::Channel, ConversationsQuery},
    request::Request,
    response::{Response, ResponseMetadata},
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChannelTypes {
    inner: Vec<ChannelType>,
}

impl From<Vec<ChannelType>> for ChannelTypes {
    fn from(inner: Vec<ChannelType>) -> Self {
        Self { inner }
    }
}

/// Channel types.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ChannelType {
    Public,
    Private,
    Mpim,
    Im,
}

impl From<&str> for ChannelType {
    fn from(value: &str) -> Self {
        match value {
            "public_channel" => ChannelType::Public,
            "private_channel" => ChannelType::Private,
            "mpim" => ChannelType::Mpim,
            "im" => ChannelType::Im,
            _ => panic!("Invalid channel type: {}", value),
        }
    }
}

impl AsRef<str> for ChannelType {
    fn as_ref(&self) -> &str {
        match self {
            ChannelType::Public => "public_channel",
            ChannelType::Private => "private_channel",
            ChannelType::Mpim => "mpim",
            ChannelType::Im => "im",
        }
    }
}

impl Display for ChannelTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner
            .iter()
            .map(|t| t.as_ref())
            .collect::<Vec<&str>>()
            .join(",")
            .fmt(f)
    }
}

/// A request for `conversations.list` API.
///
/// See: https://api.slack.com/methods/conversations.list
#[derive(Serialize, Debug, Clone)]
pub struct List {
    /// Set to true to exclude archived channels from the list.
    pub exclude_archived: Option<bool>,
    /// Mix and match channel types by providing a comma-separated list of any combination of
    /// public_channel, private_channel, mpim, im
    #[serde(serialize_with = "from_channel_types")]
    pub types: Option<ChannelTypes>,
    /// Paginate through collections of data by setting the cursor parameter to a next_cursor
    /// attribute returned by a previous request's response_metadata. Default value fetches the
    /// first "page" of the collection. See pagination for more detail.
    pub cursor: Option<String>,
    /// The maximum number of items to return. Fewer than the requested number of items may be
    /// returned, even if the end of the channel list hasn't been reached.
    pub limit: Option<u64>,
}

fn from_channel_types<S>(value: &Option<ChannelTypes>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    match value {
        Some(v) => v
            .inner
            .iter()
            .map(|t| t.as_ref())
            .collect::<Vec<&str>>()
            .join(",")
            .serialize(serializer),
        None => serializer.serialize_none(),
    }
}

impl ConversationsQuery for List {}

impl Request for List {
    type Response = ConversationsList;

    fn path(&self) -> &'static str {
        "conversations.list"
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct ConversationsList {
    pub ok: bool,
    pub channels: Option<Vec<Channel>>,
    pub response_metadata: Option<ResponseMetadata>,
}

impl Response for ConversationsList {
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
