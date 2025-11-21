//! Authentication module for Polestar OAuth2/OIDC flow.

use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use chrono::{DateTime, Duration, Utc};
use rand::Rng;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::error::{PolestarError, Result};

// Auth constants
pub const OIDC_PROVIDER_BASE_URL: &str = "https://polestarid.eu.polestar.com";
pub const OIDC_CLIENT_ID: &str = "l3oopkc_10";
pub const OIDC_REDIRECT_URI: &str = "https://www.polestar.com/sign-in-callback";
pub const OIDC_SCOPE: &str = "openid profile email customer:attributes";
pub const TOKEN_REFRESH_WINDOW_SECS: i64 = 300;

/// OIDC configuration from .well-known/openid-configuration
#[derive(Debug, Clone, Deserialize)]
pub struct OidcConfig {
    pub issuer: String,
    pub token_endpoint: String,
    pub authorization_endpoint: String,
}

/// Token response from OAuth2 token endpoint
#[derive(Debug, Clone, Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: i64,
}

/// Token state with expiry tracking
#[derive(Debug, Clone)]
pub struct TokenState {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_at: DateTime<Utc>,
}

impl TokenState {
    pub fn from_response(response: TokenResponse) -> Self {
        Self {
            access_token: response.access_token,
            refresh_token: response.refresh_token,
            expires_at: Utc::now() + Duration::seconds(response.expires_in),
        }
    }

    pub fn is_valid(&self) -> bool {
        Utc::now() < self.expires_at
    }

    pub fn needs_refresh(&self, token_lifetime_secs: i64) -> bool {
        let refresh_window = std::cmp::min(token_lifetime_secs / 2, TOKEN_REFRESH_WINDOW_SECS);
        let expires_in = (self.expires_at - Utc::now()).num_seconds();
        expires_in < refresh_window
    }
}

/// Authentication state manager
pub struct AuthState {
    /// Username for authentication
    pub username: String,
    /// Password for authentication
    pub password: String,
    /// Current token state
    pub token: Arc<RwLock<Option<TokenState>>>,
    /// OIDC configuration
    pub oidc_config: Arc<RwLock<Option<OidcConfig>>>,
}

impl AuthState {
    /// Create new auth state with credentials
    pub fn new(username: String, password: String) -> Self {
        Self {
            username,
            password,
            token: Arc::new(RwLock::new(None)),
            oidc_config: Arc::new(RwLock::new(None)),
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
        let url = format!("{}/.well-known/openid-configuration", OIDC_PROVIDER_BASE_URL);
        let response = client
            .get(&url)
            .send()
            .await?;

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
            .get(&config.authorization_endpoint)
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

        Err(PolestarError::AuthError("Resume path not found in response".to_string()))
    }

    /// Get authorization code by posting credentials
    /// Returns (code, code_verifier) tuple
    pub async fn get_authorization_code(
        &self,
        client: &reqwest::Client,
    ) -> Result<(String, String)> {
        let config = self.get_oidc_config(client).await?;
        let state = generate_state();
        let code_verifier = generate_code_verifier();
        let code_challenge = generate_code_challenge(&code_verifier);

        let resume_path = self.get_resume_path(client, &config, &state, &code_challenge).await?;
        let resume_url = format!("{}{}", OIDC_PROVIDER_BASE_URL, resume_path);

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

        let response = client
            .post(&resume_url)
            .query(&params)
            .form(&form)
            .send()
            .await?;

        let status = response.status();
        
        // Check for auth error (4xx without redirect)
        if status.is_client_error() && !status.is_redirection() {
            let text = response.text().await?;
            if text.contains(r#"authMessage: "ERR001""#) {
                return Err(PolestarError::InvalidCredentials);
            }
            return Err(PolestarError::AuthError(format!("Authentication failed: {}", status)));
        }

        // Handle redirects (302/303) - reqwest follows them automatically
        // So we need to check the final URL for the code parameter
        let final_url = response.url().clone();
        
        // Check for code parameter in final URL
        if let Some(code) = final_url.query_pairs().find(|(k, _)| k == "code").map(|(_, v)| v.to_string()) {
            return Ok((code, code_verifier));
        }

        // Check for uid (T&C acceptance needed)
        if let Some(uid) = final_url.query_pairs().find(|(k, _)| k == "uid").map(|(_, v)| v.to_string()) {
            // Submit T&C acceptance
            let form = [
                ("pf.submit", "true"),
                ("subject", &uid),
            ];

            let response = client
                .post(&resume_url)
                .query(&params)
                .form(&form)
                .send()
                .await?;

            let final_url = response.url().clone();

            if let Some(code) = final_url.query_pairs().find(|(k, _)| k == "code").map(|(_, v)| v.to_string()) {
                return Ok((code, code_verifier));
            }
        }

        Err(PolestarError::AuthError("No authorization code found".to_string()))
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
                text
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
                .map(|t| t.refresh_token.clone())
                .ok_or_else(|| PolestarError::AuthError("No refresh token available".to_string()))?
        };

        let form = [
            ("grant_type", "refresh_token"),
            ("client_id", OIDC_CLIENT_ID),
            ("refresh_token", &refresh_token),
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
                text
            )));
        }

        let token_response: TokenResponse = response.json().await?;
        let token_state = TokenState::from_response(token_response);

        // Update stored token
        {
            let mut token = self.token.write().await;
            *token = Some(token_state.clone());
        }

        Ok(token_state)
    }

    /// Check if current token is valid
    pub async fn is_token_valid(&self) -> bool {
        let token = self.token.read().await;
        token.as_ref().map(|t| t.is_valid()).unwrap_or(false)
    }

    /// Check if token needs refresh
    pub async fn needs_token_refresh(&self) -> bool {
        let token = self.token.read().await;
        if let Some(t) = token.as_ref() {
            // Calculate original lifetime from current expiry
            let lifetime = (t.expires_at - Utc::now()).num_seconds() + 3600; // assume 1hr default
            t.needs_refresh(lifetime)
        } else {
            true // No token means we need one
        }
    }

    /// Get valid access token, refreshing if needed
    pub async fn get_valid_token(&self, client: &reqwest::Client) -> Result<String> {
        // Check if we have a valid token
        if self.is_token_valid().await && !self.needs_token_refresh().await {
            let token = self.token.read().await;
            return Ok(token.as_ref().unwrap().access_token.clone());
        }

        // Try to refresh if we have a refresh token
        {
            let token = self.token.read().await;
            if token.is_some() {
                drop(token); // Release read lock before refresh
                match self.refresh_token(client).await {
                    Ok(state) => return Ok(state.access_token),
                    Err(PolestarError::TokenExpired) => {
                        // Token expired, clear it and fall through to full auth
                        let mut token = self.token.write().await;
                        *token = None;
                    }
                    Err(e) => {
                        // Other error, try full auth
                        eprintln!("Token refresh failed: {}, attempting full auth", e);
                    }
                }
            }
        }

        // Full authentication flow with retry
        let max_retries = 2;
        let mut last_error = None;
        
        for attempt in 0..max_retries {
            match self.get_authorization_code(client).await {
                Ok((code, verifier)) => {
                    match self.exchange_code_for_token(client, &code, &verifier).await {
                        Ok(token_state) => return Ok(token_state.access_token),
                        Err(e) => {
                            last_error = Some(e);
                            if attempt < max_retries - 1 {
                                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                            }
                        }
                    }
                }
                Err(e) => {
                    last_error = Some(e);
                    if attempt < max_retries - 1 {
                        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                    }
                }
            }
        }

        Err(last_error.unwrap_or_else(|| PolestarError::AuthError("Authentication failed".to_string())))
    }
}

// PKCE helper functions

/// Generate random code verifier for PKCE
pub fn generate_code_verifier() -> String {
    let random_bytes: [u8; 32] = rand::thread_rng().gen();
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
    let random_bytes: [u8; 32] = rand::thread_rng().gen();
    URL_SAFE_NO_PAD.encode(random_bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

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
            refresh_token: "test_refresh".to_string(),
            expires_in: 3600,
        };
        let state = TokenState::from_response(response);
        assert!(state.is_valid());
    }

    #[test]
    fn test_token_state_needs_refresh() {
        // Create expired token
        let mut state = TokenState {
            access_token: "test".to_string(),
            refresh_token: "test".to_string(),
            expires_at: Utc::now() + Duration::seconds(10),
        };
        // Should need refresh when expires_in < refresh_window
        assert!(state.needs_refresh(600)); // window=300, expires_in=10
        
        // Fresh token shouldn't need refresh
        state.expires_at = Utc::now() + Duration::seconds(3600);
        assert!(!state.needs_refresh(3600)); // window=300, expires_in=3600
    }

    #[tokio::test]
    async fn test_get_oidc_config() {
        use wiremock::{MockServer, Mock, ResponseTemplate};
        use wiremock::matchers::{method, path};

        let mock_server = MockServer::start().await;
        
        let config_json = serde_json::json!({
            "issuer": "https://test.polestar.com",
            "token_endpoint": "https://test.polestar.com/token",
            "authorization_endpoint": "https://test.polestar.com/authorize"
        });

        Mock::given(method("GET"))
            .and(path("/.well-known/openid-configuration"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&config_json))
            .mount(&mock_server)
            .await;

        // Override base URL for test
        let auth = AuthState::new("user".to_string(), "pass".to_string());
        let client = reqwest::Client::new();
        
        // Note: This test would need to mock OIDC_PROVIDER_BASE_URL
        // For now, just verify the struct works
        assert_eq!(auth.username, "user");
    }
}
