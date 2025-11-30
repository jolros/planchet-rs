use std::fmt;
use serde::Deserialize;

/// A specific kind of API error.
#[derive(Debug, PartialEq)]
pub enum KnownApiError {
    /// The provided API key is invalid or has expired (HTTP 401).
    Unauthorized,
    /// The requested resource could not be found (HTTP 404).
    NotFound,
    /// A parameter in the request was invalid or missing (HTTP 400).
    InvalidParameter,
    /// The API rate limit has been exceeded (HTTP 429).
    RateLimitExceeded,
    /// No user is associated with the provided API key (HTTP 501).
    /// This is specific to the `client_credentials` grant type.
    NoUserAssociatedWithApiKey,
}

/// An error returned by the Numista API.
#[derive(Debug)]
pub struct ApiError {
    pub message: String,
    pub status: u16,
    pub kind: Option<KnownApiError>,
}

/// The error type for this crate.
#[derive(Debug)]
pub enum Error {
    /// The API key was not provided in the `ClientBuilder`.
    ApiKeyMissing,
    /// An error related to the underlying HTTP client or middleware stack.
    Request(Box<dyn std::error::Error + Send + Sync>),
    /// An error from `serde_json`.
    Json(serde_json::Error),
    /// An error returned by the Numista API.
    ApiError(ApiError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::ApiKeyMissing => write!(f, "Numista API key is required"),
            Error::Request(e) => write!(f, "Request error: {}", e),
            Error::Json(e) => write!(f, "JSON error: {}", e),
            Error::ApiError(e) => write!(f, "API error (status {}): {}", e.status, e.message),
        }
    }
}

impl std::error::Error for Error {}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::Request(Box::new(err))
    }
}

impl From<reqwest_middleware::Error> for Error {
    fn from(err: reqwest_middleware::Error) -> Self {
        Error::Request(Box::new(err))
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::Json(err)
    }
}

/// A `Result` type alias for this crate's `Error` type.
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, Deserialize)]
pub struct ApiErrorResponse {
    pub error_message: String,
}
