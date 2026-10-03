//! `se.reciba.api.submit.Turnstile` - the Cloudflare Turnstile siteverify check.

use crate::java::{url_encode_form, utf16_len};
use crate::log;
use crate::net;
use serde_json::Value;
use std::collections::HashSet;
use std::time::Duration;

/// `case class TurnstileSettings(secret: String, hostnames: Set[String])`.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TurnstileSettings {
    pub secret: String,
    pub hostnames: HashSet<String>,
}

/// `object Turnstile`: the two pure acceptance checks.
pub struct Turnstile;

impl Turnstile {
    pub const EXPECTED_ACTION: &'static str = "contribute";
    pub const SITEVERIFY_URL: &'static str =
        "https://challenges.cloudflare.com/turnstile/v0/siteverify";

    /// `success` must be `true`, `action` must be `contribute`, and `hostname`
    /// must be one of the configured hostnames. A field of the wrong type is a
    /// failure, like a failed circe cursor.
    pub fn accepted(body: &Value, hostnames: &HashSet<String>) -> bool {
        let success = body.get("success").and_then(Value::as_bool);
        let action = body.get("action").and_then(Value::as_str);
        let hostname = body.get("hostname").and_then(Value::as_str);
        success == Some(true)
            && action == Some(Turnstile::EXPECTED_ACTION)
            && hostname.map(|name| hostnames.contains(name)).unwrap_or(false)
    }

    pub fn token_accepted(token: &str, hostnames: &HashSet<String>) -> bool {
        !token.is_empty() && utf16_len(token) <= 2048 && !hostnames.is_empty()
    }
}

pub trait TurnstileChecker {
    fn allow(&self, token: &str) -> bool;
}

/// `class CloudflareTurnstile[F[_]: Async](settings: TurnstileSettings)`.
#[derive(Debug, Clone)]
pub struct CloudflareTurnstile {
    settings: TurnstileSettings,
}

impl CloudflareTurnstile {
    pub fn new(settings: TurnstileSettings) -> Self {
        CloudflareTurnstile { settings }
    }

    pub fn settings(&self) -> &TurnstileSettings {
        &self.settings
    }

    pub fn allow(&self, token: &str) -> bool {
        if !Turnstile::token_accepted(token, &self.settings.hostnames) {
            return false;
        }
        let agent = net::agent(Duration::from_secs(10), Duration::from_secs(10));
        let body = form_body(&self.settings.secret, token);
        let response = agent
            .post(Turnstile::SITEVERIFY_URL)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .send(body.as_str());
        match response {
            Err(error) => {
                log::warn(&format!(
                    "turnstile siteverify failed: {}",
                    net::error_name(&error)
                ));
                false
            }
            Ok(response) => {
                if response.status().as_u16() != 200 {
                    false
                } else {
                    let text = net::read_body(response);
                    match serde_json::from_str::<Value>(&text) {
                        Ok(json) => Turnstile::accepted(&json, &self.settings.hostnames),
                        Err(_) => false,
                    }
                }
            }
        }
    }
}

impl TurnstileChecker for CloudflareTurnstile {
    fn allow(&self, token: &str) -> bool {
        CloudflareTurnstile::allow(self, token)
    }
}

fn form_body(secret: &str, token: &str) -> String {
    let fields = [("secret", secret), ("response", token)];
    fields
        .iter()
        .map(|(key, value)| {
            format!("{}={}", url_encode_form(key), url_encode_form(value))
        })
        .collect::<Vec<String>>()
        .join("&")
}
