//! OpenID Connect authorization-code login: discovery, the token exchange and
//! ID-token verification against the provider's JWKS.
//!
//! Only the pieces the single-owner tracker needs are implemented; it is a
//! confidential client (it has a `client_secret`) and relies on `state` and
//! `nonce` for the code flow, so PKCE is not used.

use crate::config::OidcConfig;
use jsonwebtoken::jwk::JwkSet;
use jsonwebtoken::{DecodingKey, Validation, decode, decode_header};
use serde::Deserialize;
use std::sync::Mutex;

/// `/.well-known/openid-configuration`.
#[derive(Debug, Clone, Deserialize)]
pub struct Discovery {
    pub issuer: String,
    pub authorization_endpoint: String,
    pub token_endpoint: String,
    pub jwks_uri: String,
    /// Optional RP-initiated logout endpoint.
    #[serde(default)]
    pub end_session_endpoint: Option<String>,
}

#[derive(Debug, Deserialize)]
struct TokenResponse {
    #[serde(default)]
    id_token: Option<String>,
}

/// The claims we read out of the verified ID token.
#[derive(Debug, Clone, Deserialize)]
pub struct IdTokenClaims {
    pub sub: String,
    #[serde(default)]
    pub email: Option<String>,
    #[serde(default)]
    pub nonce: Option<String>,
}

/// Holds the OIDC config and a cached discovery document.
pub struct Provider {
    config: OidcConfig,
    discovery: Mutex<Option<Discovery>>,
}

impl Provider {
    pub fn new(config: OidcConfig) -> Provider {
        Provider {
            config,
            discovery: Mutex::new(None),
        }
    }

    pub fn config(&self) -> &OidcConfig {
        &self.config
    }

    /// The discovery document, fetched once and cached. Discovery changes are
    /// rare enough that a restart is an acceptable way to pick them up.
    pub fn discovery(&self) -> Result<Discovery, String> {
        if let Some(found) = self
            .discovery
            .lock()
            .expect("discovery cache is not poisoned")
            .clone()
        {
            return Ok(found);
        }
        let url = format!(
            "{}/.well-known/openid-configuration",
            self.config.issuer.trim_end_matches('/')
        );
        let discovery: Discovery = ureq::get(&url)
            .call()
            .map_err(|e| format!("discovery request failed: {e}"))?
            .into_body()
            .read_json()
            .map_err(|e| format!("discovery is not valid JSON: {e}"))?;
        *self
            .discovery
            .lock()
            .expect("discovery cache is not poisoned") = Some(discovery.clone());
        Ok(discovery)
    }

    /// The provider's authorization endpoint URL for a login attempt. PKCE
    /// (RFC 7636) is always used, so `code_challenge` is the S256 digest of the
    /// `code_verifier` returned on the token exchange.
    pub fn authorize_url(
        &self,
        discovery: &Discovery,
        state: &str,
        nonce: &str,
        code_challenge: &str,
    ) -> String {
        let params = [
            ("response_type", "code"),
            ("client_id", self.config.client_id.as_str()),
            ("redirect_uri", self.config.redirect_url.as_str()),
            ("scope", "openid email"),
            ("state", state),
            ("nonce", nonce),
            ("code_challenge", code_challenge),
            ("code_challenge_method", "S256"),
        ];
        let query = params
            .iter()
            .map(|(key, value)| format!("{}={}", url_encode(key), url_encode(value)))
            .collect::<Vec<_>>()
            .join("&");
        format!("{}?{}", discovery.authorization_endpoint, query)
    }

    /// Exchanges an authorization code for tokens and returns the verified ID
    /// token claims.
    pub fn exchange(
        &self,
        discovery: &Discovery,
        code: &str,
        nonce: &str,
        code_verifier: &str,
    ) -> Result<IdTokenClaims, String> {
        let response = ureq::post(&discovery.token_endpoint)
            .send_form([
                ("grant_type", "authorization_code"),
                ("code", code),
                ("redirect_uri", self.config.redirect_url.as_str()),
                ("client_id", self.config.client_id.as_str()),
                ("client_secret", self.config.client_secret.as_str()),
                ("code_verifier", code_verifier),
            ])
            .map_err(|e| format!("token request failed: {e}"))?;
        if response.status() != 200 {
            return Err(format!("token endpoint answered {}", response.status()));
        }
        let token: TokenResponse = response
            .into_body()
            .read_json()
            .map_err(|e| format!("token response is not valid JSON: {e}"))?;
        let id_token = token.id_token.ok_or("token response has no id_token")?;

        let jwks: JwkSet = ureq::get(&discovery.jwks_uri)
            .call()
            .map_err(|e| format!("jwks request failed: {e}"))?
            .into_body()
            .read_json()
            .map_err(|e| format!("jwks is not valid JSON: {e}"))?;

        verify_id_token(&jwks, &discovery.issuer, &self.config.client_id, &id_token, nonce)
    }
}

/// Verifies an ID token's signature, issuer, audience and nonce, returning its
/// claims. Split out from the network path so it can be tested directly.
pub fn verify_id_token(
    jwks: &JwkSet,
    issuer: &str,
    client_id: &str,
    token: &str,
    expected_nonce: &str,
) -> Result<IdTokenClaims, String> {
    let header = decode_header(token).map_err(|e| format!("id token header: {e}"))?;
    let kid = header.kid.as_deref().ok_or("id token has no kid")?;
    let jwk = jwks.find(kid).ok_or("no JWKS key matches the id token")?;
    let key = DecodingKey::from_jwk(jwk).map_err(|e| format!("unusable JWKS key: {e}"))?;

    let mut validation = Validation::new(header.alg);
    validation.set_issuer(&[issuer]);
    validation.set_audience(&[client_id]);

    let data = decode::<IdTokenClaims>(token, &key, &validation)
        .map_err(|e| format!("id token rejected: {e}"))?;
    match data.claims.nonce.as_deref() {
        Some(nonce) if nonce == expected_nonce => Ok(data.claims),
        _ => Err("id token nonce does not match the login attempt".to_string()),
    }
}

/// Percent-encodes a query-string component per RFC 3986 (unreserved bytes pass
/// through, everything else becomes `%XX`).
fn url_encode(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    for byte in input.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(byte as char)
            }
            _ => out.push_str(&format!("%{byte:02X}")),
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::{url_encode, verify_id_token};
    use jsonwebtoken::jwk::JwkSet;

    #[test]
    fn query_components_are_percent_encoded() {
        assert_eq!(url_encode("openid email"), "openid%20email");
        assert_eq!(
            url_encode("https://api.example.com/auth/callback"),
            "https%3A%2F%2Fapi.example.com%2Fauth%2Fcallback"
        );
        assert_eq!(url_encode("a-b_c.d~e"), "a-b_c.d~e");
    }

    #[test]
    fn a_malformed_id_token_is_rejected() {
        let jwks = JwkSet { keys: Vec::new() };
        let error = verify_id_token(&jwks, "https://issuer", "client", "not-a-jwt", "nonce")
            .expect_err("a non-JWT must fail");
        assert!(error.contains("header"), "{error}");
    }

    #[test]
    fn a_token_without_a_matching_key_is_rejected() {
        // A well-formed header (alg=RS256, kid=missing) but no JWKS entry.
        let header = jsonwebtoken::encode(
            &jsonwebtoken::Header::new(jsonwebtoken::Algorithm::HS256),
            &serde_json::json!({"sub": "x"}),
            &jsonwebtoken::EncodingKey::from_secret(b"secret"),
        )
        .unwrap();
        let jwks = JwkSet { keys: Vec::new() };
        // No `kid` in the header, so verification stops before any key lookup.
        assert!(verify_id_token(&jwks, "https://issuer", "client", &header, "nonce").is_err());
    }
}
