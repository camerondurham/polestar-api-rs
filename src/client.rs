//! HTTP client for interacting with the Polestar API.

use crate::auth::AuthState;
use crate::error::{PolestarError, Result};
use crate::graphql;
use crate::models::{telemetry::Telemetry, vehicle::Vehicle};
use std::sync::Arc;

/// Main client for interacting with the Polestar API.
///
/// # Example
///
/// ```no_run
/// use polestar_api::PolestarClient;
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
    auth_state: Arc<AuthState>,
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
    /// # use polestar_api::PolestarClient;
    /// let client = PolestarClient::new("user@example.com", "password").unwrap();
    /// ```
    pub fn new(username: impl Into<String>, password: impl Into<String>) -> Result<Self> {
        let http_client = reqwest::Client::builder()
            .cookie_store(true)
            .timeout(std::time::Duration::from_secs(30))
            .build()?;

        let auth_state = Arc::new(AuthState::new(username.into(), password.into()));

        Ok(Self {
            http_client,
            auth_state,
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
    /// # use polestar_api::PolestarClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = PolestarClient::new("user@example.com", "password")?;
    /// let telemetry = client.get_telemetry("VIN123").await?;
    /// if let Some(charge) = telemetry.battery.charge_level_percentage {
    ///     println!("Battery: {}%", charge);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_telemetry(&self, vin: &str) -> Result<Telemetry> {
        let variables = serde_json::json!({"vins": [vin]});

        let token = self.authenticate().await?;

        let body = serde_json::json!({
            "query": graphql::queries::CAR_TELEMETRICS_V2,
            "variables": variables
        });

        let response = self
            .http_client
            .post(&self.pc_api_base)
            .header("authorization", format!("Bearer {}", token))
            .header("content-type", "application/json")
            .header("origin", "https://www.polestar.com")
            .json(&body)
            .send()
            .await?;

        let json: serde_json::Value = response.json().await?;

        if let Some(errors) = json.get("errors") {
            if let Some(message) = errors.get(0).and_then(|e| e.get("message")) {
                return Err(PolestarError::GraphQLError(message.to_string()));
            }
        }

        let telemetry_data = json
            .get("data")
            .and_then(|d| d.get("carTelematicsV2"))
            .ok_or_else(|| PolestarError::ApiError("No carTelematicsV2 field in response".to_string()))?;

        let response: crate::models::telemetry::TelemetryResponse =
            serde_json::from_value(telemetry_data.clone())?;

        let battery = response
            .battery
            .into_iter()
            .next()
            .ok_or_else(|| PolestarError::ApiError("No battery data in response".to_string()))?;

        let health = response
            .health
            .into_iter()
            .next()
            .ok_or_else(|| PolestarError::ApiError("No health data in response".to_string()))?;

        let odometer = response.odometer.into_iter().next().and_then(|o| o);

        Ok(Telemetry {
            battery,
            health,
            odometer,
        })
    }

    /// Fetches complete vehicle information for the specified VIN.
    ///
    /// # Arguments
    ///
    /// * `vin` - Vehicle Identification Number
    pub async fn get_vehicle(&self, vin: &str) -> Result<Vehicle> {
        self.get_vehicle_internal(vin, false).await
    }

    /// Fetches verbose vehicle information for the specified VIN.
    ///
    /// This retrieves extended vehicle data including service history, emissions data,
    /// detailed specifications, features, and all available metadata.
    ///
    /// # Arguments
    ///
    /// * `vin` - Vehicle Identification Number
    pub async fn get_vehicle_verbose(&self, vin: &str) -> Result<Vehicle> {
        self.get_vehicle_internal(vin, true).await
    }

    /// Internal method to fetch vehicle information.
    async fn get_vehicle_internal(&self, vin: &str, verbose: bool) -> Result<Vehicle> {
        let variables = serde_json::json!({});

        // Get token
        let token = self.authenticate().await?;

        let query = if verbose {
            graphql::queries::GET_CONSUMER_CARS_V2_VERBOSE
        } else {
            graphql::queries::GET_CONSUMER_CARS_V2
        };

        let body = serde_json::json!({
            "query": query,
            "variables": variables
        });

        let response = self
            .http_client
            .post(&self.pc_api_base)
            .header("authorization", format!("Bearer {}", token))
            .header("content-type", "application/json")
            .header("origin", "https://www.polestar.com")
            .json(&body)
            .send()
            .await?;

        let json: serde_json::Value = response.json().await?;

        // Check for GraphQL errors
        if let Some(errors) = json.get("errors") {
            if let Some(message) = errors.get(0).and_then(|e| e.get("message")) {
                return Err(PolestarError::GraphQLError(message.to_string()));
            }
        }

        // Extract data.getConsumerCarsV2 field
        let vehicles_data = json
            .get("data")
            .and_then(|d| d.get("getConsumerCarsV2"))
            .ok_or_else(|| PolestarError::ApiError("No getConsumerCarsV2 field in response".to_string()))?;

        let vehicles: Vec<Vehicle> = serde_json::from_value(vehicles_data.clone())?;

        // Find the vehicle with matching VIN
        vehicles
            .into_iter()
            .find(|v| v.vin == vin)
            .ok_or_else(|| PolestarError::InvalidVin(format!("VIN {} not found", vin)))
    }

    /// Authenticates with Polestar and returns an access token.
    ///
    /// This method implements the web-based login flow using the stored credentials.
    /// The token is cached and reused for subsequent requests.
    async fn authenticate(&self) -> Result<String> {
        self.auth_state.get_valid_token(&self.http_client).await
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
        // Get valid token (will authenticate if needed)
        let token = self.authenticate().await?;

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
