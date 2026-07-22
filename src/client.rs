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
        let vin = normalize_vin(vin)?;

        let data: TelemetryQueryData = self
            .post_graphql(
                graphql::queries::CAR_TELEMETRICS_V2,
                serde_json::json!({"vins": [&vin]}),
            )
            .await?;

        let telemetry = telemetry_for_vin(data.car_telematics_v2.unwrap_or_default(), &vin);
        if telemetry.battery.is_none() && telemetry.health.is_none() && telemetry.odometer.is_none()
        {
            return Err(PolestarError::NoTelemetry(vin));
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

    /// Lists vehicles with richer details where available.
    pub async fn get_vehicles_verbose(&self) -> Result<Vec<Vehicle>> {
        let queries = [
            graphql::queries::GET_CONSUMER_CARS_V2_VERBOSE,
            graphql::queries::GET_CONSUMER_CARS_V2_VERBOSE_SOFTWARE,
            graphql::queries::GET_CONSUMER_CARS_V2_VERBOSE_SOFTWARE_ONLY,
            graphql::queries::GET_CONSUMER_CARS_V2_VERBOSE_SOFTWARE_SCALAR,
            graphql::queries::GET_CONSUMER_CARS_V2_VERBOSE_CONTENT_ONLY,
            graphql::queries::GET_CONSUMER_CARS_V2_VERBOSE_PERFORMANCE_SPEC_SCALAR,
            graphql::queries::GET_CONSUMER_CARS_V2_VERBOSE_NO_PERFORMANCE,
            graphql::queries::GET_CONSUMER_CARS_V2_VERBOSE_LOCALE,
            graphql::queries::GET_CONSUMER_CARS_V2_VERBOSE_HAS_PERFORMANCE,
        ];

        let mut first_error: Option<crate::error::PolestarError> = None;
        for query in queries {
            match self.post_graphql_raw(query, serde_json::json!({})).await {
                Ok(data) => match serde_json::from_value::<VehiclesQueryData>(data) {
                    Ok(data) => return Ok(data.get_consumer_cars_v2.unwrap_or_default()),
                    Err(err) => {
                        let err = PolestarError::ParseError(err);
                        if first_error.is_none() {
                            first_error = Some(err);
                        }
                    }
                },
                Err(err) if err.is_graphql_schema_error() => {
                    if first_error.is_none() {
                        first_error = Some(err);
                    }
                    continue;
                }
                Err(err) => return Err(err),
            }
        }

        Err(first_error.unwrap_or_else(|| {
            crate::error::PolestarError::ApiError(
                "No verbose vehicle query variant succeeded".to_string(),
            )
        }))
    }

    /// Fetches the supported vehicle summary for the specified VIN.
    pub async fn get_vehicle(&self, vin: &str) -> Result<Vehicle> {
        let vin = normalize_vin(vin)?;

        self.get_vehicles()
            .await?
            .into_iter()
            .find(|vehicle| vehicle.vin.eq_ignore_ascii_case(&vin))
            .ok_or_else(|| PolestarError::InvalidVin(format!("VIN {vin} not found in account")))
    }

    /// Fetches the verbose vehicle information for the specified VIN.
    pub async fn get_vehicle_verbose(&self, vin: &str) -> Result<Vehicle> {
        let vin = normalize_vin(vin)?;

        match self.get_vehicles_verbose().await {
            Ok(vehicles) => vehicles
                .into_iter()
                .find(|vehicle| vehicle.vin.eq_ignore_ascii_case(&vin))
                .ok_or_else(|| {
                    PolestarError::InvalidVin(format!("VIN {vin} not found in account"))
                }),
            Err(err) if err.is_verbose_probe_error() => self.get_vehicle(&vin).await,
            Err(err) => Err(err),
        }
    }

    async fn authenticate(&self) -> Result<String> {
        self.auth_state.get_valid_token(&self.http_client).await
    }

    async fn post_graphql<T>(&self, query: &str, variables: serde_json::Value) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let data = self.post_graphql_raw(query, variables).await?;
        Ok(serde_json::from_value(data)?)
    }

    async fn post_graphql_raw(
        &self,
        query: &str,
        variables: serde_json::Value,
    ) -> Result<serde_json::Value> {
        let request_body = serde_json::json!({
            "query": query,
            "variables": variables
        });

        for attempt in 0..=1 {
            let token = self.authenticate().await?;
            let response = self
                .http_client
                .post(&self.pc_api_url)
                .bearer_auth(&token)
                .header("content-type", "application/json")
                .header("origin", "https://www.polestar.com")
                .json(&request_body)
                .send()
                .await?;

            let status = response.status();
            if status == reqwest::StatusCode::UNAUTHORIZED {
                self.auth_state.invalidate_access_token(&token).await;
                if attempt == 0 {
                    continue;
                }
                return Err(PolestarError::AuthError(
                    "API rejected the access token after re-authentication".to_string(),
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

            let envelope: GraphQlEnvelope<serde_json::Value> = serde_json::from_str(&body)?;
            if !envelope.errors.is_empty() {
                let messages = envelope
                    .errors
                    .into_iter()
                    .map(|error| error.message)
                    .collect::<Vec<_>>()
                    .join("; ");
                return Err(PolestarError::GraphQLError(redact_str(&messages)));
            }

            return envelope.data.ok_or_else(|| {
                PolestarError::ApiError("GraphQL response had no data".to_string())
            });
        }

        unreachable!("GraphQL request loop always returns")
    }
}

fn normalize_vin(vin: &str) -> Result<String> {
    let normalized = vin.to_ascii_uppercase();
    let is_vin_character = |byte: u8| {
        byte.is_ascii_digit() || matches!(byte, b'A'..=b'H' | b'J'..=b'N' | b'P' | b'R'..=b'Z')
    };
    if normalized.len() != 17 || !normalized.bytes().all(is_vin_character) {
        return Err(PolestarError::InvalidVin(
            "expected a 17-character VIN using digits and letters other than I, O, or Q"
                .to_string(),
        ));
    }
    Ok(normalized)
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
    entries
        .into_iter()
        .flatten()
        .find(|entry| entry.vin.eq_ignore_ascii_case(vin))
}

fn find_health(entries: Vec<Option<Health>>, vin: &str) -> Option<Health> {
    entries
        .into_iter()
        .flatten()
        .find(|entry| entry.vin.eq_ignore_ascii_case(vin))
}

fn find_odometer(entries: Vec<Option<Odometer>>, vin: &str) -> Option<Odometer> {
    entries
        .into_iter()
        .flatten()
        .find(|entry| entry.vin.eq_ignore_ascii_case(vin))
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
    use crate::auth::{OidcConfig, TokenState};
    use chrono::{Duration, Utc};
    use std::io::{Read, Write};
    use std::net::{TcpListener, TcpStream};
    use std::thread;

    #[test]
    fn client_creation_does_not_connect() {
        assert!(PolestarClient::new("redacted_user", "redacted_password").is_ok());
    }

    #[test]
    fn validates_and_normalizes_vins() {
        assert_eq!(
            normalize_vin("ABCDEF12345678901").unwrap(),
            "ABCDEF12345678901"
        );
        assert!(normalize_vin("VIN123").is_err());
        assert!(normalize_vin("ABCDEFGHJKLMNPR-1").is_err());
        assert!(normalize_vin("ABCDEFGHJKLMNPR1I").is_err());
        assert!(normalize_vin("ABCDEFGHJKLMNPR1O").is_err());
        assert!(normalize_vin("ABCDEFGHJKLMNPR1Q").is_err());
    }

    #[test]
    fn selects_samples_for_the_requested_vin() {
        let response: TelemetryResponse = serde_json::from_value(serde_json::json!({
            "battery": [
                {
                    "vin": "ABCDEF12345678902",
                    "batteryChargeLevelPercentage": 10,
                    "timestamp": { "seconds": "1", "nanos": 0 }
                },
                {
                    "vin": "ABCDEF12345678903",
                    "batteryChargeLevelPercentage": 79,
                    "timestamp": { "seconds": "2", "nanos": 0 }
                }
            ],
            "health": [null],
            "odometer": [null]
        }))
        .unwrap();

        let telemetry = telemetry_for_vin(response, "abcdef12345678903");
        assert_eq!(
            telemetry
                .battery
                .and_then(|battery| battery.charge_level_percentage),
            Some(79)
        );
        assert!(telemetry.health.is_none());
    }

    #[tokio::test]
    async fn retries_once_after_401_and_preserves_an_omitted_refresh_token() {
        let api_listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let api_address = api_listener.local_addr().unwrap();
        let api_server = thread::spawn(move || {
            let first = respond_once(&api_listener, "401 Unauthorized", "");
            let second = respond_once(&api_listener, "200 OK", r#"{"data":{"retried":true}}"#);
            vec![first, second]
        });

        let token_listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let token_address = token_listener.local_addr().unwrap();
        let token_server = thread::spawn(move || {
            respond_once(
                &token_listener,
                "200 OK",
                r#"{"access_token":"new-access","expires_in":3600}"#,
            )
        });

        let (client, auth_state) = test_client(api_address, token_address).await;

        let data: serde_json::Value = client
            .post_graphql("query Test { retried }", serde_json::json!({}))
            .await
            .unwrap();
        assert_eq!(data, serde_json::json!({"retried": true}));

        let api_requests = api_server.join().unwrap();
        let token_request = token_server.join().unwrap();
        assert!(api_requests[0]
            .to_ascii_lowercase()
            .contains("authorization: bearer old-access"));
        assert!(api_requests[1]
            .to_ascii_lowercase()
            .contains("authorization: bearer new-access"));
        assert!(token_request.contains("grant_type=refresh_token"));
        assert!(token_request.contains("refresh_token=old-refresh"));

        let token = auth_state.token.read().await;
        let token = token.as_ref().unwrap();
        assert_eq!(token.access_token, "new-access");
        assert_eq!(token.refresh_token.as_deref(), Some("old-refresh"));
    }

    #[tokio::test]
    async fn stops_after_a_second_401() {
        let api_listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let api_address = api_listener.local_addr().unwrap();
        let api_server = thread::spawn(move || {
            let first = respond_once(&api_listener, "401 Unauthorized", "");
            let second = respond_once(&api_listener, "401 Unauthorized", "");
            vec![first, second]
        });

        let token_listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let token_address = token_listener.local_addr().unwrap();
        let token_server = thread::spawn(move || {
            respond_once(
                &token_listener,
                "200 OK",
                r#"{"access_token":"new-access","expires_in":3600}"#,
            )
        });

        let (client, _) = test_client(api_address, token_address).await;
        let result: Result<serde_json::Value> = client
            .post_graphql("query Test { retried }", serde_json::json!({}))
            .await;

        assert!(matches!(result, Err(PolestarError::AuthError(_))));
        let api_requests = api_server.join().unwrap();
        token_server.join().unwrap();
        assert_eq!(api_requests.len(), 2);
        assert!(api_requests[0]
            .to_ascii_lowercase()
            .contains("authorization: bearer old-access"));
        assert!(api_requests[1]
            .to_ascii_lowercase()
            .contains("authorization: bearer new-access"));
    }

    #[tokio::test]
    async fn falls_back_to_leaner_verbose_query_on_shape_drift() {
        let api_listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let api_address = api_listener.local_addr().unwrap();
        let api_server = thread::spawn(move || {
            let first = respond_once(
                &api_listener,
                "200 OK",
                r#"{"data":{"getConsumerCarsV2":[{"vin":"ABCDEFGHJKLMNPRST4","content":{"model":{"code":"P2","name":"Polestar 2"},"images":{"studio":{"url":"https://example.com/car.jpg","angles":[1,2]}}}}]}}"#,
            );
            let second = respond_once(
                &api_listener,
                "200 OK",
                r#"{"data":{"getConsumerCarsV2":[{"vin":"ABCDEFGHJKLMNPRST4","modelName":"Polestar 2","hasPerformancePackage":true}]}}"#,
            );
            vec![first, second]
        });

        let token_listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let token_address = token_listener.local_addr().unwrap();
        let (client, _) = test_client(api_address, token_address).await;

        let vehicles = client
            .get_vehicles_verbose()
            .await
            .expect("verbose vehicle fallback should succeed");

        let requests = api_server.join().unwrap();
        assert_eq!(vehicles.len(), 1);
        assert_eq!(vehicles[0].vin, "ABCDEFGHJKLMNPRST4");
        assert_eq!(requests.len(), 2);
        assert!(requests[0].contains("GetConsumerCarsV2Verbose"));
        assert!(requests[1].contains("GetConsumerCarsV2VerboseSoftware"));
    }

    #[test]
    fn parses_graphql_errors_without_data() {
        let envelope: GraphQlEnvelope<serde_json::Value> =
            serde_json::from_value(serde_json::json!({"errors": [{"message": "not authorized"}]}))
                .unwrap();

        assert!(envelope.data.is_none());
        assert_eq!(envelope.errors[0].message, "not authorized");
    }

    async fn test_client(
        api_address: std::net::SocketAddr,
        token_address: std::net::SocketAddr,
    ) -> (PolestarClient, Arc<AuthState>) {
        let auth_state = Arc::new(AuthState::new(
            "redacted_user".to_string(),
            "redacted_password".to_string(),
        ));
        *auth_state.token.write().await = Some(TokenState {
            access_token: "old-access".to_string().into(),
            refresh_token: Some("old-refresh".to_string().into()),
            expires_at: Utc::now() + Duration::hours(1),
            token_lifetime_secs: 3600,
        });
        *auth_state.oidc_config.write().await = Some(OidcConfig {
            issuer: format!("http://{token_address}"),
            token_endpoint: format!("http://{token_address}/token"),
            authorization_endpoint: format!("http://{token_address}/authorize"),
        });

        let client = PolestarClient {
            http_client: reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(5))
                .build()
                .unwrap(),
            auth_state: Arc::clone(&auth_state),
            pc_api_url: format!("http://{api_address}/graphql"),
        };
        (client, auth_state)
    }

    fn respond_once(listener: &TcpListener, status: &str, body: &str) -> String {
        let (mut stream, _) = listener.accept().unwrap();
        let request = read_http_request(&mut stream);
        write!(
            stream,
            "HTTP/1.1 {status}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
            body.len()
        )
        .unwrap();
        request
    }

    fn read_http_request(stream: &mut TcpStream) -> String {
        stream
            .set_read_timeout(Some(std::time::Duration::from_secs(5)))
            .unwrap();
        let mut request = Vec::new();
        let mut buffer = [0_u8; 1024];

        loop {
            let bytes_read = stream.read(&mut buffer).unwrap();
            if bytes_read == 0 {
                break;
            }
            request.extend_from_slice(&buffer[..bytes_read]);

            let Some(header_end) = request.windows(4).position(|bytes| bytes == b"\r\n\r\n") else {
                continue;
            };
            let headers = String::from_utf8_lossy(&request[..header_end]);
            let content_length = headers
                .lines()
                .find_map(|line| {
                    let (name, value) = line.split_once(':')?;
                    name.eq_ignore_ascii_case("content-length")
                        .then(|| value.trim().parse::<usize>().ok())
                        .flatten()
                })
                .unwrap_or(0);
            if request.len() >= header_end + 4 + content_length {
                break;
            }
        }

        String::from_utf8(request).unwrap()
    }
}
