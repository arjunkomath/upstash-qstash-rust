use reqwest::{header::InvalidHeaderValue, Error as ReqError};
use serde_json::Error as SerdeError;
use thiserror::Error;
use url::ParseError;

pub type Result<T> = std::result::Result<T, QStashError>;

#[derive(Error, Debug)]
pub enum QStashError {
    #[error("build QStash client failed")]
    ClientError(#[from] ReqError),
    #[error("invalid header value")]
    InvalidHeaderValue(#[from] InvalidHeaderValue),
    #[error("parse Url error")]
    UrlError(#[from] ParseError),
    #[error("serialize or deserialize error")]
    SerdeError(#[from] SerdeError),
}
