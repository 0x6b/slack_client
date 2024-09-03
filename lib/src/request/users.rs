use serde::Serialize;

use crate::{
    request::Request,
    response::users::{UsersInfo, UsersList},
};

/// A marker trait which denotes a request for the `users` API.
pub trait UsersQuery: Request {}

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

/// A request for `users.list` API.
///
/// See: https://api.slack.com/methods/users.list
#[derive(Serialize, Debug, Clone)]
pub struct List {
    /// Paginate through collections of data by setting the cursor parameter to a next_cursor
    /// attribute returned by a previous request's response_metadata. Default value fetches the
    /// first "page" of the collection. See pagination for more detail.
    pub cursor: Option<String>,
    /// The maximum number of items to return. Fewer than the requested number of items may be
    /// returned, even if the end of the users list hasn't been reached.
    pub limit: Option<u64>,
}
impl UsersQuery for List {}
impl Request for List {
    type Response = UsersList;

    fn path(&self) -> &'static str {
        "users.list"
    }
}
