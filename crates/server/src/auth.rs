//! OIDC login routes, session cookies and CSRF for the admin tracker.

use crate::config::AdminConfig;
use crate::db::{Db, Session, User};
use crate::http::{Request, Response, parse_form};
use crate::oidc::Provider;
use chrono::{Duration, Utc};

/// The session cookie name.
pub const SESSION_COOKIE: &str = "recibase_session";

/// How long a login session lasts.
const SESSION_TTL_DAYS: i64 = 30;
/// How long a half-finished login (`state`/`nonce`) stays valid.
const FLOW_TTL_MINUTES: i64 = 10;

/// Everything the tracker routes need: the store, the OIDC provider and the
/// cookie policy.
pub struct Admin {
    pub db: Db,
    pub provider: Provider,
    pub config: AdminConfig,
}

impl Admin {
    pub fn new(db: Db, config: AdminConfig) -> Admin {
        let provider = Provider::new(config.oidc.clone());
        Admin {
            db,
            provider,
            config,
        }
    }

    /// `GET /auth/login`: start an authorization-code flow and bounce to the
    /// provider.
    pub fn login(&self) -> Response {
        let now = Utc::now();
        if let Err(error) = self.db.purge_expired(now) {
            eprintln!("could not prune expired auth rows: {error}");
        }
        let state = crate::tokens::random_hex(32);
        let nonce = crate::tokens::random_hex(32);
        let (code_verifier, code_challenge) = crate::tokens::pkce_pair();
        if let Err(error) = self.db.create_auth_flow(
            &state,
            &nonce,
            &code_verifier,
            now + Duration::minutes(FLOW_TTL_MINUTES),
        ) {
            eprintln!("could not start a login flow: {error}");
            return text(500, "login unavailable");
        }
        let discovery = match self.provider.discovery() {
            Ok(discovery) => discovery,
            Err(error) => {
                eprintln!("OIDC discovery failed: {error}");
                return text(503, "identity provider unavailable");
            }
        };
        let url = self
            .provider
            .authorize_url(&discovery, &state, &nonce, &code_challenge);
        Response::redirect(302, &url)
    }

    /// `GET /auth/callback`: exchange the code, verify the identity, and open a
    /// session for the owner.
    pub fn callback(&self, request: &Request) -> Response {
        if let Some(error) = request.query_param("error") {
            let description = request
                .query_param("error_description")
                .unwrap_or_default();
            return text(400, &format!("login failed: {error} {description}"));
        }
        let (Some(code), Some(state)) = (request.query_param("code"), request.query_param("state"))
        else {
            return text(400, "missing code or state");
        };

        let now = Utc::now();
        let flow = match self.db.take_auth_flow(&state) {
            Ok(Some(flow)) if flow.expires_at > now => flow,
            Ok(_) => return text(400, "unknown or expired login state"),
            Err(error) => {
                eprintln!("login state lookup failed: {error}");
                return text(500, "login unavailable");
            }
        };

        let discovery = match self.provider.discovery() {
            Ok(discovery) => discovery,
            Err(error) => {
                eprintln!("OIDC discovery failed: {error}");
                return text(503, "identity provider unavailable");
            }
        };
        let claims = match self
            .provider
            .exchange(&discovery, &code, &flow.nonce, &flow.code_verifier)
        {
            Ok(claims) => claims,
            Err(error) => {
                eprintln!("token exchange failed: {error}");
                return text(502, "identity provider rejected the login");
            }
        };
        match self.claim_owner(&claims.sub) {
            Ok(Claim::Owner) => {}
            Ok(Claim::Adopted) => eprintln!(
                "adopted subject {:?} as the tracker owner; set RECIBASE_OWNER_SUBJECT to pin it",
                claims.sub
            ),
            Ok(Claim::Rejected) => {
                eprintln!(
                    "login rejected: subject {:?} is not the configured owner",
                    claims.sub
                );
                return text(403, "this account is not authorised");
            }
            Err(error) => {
                eprintln!("owner lookup failed: {error}");
                return text(500, "login unavailable");
            }
        }

        let user = match self
            .db
            .upsert_user(&claims.sub, claims.email.as_deref())
        {
            Ok(user) => user,
            Err(error) => {
                eprintln!("could not record the signed-in user: {error}");
                return text(500, "login unavailable");
            }
        };
        let csrf_token = crate::tokens::random_hex(16);
        let session_id = match self.db.create_session(
            user.id,
            &csrf_token,
            now + Duration::days(SESSION_TTL_DAYS),
        ) {
            Ok(id) => id,
            Err(error) => {
                eprintln!("could not open a session: {error}");
                return text(500, "login unavailable");
            }
        };

        let mut response = Response::redirect(303, &self.config.post_login_redirect);
        response.set_cookie(self.session_cookie(&session_id, SESSION_TTL_DAYS * 86_400));
        response
    }

    /// `POST /auth/logout`: drop the session and clear the cookie.
    pub fn logout(&self, request: &Request) -> Response {
        if let Some(id) = request.cookie(SESSION_COOKIE)
            && let Err(error) = self.db.delete_session(&id)
        {
            eprintln!("could not delete the session: {error}");
        }
        let mut response = Response::redirect(303, "/");
        response.set_cookie(self.clear_cookie());
        response
    }

    /// The session and its user for the request, when the cookie is present and
    /// valid.
    pub fn session(&self, request: &Request) -> Option<(Session, User)> {
        let id = request.cookie(SESSION_COOKIE)?;
        match self.db.session(&id, Utc::now()) {
            Ok(found) => found,
            Err(error) => {
                eprintln!("session lookup failed: {error}");
                None
            }
        }
    }

    /// The owner's OIDC subject: the pinned one, or the adopted one from the
    /// store. `None` until the first login has happened.
    pub fn owner_subject(&self) -> Option<String> {
        self.config
            .oidc
            .owner_subject
            .clone()
            .or_else(|| self.db.setting(crate::db::OWNER_SUBJECT).ok().flatten())
    }

    /// Whether `subject` may sign in: the pinned owner, the adopted one, or -
    /// when nothing is pinned and none is adopted yet - this first subject,
    /// which is recorded as the owner.
    fn claim_owner(&self, subject: &str) -> Result<Claim, String> {
        if let Some(configured) = &self.config.oidc.owner_subject {
            return Ok(if configured == subject {
                Claim::Owner
            } else {
                Claim::Rejected
            });
        }
        match self
            .db
            .setting(crate::db::OWNER_SUBJECT)
            .map_err(|error| error.to_string())?
        {
            Some(owner) if owner == subject => Ok(Claim::Owner),
            Some(_) => Ok(Claim::Rejected),
            None => {
                self.db
                    .set_setting(crate::db::OWNER_SUBJECT, subject)
                    .map_err(|error| error.to_string())?;
                Ok(Claim::Adopted)
            }
        }
    }

    fn session_cookie(&self, id: &str, max_age: i64) -> String {
        let secure = if self.config.cookie_secure { "; Secure" } else { "" };
        format!(
            "{SESSION_COOKIE}={id}; HttpOnly; SameSite=Lax; Path=/; Max-Age={max_age}{secure}"
        )
    }

    fn clear_cookie(&self) -> String {
        let secure = if self.config.cookie_secure { "; Secure" } else { "" };
        format!("{SESSION_COOKIE}=; HttpOnly; SameSite=Lax; Path=/; Max-Age=0{secure}")
    }

    /// Whether the form body carries the session's CSRF token.
    pub fn csrf_ok(&self, request: &Request, session: &Session) -> bool {
        parse_form(&request.body)
            .iter()
            .any(|(name, value)| name == "csrf" && value == &session.csrf_token)
    }

    /// Whether a state-changing request comes from the app's own origin. A
    /// browser always sends `Origin` on a cross-origin form POST; a missing one
    /// is accepted (non-browser clients are already authenticated by cookie).
    pub fn origin_ok(&self, request: &Request) -> bool {
        match request.header("Origin") {
            None => true,
            Some(origin) => match app_origin(&self.config.oidc.redirect_url) {
                Some(expected) => origin.trim_end_matches('/') == expected,
                None => false,
            },
        }
    }
}

/// The result of checking a login subject against the tracker's owner.
enum Claim {
    /// The configured or previously adopted owner.
    Owner,
    /// No owner was recorded; this subject has just been adopted.
    Adopted,
    /// Someone other than the owner.
    Rejected,
}

/// `scheme://authority` of a URL, dropping any path.
fn app_origin(url: &str) -> Option<String> {
    let (scheme, rest) = url.split_once("://")?;
    let authority = rest.split('/').next()?;
    Some(format!("{scheme}://{authority}"))
}

/// A plain-text response with a status.
pub fn text(status: u16, message: &str) -> Response {
    Response::status(status, "text/plain; charset=UTF-8", message.as_bytes().to_vec())
}
