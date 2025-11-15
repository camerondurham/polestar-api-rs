# Polestar API Rust Wrapper - Architecture Design

## Overview

This document describes the technical architecture and design decisions for the `polestar-api-rs` library, a lightweight Rust wrapper for the Polestar vehicle GraphQL API.

## Design Principles

### 1. Type Safety First
Leverage Rust's type system to catch errors at compile time rather than runtime.

### 2. Zero-Cost Abstractions
Provide ergonomic APIs without sacrificing performance.

### 3. Async by Default
Built on async/await to support high-concurrency scenarios without blocking.

### 4. Fail Fast
Use `Result<T, E>` extensively and provide clear error messages.

### 5. Minimal Dependencies
Keep the dependency tree small to reduce compile times and security surface.

## Architecture Diagram

```
┌─────────────────────────────────────────────────────────────┐
│                        User Application                      │
└────────────────────────────┬────────────────────────────────┘
                             │
                             ▼
┌─────────────────────────────────────────────────────────────┐
│                    PolestarClient (lib.rs)                   │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐      │
│  │  Telemetry   │  │   Vehicle    │  │     Specs    │      │
│  │    API       │  │     API      │  │     API      │      │
│  └──────┬───────┘  └──────┬───────┘  └──────┬───────┘      │
└─────────┼──────────────────┼──────────────────┼─────────────┘
          │                  │                  │
          └──────────────────┼──────────────────┘
                             ▼
┌─────────────────────────────────────────────────────────────┐
│                   HTTP Client (client.rs)                    │
│  ┌────────────────────────────────────────────────────┐     │
│  │  • Authentication (Bearer Token)                   │     │
│  │  • Request Building                                │     │
│  │  • Retry Logic                                     │     │
│  │  • Error Handling                                  │     │
│  └────────────────────────────────────────────────────┘     │
└────────────────────────────┬────────────────────────────────┘
                             ▼
┌─────────────────────────────────────────────────────────────┐
│                    GraphQL Layer (graphql/)                  │
│  ┌──────────────────────────────────────────────────────┐   │
│  │  • Query Templates                                   │   │
│  │  • Variable Substitution                             │   │
│  │  • Response Parsing                                  │   │
│  └──────────────────────────────────────────────────────┘   │
└────────────────────────────┬────────────────────────────────┘
                             ▼
┌─────────────────────────────────────────────────────────────┐
│                    Data Models (models/)                     │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐      │
│  │  Telemetry   │  │   Vehicle    │  │     Specs    │      │
│  │   Models     │  │   Models     │  │   Models     │      │
│  └──────────────┘  └──────────────┘  └──────────────┘      │
└─────────────────────────────────────────────────────────────┘
                             ▼
┌─────────────────────────────────────────────────────────────┐
│                 External Polestar API                        │
│  ┌────────────────────┐  ┌────────────────────┐            │
│  │ pc-api.polestar    │  │ cms-api.polestar   │            │
│  │ (Telemetry + Data) │  │ (Specifications)   │            │
│  └────────────────────┘  └────────────────────┘            │
└─────────────────────────────────────────────────────────────┘
```

## Core Components

### 1. PolestarClient (`src/lib.rs`, `src/client.rs`)

The main entry point for users of the library.

```rust
pub struct PolestarClient {
    http_client: reqwest::Client,
    token: String,
    pc_api_base: String,
    cms_api_base: String,
}

impl PolestarClient {
    pub fn new(token: impl Into<String>) -> Result<Self, PolestarError>;

    pub async fn get_telemetry(&self, vin: &str) -> Result<Telemetry, PolestarError>;
    pub async fn get_vehicle(&self, vin: &str) -> Result<Vehicle, PolestarError>;
    pub async fn get_specifications(&self, vin: &str) -> Result<Specifications, PolestarError>;
}
```

**Responsibilities**:
- Manage HTTP client lifecycle
- Handle authentication tokens
- Provide high-level API methods
- Route requests to appropriate endpoints

**Design Decisions**:
- Single client instance can be shared across threads (`Clone` + `Arc`)
- Immutable after construction for thread safety
- Generic `Into<String>` for flexible token input

### 2. HTTP Client Layer (`src/client.rs`)

Handles low-level HTTP communication with Polestar APIs.

```rust
impl PolestarClient {
    async fn post_graphql<T>(
        &self,
        endpoint: &str,
        query: &str,
        variables: serde_json::Value,
    ) -> Result<T, PolestarError>
    where
        T: serde::de::DeserializeOwned;
}
```

**Responsibilities**:
- Construct HTTP requests with proper headers
- Execute GraphQL queries
- Handle network errors and retries
- Parse responses into typed structures

**Design Decisions**:
- Use `reqwest` with async/tokio runtime
- Enable rustls-tls for modern TLS support
- Set appropriate timeouts (30s default)
- Include all required headers:
  ```
  Authorization: Bearer {token}
  Content-Type: application/json
  Origin: https://www.polestar.com
  ```

### 3. GraphQL Layer (`src/graphql/`)

Manages GraphQL query construction and execution.

```rust
pub mod queries {
    pub const CAR_TELEMETRICS_V2: &str = r#"
        query CarTelematicsV2($vin: String!) {
            getCarTelematicsV2(vin: $vin) {
                data {
                    batteryChargeLevelPercentage
                    batteryChargeStatus
                    chargingPowerWatts
                    # ... more fields
                }
            }
        }
    "#;

    pub const GET_CONSUMER_CARS_V2: &str = "...";
    pub const GET_CAR_SPECIFICATIONS: &str = "...";
}
```

**Responsibilities**:
- Define GraphQL query strings
- Type-safe variable substitution
- Map GraphQL responses to Rust types

**Design Decisions**:
- Store queries as static strings (zero runtime cost)
- Use serde for automatic JSON mapping
- Handle GraphQL error format:
  ```json
  {
    "data": null,
    "errors": [{"message": "..."}]
  }
  ```

### 4. Data Models (`src/models/`)

Type-safe representations of API responses.

#### Telemetry Models (`models/telemetry.rs`)

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Telemetry {
    pub battery: Battery,
    pub odometer: Odometer,
    pub health: Health,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Battery {
    #[serde(rename = "batteryChargeLevelPercentage")]
    pub charge_level_percentage: Option<f64>,

    #[serde(rename = "batteryChargeStatus")]
    pub charge_status: Option<String>,

    #[serde(rename = "chargingPowerWatts")]
    pub charging_power_watts: Option<f64>,

    #[serde(rename = "estimatedChargingTimeToFullMinutes")]
    pub estimated_charging_time_minutes: Option<i64>,

    #[serde(rename = "estimatedDistanceToEmptyKm")]
    pub estimated_distance_to_empty_km: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Odometer {
    #[serde(rename = "averageSpeedKmPerHour")]
    pub average_speed_kmh: Option<f64>,

    #[serde(rename = "odometerMeters")]
    pub odometer_meters: Option<i64>,
}
```

#### Vehicle Models (`models/vehicle.rs`)

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vehicle {
    pub vin: String,
    pub registration_number: Option<String>,
    pub market: Option<String>,
    pub content: VehicleContent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VehicleContent {
    pub model: ModelInfo,
    pub images: Images,
    pub specifications: VehicleSpecifications,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VehicleSpecifications {
    pub motor: MotorSpec,
    pub battery: BatterySpec,
    pub dimensions: Dimensions,
    pub performance: Performance,
}
```

**Design Decisions**:
- Use `Option<T>` extensively (API has many optional fields)
- Serde rename for Rust naming conventions (snake_case)
- Derive `Debug`, `Clone` for developer ergonomics
- Use appropriate numeric types (`f64` for floats, `i64` for large integers)

### 5. Error Handling (`src/error.rs`)

Comprehensive error types for all failure modes.

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PolestarError {
    #[error("Authentication failed: {0}")]
    AuthError(String),

    #[error("API request failed: {0}")]
    ApiError(String),

    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),

    #[error("JSON parsing error: {0}")]
    ParseError(#[from] serde_json::Error),

    #[error("Invalid VIN: {0}")]
    InvalidVin(String),

    #[error("Rate limit exceeded")]
    RateLimitExceeded,

    #[error("GraphQL error: {0}")]
    GraphQLError(String),
}

pub type Result<T> = std::result::Result<T, PolestarError>;
```

**Design Decisions**:
- Use `thiserror` for ergonomic error definitions
- Implement `From` conversions for automatic error conversion
- Provide detailed error messages for debugging
- Export custom `Result<T>` type alias

## Advanced Features

### 1. Retry Logic

Implement exponential backoff for transient failures:

```rust
const MAX_RETRIES: u32 = 3;
const INITIAL_BACKOFF: Duration = Duration::from_millis(100);

async fn retry_with_backoff<F, T>(mut f: F) -> Result<T>
where
    F: FnMut() -> BoxFuture<'static, Result<T>>,
{
    let mut attempt = 0;
    loop {
        match f().await {
            Ok(result) => return Ok(result),
            Err(e) if attempt < MAX_RETRIES && is_retryable(&e) => {
                let backoff = INITIAL_BACKOFF * 2_u32.pow(attempt);
                tokio::time::sleep(backoff).await;
                attempt += 1;
            }
            Err(e) => return Err(e),
        }
    }
}
```

**Retryable Errors**:
- Network timeouts
- 5xx server errors
- 429 Rate Limit (with longer backoff)

### 2. Caching Layer (Optional Feature)

```rust
#[cfg(feature = "cache")]
use moka::future::Cache;

pub struct CachedPolestarClient {
    client: PolestarClient,
    cache: Cache<String, Telemetry>,
}

impl CachedPolestarClient {
    pub async fn get_telemetry(&self, vin: &str) -> Result<Telemetry> {
        if let Some(cached) = self.cache.get(vin).await {
            return Ok(cached);
        }

        let result = self.client.get_telemetry(vin).await?;
        self.cache.insert(vin.to_string(), result.clone()).await;
        Ok(result)
    }
}
```

**Cache Strategy**:
- TTL: 180 seconds (matching API `s-maxage=180`)
- LRU eviction for memory bounds
- Opt-in via `cache` feature flag

### 3. Builder Pattern

```rust
pub struct PolestarClientBuilder {
    token: Option<String>,
    timeout: Option<Duration>,
    pc_api_base: Option<String>,
    cms_api_base: Option<String>,
}

impl PolestarClientBuilder {
    pub fn new() -> Self;
    pub fn token(mut self, token: impl Into<String>) -> Self;
    pub fn timeout(mut self, timeout: Duration) -> Self;
    pub fn pc_api_base(mut self, url: impl Into<String>) -> Self;
    pub fn build(self) -> Result<PolestarClient>;
}
```

**Benefits**:
- Flexible configuration
- Optional parameters with defaults
- Validation at build time

## Data Flow

### Example: Fetching Telemetry

```
User Code
   │
   ├─> client.get_telemetry("VIN123")
   │
   ▼
PolestarClient::get_telemetry()
   │
   ├─> Construct GraphQL query
   ├─> Add variables: {"vin": "VIN123"}
   │
   ▼
PolestarClient::post_graphql()
   │
   ├─> Build HTTP POST request
   ├─> Set headers (Auth, Content-Type, Origin)
   ├─> Set endpoint: pc-api.polestar.com
   │
   ▼
reqwest::Client::post()
   │
   ├─> Network I/O (async)
   │
   ▼
Response (HTTP 200 + JSON)
   │
   ├─> Check for GraphQL errors
   ├─> Parse JSON → Telemetry struct
   │
   ▼
Return Result<Telemetry>
   │
   ▼
User Code (receives Telemetry)
```

## Security Considerations

### 1. Token Management
- **NEVER** log or print tokens
- **NEVER** commit tokens to version control
- Recommend environment variables or secure vaults
- Consider zeroizing token memory on drop (future enhancement)

### 2. Input Validation
- Validate VIN format (17 characters, alphanumeric)
- Sanitize inputs to prevent injection
- Use prepared queries (GraphQL variables)

### 3. HTTPS Only
- Enforce HTTPS for all API calls
- Use rustls for modern, secure TLS
- Verify server certificates

### 4. Dependency Auditing
- Regular `cargo audit` runs in CI
- Keep dependencies up to date
- Minimize dependency count

## Performance Considerations

### 1. Connection Pooling
`reqwest::Client` automatically pools HTTP connections for reuse.

### 2. Async Runtime
- Uses Tokio for efficient async I/O
- Non-blocking operations throughout
- Can handle thousands of concurrent requests

### 3. Zero-Copy Where Possible
- Use `&str` over `String` in function parameters
- Clone only when necessary
- Leverage Rust's move semantics

### 4. Serialization
- `serde_json` is highly optimized
- Consider `simd-json` for extreme performance needs (future)

## Testing Strategy

### Unit Tests
```rust
#[cfg(test)]
mod tests {
    #[test]
    fn test_battery_deserialization() {
        let json = r#"{"batteryChargeLevelPercentage": 85.5}"#;
        let battery: Battery = serde_json::from_str(json).unwrap();
        assert_eq!(battery.charge_level_percentage, Some(85.5));
    }
}
```

### Integration Tests
```rust
#[tokio::test]
async fn test_get_telemetry() {
    let client = PolestarClient::new(env::var("POLESTAR_USERNAME").unwrap()).unwrap();
    let result = client.get_telemetry("LPSED3KA1NL059445").await;
    assert!(result.is_ok());
}
```

### Mock Tests
```rust
#[tokio::test]
async fn test_network_error_handling() {
    let mut server = mockito::Server::new();
    let mock = server.mock("POST", "/mystar-v2")
        .with_status(500)
        .create();

    // Test client handles 500 error gracefully
}
```

## Versioning Strategy

Follow Semantic Versioning (SemVer):

- **Major**: Breaking API changes
- **Minor**: New features, backward compatible
- **Patch**: Bug fixes, backward compatible

Example:
- `0.1.0` - Initial MVP
- `0.2.0` - Add caching feature
- `1.0.0` - Stable API, production ready

## Future Enhancements

### 1. WebSocket Support
Real-time telemetry updates via WebSocket connections.

### 2. Token Refresh
Automatic token renewal when expired.

### 3. Multi-Vehicle Support
Batch operations for multiple vehicles.

### 4. CLI Tool
Standalone binary for testing and monitoring:
```bash
polestar-cli telemetry --vin VIN123
```

### 5. Metrics & Observability
Integration with metrics libraries (Prometheus, OpenTelemetry).

## References

- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [Tokio Documentation](https://tokio.rs/)
- [GraphQL Best Practices](https://graphql.org/learn/best-practices/)
- [Serde Documentation](https://serde.rs/)
