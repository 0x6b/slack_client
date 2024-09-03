pub mod message;
mod slack_api_client;

pub use message::SlackMessage;
// Re-export the API modules. Looks not a good idea.
pub use slack_api::{
    bots, conversations, request, request::Request, response, response::Response, usergroups, users,
};
pub use slack_api_client::SlackApiClient;
