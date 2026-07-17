//! Error types for the Polestar API client.

use thiserror::Error;

/// Errors that can occur when interacting with the Polestar API.
#[derive(Error, Debug)]
pub enum PolestarError {
    /// Authentication failed (invalid or expired token).
    #[error("Authentication failed: {0}")]
    AuthError(String),

    /// Invalid credentials provided (ERR001).
    #[error("Invalid username or password")]
    InvalidCredentials,

    /// Token has expired and refresh failed.
    #[error("Token expired")]
    TokenExpired,

    /// OIDC configuration unavailable.
    #[error("OIDC configuration unavailable: {0}")]
    OidcConfigError(String),

    /// API request failed with an error message.
    #[error("API request failed: {0}")]
    ApiError(String),

    /// Network error occurred during the request.
    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),

    /// JSON parsing error.
    #[error("JSON parsing error: {0}")]
    ParseError(#[from] serde_json::Error),

    /// Invalid VIN provided.
    #[error("Invalid VIN: {0}")]
    InvalidVin(String),

    /// The API returned no telemetry samples for the requested VIN.
    #[error("No telemetry is currently available for VIN {0}")]
    NoTelemetry(String),

    /// Rate limit exceeded (HTTP 429).
    #[error("Rate limit exceeded")]
    RateLimitExceeded,

    /// GraphQL error from the API.
    #[error("GraphQL error: {0}")]
    GraphQLError(String),
}

/// Convenient Result type alias for Polestar API operations.
pub type Result<T> = std::result::Result<T, PolestarError>;
