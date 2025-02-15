use serde::{Deserialize, Serialize};

use crate::{request::Request, response::Response, usergroups::UsergroupsQuery};

/// A request for `usergroups.users.list` API.
///
/// See: https://api.slack.com/methods/usergroups.users.list
#[derive(Serialize, Debug, Clone)]
pub struct Users {
    /// The ID of the usergroup.
    #[serde(rename = "usergroup")]
    pub id: String,
}

impl UsergroupsQuery for Users {}

impl Request for Users {
    type Response = UsergroupsUsers;

    fn path(&self) -> &'static str {
        "usergroups.users.list"
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct UsergroupsUsers {
    pub ok: bool,
    pub users: Option<Vec<String>>,
}
impl Response for UsergroupsUsers {
    fn is_ok(&self) -> bool {
        self.ok
    }
}
