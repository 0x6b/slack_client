use serde::{Deserialize, Serialize};

use crate::{request::Request, response::Response, usergroups::UsergroupsQuery};

/// A request for `usergroups.list` API. No parameters.
///
/// See: https://api.slack.com/methods/usergroups.list
#[derive(Serialize, Debug, Clone)]
pub struct List {
    /// Include the number of users in each User Group.
    pub include_count: Option<bool>,
    /// Include disabled User Groups.
    pub include_disabled: Option<bool>,
    /// Include the list of users for each User Group.
    pub include_users: Option<bool>,
    /// The id of the usergroup you would like to filter the results down to.
    pub usergroup_ids: Option<String>,
}

impl UsergroupsQuery for List {}

impl Request for List {
    type Response = UsergroupsList;

    fn path(&self) -> &'static str {
        "usergroups.list"
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct UsergroupsList {
    pub ok: bool,
    pub usergroups: Option<Vec<Usergroup>>,
}
impl Response for UsergroupsList {
    fn is_ok(&self) -> bool {
        self.ok
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct Usergroup {
    /// The ID of the usergroup.
    pub id: String,
    /// Team ID
    pub team_id: String,
    /// The name of the usergroup.
    pub name: String,
    /// The description of the usergroup.
    pub description: Option<String>,
    /// The name of the usergroup.
    pub handle: String,
    /// Number of users
    pub user_count: u64,
}
