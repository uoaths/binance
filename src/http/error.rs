use std::error::Error;
use std::fmt::{Display, Formatter, Result};

use reqwest::Error as RequestError;
use serde_json::error::Error as SerdeJsonError;
use url::ParseError;

#[derive(Debug)]
pub enum ClientError {
    Authorization(String),
    ParseError(String),
    Request(String),
}

impl Error for ClientError {}
impl Display for ClientError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let message = match self {
            Self::Authorization(e) => e.to_string(),
            Self::ParseError(e) => e.to_string(),
            Self::Request(e) => e.to_string(),
        };

        write!(f, "{}", message)
    }
}

impl From<RequestError> for ClientError {
    fn from(value: RequestError) -> Self {
        Self::Request(value.to_string())
    }
}

impl From<ParseError> for ClientError {
    fn from(value: ParseError) -> Self {
        Self::ParseError(value.to_string())
    }
}

impl From<SerdeJsonError> for ClientError {
    fn from(value: SerdeJsonError) -> Self {
        Self::ParseError(value.to_string())
    }
}
