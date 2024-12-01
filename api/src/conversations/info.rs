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

impl<'a> ConversationsQuery for Info<'a> {}

impl<'a> Request for Info<'a> {
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

#[derive(Deserialize, Debug, Clone)]
pub struct Channel {
    pub id: String,
    pub name: String,
    pub is_channel: bool,
    pub is_private: bool,
    pub is_shared: bool,
    pub is_ext_shared: bool,
    pub is_org_shared: bool,
    pub is_archived: bool,
    pub previous_names: Option<Vec<String>>,
    pub num_members: Option<i64>,
    pub created: i64,
    pub updated: i64,
    pub creator: String,

    pub is_im: Option<bool>,
    pub is_mpim: Option<bool>,
    pub name_normalized: Option<String>,
    pub purpose: Option<Purpose>,
    pub user: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Purpose {
    pub value: String,
}
