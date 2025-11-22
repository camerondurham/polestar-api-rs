# Authentication Implementation Plan

## Overview
Implement Polestar OAuth2/OIDC authentication flow based on pypolestar reference.

## Auth Flow Steps
1. OIDC discovery → get endpoints
2. Generate PKCE challenge (code_verifier, code_challenge)
3. GET authorization endpoint → extract resume_path from response
4. POST credentials to resume_path → get redirect with code
5. Handle T&C acceptance if uid present (no code)
6. Exchange code for tokens
7. Store access_token, refresh_token, expiry
8. Refresh when needed

## Constants (from pypolestar)
```
OIDC_PROVIDER: https://polestarid.eu.polestar.com
CLIENT_ID:
REDIRECT_URI: https://www.polestar.com/sign-in-callback
SCOPE: openid profile email customer:attributes
TOKEN_REFRESH_WINDOW: 300s
```

## Task List

### Phase 1: Core Auth Types ✅
- [x] Add auth module `src/auth.rs`
- [x] Define `OidcConfig` struct (issuer, token_endpoint, authorization_endpoint)
- [x] Define `TokenResponse` struct (access_token, refresh_token, expires_in)
- [x] Add auth constants to `src/lib.rs` or separate constants file
- [x] Add PKCE helper functions (generate_code_verifier, generate_code_challenge)

### Phase 2: OIDC Discovery ✅
- [x] Implement `get_oidc_config()` - fetch .well-known/openid-configuration
- [x] Parse JSON response into OidcConfig
- [x] Add error handling for network/parse failures

### Phase 3: Authorization Code Flow ✅
- [x] Implement `get_resume_path()` - GET authorization endpoint, extract resume path from HTML
- [x] Implement `get_authorization_code()` - POST credentials to resume path
- [x] Handle 302/303 redirects
- [x] Extract code from redirect params
- [x] Handle T&C acceptance (uid present, no code) - POST confirmation, retry

### Phase 4: Token Exchange ✅
- [x] Implement `exchange_code_for_token()` - POST to token endpoint
- [x] Parse token response
- [x] Calculate token expiry from expires_in
- [x] Store tokens in client state

### Phase 5: Token Management ✅
- [x] Implement `refresh_token()` - POST refresh_token grant
- [x] Implement `is_token_valid()` - check expiry
- [x] Implement `needs_refresh()` - check refresh window
- [x] Add token mutex/lock for thread safety

### Phase 6: Integration ✅
- [x] Update `PolestarClient::new()` to call auth on init
- [x] Update `authenticate()` method to use new flow
- [x] Add `get_token()` method - returns valid token (refresh if needed)
- [x] Update `post_graphql()` to use `get_token()` for bearer token
- [x] Add token caching (optional - in-memory first)

### Phase 7: Error Handling ✅
- [x] Add specific auth errors (InvalidCredentials, TokenExpired, etc)
- [x] Handle ERR001 (invalid username/password)
- [x] Handle missing code/uid edge cases
- [x] Add retry logic for transient failures

### Phase 8: Testing ✅
- [x] Unit tests for PKCE generation
- [x] Mock tests for each auth step
- [x] Integration test with real credentials
- [x] Test token refresh flow
- [x] Test error cases

## Implementation Complete! 🎉

All phases completed successfully. The authentication flow is fully functional and tested with real Polestar credentials.

### Verified Working:
- OIDC discovery
- Authorization code flow with PKCE
- Token exchange
- Token refresh
- Automatic token management
- Error handling (invalid credentials, token expiry, etc.)
- Integration with PolestarClient

### Example Output:
```
=== Vehicle Information ===
  VIN: LPXXXXXXXXNXXXXXX
  Market: US

=== Model ===
  Name: Polestar 2
  Code: 534

=== Battery ===
  78 kWh

=== Torque ===
  487 lb-ft
```

## Dependencies Needed
```toml
reqwest = { version = "0.11", features = ["json", "cookies"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
base64 = "0.21"
sha2 = "0.10"
rand = "0.8"
regex = "1.10"
```

## Key Implementation Notes
- Use reqwest cookie store for session management
- PKCE uses SHA256 + base64url encoding
- Resume path extracted via regex from HTML response
- Token refresh should happen 300s before expiry or at 50% lifetime
- Thread-safe token access via Mutex/RwLock

## Reference Files
- `pypolestar/pypolestar/auth.py` - main auth logic
- `pypolestar/pypolestar/const.py` - constants
