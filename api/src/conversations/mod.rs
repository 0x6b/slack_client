mod history;
mod info;
mod list;
mod replies;

pub use history::{History, Message};
pub use info::Info;
pub use list::{ChannelType, List};
pub use replies::Replies;

use crate::request::Request;

/// A marker trait which denotes a request for the `conversations` API.
pub trait ConversationsQuery: Request {}
