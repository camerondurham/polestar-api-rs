//! HTTP client for interacting with the Polestar API.

use crate::auth::AuthState;
use crate::error::{PolestarError, Result};
use crate::graphql;
use crate::models::{
    telemetry::{Battery, Health, Odometer, Telemetry, TelemetryResponse},
    vehicle::Vehicle,
};
use crate::redact::redact_str;
use serde::Deserialize;
use std::sync::Arc;

const PC_API_URL: &str = "https://pc-api.polestar.com/eu-north-1/mystar-v2/";

/// Main client for interacting with the Polestar API.
///
/// Authentication is lazy: creating a client does not make a network request.
/// The first data request performs the Polestar OIDC login and later requests
/// reuse or refresh the in-memory token.
#[derive(Clone)]
pub struct PolestarClient {
    http_client: reqwest::Client,
    auth_state: Arc<AuthState>,
    pc_api_url: String,
}

impl PolestarClient {
    /// Creates a new Polestar API client with the provided account credentials.
    pub fn new(username: impl Into<String>, password: impl Into<String>) -> Result<Self> {
        let http_client = reqwest::Client::builder()
            .cookie_store(true)
            .timeout(std::time::Duration::from_secs(30))
            .user_agent(concat!("polestar-api-rs/", env!("CARGO_PKG_VERSION")))
            .build()?;

        Ok(Self {
            http_client,
            auth_state: Arc::new(AuthState::new(username.into(), password.into())),
            pc_api_url: PC_API_URL.to_string(),
        })
    }

    /// Fetches the latest available telemetry for one VIN.
    ///
    /// Polestar may return no sample for an individual signal. Consequently,
    /// `battery`, `health`, and `odometer` are independently optional in the
    /// returned [`Telemetry`]. An error is returned only when no telemetry at all
    /// is available for the requested VIN.
    pub async fn get_telemetry(&self, vin: &str) -> Result<Telemetry> {
        validate_vin(vin)?;

        let data: TelemetryQueryData = self
            .post_graphql(
                graphql::queries::CAR_TELEMETRICS_V2,
                serde_json::json!({"vins": [vin]}),
            )
            .await?;

        let telemetry = telemetry_for_vin(data.car_telematics_v2.unwrap_or_default(), vin);
        if telemetry.battery.is_none() && telemetry.health.is_none() && telemetry.odometer.is_none()
        {
            return Err(PolestarError::NoTelemetry(vin.to_string()));
        }

        Ok(telemetry)
    }

    /// Lists the vehicles associated with the authenticated Polestar account.
    ///
    /// This is also useful for discovering a VIN instead of configuring one
    /// manually.
    pub async fn get_vehicles(&self) -> Result<Vec<Vehicle>> {
        let data: VehiclesQueryData = self
            .post_graphql(
                graphql::queries::GET_CONSUMER_CARS_V2,
                serde_json::json!({}),
            )
            .await?;

        Ok(data.get_consumer_cars_v2.unwrap_or_default())
    }

    /// Fetches the supported vehicle summary for the specified VIN.
    pub async fn get_vehicle(&self, vin: &str) -> Result<Vehicle> {
        validate_vin(vin)?;

        self.get_vehicles()
            .await?
            .into_iter()
            .find(|vehicle| vehicle.vin == vin)
            .ok_or_else(|| PolestarError::InvalidVin(format!("VIN {vin} not found in account")))
    }

    /// Fetches vehicle information for compatibility with earlier releases.
    ///
    /// Polestar no longer exposes a dependable verbose vehicle-information
    /// contract, so this now returns the same supported summary as
    /// [`Self::get_vehicle`].
    pub async fn get_vehicle_verbose(&self, vin: &str) -> Result<Vehicle> {
        self.get_vehicle(vin).await
    }

    async fn authenticate(&self) -> Result<String> {
        self.auth_state.get_valid_token(&self.http_client).await
    }

    async fn post_graphql<T>(&self, query: &str, variables: serde_json::Value) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let token = self.authenticate().await?;
        let response = self
            .http_client
            .post(&self.pc_api_url)
            .bearer_auth(token)
            .header("content-type", "application/json")
            .header("origin", "https://www.polestar.com")
            .json(&serde_json::json!({
                "query": query,
                "variables": variables
            }))
            .send()
            .await?;

        let status = response.status();
        if status == reqwest::StatusCode::UNAUTHORIZED {
            return Err(PolestarError::AuthError(
                "API rejected the access token".to_string(),
            ));
        }
        if status == reqwest::StatusCode::TOO_MANY_REQUESTS {
            return Err(PolestarError::RateLimitExceeded);
        }

        let body = response.text().await?;
        if !status.is_success() {
            return Err(PolestarError::ApiError(format!(
                "HTTP {status}: {}",
                redact_str(&body)
            )));
        }

        let envelope: GraphQlEnvelope<T> = serde_json::from_str(&body)?;
        if !envelope.errors.is_empty() {
            let messages = envelope
                .errors
                .into_iter()
                .map(|error| error.message)
                .collect::<Vec<_>>()
                .join("; ");
            return Err(PolestarError::GraphQLError(redact_str(&messages)));
        }

        envelope
            .data
            .ok_or_else(|| PolestarError::ApiError("GraphQL response had no data".to_string()))
    }
}

fn validate_vin(vin: &str) -> Result<()> {
    if vin.len() != 17 || !vin.bytes().all(|byte| byte.is_ascii_alphanumeric()) {
        return Err(PolestarError::InvalidVin(
            "expected a 17-character alphanumeric VIN".to_string(),
        ));
    }
    Ok(())
}

fn telemetry_for_vin(response: TelemetryResponse, vin: &str) -> Telemetry {
    let battery = find_battery(response.battery, vin);
    let health = find_health(response.health, vin);
    let odometer = find_odometer(response.odometer, vin);

    Telemetry {
        battery,
        health,
        odometer,
    }
}

fn find_battery(entries: Vec<Option<Battery>>, vin: &str) -> Option<Battery> {
    entries.into_iter().flatten().find(|entry| entry.vin == vin)
}

fn find_health(entries: Vec<Option<Health>>, vin: &str) -> Option<Health> {
    entries.into_iter().flatten().find(|entry| entry.vin == vin)
}

fn find_odometer(entries: Vec<Option<Odometer>>, vin: &str) -> Option<Odometer> {
    entries.into_iter().flatten().find(|entry| entry.vin == vin)
}

#[derive(Deserialize)]
struct TelemetryQueryData {
    #[serde(rename = "carTelematicsV2")]
    car_telematics_v2: Option<TelemetryResponse>,
}

#[derive(Deserialize)]
struct VehiclesQueryData {
    #[serde(rename = "getConsumerCarsV2")]
    get_consumer_cars_v2: Option<Vec<Vehicle>>,
}

#[derive(Deserialize)]
struct GraphQlEnvelope<T> {
    data: Option<T>,
    #[serde(default)]
    errors: Vec<GraphQlError>,
}

#[derive(Deserialize)]
struct GraphQlError {
    message: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn client_creation_does_not_connect() {
        assert!(PolestarClient::new("user@example.com", "password").is_ok());
    }

    #[test]
    fn validates_vins() {
        assert!(validate_vin("YSMYKEAE7RB000000").is_ok());
        assert!(validate_vin("VIN123").is_err());
        assert!(validate_vin("YSMYKEAE7RB00-000").is_err());
    }

    #[test]
    fn selects_samples_for_the_requested_vin() {
        let response: TelemetryResponse = serde_json::from_value(serde_json::json!({
            "battery": [
                {
                    "vin": "AAAAAAAA1AA111111",
                    "batteryChargeLevelPercentage": 10,
                    "timestamp": { "seconds": "1", "nanos": 0 }
                },
                {
                    "vin": "YSMYKEAE7RB000000",
                    "batteryChargeLevelPercentage": 79,
                    "timestamp": { "seconds": "2", "nanos": 0 }
                }
            ],
            "health": [null],
            "odometer": [null]
        }))
        .unwrap();

        let telemetry = telemetry_for_vin(response, "YSMYKEAE7RB000000");
        assert_eq!(
            telemetry
                .battery
                .and_then(|battery| battery.charge_level_percentage),
            Some(79)
        );
        assert!(telemetry.health.is_none());
    }

    #[test]
    fn parses_graphql_errors_without_data() {
        let envelope: GraphQlEnvelope<serde_json::Value> =
            serde_json::from_value(serde_json::json!({"errors": [{"message": "not authorized"}]}))
                .unwrap();

        assert!(envelope.data.is_none());
        assert_eq!(envelope.errors[0].message, "not authorized");
    }
}
