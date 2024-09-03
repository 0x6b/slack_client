use serde::{Deserialize, Serialize};

use crate::{
    client::{
        request::Request,
        response::{Response, ResponseMetadata},
    },
    users::{info::User, UsersQuery},
};

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
