
//! The HTTP client for the recipe API, the Rust spelling of `requests`.

use std::time::Duration;

use serde_json::Value;

use crate::cached_backend::BackendUnavailable;

/// One response from the API, in the shape `contribute.rs` needs.
#[derive(Debug, Clone)]
pub struct BackendResponse {
    pub status: u16,
    pub content_type: String,
    pub text: String,
}

pub struct BackendClient {
    base_url: String,
    get_agent: ureq::Agent,
    post_agent: ureq::Agent,
}

impl BackendClient {
    /// `backendBaseUrl` from `app.py`; the trailing slash is the caller's.
    pub fn new(base_url: String) -> BackendClient {
        BackendClient {
            base_url,
            // `requests.get(..., timeout=10)`.
            get_agent: agent(Duration::from_secs(10)),
            // `requests.post(..., timeout=30)`.
            post_agent: agent(Duration::from_secs(30)),
        }
    }

    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    /// `requests.get(url, timeout=10).raise_for_status().json()`.
    pub fn get_json(&self, path: &str) -> Result<Value, BackendUnavailable> {
        let response = self.get(path)?;
        serde_json::from_str(&response.text).map_err(|_| BackendUnavailable)
    }

    /// `requests.get` without `raise_for_status`: the caller decides what a
    /// 404 means.
    pub fn get(&self, path: &str) -> Result<BackendResponse, BackendUnavailable> {
        let url = format!("{}{}", self.base_url, path);
        let mut response = self
            .get_agent
            .get(&url)
            .call()
            .map_err(|_| BackendUnavailable)?;
        read_response(&mut response)
    }

    /// `requests.post(url, json=..., headers=..., timeout=30)`.
    pub fn post_json(
        &self,
        path: &str,
        body: &Value,
        authorization: &str,
    ) -> Result<BackendResponse, BackendUnavailable> {
        let url = format!("{}{}", self.base_url, path);
        let payload = serde_json::to_string(body).unwrap_or_else(|_| "null".to_string());
        let mut response = self
            .post_agent
            .post(&url)
            .header("Content-Type", "application/json")
            .header("Authorization", authorization)
            .send(payload.as_bytes())
            .map_err(|_| BackendUnavailable)?;
        read_response(&mut response)
    }
}

fn agent(timeout: Duration) -> ureq::Agent {
    ureq::Agent::config_builder()
        .timeout_global(Some(timeout))
        // `requests` does not raise for a 4xx/5xx; neither does this, so the
        // caller can look at the status the way `app.py` does.
        .http_status_as_error(false)
        .build()
        .new_agent()
}

fn read_response(
    response: &mut ureq::http::Response<ureq::Body>,
) -> Result<BackendResponse, BackendUnavailable> {
    let status = response.status().as_u16();
    let content_type = response
        .headers()
        .get("content-type")
        .and_then(|value| value.to_str().ok())
        .unwrap_or("")
        .to_string();
    let text = response
        .body_mut()
        .read_to_string()
        .map_err(|_| BackendUnavailable)?;
    Ok(BackendResponse { status, content_type, text })
}
