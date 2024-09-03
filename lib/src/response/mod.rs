use std::fmt::Debug;

use serde::Deserialize;

pub mod usergroups;

/// A marker trait for a response from the Slack API, which is just an alias for
/// `serde::de::DeserializeOwned`. Other restrictions may be added in the future.
#[allow(dead_code)]
pub trait Response: serde::de::DeserializeOwned + Debug + Clone {
    fn is_ok(&self) -> bool {
        false
    }

    fn next_cursor(&self) -> Option<String> {
        None
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct ResponseMetadata {
    pub next_cursor: String,
}
