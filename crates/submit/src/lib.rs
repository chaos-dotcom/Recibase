//! `se.reciba.api.submit` - recipe submission, Scala source generation, the
//! Cloudflare Turnstile check and the GitHub pull request client.

pub mod config;
pub mod github_client;
pub mod pull_requests;
pub mod recipe_source;
pub mod recipe_submission;
pub mod scala_literal;
pub mod turnstile;

pub use config::{GithubSettings, RecipeSubmissionConfig, DEFAULT_BRANCH, DEFAULT_REPOSITORY};
pub use github_client::{GithubClient, DEFAULT_API_BASE};
pub use pull_requests::{PullRequestFailure, RecipePullRequests};
pub use recipe_source::{GeneratedRecipe, RecipeSource, SubmitRejection};
pub use recipe_submission::{IngredientSubmission, Passcode, RecipeSubmission};
pub use scala_literal::ScalaLiteral;
pub use turnstile::{CloudflareTurnstile, Turnstile, TurnstileChecker, TurnstileSettings};

/// The Scala code logs through slf4j. The port writes the same lines to stderr
/// so that no log text ever ends up in a returned value.
pub(crate) mod log {
    pub fn warn(message: &str) {
        eprintln!("WARN {message}");
    }

    pub fn info(message: &str) {
        eprintln!("INFO {message}");
    }
}

/// The bits of `java.net.http.HttpClient` the two HTTP clients need: redirects
/// are never followed, request timeouts are per request, and a non-2xx status
/// is returned rather than raised.
pub(crate) mod net {
    use std::time::Duration;

    pub fn agent(connect_timeout: Duration, request_timeout: Duration) -> ureq::Agent {
        let config = ureq::Agent::config_builder()
            .http_status_as_error(false)
            .max_redirects(0)
            .timeout_connect(Some(connect_timeout))
            .timeout_global(Some(request_timeout))
            // The Scala sets every header it wants explicitly.
            .user_agent("")
            .accept("")
            .accept_encoding("")
            .build();
        ureq::Agent::new_with_config(config)
    }

    /// `HttpResponse.BodyHandlers.ofString(UTF_8)`: invalid bytes are replaced.
    pub fn read_body(response: ureq::http::Response<ureq::Body>) -> String {
        match response.into_body().read_to_vec() {
            Ok(bytes) => String::from_utf8_lossy(&bytes).into_owned(),
            Err(_) => String::new(),
        }
    }

    /// The class of the failure, as the Scala's `getClass.getSimpleName` would
    /// name it.
    pub fn error_name(error: &ureq::Error) -> &'static str {
        match error {
            ureq::Error::Io(_) => "IOException",
            ureq::Error::Timeout(_) => "HttpTimeoutException",
            ureq::Error::HostNotFound => "UnknownHostException",
            ureq::Error::ConnectionFailed => "ConnectException",
            ureq::Error::Protocol(_) => "IOException",
            ureq::Error::BadUri(_) => "IllegalArgumentException",
            ureq::Error::StatusCode(_) => "HttpException",
            _ => "Exception",
        }
    }
}

/// Helpers that copy `java.lang.String`, `java.net.URLEncoder` and `Regex`
/// behaviour that the port has to reproduce exactly.
pub(crate) mod java {
    /// `String.length`: the number of UTF-16 code units.
    pub fn utf16_len(value: &str) -> usize {
        value.encode_utf16().count()
    }

    /// `String.take(n)`: the first `n` UTF-16 code units.
    pub fn take_utf16(value: &str, limit: usize) -> String {
        let mut out = String::new();
        let mut used = 0usize;
        for c in value.chars() {
            let width = c.len_utf16();
            if used + width > limit {
                break;
            }
            out.push(c);
            used += width;
        }
        out
    }

    /// `Character.toUpperCase`: the single-code-unit mapping, or the character
    /// itself when the uppercase form is longer than one code unit.
    fn char_upper(c: char) -> char {
        let mut upper = c.to_uppercase();
        match (upper.next(), upper.next()) {
            (Some(single), None) => single,
            _ => c,
        }
    }

    /// `Character.toLowerCase`, as [`char_upper`].
    fn char_lower(c: char) -> char {
        let mut lower = c.to_lowercase();
        match (lower.next(), lower.next()) {
            (Some(single), None) => single,
            _ => c,
        }
    }

    /// `String.equalsIgnoreCase`: `toUpperCase` or `toLowerCase` per code unit.
    pub fn equals_ignore_case(a: &str, b: &str) -> bool {
        let mut left = a.chars();
        let mut right = b.chars();
        loop {
            match (left.next(), right.next()) {
                (None, None) => return true,
                (Some(x), Some(y)) => {
                    if x == y || char_upper(x) == char_upper(y) || char_lower(x) == char_lower(y)
                    {
                        continue;
                    }
                    return false;
                }
                _ => return false,
            }
        }
    }

    /// `java.net.URLEncoder.encode(value, "UTF-8")`: space becomes `+`, `*` stays
    /// as-is, `~` is encoded, and the hex digits are upper case.
    pub fn url_encode_form(value: &str) -> String {
        let mut out = String::new();
        for byte in value.as_bytes() {
            let c = *byte as char;
            match c {
                'a'..='z' | 'A'..='Z' | '0'..='9' | '.' | '-' | '*' | '_' => out.push(c),
                ' ' => out.push('+'),
                _ => out.push_str(&format!("%{:02X}", byte)),
            }
        }
        out
    }

    /// `String.replace(target, replacement)`. With an empty target Java inserts
    /// the replacement at every character boundary, including both ends.
    pub fn replace_literal(haystack: &str, target: &str, replacement: &str) -> String {
        if target.is_empty() {
            let mut out = String::new();
            for c in haystack.chars() {
                out.push_str(replacement);
                out.push(c);
            }
            out.push_str(replacement);
            out
        } else {
            haystack.replace(target, replacement)
        }
    }
}
