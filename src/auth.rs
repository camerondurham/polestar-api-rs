//! Authentication module for Polestar OAuth2/OIDC flow.

use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use chrono::{DateTime, Duration, Utc};
use reqwest::header::LOCATION;
use serde::Deserialize;
use sha2::{Digest, Sha256};
use std::fmt;
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};
use zeroize::Zeroizing;

use crate::error::{PolestarError, Result};
use crate::redact::redact_str;

/// Base URL for Polestar's OIDC provider.
pub const OIDC_PROVIDER_BASE_URL: &str = "https://polestarid.eu.polestar.com";
/// Public client identifier used by the Polestar web application.
pub const OIDC_CLIENT_ID: &str = "l3oopkc_10";
const OIDC_PROVIDER_HOST: &str = "polestarid.eu.polestar.com";
/// OAuth callback used by the Polestar web application.
pub const OIDC_REDIRECT_URI: &str = "https://www.polestar.com/sign-in-callback";
/// OAuth scopes required by the vehicle API.
pub const OIDC_SCOPE: &str = "openid profile email customer:attributes";
/// Refresh an access token this many seconds before it expires at most.
pub const TOKEN_REFRESH_WINDOW_SECS: i64 = 300;

/// OIDC configuration from .well-known/openid-configuration
#[derive(Debug, Clone, Deserialize)]
pub struct OidcConfig {
    /// Expected token issuer.
    pub issuer: String,
    /// OAuth token exchange endpoint.
    pub token_endpoint: String,
    /// OAuth authorization endpoint.
    pub authorization_endpoint: String,
}

/// Token response from OAuth2 token endpoint
#[derive(Clone, Deserialize)]
pub struct TokenResponse {
    /// Short-lived bearer token used for API requests.
    pub access_token: String,
    /// Long-lived token used to refresh the access token, when one is issued.
    ///
    /// OAuth servers are allowed to omit this field from refresh responses. In
    /// that case the previously issued refresh token remains in use.
    #[serde(default)]
    pub refresh_token: Option<String>,
    /// Access-token lifetime in seconds.
    pub expires_in: i64,
}

impl fmt::Debug for TokenResponse {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("TokenResponse")
            .field("access_token", &"[REDACTED]")
            .field(
                "refresh_token",
                &self.refresh_token.as_ref().map(|_| "[REDACTED]"),
            )
            .field("expires_in", &self.expires_in)
            .finish()
    }
}

/// Token state with expiry tracking
#[derive(Clone)]
pub struct TokenState {
    /// Short-lived bearer token used for API requests.
    pub access_token: Zeroizing<String>,
    /// Long-lived token used to refresh the access token, when one was issued.
    pub refresh_token: Option<Zeroizing<String>>,
    /// Absolute access-token expiry time.
    pub expires_at: DateTime<Utc>,
    /// Original access-token lifetime in seconds.
    pub token_lifetime_secs: i64,
}

impl fmt::Debug for TokenState {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("TokenState")
            .field("access_token", &"[REDACTED]")
            .field(
                "refresh_token",
                &self.refresh_token.as_ref().map(|_| "[REDACTED]"),
            )
            .field("expires_at", &self.expires_at)
            .field("token_lifetime_secs", &self.token_lifetime_secs)
            .finish()
    }
}

impl TokenState {
    /// Build token state and calculate its absolute expiry time.
    pub fn from_response(response: TokenResponse) -> Self {
        Self {
            access_token: Zeroizing::new(response.access_token),
            refresh_token: response.refresh_token.map(Zeroizing::new),
            expires_at: Utc::now() + Duration::seconds(response.expires_in),
            token_lifetime_secs: response.expires_in,
        }
    }

    /// Return whether the access token has not expired.
    pub fn is_valid(&self) -> bool {
        Utc::now() < self.expires_at
    }

    /// Return whether the access token is within its proactive refresh window.
    pub fn needs_refresh(&self) -> bool {
        let refresh_window = std::cmp::min(self.token_lifetime_secs / 2, TOKEN_REFRESH_WINDOW_SECS);
        let expires_in = (self.expires_at - Utc::now()).num_seconds();
        expires_in < refresh_window
    }
}

/// Authentication state manager
pub struct AuthState {
    username: Zeroizing<String>,
    password: Zeroizing<String>,
    /// Current token state
    pub token: Arc<RwLock<Option<TokenState>>>,
    /// OIDC configuration
    pub oidc_config: Arc<RwLock<Option<OidcConfig>>>,
    auth_lock: Mutex<()>,
}

impl AuthState {
    /// Create new auth state with credentials
    pub fn new(username: String, password: String) -> Self {
        Self {
            username: Zeroizing::new(username),
            password: Zeroizing::new(password),
            token: Arc::new(RwLock::new(None)),
            oidc_config: Arc::new(RwLock::new(None)),
            auth_lock: Mutex::new(()),
        }
    }

    /// Fetch OIDC configuration from .well-known endpoint
    pub async fn get_oidc_config(&self, client: &reqwest::Client) -> Result<OidcConfig> {
        // Check if already cached
        {
            let config = self.oidc_config.read().await;
            if let Some(cfg) = config.as_ref() {
                return Ok(cfg.clone());
            }
        }

        // Fetch from well-known endpoint
        let url = format!(
            "{}/.well-known/openid-configuration",
            OIDC_PROVIDER_BASE_URL
        );
        let response = client.get(&url).send().await?;

        if !response.status().is_success() {
            return Err(PolestarError::OidcConfigError(format!(
                "HTTP {}",
                response.status()
            )));
        }

        let config: OidcConfig = response.json().await?;

        // Cache it
        {
            let mut cached = self.oidc_config.write().await;
            *cached = Some(config.clone());
        }

        Ok(config)
    }

    /// Get resume path from authorization endpoint
    async fn get_resume_path(
        &self,
        client: &reqwest::Client,
        config: &OidcConfig,
        state: &str,
        code_challenge: &str,
    ) -> Result<String> {
        let params = [
            ("client_id", OIDC_CLIENT_ID),
            ("redirect_uri", OIDC_REDIRECT_URI),
            ("response_type", "code"),
            ("scope", OIDC_SCOPE),
            ("state", state),
            ("code_challenge", code_challenge),
            ("code_challenge_method", "S256"),
            ("response_mode", "query"),
        ];

        let response = client
            .get(validated_oidc_url(&config.authorization_endpoint, "authorization endpoint")?)
            .query(&params)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(PolestarError::AuthError(format!(
                "Failed to get resume path: {}",
                response.status()
            )));
        }

        let text = response.text().await?;

        // Extract resume path from HTML using regex
        let re = regex::Regex::new(r#"(?:url|action):\s*"(.+?)""#)
            .map_err(|e| PolestarError::ApiError(format!("Regex error: {}", e)))?;

        if let Some(caps) = re.captures(&text) {
            if let Some(path) = caps.get(1) {
                return Ok(path.as_str().to_string());
            }
        }

        Err(PolestarError::AuthError(
            "Resume path not found in response".to_string(),
        ))
    }

    /// Get authorization code by posting credentials
    /// Returns (code, code_verifier) tuple
    pub async fn get_authorization_code(
        &self,
        client: &reqwest::Client,
    ) -> Result<(String, String)> {
        let config = self.get_oidc_config(client).await?;
        // Login depends on inspecting the provider's redirect responses. Use a
        // dedicated client so callers do not need to disable redirects on
        // their general-purpose HTTP client.
        let authorization_client = build_authorization_client()?;
        let state = generate_state();
        let code_verifier = generate_code_verifier();
        let code_challenge = generate_code_challenge(&code_verifier);

        let resume_path = self
            .get_resume_path(&authorization_client, &config, &state, &code_challenge)
            .await?;
        let resume_url = resolve_resume_url(&resume_path)?;

        // Build query params for resume URL
        let params = [
            ("client_id", OIDC_CLIENT_ID),
            ("redirect_uri", OIDC_REDIRECT_URI),
            ("response_type", "code"),
            ("scope", OIDC_SCOPE),
            ("state", state.as_str()),
            ("code_challenge", code_challenge.as_str()),
            ("code_challenge_method", "S256"),
            ("response_mode", "query"),
        ];

        // POST credentials
        let form = [
            ("pf.username", self.username.as_str()),
            ("pf.pass", self.password.as_str()),
        ];

        let mut response = authorization_client
            .post(resume_url.clone())
            .query(&params)
            .form(&form)
            .send()
            .await?;

        if !response.status().is_redirection() {
            let status = response.status();
            let text = response.text().await?;
            if text.contains(r#"authMessage: "ERR001""#) {
                return Err(PolestarError::InvalidCredentials);
            }
            return Err(PolestarError::AuthError(format!(
                "Authentication failed: expected redirect, received {status}"
            )));
        }

        let mut redirect_url = redirect_target(&response)?;
        let code = if query_value(&redirect_url, "code").is_some()
            || query_value(&redirect_url, "error").is_some()
        {
            authorization_code_from_redirect(&redirect_url, &state)?
        } else {
            let uid = query_value(&redirect_url, "uid").ok_or_else(|| {
                PolestarError::AuthError(
                    "Authentication redirect did not contain a code or confirmation id".to_string(),
                )
            })?;
            let confirmation_form = [("pf.submit", "true"), ("subject", uid.as_str())];

            response = authorization_client
                .post(resume_url.clone())
                .query(&params)
                .form(&confirmation_form)
                .send()
                .await?;

            if !response.status().is_redirection() {
                return Err(PolestarError::AuthError(format!(
                    "Terms confirmation failed: expected redirect, received {}",
                    response.status()
                )));
            }

            redirect_url = redirect_target(&response)?;
            authorization_code_from_redirect(&redirect_url, &state)?
        };

        // Complete the browser callback so the provider can finalize its session cookies.
        let callback_response = authorization_client.get(redirect_url).send().await?;
        if !callback_response.status().is_success() {
            return Err(PolestarError::AuthError(format!(
                "Authentication callback failed: {}",
                callback_response.status()
            )));
        }

        Ok((code, code_verifier))
    }

    /// Exchange authorization code for access token
    pub async fn exchange_code_for_token(
        &self,
        client: &reqwest::Client,
        code: &str,
        code_verifier: &str,
    ) -> Result<TokenState> {
        let config = self.get_oidc_config(client).await?;

        let form = [
            ("grant_type", "authorization_code"),
            ("client_id", OIDC_CLIENT_ID),
            ("code", code),
            ("redirect_uri", OIDC_REDIRECT_URI),
            ("code_verifier", code_verifier),
        ];

        let response = client
            .post(&config.token_endpoint)
            .form(&form)
            .send()
            .await?;

        if !response.status().is_success() {
            let text = response.text().await?;
            return Err(PolestarError::AuthError(format!(
                "Token exchange failed: {}",
                redact_str(&text)
            )));
        }

        let token_response: TokenResponse = response.json().await?;
        let token_state = TokenState::from_response(token_response);

        // Store in auth state
        {
            let mut token = self.token.write().await;
            *token = Some(token_state.clone());
        }

        Ok(token_state)
    }

    /// Refresh access token using refresh token
    pub async fn refresh_token(&self, client: &reqwest::Client) -> Result<TokenState> {
        let config = self.get_oidc_config(client).await?;

        // Get current refresh token
        let refresh_token = {
            let token = self.token.read().await;
            token
                .as_ref()
                .and_then(|token| token.refresh_token.clone())
                .ok_or_else(|| PolestarError::AuthError("No refresh token available".to_string()))?
        };
        let refresh_token = refresh_token.to_string();

        let form = [
            ("grant_type", "refresh_token"),
            ("client_id", OIDC_CLIENT_ID),
            ("refresh_token", refresh_token.as_str()),
        ];

        let response = client
            .post(&config.token_endpoint)
            .form(&form)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await?;

            // Check if refresh token is invalid/expired
            if status == reqwest::StatusCode::UNAUTHORIZED || text.contains("invalid_grant") {
                return Err(PolestarError::TokenExpired);
            }

            return Err(PolestarError::AuthError(format!(
                "Token refresh failed: {}",
                redact_str(&text)
            )));
        }

        let mut token_response: TokenResponse = response.json().await?;
        if token_response.refresh_token.is_none() {
            token_response.refresh_token = Some(refresh_token);
        }
        let token_state = TokenState::from_response(token_response);

        // Update stored token
        {
            let mut token = self.token.write().await;
            *token = Some(token_state.clone());
        }

        Ok(token_state)
    }

    /// Mark a rejected access token as expired without discarding its refresh token.
    ///
    /// The comparison prevents a delayed 401 response from invalidating a newer
    /// token installed by another task.
    pub(crate) async fn invalidate_access_token(&self, rejected_access_token: &str) -> bool {
        let mut token = self.token.write().await;
        let Some(token) = token.as_mut() else {
            return false;
        };
        if token.access_token.as_str() != rejected_access_token {
            return false;
        }

        token.expires_at = Utc::now() - Duration::seconds(1);
        true
    }

    /// Check if current token is valid
    pub async fn is_token_valid(&self) -> bool {
        let token = self.token.read().await;
        token.as_ref().map(|t| t.is_valid()).unwrap_or(false)
    }

    /// Check if token needs refresh
    pub async fn needs_token_refresh(&self) -> bool {
        let token = self.token.read().await;
        token
            .as_ref()
            .map(TokenState::needs_refresh)
            .unwrap_or(true)
    }

    /// Get valid access token, refreshing if needed
    pub async fn get_valid_token(&self, client: &reqwest::Client) -> Result<String> {
        if self.is_token_valid().await && !self.needs_token_refresh().await {
            let token = self.token.read().await;
            if let Some(token) = token.as_ref() {
                return Ok(token.access_token.to_string());
            }
        }

        // Only one task should refresh or perform the login flow at a time.
        let _auth_guard = self.auth_lock.lock().await;

        // Another task may have refreshed the token while this task waited.
        if self.is_token_valid().await && !self.needs_token_refresh().await {
            let token = self.token.read().await;
            if let Some(token) = token.as_ref() {
                return Ok(token.access_token.to_string());
            }
        }

        let can_refresh = self
            .token
            .read()
            .await
            .as_ref()
            .is_some_and(|token| token.refresh_token.is_some());
        if can_refresh {
            match self.refresh_token(client).await {
                Ok(state) => return Ok(state.access_token.to_string()),
                Err(PolestarError::TokenExpired) => {
                    let mut token = self.token.write().await;
                    *token = None;
                }
                Err(_) => {}
            }
        }

        // Full authentication flow with retry
        let max_retries = 2;
        let mut last_error = None;

        for attempt in 0..max_retries {
            match self.get_authorization_code(client).await {
                Ok((code, verifier)) => {
                    match self.exchange_code_for_token(client, &code, &verifier).await {
                        Ok(token_state) => return Ok(token_state.access_token.to_string()),
                        Err(e) => {
                            last_error = Some(e);
                            if attempt < max_retries - 1 {
                                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                            }
                        }
                    }
                }
                Err(e) => {
                    if matches!(e, PolestarError::InvalidCredentials) {
                        return Err(e);
                    }
                    last_error = Some(e);
                    if attempt < max_retries - 1 {
                        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                    }
                }
            }
        }

        Err(last_error
            .unwrap_or_else(|| PolestarError::AuthError("Authentication failed".to_string())))
    }
}

fn build_authorization_client() -> Result<reqwest::Client> {
    Ok(reqwest::Client::builder()
        .cookie_store(true)
        .redirect(reqwest::redirect::Policy::none())
        .timeout(std::time::Duration::from_secs(30))
        .user_agent(concat!("polestar-api-rs/", env!("CARGO_PKG_VERSION")))
        .build()?)
}

fn resolve_resume_url(raw_resume_path: &str) -> Result<reqwest::Url> {
    let base_url = reqwest::Url::parse(OIDC_PROVIDER_BASE_URL)
        .map_err(|_| PolestarError::AuthError("Invalid OIDC base URL".to_string()))?;

    let candidate = raw_resume_path.trim();
    let candidate_url = if candidate.starts_with('/') {
        base_url.join(candidate).map_err(|_| {
            PolestarError::AuthError("Invalid OIDC resume path".to_string())
        })?
    } else if candidate.starts_with("http://") || candidate.starts_with("https://") {
        reqwest::Url::parse(candidate).map_err(|error| {
            PolestarError::AuthError(format!("Invalid resume URL: {error}"))
        })?
    } else {
        base_url.join(&format!("/{candidate}")).map_err(|_| {
            PolestarError::AuthError("Invalid OIDC resume path".to_string())
        })?
    };

    validate_oidc_origin(&candidate_url, "resume URL")?;
    Ok(candidate_url)
}

fn validated_oidc_url(raw_url: &str, endpoint_name: &str) -> Result<reqwest::Url> {
    let parsed = reqwest::Url::parse(raw_url).map_err(|error| {
        PolestarError::AuthError(format!("Invalid {endpoint_name}: {error}"))
    })?;
    validate_oidc_origin(&parsed, endpoint_name)?;
    Ok(parsed)
}

fn validate_oidc_origin(url: &reqwest::Url, context: &str) -> Result<()> {
    if url.scheme() != "https" {
        return Err(PolestarError::AuthError(format!(
            "{context} must use HTTPS",
        )));
    }
    if url.host_str() != Some(OIDC_PROVIDER_HOST) {
        return Err(PolestarError::AuthError(format!(
            "{context} must target {OIDC_PROVIDER_HOST}, got {}",
            url.host_str().unwrap_or("unknown")
        )));
    }
    if !url.username().is_empty() || url.password().is_some() {
        return Err(PolestarError::AuthError(format!(
            "{context} must not include credentials",
        )));
    }

    Ok(())
}

fn redirect_target(response: &reqwest::Response) -> Result<reqwest::Url> {
    let location = response
        .headers()
        .get(LOCATION)
        .ok_or_else(|| PolestarError::AuthError("Redirect missing Location header".to_string()))?
        .to_str()
        .map_err(|_| PolestarError::AuthError("Redirect Location is not valid text".to_string()))?;

    response.url().join(location).map_err(|error| {
        PolestarError::AuthError(format!("Invalid authentication redirect URL: {error}"))
    })
}

fn query_value(url: &reqwest::Url, name: &str) -> Option<String> {
    url.query_pairs()
        .find(|(key, _)| key == name)
        .map(|(_, value)| value.into_owned())
}

fn authorization_code_from_redirect(url: &reqwest::Url, expected_state: &str) -> Result<String> {
    let expected_url = reqwest::Url::parse(OIDC_REDIRECT_URI)
        .map_err(|error| PolestarError::AuthError(format!("Invalid callback URL: {error}")))?;
    let is_expected_callback = url.scheme() == expected_url.scheme()
        && url.host_str() == expected_url.host_str()
        && url.port_or_known_default() == expected_url.port_or_known_default()
        && url.path() == expected_url.path()
        && url.username().is_empty()
        && url.password().is_none();
    if !is_expected_callback {
        return Err(PolestarError::AuthError(
            "Authorization response used an unexpected callback URL".to_string(),
        ));
    }

    let returned_state = query_value(url, "state").ok_or_else(|| {
        PolestarError::AuthError("Authorization response did not contain state".to_string())
    })?;
    if returned_state != expected_state {
        return Err(PolestarError::AuthError(
            "Authorization response state did not match the request".to_string(),
        ));
    }

    if let Some(error) = query_value(url, "error") {
        return Err(PolestarError::AuthError(format!(
            "Authorization server returned {}",
            redact_str(&error)
        )));
    }

    query_value(url, "code").ok_or_else(|| {
        PolestarError::AuthError("Authorization response did not contain a code".to_string())
    })
}

// PKCE helper functions

/// Generate random code verifier for PKCE
pub fn generate_code_verifier() -> String {
    let random_bytes: [u8; 32] = rand::random();
    URL_SAFE_NO_PAD.encode(random_bytes)
}

/// Generate code challenge from verifier using SHA256
pub fn generate_code_challenge(verifier: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(verifier.as_bytes());
    let hash = hasher.finalize();
    URL_SAFE_NO_PAD.encode(hash)
}

/// Generate random state parameter
pub fn generate_state() -> String {
    let random_bytes: [u8; 32] = rand::random();
    URL_SAFE_NO_PAD.encode(random_bytes)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Read, Write};
    use std::net::TcpListener;
    use std::thread;

    #[test]
    fn test_generate_code_verifier() {
        let verifier = generate_code_verifier();
        assert!(!verifier.is_empty());
        assert_eq!(verifier.len(), 43); // 32 bytes base64url = 43 chars
    }

    #[test]
    fn test_generate_code_challenge() {
        let verifier = "test_verifier";
        let challenge = generate_code_challenge(verifier);
        assert!(!challenge.is_empty());
        assert_eq!(challenge.len(), 43); // SHA256 = 32 bytes = 43 chars base64url
    }

    #[test]
    fn test_generate_state() {
        let state = generate_state();
        assert!(!state.is_empty());
        assert_eq!(state.len(), 43);
    }

    #[test]
    fn test_token_state_is_valid() {
        let response = TokenResponse {
            access_token: "test_token".to_string(),
            refresh_token: Some("test_refresh".to_string()),
            expires_in: 3600,
        };
        let state = TokenState::from_response(response);
        assert!(state.is_valid());
    }

    #[test]
    fn test_token_state_needs_refresh() {
        // Create expired token
        let mut state = TokenState {
            access_token: "test".to_string().into(),
            refresh_token: Some("test".to_string().into()),
            expires_at: Utc::now() + Duration::seconds(10),
            token_lifetime_secs: 600,
        };
        // Should need refresh when expires_in < refresh_window
        assert!(state.needs_refresh()); // window=300, expires_in=10

        // Fresh token shouldn't need refresh
        state.expires_at = Utc::now() + Duration::seconds(3600);
        assert!(!state.needs_refresh()); // window=300, expires_in=3600
    }

    #[test]
    fn test_oidc_config_deserialization() {
        let config: OidcConfig = serde_json::from_value(serde_json::json!({
            "issuer": "https://test.polestar.com",
            "token_endpoint": "https://test.polestar.com/token",
            "authorization_endpoint": "https://test.polestar.com/authorize"
        }))
        .unwrap();

        assert_eq!(config.issuer, "https://test.polestar.com");
        assert_eq!(config.token_endpoint, "https://test.polestar.com/token");
    }

    #[test]
    fn token_response_accepts_an_omitted_refresh_token() {
        let response: TokenResponse = serde_json::from_value(serde_json::json!({
            "access_token": "new-access",
            "expires_in": 3600
        }))
        .unwrap();

        assert_eq!(response.access_token, "new-access");
        assert!(response.refresh_token.is_none());
    }

    #[test]
    fn token_debug_output_redacts_credentials() {
        let response = TokenResponse {
            access_token: "secret-access".to_string(),
            refresh_token: Some("secret-refresh".to_string()),
            expires_in: 3600,
        };
        let state = TokenState::from_response(response.clone());

        let response_debug = format!("{response:?}");
        let state_debug = format!("{state:?}");
        for output in [&response_debug, &state_debug] {
            assert!(!output.contains("secret-access"));
            assert!(!output.contains("secret-refresh"));
            assert!(output.contains("[REDACTED]"));
        }
    }

    #[test]
    fn validates_authorization_callback_and_state() {
        let url = reqwest::Url::parse(
            "https://www.polestar.com/sign-in-callback?code=auth-code&state=expected",
        )
        .unwrap();

        assert_eq!(
            authorization_code_from_redirect(&url, "expected").unwrap(),
            "auth-code"
        );
    }

    #[test]
    fn rejects_missing_or_mismatched_authorization_state() {
        let missing =
            reqwest::Url::parse("https://www.polestar.com/sign-in-callback?code=auth-code")
                .unwrap();
        let mismatched = reqwest::Url::parse(
            "https://www.polestar.com/sign-in-callback?code=auth-code&state=wrong",
        )
        .unwrap();

        assert!(authorization_code_from_redirect(&missing, "expected").is_err());
        assert!(authorization_code_from_redirect(&mismatched, "expected").is_err());
    }

    #[test]
    fn reports_an_oauth_error_from_the_expected_callback() {
        let url = reqwest::Url::parse(
            "https://www.polestar.com/sign-in-callback?error=access_denied&state=expected",
        )
        .unwrap();

        let error = authorization_code_from_redirect(&url, "expected").unwrap_err();
        assert!(error.to_string().contains("access_denied"));
    }

    #[test]
    fn rejects_authorization_code_sent_to_an_unexpected_callback() {
        let url = reqwest::Url::parse(
            "https://attacker.example/sign-in-callback?code=auth-code&state=expected",
        )
        .unwrap();

        assert!(authorization_code_from_redirect(&url, "expected").is_err());
    }

    #[tokio::test]
    async fn invalidation_does_not_expire_a_newer_access_token() {
        let auth = AuthState::new("user".to_string(), "password".to_string());
        *auth.token.write().await = Some(TokenState {
            access_token: "new-access".to_string().into(),
            refresh_token: Some("refresh".to_string().into()),
            expires_at: Utc::now() + Duration::hours(1),
            token_lifetime_secs: 3600,
        });

        assert!(!auth.invalidate_access_token("old-access").await);
        assert!(auth.is_token_valid().await);
        assert!(auth.invalidate_access_token("new-access").await);
        assert!(!auth.is_token_valid().await);
    }

    #[tokio::test]
    async fn authorization_client_preserves_redirect_response() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        let server = thread::spawn(move || {
            let (mut stream, _) = listener.accept().unwrap();
            let mut request = [0_u8; 1024];
            let _ = stream.read(&mut request).unwrap();
            stream
                .write_all(
                    b"HTTP/1.1 302 Found\r\nLocation: http://127.0.0.1:9/callback\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
                )
                .unwrap();
        });

        let response = build_authorization_client()
            .unwrap()
            .get(format!("http://{address}/login"))
            .send()
            .await
            .unwrap();

        server.join().unwrap();
        assert!(response.status().is_redirection());
        assert_eq!(
            response.headers().get(LOCATION).unwrap(),
            "http://127.0.0.1:9/callback"
        );
    }
}
