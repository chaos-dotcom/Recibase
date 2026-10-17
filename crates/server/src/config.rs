//! Environment configuration for the admin tracker and its OIDC login.
//!
//! Everything here is optional: when `DATABASE_URL` or the OIDC variables are
//! unset the tracker routes are simply not served, and the existing read-only
//! API keeps using the CSV meal log.

/// The OpenID Connect settings for the single-owner login.
#[derive(Debug, Clone)]
pub struct OidcConfig {
    /// The issuer URL; discovery is fetched from `<issuer>/.well-known/openid-configuration`.
    pub issuer: String,
    pub client_id: String,
    pub client_secret: String,
    /// The absolute callback URL registered with the provider.
    pub redirect_url: String,
    /// The one OIDC `sub` allowed to sign in; everyone else is rejected.
    pub owner_subject: String,
}

/// The tracker configuration: OIDC plus cookie policy.
#[derive(Debug, Clone)]
pub struct AdminConfig {
    pub oidc: OidcConfig,
    /// Whether the session cookie carries `Secure`. Defaults to true; set
    /// `SESSION_COOKIE_SECURE=false` for plain-HTTP local development.
    pub cookie_secure: bool,
}

impl AdminConfig {
    /// Reads the OIDC settings from the environment. Returns `None` (and the
    /// tracker routes answer 503) unless every required value is present.
    pub fn from_env(env: &dyn Fn(&str) -> Option<String>) -> Option<AdminConfig> {
        let oidc = OidcConfig {
            issuer: env("OIDC_ISSUER_URL")?,
            client_id: env("OIDC_CLIENT_ID")?,
            client_secret: env("OIDC_CLIENT_SECRET")?,
            redirect_url: env("OIDC_REDIRECT_URL")?,
            owner_subject: env("RECIBASE_OWNER_SUBJECT")?,
        };
        let cookie_secure = env("SESSION_COOKIE_SECURE")
            .map(|value| !value.eq_ignore_ascii_case("false"))
            .unwrap_or(true);
        Some(AdminConfig {
            oidc,
            cookie_secure,
        })
    }
}

/// The Postgres connection string, if the tracker is configured.
pub fn database_url(env: &dyn Fn(&str) -> Option<String>) -> Option<String> {
    env("DATABASE_URL")
}
