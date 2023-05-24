use reqwest::{header::InvalidHeaderValue, Error as ReqError};
use serde_json::Error as SerdeError;
use thiserror::Error;
use url::ParseError;

pub type Result<T> = std::result::Result<T, QStashError>;

#[derive(Error, Debug)]
pub enum QStashError {
    #[error("http client failed: {0}")]
    ClientError(#[from] ReqError),
    #[error("invalid header value: {0}")]
    InvalidHeaderValue(#[from] InvalidHeaderValue),
    #[error("parse Url error")]
    UrlError(#[from] ParseError),
    #[error("serialize or deserialize error: {0}")]
    SerdeError(#[from] SerdeError),
    #[error("unknown error")]
    Unknown,
}
