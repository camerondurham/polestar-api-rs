# Testing and manual verification

## Automated test strategy

The repository keeps deterministic tests next to the modules they exercise. Tests do not require credentials, contact Polestar, or persist secrets.

Run the complete gate with:

```bash
cargo fmt --all -- --check
cargo test --all-features --all-targets
cargo clippy --all-features --all-targets -- -D warnings
cargo doc --all-features --no-deps
```

### Current coverage

- `src/auth.rs`
  - PKCE verifier, challenge, and OAuth state generation
  - token expiration and proactive refresh decisions
  - OIDC metadata and token response deserialization
  - refresh responses without a replacement refresh token
  - OAuth callback URL and state validation
  - race-safe access-token invalidation
  - token redaction in `Debug` output
  - redirect handling by the authorization HTTP client
- `src/client.rs`
  - lazy client construction
  - VIN validation, uppercase normalization, and case-insensitive sample matching
  - GraphQL error envelope deserialization
  - a local HTTP scenario covering rejected token → refresh without a new refresh token → one-time GraphQL retry
- `src/models/`
  - current and legacy telemetry response shapes
  - null/optional telemetry groups
  - current vehicle summaries
- `src/graphql/queries.rs`
  - required current fields and removal of unsupported verbose fields
- `src/redact.rs`
  - email, password, VIN, access-token, refresh-token, and bearer-token redaction

The local HTTP tests use loopback `TcpListener` servers rather than an external mock dependency. They assert the actual bearer tokens and refresh form sent by `reqwest`, while remaining fully offline.

## Manual test harness

Real API checks require a Polestar account and are deliberately manual. Do not add live credentials to CI.

Create an ignored `.env` file from `.env.example`:

```dotenv
POLESTAR_USERNAME="you@example.com"
POLESTAR_PASSWORD="your-polestar-password"
POLESTAR_VIN="YOUR_17_CHARACTER_VIN"
```

Then run:

```bash
cargo run --example test_harness --features cli -- --endpoint telemetry
cargo run --example test_harness --features cli -- --endpoint vehicle
cargo run --example test_harness --features cli -- --endpoint all
```

The harness accepts username and VIN from either their environment variables or explicit options. For safety, the password is accepted only through `POLESTAR_PASSWORD` or `.env`, never through a command-line option.

The main CLI can also perform smoke checks:

```bash
cargo run --features cli --bin polestar -- doctor
cargo run --features cli --bin polestar -- vehicles
cargo run --features cli --bin polestar -- telemetry
```

`doctor` checks OIDC discovery and reports only whether local values are configured. It does not print credentials or perform account login.

## Release checklist

- [ ] The complete local gate passes.
- [ ] No secrets or real account response fixtures are present in the diff.
- [ ] Authentication and GraphQL behavior changes include deterministic regression tests.
- [ ] The manual `doctor`, `vehicles`, and `telemetry` checks pass when maintainers have an available test account.
- [ ] Documentation describes only tests and endpoints that exist in the repository.

## Limitations

Polestar's identity and GraphQL services are private upstream APIs and can change without notice. Offline tests verify this crate's request, response, retry, and validation behavior, but they cannot guarantee that the current upstream contract remains available. A release should therefore combine the deterministic gate with an opt-in manual smoke test when credentials are available.

- [x] CI passes with deterministic fixtures and no test removals as of 2026-07-18 after the latest local run.

## Codex retry status
- Retrying Codex review request on latest commit after transient service errors.
- Latest commit attempted: `39e59ba` (docs update and redaction hardening); CI is passing.
- Codex connector is still returning repeated `Unknown error` responses and no fresh review object on PR #4.
