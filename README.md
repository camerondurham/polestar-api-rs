# polestar-api

A lightweight, type-safe Rust wrapper for the Polestar vehicle GraphQL API.

> **Disclaimer**: This library is not affiliated with nor supported by Polestar.

[![CI](https://github.com/camerondurham/polestar-api/workflows/Tests/badge.svg)](https://github.com/camerondurham/polestar-api/actions)
[![Crates.io](https://img.shields.io/crates/v/polestar-api.svg)](https://crates.io/crates/polestar-api)
[![Documentation](https://docs.rs/polestar-api/badge.svg)](https://docs.rs/polestar-api)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)]()

## Supported Endpoints

- [x] **Vehicle Telemetry** - Battery status, charging info, odometer, health
- [x] **Consumer Vehicle Data** - Vehicle specifications, features, images
- [ ] **Specifications** - Detailed vehicle specification metadata (planned)

## Quick Start

### Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
polestar-api = "0.1"
tokio = { version = "1", features = ["full"] }
```

### Usage

```rust
use polestar_api::PolestarClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a client with your Polestar account credentials
    let client = PolestarClient::new("your_email@example.com", "your_password")?;

    // Fetch vehicle telemetry
    let telemetry = client.get_telemetry("YOUR_VIN").await?;

    println!("Battery: {}%",
        telemetry.battery.charge_level_percentage.unwrap_or(0));

    Ok(())
}
```

## Authentication

The client authenticates using your Polestar account credentials (username and password). The library handles the web-based login flow to obtain access tokens from the Polestar authentication service.

> **Note**: Authentication implementation is currently in progress. The client accepts username and password credentials which will be used to authenticate with the Polestar API. See the [pypolestar reference implementation](https://github.com/pypolestar/polestar-api) for examples of the authentication flow.

**Security**: Never commit credentials to version control. Use environment variables:

```bash
export POLESTAR_USERNAME="your_email@example.com"
export POLESTAR_PASSWORD="your_password"
export POLESTAR_VIN="your_vin"
```

**Redaction helper**: The crate includes a small redaction utility you can run on any log message to mask emails, passwords, VINs, and access/refresh tokens before they hit logs:

```rust
use polestar_api::redact::redact_str;

let message = format!("Refreshing token for {}", username);
log::warn!("{}", redact_str(&message));
```

## Examples

### Get Battery Status

```rust
let client = PolestarClient::new("user@example.com", "password")?;
let telemetry = client.get_telemetry("VIN").await?;

if let Some(charge) = telemetry.battery.charge_level_percentage {
    println!("Battery: {}%", charge);
}

if let Some(status) = &telemetry.battery.charge_status {
    println!("Status: {}", status);
}
```

**Example Output:**
```
Battery: 69%
Status: CHARGING_STATUS_IDLE
Time to Full: 0 minutes
Estimated Range: 230 km
Total Distance: 67712.9 km
```

### Get Vehicle Information

```rust
let client = PolestarClient::new("user@example.com", "password")?;
let vehicle = client.get_vehicle("VIN").await?;

println!("Model: {}", vehicle.content.model.name.unwrap_or_default());
println!("Market: {}", vehicle.market.unwrap_or_default());
```

See the [`examples/`](examples/) directory for more complete examples.

## Documentation

- [Implementation Plan](PLAN.md) - Development roadmap
- [Architecture](ARCHITECTURE.md) - Technical design and decisions
- [Test Harness](TEST_HARNESS.md) - Testing strategy and tools
- [API Reference](resources/Polestar-API-Reference.md) - Polestar API documentation

## Testing

Run all tests:
```bash
cargo test
```

Run integration tests (requires credentials):
```bash
export POLESTAR_USERNAME="your_email@example.com"
export POLESTAR_PASSWORD="your_password"
export POLESTAR_VIN="your_vin"
cargo test --test integration_tests
```

Use the test harness CLI:
```bash
cargo run --example test_harness --features cli -- \
  --username "$POLESTAR_USERNAME" \
  --password "$POLESTAR_PASSWORD" \
  --vin "$POLESTAR_VIN" \
  --endpoint all
```

## Development

The repository includes a git pre-commit hook that automatically strips trailing whitespace from all staged files. The hook is located at `.git/hooks/pre-commit` and runs automatically on each commit.

## Development Status

**Current Version**: 0.1.0 (Planning Phase)

This project is in active development. The current release includes:
- ✅ Comprehensive planning documentation
- ✅ Architecture design
- ✅ Test harness specification
- 🚧 Implementation in progress

See [PLAN.md](PLAN.md) for the complete roadmap.

## Contributing

Contributions are welcome! Please open an issue or pull request on GitHub. Contribution guidelines will be added in a future release.

## License

Dual-licensed under either of:

- Apache License, Version 2.0 ([apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))
- MIT license ([opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))

at your option.

## Disclaimer

This is an unofficial, community-maintained wrapper for the Polestar API. It is not affiliated with, endorsed by, or supported by Polestar or Volvo Car Corporation.

## Acknowledgments

- [pypolestar](https://github.com/pypolestar/polestar-api) - Python reference implementation
- The Rust community for excellent async tooling

## References

- [Polestar Official Site](https://www.polestar.com)
- [GraphQL Specification](https://spec.graphql.org/)
