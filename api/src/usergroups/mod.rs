mod list;

pub use list::{List, Usergroup};

use crate::request::Request;

/// A marker trait which denotes a request for the `usergroups` API.
pub trait UsergroupsQuery: Request {}
