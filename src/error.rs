use thiserror::Error;

/// Errors returned by the KiriminAja SDK.
#[derive(Debug, Error)]
pub enum Error {
    #[error("invalid argument: {0}")]
    InvalidArgument(String),

    #[error("invalid URL: {0}")]
    InvalidUrl(String),

    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("decode response: {0}")]
    Decode(#[from] serde_json::Error),

    #[error("API error {status}: {body}")]
    Api {
        status: u16,
        status_text: String,
        body: String,
    },
}

pub type Result<T> = std::result::Result<T, Error>;
