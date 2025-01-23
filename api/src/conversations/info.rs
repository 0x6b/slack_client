use serde::{Deserialize, Serialize};

use crate::{conversations::ConversationsQuery, request::Request, response::Response};

/// A request for `conversations.info` API.
///
/// See: https://api.slack.com/methods/conversations.info
#[derive(Serialize, Debug, Clone)]
pub struct Info<'a> {
    /// Conversation ID to learn more about.
    pub channel: &'a str,
}

impl ConversationsQuery for Info<'_> {}

impl Request for Info<'_> {
    type Response = ConversationsInfo;

    fn path(&self) -> &'static str {
        "conversations.info"
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct ConversationsInfo {
    pub ok: bool,
    pub channel: Option<Channel>,
}

impl Response for ConversationsInfo {
    fn is_ok(&self) -> bool {
        self.ok
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Channel {
    pub id: String,
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_channel: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_private: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_shared: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_ext_shared: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_org_shared: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_archived: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_names: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_members: Option<i64>,
    pub created: i64,
    pub updated: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_im: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_mpim: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_normalized: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purpose: Option<Purpose>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Purpose {
    pub value: String,
}
