mod info;

pub use info::Info;

use crate::client::request::Request;

/// A marker trait which denotes a request for the `users` API.
pub trait BotsQuery: Request {}
