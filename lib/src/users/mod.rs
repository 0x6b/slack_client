mod info;
mod list;

pub use info::{Info, User};
pub use list::List;

use crate::client::request::Request;

/// A marker trait which denotes a request for the `users` API.
pub trait UsersQuery: Request {}
