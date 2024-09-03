use serde::Deserialize;

use crate::response::{Response, ResponseMetadata};

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

#[derive(Deserialize, Debug, Clone)]
pub struct UsersList {
    pub ok: bool,
    pub members: Option<Vec<User>>,
    pub response_metadata: Option<ResponseMetadata>,
}
impl Response for UsersList {
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
