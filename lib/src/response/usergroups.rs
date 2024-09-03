use serde::Deserialize;

use crate::response::Response;

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
