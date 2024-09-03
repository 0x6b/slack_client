mod client;
pub mod message;

pub use client::Client;
pub use message::SlackMessage;
// Re-export the API modules. Looks not a good idea.
pub use slack_api::{
    bots, conversations, request, request::Request, response, response::Response, usergroups, users,
};
