use serde::{Deserialize, Serialize};

use crate::{
    bots::BotsQuery,
    client::{request::Request, response::Response},
};

/// A request for `users.info` API.
///
/// See: https://api.slack.com/methods/users.info
#[derive(Serialize, Debug, Clone)]
pub struct Info<'a> {
    /// User ID to get info on
    #[serde(rename = "bot")]
    pub id: &'a str,
}

impl<'a> BotsQuery for Info<'a> {}

impl<'a> Request for Info<'a> {
    type Response = BotsInfo;

    fn path(&self) -> &'static str {
        "bots.info"
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct BotsInfo {
    pub ok: bool,
    /// The bot object.
    pub bot: Option<Bot>,
}
impl Response for BotsInfo {
    fn is_ok(&self) -> bool {
        self.ok
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct Bot {
    /// The name of the bot.
    pub name: String,
}
