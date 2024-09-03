use serde::{Deserialize, Serialize};

use crate::{request::Request, response::Response, users::UsersQuery};

/// A request for `users.info` API.
///
/// See: https://api.slack.com/methods/users.info
#[derive(Serialize, Debug, Clone)]
pub struct Info<'a> {
    /// User ID to get info on
    #[serde(rename = "user")]
    pub id: &'a str,
}
impl<'a> UsersQuery for Info<'a> {}
impl<'a> Request for Info<'a> {
    type Response = UsersInfo;

    fn path(&self) -> &'static str {
        "users.info"
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct UsersInfo {
    pub ok: bool,
    /// The user object.
    pub user: Option<User>,
}
impl Response for UsersInfo {
    fn is_ok(&self) -> bool {
        self.ok
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct User {
    /// The user ID.
    pub id: String,
    /// The team ID.
    pub team_id: String,
    /// The name of the user.
    pub name: String,
    /// The real name of the user.
    pub real_name: Option<String>,
    /// The profile object.
    pub profile: Profile,
    /// Whether the user is a bot.
    pub is_bot: bool,
    /// Whether the user is deleted.
    pub deleted: bool,
    /// Whether the user is an app user.
    pub is_app_user: bool,
    /// Timezone of the user.
    pub tz: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Profile {
    /// The display name, which you can see at the Slack client app.
    pub display_name: String,
    /// The display name normalized.
    pub display_name_normalized: String,
    /// The real name of the user.
    pub real_name: String,
    /// The real name normalized.
    pub real_name_normalized: String,
    /// The email address of the user.
    pub email: Option<String>,
    /// The title of the user.
    pub title: String,
    /// Profile image URLs.
    pub image_24: Option<String>,
    pub image_32: Option<String>,
    pub image_48: Option<String>,
    pub image_72: Option<String>,
    pub image_192: Option<String>,
    pub image_512: Option<String>,
    pub image_1024: Option<String>,
    pub image_original: Option<String>,
}
