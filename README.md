# polestar-api

An unofficial, type-safe Rust client for reading Polestar account vehicles and
cloud telemetry.

> This project is not affiliated with or supported by Polestar. The underlying
> API is private and can change without notice.

[![CI](https://github.com/camerondurham/polestar-api-rs/actions/workflows/rust.yml/badge.svg)](https://github.com/camerondurham/polestar-api-rs/actions/workflows/rust.yml)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)]()

## What it reads

- Battery percentage, charging status, estimated time to full, and estimated range
- Odometer
- Service interval and available health warnings
- Vehicle summaries and VIN discovery for the authenticated account

Individual telemetry groups are optional because Polestar does not return every
signal for every vehicle model. This is snapshot data from Polestar's cloud, not
a direct or real-time connection to the car.

The client does not currently expose location, doors, climate controls, charge
target changes, or other remote-control operations.

## What you need

1. The email address for the Polestar ID used in the mobile app
2. That account's password
3. The VIN only when the account contains multiple vehicles; a single VIN is
   discovered automatically
4. Outbound HTTPS access to Polestar's identity and vehicle API services

No API key, OAuth client secret, or developer account is required. Authentication
uses the same public OIDC/PKCE flow as Polestar's web application.

## Run the CLI

Rust 1.85 or newer is required.

```bash
cp .env.example .env
```

Edit `.env` and set at least:

```dotenv
POLESTAR_USERNAME="you@example.com"
POLESTAR_PASSWORD="your-polestar-password"

# Optional for an account with one vehicle
POLESTAR_VIN="YOUR_17_CHARACTER_VIN"
```

`.env` is ignored by Git. Do not commit or paste these credentials into logs,
issues, or chat. The CLI intentionally accepts the password only through
`POLESTAR_PASSWORD` (including via `.env`), not as a command-line argument.

Verify the public auth endpoint and see which local values are configured:

```bash
cargo run --features cli --bin polestar -- doctor
```

Fetch telemetry:

```bash
cargo run --features cli --bin polestar -- telemetry
```

Fetch machine-readable telemetry:

```bash
cargo run --features cli --bin polestar -- telemetry --json
```

Use imperial distance units (including JSON) with `--imperial`:

```bash
cargo run --features cli --bin polestar -- telemetry --imperial
cargo run --features cli --bin polestar -- telemetry --json --imperial
```

List account vehicles (full VINs are emitted only with `--json`):

```bash
cargo run --features cli --bin polestar -- vehicles
cargo run --features cli --bin polestar -- vehicles --json
```

The telemetry command is the default, so this is equivalent:

```bash
cargo run --features cli --bin polestar
```

## Use the library

```rust,no_run
use polestar_api::PolestarClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = PolestarClient::new(
        std::env::var("POLESTAR_USERNAME")?,
        std::env::var("POLESTAR_PASSWORD")?,
    )?;

    let vehicles = client.get_vehicles().await?;
    let vehicle = vehicles.first().ok_or("no vehicle in account")?;
    let telemetry = client.get_telemetry(&vehicle.vin).await?;

    if let Some(charge) = telemetry
        .battery
        .and_then(|battery| battery.charge_level_percentage)
    {
        println!("Battery: {charge}%");
    }

    Ok(())
}
```

Tokens are kept in memory, refreshed before expiry, and are not persisted by the
crate. The client serializes concurrent login/refresh attempts so cloned clients
can safely share one authentication state.

## Development

Run the complete local gate:

```bash
cargo fmt --all -- --check
cargo test --all-features --all-targets
cargo clippy --all-features --all-targets -- -D warnings
cargo doc --all-features --no-deps
```

Live telemetry cannot be tested without a real Polestar account. Deterministic
local tests cover current response shapes, VIN normalization, OAuth callback
validation, refresh responses without replacement refresh tokens, and the full
HTTP 401 → token refresh → one-time GraphQL retry path. Real-account checks are
manual and must not run in CI.

Additional project notes are under [`docs/`](docs/), and the captured historical
API reference is in [`resources/Polestar-API-Reference.md`](resources/Polestar-API-Reference.md).

## Compatibility

The current GraphQL fields and OIDC flow are aligned with the maintained
[`pypolestar`](https://github.com/pypolestar/pypolestar) implementation. The old
verbose vehicle query is no longer considered stable; `get_vehicle_verbose()` is
retained as a compatibility method and now returns the supported vehicle summary.

`get_vehicles_verbose()` now also accepts schema drift in performance-upgrade
payloads by probing multiple query variants, including scalar and object shapes for
`performanceOptimization` and `performanceOptimizationSpecification`.

## License

Licensed under either the MIT License or Apache License 2.0, at your option.
