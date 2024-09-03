use std::fmt::Debug;

use serde::Serialize;

use crate::{request_method::RequestMethod, response::Response};

/// A trait for a request to the Slack API, which defines the path to the endpoint and the response
/// type as its associated type.
pub trait Request: Serialize + Debug + Clone {
    type Response: Response;

    /// Returns the path to the endpoint.
    fn path(&self) -> &'static str;

    /// Returns the HTTP request method.
    fn method(&self) -> RequestMethod {
        RequestMethod::Get
    }
}
