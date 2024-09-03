use std::fmt::Debug;

use serde::{de::DeserializeOwned, Deserialize};

/// A marker trait for a response from the Slack API, which is just an alias for
/// `serde::de::DeserializeOwned`. Other restrictions may be added in the future.
#[allow(dead_code)]
pub trait Response: DeserializeOwned + Debug + Clone {
    /// Returns `true` if the response is successful.
    fn is_ok(&self) -> bool {
        false
    }

    /// Returns the next cursor for pagination, if any.
    fn next_cursor(&self) -> Option<String> {
        None
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct ResponseMetadata {
    pub next_cursor: String,
}
