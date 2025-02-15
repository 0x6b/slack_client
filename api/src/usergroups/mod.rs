mod list;
mod users;

pub use list::{List, Usergroup};
pub use users::Users;

use crate::request::Request;

/// A marker trait which denotes a request for the `usergroups` API.
pub trait UsergroupsQuery: Request {}
pub trait UsergroupsUsersQuery: Request {}
