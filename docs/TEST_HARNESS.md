# Polestar API Test Harness

## Overview

This document describes the test harness for validating the Polestar API Rust wrapper. The test harness includes unit tests, integration tests, mock tests, and a CLI tool for manual API verification.

## Testing Philosophy

### 1. Test Pyramid

```
         ┌─────────────┐
         │   Manual    │  ← CLI test harness
         │  Testing    │
         └─────────────┘
        ┌───────────────┐
        │  Integration  │  ← Real API tests
        │     Tests     │
        └───────────────┘
       ┌─────────────────┐
       │   Mock Tests    │  ← HTTP mocking
       └─────────────────┘
      ┌───────────────────┐
      │    Unit Tests     │  ← Model validation
      └───────────────────┘
```

### 2. Coverage Goals
- **Unit Tests**: 80%+ coverage
- **Integration Tests**: All endpoints covered
- **Mock Tests**: All error paths covered
- **Manual Tests**: CLI tool for exploratory testing

## 1. Unit Tests

### 1.1 Model Deserialization Tests

Test that API responses correctly deserialize into Rust types.

**Location**: `src/models/telemetry.rs`, `src/models/vehicle.rs`, `src/models/specs.rs`

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_battery_full_data() {
        let json = r#"{
            "vin": "TEST123",
            "timestamp": {"seconds": "1234567890", "nanos": 0},
            "batteryChargeLevelPercentage": 85,
            "chargingStatus": "CHARGING_STATUS_CHARGING",
            "estimatedChargingTimeToFullMinutes": 45,
            "estimatedDistanceToEmptyKm": 320,
            "estimatedDistanceToEmptyMiles": 200
        }"#;

        let battery: Battery = serde_json::from_str(json).unwrap();
        assert_eq!(battery.charge_level_percentage, Some(85));
        assert_eq!(battery.charge_status, Some("CHARGING_STATUS_CHARGING".to_string()));
        assert_eq!(battery.estimated_charging_time_minutes, Some(45));
        assert_eq!(battery.estimated_distance_to_empty_km, Some(320));
    }

    #[test]
    fn test_battery_partial_data() {
        let json = r#"{
            "vin": "TEST123",
            "timestamp": {"seconds": "1234567890", "nanos": 0},
            "batteryChargeLevelPercentage": 50
        }"#;

        let battery: Battery = serde_json::from_str(json).unwrap();
        assert_eq!(battery.charge_level_percentage, Some(50));
        assert_eq!(battery.charge_status, None);
    }

    #[test]
    fn test_battery_null_fields() {
        let json = r#"{
            "vin": "TEST123",
            "timestamp": {"seconds": "1234567890", "nanos": 0},
            "batteryChargeLevelPercentage": null,
            "chargingStatus": "CHARGING_STATUS_IDLE"
        }"#;

        let battery: Battery = serde_json::from_str(json).unwrap();
        assert_eq!(battery.charge_level_percentage, None);
        assert_eq!(battery.charge_status, Some("CHARGING_STATUS_IDLE".to_string()));
    }

    #[test]
    fn test_telemetry_complete_response() {
        let json = include_str!("../../tests/fixtures/telemetry_response.json");
        let telemetry: Telemetry = serde_json::from_str(json).unwrap();

        // Validate structure
        assert!(telemetry.battery.charge_level_percentage.is_some());
        assert!(telemetry.odometer.odometer_meters.is_some());
    }
}
```

### 1.2 Error Handling Tests

Test that errors are properly constructed and converted.

**Location**: `src/error.rs`

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_auth_error_display() {
        let err = PolestarError::AuthError("Invalid token".to_string());
        assert_eq!(err.to_string(), "Authentication failed: Invalid token");
    }

    #[test]
    fn test_graphql_error_construction() {
        let err = PolestarError::GraphQLError("Field not found".to_string());
        assert!(matches!(err, PolestarError::GraphQLError(_)));
    }

    #[test]
    fn test_reqwest_error_conversion() {
        // Test that reqwest::Error converts to PolestarError::NetworkError
    }
}
```

### 1.3 Query Construction Tests

Test that GraphQL queries are correctly formatted.

**Location**: `src/graphql/queries.rs`

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_telemetry_query_structure() {
        assert!(CAR_TELEMETRICS_V2.contains("CarTelematicsV2"));
        assert!(CAR_TELEMETRICS_V2.contains("$vin: String!"));
        assert!(CAR_TELEMETRICS_V2.contains("batteryChargeLevelPercentage"));
    }

    #[test]
    fn test_query_variable_substitution() {
        let variables = serde_json::json!({
            "vin": "VIN1234567890"
        });

        // Validate JSON structure
        assert_eq!(variables["vin"], "VIN1234567890");
    }
}
```

## 2. Mock Tests

### 2.1 HTTP Mock Setup

Use `wiremock` or `mockito` to simulate API responses.

**Location**: `tests/mock_tests.rs`

```rust
use wiremock::{Mock, MockServer, ResponseTemplate};
use wiremock::matchers::{method, path, header};

#[tokio::test]
async fn test_successful_telemetry_request() {
    let mock_server = MockServer::start().await;

    // Mock successful response
    Mock::given(method("POST"))
        .and(path("/eu-north-1/mystar-v2"))
        .and(header("authorization", "Bearer test_token"))
        .respond_with(ResponseTemplate::new(200)
            .set_body_json(serde_json::json!({
                "data": {
                    "getCarTelematicsV2": {
                        "data": {
                            "batteryChargeLevelPercentage": 75.0,
                            "batteryChargeStatus": "idle"
                        }
                    }
                }
            })))
        .mount(&mock_server)
        .await;

    // Test client against mock
    let client = PolestarClient::builder()
        .token("test_token")
        .pc_api_base(&mock_server.uri())
        .build()
        .unwrap();

    let result = client.get_telemetry("VIN123").await;
    assert!(result.is_ok());

    let telemetry = result.unwrap();
    assert_eq!(telemetry.battery.charge_level_percentage, Some(75.0));
}

#[tokio::test]
async fn test_authentication_failure() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/eu-north-1/mystar-v2"))
        .respond_with(ResponseTemplate::new(401)
            .set_body_json(serde_json::json!({
                "errors": [{
                    "message": "Unauthorized",
                    "errorType": "Unauthorized"
                }]
            })))
        .mount(&mock_server)
        .await;

    let client = PolestarClient::builder()
        .token("invalid_token")
        .pc_api_base(&mock_server.uri())
        .build()
        .unwrap();

    let result = client.get_telemetry("VIN123").await;
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), PolestarError::AuthError(_)));
}

#[tokio::test]
async fn test_network_timeout() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .respond_with(ResponseTemplate::new(200)
            .set_delay(std::time::Duration::from_secs(10)))
        .mount(&mock_server)
        .await;

    let client = PolestarClient::builder()
        .token("test_token")
        .timeout(std::time::Duration::from_secs(1))
        .pc_api_base(&mock_server.uri())
        .build()
        .unwrap();

    let result = client.get_telemetry("VIN123").await;
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), PolestarError::NetworkError(_)));
}

#[tokio::test]
async fn test_graphql_error_response() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .respond_with(ResponseTemplate::new(200)
            .set_body_json(serde_json::json!({
                "data": null,
                "errors": [{
                    "message": "VIN not found",
                    "path": ["getCarTelematicsV2"]
                }]
            })))
        .mount(&mock_server)
        .await;

    let client = PolestarClient::builder()
        .token("test_token")
        .pc_api_base(&mock_server.uri())
        .build()
        .unwrap();

    let result = client.get_telemetry("INVALID_VIN").await;
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), PolestarError::GraphQLError(_)));
}

#[tokio::test]
async fn test_rate_limiting() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .respond_with(ResponseTemplate::new(429)
            .insert_header("retry-after", "60"))
        .mount(&mock_server)
        .await;

    let client = PolestarClient::builder()
        .token("test_token")
        .pc_api_base(&mock_server.uri())
        .build()
        .unwrap();

    let result = client.get_telemetry("VIN123").await;
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), PolestarError::RateLimitExceeded));
}
```

## 3. Integration Tests

### 3.1 Real API Tests

Test against the actual Polestar API (requires valid credentials).

**Location**: `tests/integration_tests.rs`

```rust
use polestar_api_rs::PolestarClient;
use std::env;

// Helper to skip tests if credentials not available
fn get_test_client() -> Option<PolestarClient> {
    let username = env::var("POLESTAR_USERNAME").ok()?;
    let password = env::var("POLESTAR_PASSWORD").ok()?;
    PolestarClient::new(username, password).ok()
}

fn get_test_vin() -> String {
    env::var("POLESTAR_VIN")
        .unwrap_or_else(|_| "VIN1234567890".to_string())
}

#[tokio::test]
async fn test_real_telemetry_api() {
    let Some(client) = get_test_client() else {
        eprintln!("Skipping test: POLESTAR_USERNAME not set");
        return;
    };

    let vin = get_test_vin();
    let result = client.get_telemetry(&vin).await;

    match result {
        Ok(telemetry) => {
            println!("Telemetry: {:#?}", telemetry);
            // Validate response structure
            assert!(telemetry.battery.charge_level_percentage.is_some() ||
                    telemetry.battery.charge_status.is_some());
        }
        Err(e) => {
            panic!("API call failed: {}", e);
        }
    }
}

#[tokio::test]
async fn test_real_vehicle_api() {
    let Some(client) = get_test_client() else {
        eprintln!("Skipping test: POLESTAR_USERNAME not set");
        return;
    };

    let vin = get_test_vin();
    let result = client.get_vehicle(&vin).await;

    match result {
        Ok(vehicle) => {
            println!("Vehicle: {:#?}", vehicle);
            assert_eq!(vehicle.vin, vin);
            assert!(vehicle.content.model.name.is_some());
        }
        Err(e) => {
            panic!("API call failed: {}", e);
        }
    }
}

#[tokio::test]
async fn test_real_specifications_api() {
    let Some(client) = get_test_client() else {
        eprintln!("Skipping test: POLESTAR_USERNAME not set");
        return;
    };

    let vin = get_test_vin();
    let result = client.get_specifications(&vin).await;

    match result {
        Ok(specs) => {
            println!("Specifications: {:#?}", specs);
            assert!(!specs.groups.is_empty());
        }
        Err(e) => {
            panic!("API call failed: {}", e);
        }
    }
}

#[tokio::test]
async fn test_invalid_vin() {
    let Some(client) = get_test_client() else {
        eprintln!("Skipping test: POLESTAR_USERNAME not set");
        return;
    };

    let result = client.get_telemetry("INVALID").await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_concurrent_requests() {
    let Some(client) = get_test_client() else {
        eprintln!("Skipping test: POLESTAR_USERNAME not set");
        return;
    };

    let vin = get_test_vin();

    // Make 5 concurrent requests
    let futures: Vec<_> = (0..5)
        .map(|_| client.get_telemetry(&vin))
        .collect();

    let results = futures::future::join_all(futures).await;

    // All should succeed
    for result in results {
        assert!(result.is_ok());
    }
}
```

## 4. CLI Test Harness

### 4.1 Command-Line Tool

A standalone binary for manual API testing and validation.

**Location**: `examples/test_harness.rs`

```rust
use clap::Parser;
use polestar_api_rs::PolestarClient;
use std::time::Instant;

#[derive(Parser)]
#[command(name = "polestar-test-harness")]
#[command(about = "Test harness for Polestar API")]
struct Args {
    /// Polestar account username (email)
    #[arg(short, long, env = "POLESTAR_USERNAME")]
    username: String,

    /// Polestar account password
    #[arg(short, long, env = "POLESTAR_PASSWORD")]
    password: String,

    /// Vehicle VIN
    #[arg(short, long, env = "POLESTAR_VIN")]
    vin: String,

    /// Endpoint to test
    #[arg(short, long, value_enum)]
    endpoint: Endpoint,

    /// Pretty-print JSON output
    #[arg(short, long, default_value_t = true)]
    pretty: bool,

    /// Show timing information
    #[arg(long, default_value_t = true)]
    timing: bool,
}

#[derive(clap::ValueEnum, Clone)]
enum Endpoint {
    Telemetry,
    Vehicle,
    Specifications,
    All,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let args = Args::parse();

    let client = PolestarClient::new(args.username, args.password)?;

    match args.endpoint {
        Endpoint::Telemetry => {
            test_telemetry(&client, &args.vin, args.pretty, args.timing).await?;
        }
        Endpoint::Vehicle => {
            test_vehicle(&client, &args.vin, args.pretty, args.timing).await?;
        }
        Endpoint::Specifications => {
            test_specifications(&client, &args.vin, args.pretty, args.timing).await?;
        }
        Endpoint::All => {
            test_telemetry(&client, &args.vin, args.pretty, args.timing).await?;
            println!("\n---\n");
            test_vehicle(&client, &args.vin, args.pretty, args.timing).await?;
            println!("\n---\n");
            test_specifications(&client, &args.vin, args.pretty, args.timing).await?;
        }
    }

    Ok(())
}

async fn test_telemetry(
    client: &PolestarClient,
    vin: &str,
    pretty: bool,
    timing: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Testing Telemetry API...");

    let start = Instant::now();
    let result = client.get_telemetry(vin).await?;
    let elapsed = start.elapsed();

    if pretty {
        println!("{}", serde_json::to_string_pretty(&result)?);
    } else {
        println!("{}", serde_json::to_string(&result)?);
    }

    if timing {
        println!("\nElapsed: {:?}", elapsed);
    }

    // Validation
    println!("\n✓ Telemetry API test passed");
    if let Some(charge) = result.battery.charge_level_percentage {
        println!("  Battery: {:.1}%", charge);
    }

    Ok(())
}

async fn test_vehicle(
    client: &PolestarClient,
    vin: &str,
    pretty: bool,
    timing: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Testing Vehicle API...");

    let start = Instant::now();
    let result = client.get_vehicle(vin).await?;
    let elapsed = start.elapsed();

    if pretty {
        println!("{}", serde_json::to_string_pretty(&result)?);
    } else {
        println!("{}", serde_json::to_string(&result)?);
    }

    if timing {
        println!("\nElapsed: {:?}", elapsed);
    }

    println!("\n✓ Vehicle API test passed");
    println!("  VIN: {}", result.vin);

    Ok(())
}

async fn test_specifications(
    client: &PolestarClient,
    vin: &str,
    pretty: bool,
    timing: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Testing Specifications API...");

    let start = Instant::now();
    let result = client.get_specifications(vin).await?;
    let elapsed = start.elapsed();

    if pretty {
        println!("{}", serde_json::to_string_pretty(&result)?);
    } else {
        println!("{}", serde_json::to_string(&result)?);
    }

    if timing {
        println!("\nElapsed: {:?}", elapsed);
    }

    println!("\n✓ Specifications API test passed");

    Ok(())
}
```

### 4.2 Usage Examples

```bash
# Test telemetry endpoint
cargo run --example test_harness -- \
  --token "USERNAME_PASSWORD" \
  --vin "VIN1234567890" \
  --endpoint telemetry

# Test all endpoints
cargo run --example test_harness -- \
  --token "USERNAME_PASSWORD" \
  --vin "VIN1234567890" \
  --endpoint all \
  --pretty

# Use environment variables
export POLESTAR_USERNAME="USERNAME_PASSWORD"
export POLESTAR_VIN="VIN1234567890"
cargo run --example test_harness -- --endpoint all

# Disable pretty printing for JSON parsing
cargo run --example test_harness -- \
  --endpoint telemetry \
  --no-pretty | jq '.battery.charge_level_percentage'
```

## 5. Test Fixtures

### 5.1 Sample Data

Store sample API responses for consistent testing.

**Location**: `tests/fixtures/`

```
tests/fixtures/
├── telemetry_response.json
├── vehicle_response.json
├── specifications_response.json
├── error_401_unauthorized.json
├── error_graphql.json
└── error_rate_limit.json
```

**Example**: `tests/fixtures/telemetry_response.json`
```json
{
  "data": {
    "getCarTelematicsV2": {
      "data": {
        "batteryChargeLevelPercentage": 85.5,
        "batteryChargeStatus": "charging",
        "chargingPowerWatts": 7400,
        "estimatedChargingTimeToFullMinutes": 45,
        "estimatedDistanceToEmptyKm": 320.5,
        "odometerMeters": 12500000,
        "averageSpeedKmPerHour": 65.3
      }
    }
  }
}
```

## 6. Continuous Integration

### 6.1 GitHub Actions Workflow

**Location**: `.github/workflows/test.yml`

```yaml
name: Tests

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          profile: minimal
          override: true

      - name: Run unit tests
        run: cargo test --lib

      - name: Run mock tests
        run: cargo test --test mock_tests

      - name: Run integration tests (if credentials available)
        env:
          POLESTAR_USERNAME: ${{ secrets.POLESTAR_USERNAME }}
          POLESTAR_VIN: ${{ secrets.POLESTAR_VIN }}
        run: cargo test --test integration_tests
        continue-on-error: true

      - name: Check code coverage
        uses: actions-rs/tarpaulin@v0.1
        with:
          args: '--ignore-tests --out Lcov'

  lint:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          components: clippy, rustfmt

      - name: Run clippy
        run: cargo clippy -- -D warnings

      - name: Check formatting
        run: cargo fmt -- --check
```

## 7. Running Tests

### 7.1 All Tests
```bash
cargo test
```

### 7.2 Unit Tests Only
```bash
cargo test --lib
```

### 7.3 Integration Tests
```bash
# Requires credentials
export POLESTAR_USERNAME="your_credentials"
export POLESTAR_VIN="your_vin"
cargo test --test integration_tests
```

### 7.4 Mock Tests
```bash
cargo test --test mock_tests
```

### 7.5 With Coverage
```bash
cargo tarpaulin --out Html --output-dir coverage
```

## 8. Test Checklist

Before releasing a new version:

- [ ] All unit tests pass
- [ ] All mock tests pass
- [ ] Integration tests pass (if credentials available)
- [ ] Test harness CLI works for all endpoints
- [ ] Code coverage >= 80%
- [ ] Clippy shows no warnings
- [ ] Code is formatted (`cargo fmt`)
- [ ] Documentation examples compile
- [ ] README examples tested

## Summary

This comprehensive test harness ensures the Polestar API wrapper is:
- **Reliable**: Extensive test coverage catches bugs early
- **Maintainable**: Mock tests prevent API dependency in CI
- **Verifiable**: CLI tool allows manual validation
- **Production-Ready**: Integration tests confirm real-world functionality
