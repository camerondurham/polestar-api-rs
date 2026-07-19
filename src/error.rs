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

impl PolestarError {
    /// Returns true when the error likely came from schema drift in the GraphQL query.
    pub fn is_graphql_schema_error(&self) -> bool {
        match self {
            Self::GraphQLError(message) => {
                let message = message.to_ascii_lowercase();
                message.contains("cannot query field")
                    || message.contains("cannot query argument")
                    || message.contains("unknown argument")
                    || message.contains("unknown type")
                    || message.contains("fieldundefined")
                    || message.contains("field is not defined")
                    || message.contains("must not have a selection")
                    || message.contains("must have a selection of subfields")
            }
            _ => false,
        }
    }

    /// Returns true when the request should retry with a leaner verbose query.
    pub fn is_verbose_probe_error(&self) -> bool {
        self.is_graphql_schema_error() || matches!(self, Self::ParseError(_))
    }
}

/// Convenient Result type alias for Polestar API operations.
pub type Result<T> = std::result::Result<T, PolestarError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_graphql_schema_errors() {
        assert!(PolestarError::GraphQLError(
            "Cannot query field \"foo\" on type \"Vehicle\"".into(),
        )
        .is_graphql_schema_error());
        assert!(PolestarError::GraphQLError(
            "FieldUndefined: cannot query field software.performanceOptimization".into(),
        )
        .is_graphql_schema_error());
        assert!(PolestarError::GraphQLError(
            "Field \"software\" must not have a selection of subfields".into(),
        )
        .is_graphql_schema_error());
        assert!(
            PolestarError::GraphQLError("Cannot query argument \"locale\" on field\"".into(),)
                .is_graphql_schema_error()
        );

        assert!(!PolestarError::AuthError("bad token".into()).is_graphql_schema_error());
        assert!(!PolestarError::ApiError("random API failure".into()).is_graphql_schema_error());
    }

    #[test]
    fn detects_verbose_probe_errors() {
        let parse_error = serde_json::from_str::<i32>("\"value\"").unwrap_err();
        assert!(PolestarError::ParseError(parse_error).is_verbose_probe_error());
        assert!(PolestarError::GraphQLError("FieldUndefined".into()).is_verbose_probe_error());
        assert!(!PolestarError::NoTelemetry("ABC".to_string()).is_verbose_probe_error());
    }
}
