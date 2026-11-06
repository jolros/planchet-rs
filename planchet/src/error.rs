use thiserror::Error;

/// An error returned by the Numista API.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApiError {
    /// The error message returned by the API.
    pub message: String,
    /// The HTTP status code returned by the API.
    pub status: u16,
}

impl ApiError {
    /// Checks if the error is due to an invalid or missing parameter (HTTP 400).
    ///
    /// See <https://numista.com/api/doc> for more details.
    pub fn is_invalid_parameter(&self) -> bool {
        self.status == 400
    }

    /// Checks if the error is due to an invalid or expired API key (HTTP 401).
    ///
    /// See <https://numista.com/api/doc> for more details.
    pub fn is_unauthorized(&self) -> bool {
        self.status == 401
    }

    /// Checks if the error is due to the API key not being activated for the endpoint (HTTP 403).
    ///
    /// See <https://numista.com/api/doc> for more details.
    pub fn is_forbidden(&self) -> bool {
        self.status == 403
    }

    /// Checks if the requested resource could not be found (HTTP 404).
    ///
    /// See <https://numista.com/api/doc> for more details.
    pub fn is_not_found(&self) -> bool {
        self.status == 404
    }

    /// Checks if the API rate limit has been exceeded (HTTP 429).
    ///
    /// See <https://numista.com/api/doc> for more details.
    pub fn is_rate_limit_exceeded(&self) -> bool {
        self.status == 429
    }

    /// Checks if no user is associated with the provided API key (HTTP 501).
    ///
    /// This is specific to the `client_credentials` grant type.
    ///
    /// See <https://numista.com/api/doc> for more details.
    pub fn is_no_user_associated_with_api_key(&self) -> bool {
        self.status == 501
    }
}

/// The error type for this crate.
#[derive(Debug, Error)]
pub enum Error {
    /// The API key was not provided in the `ClientBuilder`.
    #[error("Numista API key is required")]
    ApiKeyMissing,

    /// An API key or bearer token contained invalid HTTP header characters.
    #[error("Invalid header value: {0}")]
    InvalidHeaderValue(#[from] reqwest::header::InvalidHeaderValue),

    /// An error related to the underlying HTTP client.
    #[error("Request error: {0}")]
    Request(#[from] reqwest::Error),

    /// An error from `serde_json`.
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// An error returned by the Numista API.
    #[error("API error (status {}): {}", .0.status, .0.message)]
    ApiError(ApiError),
}

/// A `Result` type alias for this crate's `Error` type.
pub type Result<T> = std::result::Result<T, Error>;
