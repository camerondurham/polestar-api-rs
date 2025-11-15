//! HTTP client for interacting with the Polestar API.

use crate::error::{PolestarError, Result};
use crate::graphql;
use crate::models::{telemetry::Telemetry, vehicle::Vehicle};

/// Main client for interacting with the Polestar API.
///
/// # Example
///
/// ```no_run
/// use polestar_api_rs::PolestarClient;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let client = PolestarClient::new("your_username", "your_password")?;
///     let telemetry = client.get_telemetry("YOUR_VIN").await?;
///     Ok(())
/// }
/// ```
#[derive(Clone)]
pub struct PolestarClient {
    http_client: reqwest::Client,
    username: String,
    password: String,
    pc_api_base: String,
    cms_api_base: String,
}

impl PolestarClient {
    /// Creates a new Polestar API client with the provided credentials.
    ///
    /// The client will use these credentials to authenticate with the Polestar API
    /// via the web-based login flow and obtain access tokens as needed.
    ///
    /// # Arguments
    ///
    /// * `username` - Polestar account username (email)
    /// * `password` - Polestar account password
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use polestar_api_rs::PolestarClient;
    /// let client = PolestarClient::new("user@example.com", "password").unwrap();
    /// ```
    pub fn new(username: impl Into<String>, password: impl Into<String>) -> Result<Self> {
        let http_client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()?;

        Ok(Self {
            http_client,
            username: username.into(),
            password: password.into(),
            pc_api_base: "https://pc-api.polestar.com/eu-north-1/mystar-v2".to_string(),
            cms_api_base: "https://cms-api.polestar.com/".to_string(),
        })
    }

    /// Fetches telemetry data for the specified VIN.
    ///
    /// # Arguments
    ///
    /// * `vin` - Vehicle Identification Number
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use polestar_api_rs::PolestarClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = PolestarClient::new("user@example.com", "password")?;
    /// let telemetry = client.get_telemetry("VIN123").await?;
    /// println!("Battery: {:?}%", telemetry.battery.charge_level_percentage);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_telemetry(&self, vin: &str) -> Result<Telemetry> {
        let variables = serde_json::json!({
            "vin": vin
        });

        self.post_graphql(&self.pc_api_base, graphql::queries::CAR_TELEMETRICS_V2, variables)
            .await
    }

    /// Fetches complete vehicle information for the specified VIN.
    ///
    /// # Arguments
    ///
    /// * `vin` - Vehicle Identification Number
    pub async fn get_vehicle(&self, vin: &str) -> Result<Vehicle> {
        let variables = serde_json::json!({
            "vin": vin
        });

        self.post_graphql(
            &self.pc_api_base,
            graphql::queries::GET_CONSUMER_CARS_V2,
            variables,
        )
        .await
    }

    /// Authenticates with Polestar and returns an access token.
    ///
    /// This method implements the web-based login flow using the stored credentials.
    /// The token is cached and reused for subsequent requests.
    ///
    /// # Note
    ///
    /// This is a placeholder implementation. The actual authentication flow
    /// will be implemented in a future version.
    async fn authenticate(&self) -> Result<String> {
        // TODO: Implement actual authentication flow
        // This should:
        // 1. Perform login with username/password
        // 2. Handle OAuth/token exchange
        // 3. Return bearer token
        // 4. Cache token with expiration

        // For now, return placeholder
        Err(PolestarError::AuthError(
            "Authentication not yet implemented. Please use pypolestar to obtain a token manually.".to_string()
        ))
    }

    /// Internal method to execute GraphQL queries.
    async fn post_graphql<T>(
        &self,
        endpoint: &str,
        query: &str,
        variables: serde_json::Value,
    ) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        // TODO: Get token via authenticate() method
        // For now, use username field as token (temporary placeholder)
        let token = &self.username;

        let body = serde_json::json!({
            "query": query,
            "variables": variables
        });

        let response = self
            .http_client
            .post(endpoint)
            .header("authorization", format!("Bearer {}", token))
            .header("content-type", "application/json")
            .header("origin", "https://www.polestar.com")
            .json(&body)
            .send()
            .await?;

        // Check status code
        let status = response.status();
        if status == reqwest::StatusCode::UNAUTHORIZED {
            return Err(PolestarError::AuthError(
                "Invalid credentials or expired session".to_string(),
            ));
        }
        if status == reqwest::StatusCode::TOO_MANY_REQUESTS {
            return Err(PolestarError::RateLimitExceeded);
        }

        // Parse response
        let json: serde_json::Value = response.json().await?;

        // Check for GraphQL errors
        if let Some(errors) = json.get("errors") {
            if let Some(message) = errors.get(0).and_then(|e| e.get("message")) {
                return Err(PolestarError::GraphQLError(message.to_string()));
            }
        }

        // Extract data field and deserialize
        let data = json
            .get("data")
            .ok_or_else(|| PolestarError::ApiError("No data field in response".to_string()))?;

        serde_json::from_value(data.clone()).map_err(|e| e.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let client = PolestarClient::new("user@example.com", "password");
        assert!(client.is_ok());
    }
}
