# polestar-api-rs

A lightweight, type-safe Rust wrapper for the Polestar vehicle GraphQL API.

[![CI](https://github.com/camerondurham/polestar-api-rs/workflows/Tests/badge.svg)](https://github.com/camerondurham/polestar-api-rs/actions)
[![Crates.io](https://img.shields.io/crates/v/polestar-api-rs.svg)](https://crates.io/crates/polestar-api-rs)
[![Documentation](https://docs.rs/polestar-api-rs/badge.svg)](https://docs.rs/polestar-api-rs)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)]()

## Features

- **Type-safe**: Leverages Rust's type system for compile-time guarantees
- **Async-first**: Built on async/await with Tokio runtime
- **Comprehensive**: Supports all major Polestar API endpoints
- **Well-tested**: Extensive unit, mock, and integration tests
- **Documented**: Full API documentation and examples

## Supported Endpoints

- ✅ **Vehicle Telemetry** - Battery status, charging info, odometer
- ✅ **Consumer Vehicle Data** - Vehicle specifications, features, images
- ✅ **Specifications** - Detailed vehicle specification metadata

## Quick Start

### Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
polestar-api-rs = "0.1"
tokio = { version = "1", features = ["full"] }
```

### Usage

```rust
use polestar_api_rs::PolestarClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a client with your authentication token
    let client = PolestarClient::new("YOUR_POLESTAR_TOKEN")?;

    // Fetch vehicle telemetry
    let telemetry = client.get_telemetry("YOUR_VIN").await?;

    println!("Battery: {:.1}%",
        telemetry.battery.charge_level_percentage.unwrap_or(0.0));

    Ok(())
}
```

## Authentication

You'll need a valid Polestar API token. Token acquisition is handled through the Polestar web interface or mobile app authentication flow.

> **Note**: Detailed authentication documentation is planned for a future release. For now, you'll need to obtain a token through existing Polestar authentication methods. See the [pypolestar reference implementation](https://github.com/pypolestar/polestar_api) for token extraction examples.

**Security**: Never commit tokens to version control. Use environment variables:

```bash
export POLESTAR_TOKEN="your_token_here"
export POLESTAR_VIN="your_vin_here"
```

## Examples

### Get Battery Status

```rust
let telemetry = client.get_telemetry("VIN").await?;

if let Some(charge) = telemetry.battery.charge_level_percentage {
    println!("Battery: {:.1}%", charge);
}

if let Some(status) = telemetry.battery.charge_status {
    println!("Status: {}", status);
}
```

### Get Vehicle Information

```rust
let vehicle = client.get_vehicle("VIN").await?;

println!("Model: {}", vehicle.content.model.name.unwrap_or_default());
println!("Market: {}", vehicle.market.unwrap_or_default());
```

### Get Specifications

```rust
let specs = client.get_specifications("VIN").await?;

for group in specs.groups {
    println!("Spec Group: {}", group.label);
}
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
export POLESTAR_TOKEN="your_token"
export POLESTAR_VIN="your_vin"
cargo test --test integration_tests
```

Use the test harness CLI:
```bash
cargo run --example test_harness --features cli -- \
  --token "$POLESTAR_TOKEN" \
  --vin "$POLESTAR_VIN" \
  --endpoint all
```

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

- [pypolestar](https://github.com/pypolestar/polestar_api) - Python reference implementation
- The Rust community for excellent async tooling

## References

- [Polestar Official Site](https://www.polestar.com)
- [GraphQL Specification](https://spec.graphql.org/)
