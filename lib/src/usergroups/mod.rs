use crate::client::request::Request;

mod list;

pub use list::{List, Usergroup};

/// A marker trait which denotes a request for the `usergroups` API.
pub trait UsergroupsQuery: Request {}
