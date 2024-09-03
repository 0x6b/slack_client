use std::{
    fmt,
    fmt::{Debug, Display, Formatter},
};

use reqwest::Method;

/// An enum representing the HTTP request method.
pub enum RequestMethod {
    Get,
    Post,
}

impl From<RequestMethod> for Method {
    fn from(method: RequestMethod) -> Self {
        match method {
            RequestMethod::Get => Method::GET,
            RequestMethod::Post => Method::POST,
        }
    }
}

impl Display for RequestMethod {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            RequestMethod::Get => write!(f, "GET"),
            RequestMethod::Post => write!(f, "POST"),
        }
    }
}

impl Debug for RequestMethod {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}
