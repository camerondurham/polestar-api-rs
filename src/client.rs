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
///     let client = PolestarClient::new("YOUR_TOKEN")?;
///     let telemetry = client.get_telemetry("YOUR_VIN").await?;
///     Ok(())
/// }
/// ```
#[derive(Clone)]
pub struct PolestarClient {
    http_client: reqwest::Client,
    token: String,
    pc_api_base: String,
    cms_api_base: String,
}

impl PolestarClient {
    /// Creates a new Polestar API client with the provided authentication token.
    ///
    /// # Arguments
    ///
    /// * `token` - Polestar API authentication token
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use polestar_api_rs::PolestarClient;
    /// let client = PolestarClient::new("your_token").unwrap();
    /// ```
    pub fn new(token: impl Into<String>) -> Result<Self> {
        let http_client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()?;

        Ok(Self {
            http_client,
            token: token.into(),
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
    /// # let client = PolestarClient::new("token")?;
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
        let body = serde_json::json!({
            "query": query,
            "variables": variables
        });

        let response = self
            .http_client
            .post(endpoint)
            .header("authorization", format!("Bearer {}", self.token))
            .header("content-type", "application/json")
            .header("origin", "https://www.polestar.com")
            .json(&body)
            .send()
            .await?;

        // Check status code
        let status = response.status();
        if status == reqwest::StatusCode::UNAUTHORIZED {
            return Err(PolestarError::AuthError(
                "Invalid or expired token".to_string(),
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
        let client = PolestarClient::new("test_token");
        assert!(client.is_ok());
    }
}
