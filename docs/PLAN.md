# Polestar API Rust Wrapper - Implementation Plan

> **Historical document:** This was the original design plan, not a statement of
> current implementation or test coverage. See the README,
> [`AUTH_IMPLEMENTATION.md`](AUTH_IMPLEMENTATION.md), and
> [`TEST_HARNESS.md`](TEST_HARNESS.md) for current behavior.

## Overview

This document outlines the implementation plan for `polestar-api-rs`, a lightweight Rust wrapper around the Polestar vehicle GraphQL API. The wrapper will provide type-safe access to vehicle telemetry, consumer data, and specifications.

## Project Goals

1. **Type Safety**: Leverage Rust's type system to provide compile-time guarantees
2. **Async-First**: Built on async/await for non-blocking I/O operations
3. **Minimal Dependencies**: Keep the dependency tree lean and maintainable
4. **Well-Tested**: Comprehensive test coverage with mock and integration tests
5. **Developer-Friendly**: Clear documentation and ergonomic API design

## API Endpoints to Support

Based on the documented Polestar API, we'll support three main operations:

### 1. Vehicle Telemetry (`CarTelematicsV2`)
- **Endpoint**: `https://pc-api.polestar.com/eu-north-1/mystar-v2`
- **Data**: Battery status, charging info, odometer, health warnings
- **Priority**: HIGH (most frequently used data)

### 2. Consumer Vehicle Data (`GetConsumerCarsV2`)
- **Endpoint**: `https://pc-api.polestar.com/eu-north-1/mystar-v2`
- **Data**: Vehicle details, specifications, images, features
- **Priority**: MEDIUM (configuration and static data)

### 3. Vehicle Specifications (`getCarSpecifications`)
- **Endpoint**: `https://cms-api.polestar.com/`
- **Data**: Specification metadata, categories, labels
- **Priority**: LOW (mostly static reference data)

## Phase 1: Project Foundation (Week 1)

### 1.1 Project Setup
- [x] Initialize Cargo workspace
- [ ] Configure `Cargo.toml` with dependencies:
  - `tokio` - Async runtime
  - `reqwest` - HTTP client with async support
  - `serde` / `serde_json` - JSON serialization
  - `thiserror` - Error handling
  - `chrono` - DateTime handling
- [ ] Set up project structure:
  ```
  src/
  ├── lib.rs           # Public API surface
  ├── client.rs        # HTTP client and authentication
  ├── models/          # Data models
  │   ├── mod.rs
  │   ├── telemetry.rs
  │   ├── vehicle.rs
  │   └── specs.rs
  ├── graphql/         # GraphQL queries
  │   ├── mod.rs
  │   └── queries.rs
  └── error.rs         # Error types
  tests/
  ├── integration/     # Integration tests
  └── fixtures/        # Test data
  examples/
  └── basic_usage.rs   # Usage examples
  ```
- [ ] Configure `.gitignore` for Rust projects
- [ ] Set up CI/CD with GitHub Actions

### 1.2 Core Types and Models
- [ ] Define `PolestarClient` struct
- [ ] Implement authentication token handling
- [ ] Create error types (`PolestarError`, `ApiError`, `AuthError`)
- [ ] Define core data models:
  - `Vehicle` - Vehicle identification
  - `Battery` - Battery and charging data
  - `Telemetry` - Complete telemetry response
  - `VehicleContent` - Specifications and features

### 1.3 HTTP Client Infrastructure
- [ ] Implement `PolestarClient::new()` with authentication
- [ ] Add request builder with proper headers:
  - Authorization (Bearer token)
  - Content-Type (application/json)
  - Origin (https://www.polestar.com)
- [ ] Implement retry logic with exponential backoff
- [ ] Add timeout configuration
- [ ] Handle HTTP/2 requirements

## Phase 2: Core API Implementation (Week 2)

### 2.1 Telemetry API
- [ ] Define GraphQL query for `CarTelematicsV2`
- [ ] Implement response deserialization
- [ ] Create method: `client.get_telemetry(vin: &str) -> Result<Telemetry>`
- [ ] Handle optional fields gracefully
- [ ] Parse timestamps with chrono

### 2.2 Consumer Data API
- [ ] Define GraphQL query for `GetConsumerCarsV2`
- [ ] Implement nested data structures:
  - Vehicle info (VIN, registration, market)
  - Content (images, specs, features)
  - Motor specifications
  - Dimensions and performance
- [ ] Create method: `client.get_vehicle(vin: &str) -> Result<Vehicle>`
- [ ] Implement image URL handling

### 2.3 Specifications API
- [ ] Define GraphQL query for `getCarSpecifications`
- [ ] Model specification groups and labels
- [ ] Create method: `client.get_specifications(vin: &str) -> Result<Specifications>`
- [ ] Handle CMS API token differences

## Phase 3: Advanced Features (Week 3)

### 3.1 Caching Layer
- [ ] Implement optional in-memory cache
- [ ] Respect API cache headers (s-maxage=180)
- [ ] Add cache invalidation methods
- [ ] Make caching opt-in via feature flag

### 3.2 Rate Limiting
- [ ] Implement rate limiter to prevent API abuse
- [ ] Add configurable request throttling
- [ ] Handle 429 (Too Many Requests) responses

### 3.3 Token Management
- [ ] Add token refresh mechanism (if supported)
- [ ] Implement token expiration detection
- [ ] Add callback for token renewal
- [ ] Secure token storage patterns

### 3.4 Builder Pattern
- [ ] Create `PolestarClientBuilder` for configuration:
  ```rust
  let client = PolestarClient::builder()
      .token("USERNAME_PASSWORD")
      .timeout(Duration::from_secs(30))
      .enable_cache()
      .build()?;
  ```

## Phase 4: Testing Infrastructure (Week 4)

### 4.1 Unit Tests
- [ ] Test data model serialization/deserialization
- [ ] Test error handling paths
- [ ] Test query construction
- [ ] Mock HTTP responses with `mockito` or `wiremock`

### 4.2 Integration Tests
- [ ] Test against real API (with valid token)
- [ ] Test all three endpoints
- [ ] Validate response parsing
- [ ] Test error scenarios (invalid VIN, auth failure)

### 4.3 Test Harness
- [ ] Create CLI tool for API testing:
  ```bash
  # Credentials and VIN are loaded from an ignored .env file.
  cargo run --example test_harness --features cli -- --endpoint telemetry
  ```
- [ ] Add response validation
- [ ] Pretty-print JSON output
- [ ] Add timing/performance metrics
- [ ] Support environment variable configuration

### 4.4 Documentation Tests
- [ ] Add doc examples that compile
- [ ] Test code snippets in README
- [ ] Validate example code

## Phase 5: Documentation & Polish (Week 5)

### 5.1 API Documentation
- [ ] Comprehensive rustdoc comments
- [ ] Add module-level documentation
- [ ] Create usage examples
- [ ] Document authentication flow

### 5.2 User Documentation
- [ ] Update README with:
  - Installation instructions
  - Quick start guide
  - Authentication setup
  - Code examples
  - API coverage table
- [ ] Create CONTRIBUTING.md
- [ ] Add LICENSE file (MIT or Apache-2.0)
- [ ] Create CHANGELOG.md

### 5.3 Examples
- [ ] `basic_telemetry.rs` - Fetch battery status
- [ ] `vehicle_info.rs` - Get vehicle specifications
- [ ] `monitor_charging.rs` - Poll charging status
- [ ] `async_example.rs` - Advanced async usage

## Dependencies Overview

### Core Dependencies
```toml
[dependencies]
tokio = { version = "1.35", features = ["full"] }
reqwest = { version = "0.11", features = ["json", "rustls-tls"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
thiserror = "1.0"
chrono = { version = "0.4", features = ["serde"] }

[dev-dependencies]
tokio-test = "0.4"
wiremock = "0.5"
mockito = "1.0"
```

### Optional Dependencies (Feature Flags)
```toml
[features]
default = []
cache = ["moka"]
cli = ["clap", "env_logger"]
```

## Success Criteria

### Minimum Viable Product (MVP)
- ✅ Successfully authenticate with Bearer token
- ✅ Fetch telemetry data for a given VIN
- ✅ Parse and return battery status
- ✅ Handle common errors gracefully
- ✅ Basic integration test suite
- ✅ README with usage examples

### Version 1.0 Goals
- ✅ All three API endpoints implemented
- ✅ Comprehensive error handling
- ✅ 80%+ test coverage
- ✅ Complete API documentation
- ✅ Working test harness
- ✅ Published to crates.io
- ✅ CI/CD pipeline (tests, clippy, fmt)

## Risk Mitigation

### Authentication Challenges
- **Risk**: Token acquisition not documented
- **Mitigation**: Document token retrieval process separately, accept token as input
- **Reference**: Study pypolestar implementation

### API Changes
- **Risk**: Polestar may change API without notice
- **Mitigation**: Version lock, add integration tests, document API version

### Rate Limiting
- **Risk**: Unknown rate limits could cause failures
- **Mitigation**: Implement conservative rate limiting, handle 429 gracefully

### GraphQL Complexity
- **Risk**: Complex nested structures hard to model
- **Mitigation**: Start with essential fields, use `#[serde(skip_serializing_if)]` for optionals

## Timeline Summary

| Phase | Duration | Deliverable |
|-------|----------|-------------|
| Phase 1 | Week 1 | Project foundation, core types |
| Phase 2 | Week 2 | Working API client for all endpoints |
| Phase 3 | Week 3 | Advanced features (caching, rate limiting) |
| Phase 4 | Week 4 | Complete test suite and harness |
| Phase 5 | Week 5 | Documentation and examples |

**Total Estimated Time**: 5 weeks for v1.0

## Next Steps

1. Review and approve this plan
2. Set up initial Cargo project structure
3. Implement Phase 1.1 (Project Setup)
4. Begin Phase 1.2 (Core Types and Models)

## References

- [Polestar API Reference](./resources/Polestar-API-Reference.md)
- [pypolestar API](https://github.com/pypolestar/polestar_api) - Python reference implementation
- [GraphQL Specification](https://spec.graphql.org/)
- [Rust Async Book](https://rust-lang.github.io/async-book/)
