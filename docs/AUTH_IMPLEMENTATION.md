# Authentication implementation

## Overview

The client uses Polestar's public OAuth 2.0/OIDC authorization-code flow with PKCE. Authentication is lazy: constructing `PolestarClient` performs no network I/O, and the first API request starts authentication.

Tokens are held only in memory. Cloned clients share an `AuthState`, and an async mutex serializes login and refresh work so concurrent callers do not perform duplicate authentication flows.

## Flow

1. Fetch and cache OIDC discovery metadata.
2. Generate a random PKCE verifier, SHA-256 challenge, and OAuth `state` value.
3. Request the provider login resume path.
4. Submit account credentials and, when required, terms confirmation.
5. Validate that the final redirect uses the configured callback URL and contains the exact expected `state`.
6. Exchange the authorization code and verifier for tokens.
7. Refresh the access token before expiry.
8. If the vehicle API rejects an otherwise unexpired token with HTTP 401, mark only that access token as expired, refresh or log in, and retry the GraphQL request once.

Refresh responses may omit `refresh_token`, as allowed by OAuth. The client retains the previous refresh token in that case. Access and refresh tokens are redacted from their `Debug` output.

## Error and retry behavior

- Invalid credentials return `PolestarError::InvalidCredentials` without retrying.
- An invalid or expired refresh token causes a fresh authorization flow.
- Transient full-login failures are retried once after a one-second delay.
- A GraphQL request is retried at most once after HTTP 401; a second 401 is returned as an authentication error.
- HTTP 429 returns `PolestarError::RateLimitExceeded` so callers can apply policy-appropriate backoff.
- Server error bodies pass through the crate's credential, token, email, and VIN redactor before being included in errors.

## Automated coverage

The local test suite covers:

- PKCE verifier, challenge, and state generation
- Token validity and proactive refresh thresholds
- OIDC metadata and token response deserialization
- Refresh responses that omit a replacement refresh token
- Preservation of the old refresh token during a real local HTTP refresh exchange
- Access-token invalidation without clobbering a newer concurrent token
- HTTP 401 → refresh → one-time GraphQL retry behavior against local mock servers
- OAuth callback URL and state acceptance, missing state, mismatched state, and unexpected callback hosts
- Redacted token `Debug` output

No automated test uses real Polestar credentials. Live authentication and telemetry remain opt-in manual checks because they require a private account and depend on an unsupported upstream API. This distinction keeps CI deterministic and prevents secrets from entering the test environment.

## Operational notes

- The HTTP clients use 30-second request timeouts and a versioned user agent.
- Authentication session cookies and tokens are not persisted.
- Passwords should be supplied through `POLESTAR_PASSWORD` or a local ignored `.env` file, never through command-line arguments.
- The upstream identity and GraphQL contracts are private and may change without notice.
