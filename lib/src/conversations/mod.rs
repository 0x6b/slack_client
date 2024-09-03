use crate::client::request::Request;

mod history;
mod info;
mod replies;

pub use history::{History, Message};
pub use info::Info;
pub use replies::Replies;

/// A marker trait which denotes a request for the `conversations` API.
pub trait ConversationsQuery: Request {}
