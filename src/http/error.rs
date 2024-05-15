use std::error::Error;
use std::fmt::{Display, Formatter, Result};

use reqwest::Error as RequestError;
use serde::{Deserialize, Serialize};
use serde_json::error::Error as SerdeJsonError;
use url::ParseError as UrlParseError;

#[derive(Debug)]
pub enum ClientError {
    Authorization(String),
    SerdeJson(String),
    UrlParse(String),
    Request(String),
    Binance(BinanceError),
}

impl Error for ClientError {}
impl Display for ClientError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let message = match self {
            Self::Authorization(e) => e.to_string(),
            Self::SerdeJson(e) => e.to_string(),
            Self::UrlParse(e) => e.to_string(),
            Self::Request(e) => e.to_string(),
            Self::Binance(e) => e.to_string(),
        };

        write!(f, "{}", message)
    }
}

impl From<RequestError> for ClientError {
    fn from(value: RequestError) -> Self {
        Self::Request(value.to_string())
    }
}

impl From<UrlParseError> for ClientError {
    fn from(value: UrlParseError) -> Self {
        Self::UrlParse(value.to_string())
    }
}

impl From<SerdeJsonError> for ClientError {
    fn from(value: SerdeJsonError) -> Self {
        Self::SerdeJson(value.to_string())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BinanceError {
    code: i64,
    msg: String,
}

impl Display for BinanceError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{} {}", self.code, self.msg)
    }
}
