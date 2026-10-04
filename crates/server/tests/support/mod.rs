//! Shared helpers for the API-contract specs.
//!
//! `contract.rs` pins the shape of this deployment's own responses;
//! `live_contract.rs` compares that shape with another deployment's. Both drive
//! `route` in-process, so what they check is the route table and the
//! serialisation rather than the socket.

#![allow(dead_code)]

use recibase_server::http::{Request, Response};
use recibase_server::routes::{Context, route};
use serde_json::Value;

/// A context with no meal log, the shape the default `/meals/` answers with.
pub fn context() -> Context {
    Context::new(Box::new(|_| None))
}

pub fn request(method: &str, target: &str) -> Request {
    let (path, query) = match target.split_once('?') {
        Some((path, query)) => (path.to_string(), query.to_string()),
        None => (target.to_string(), String::new()),
    };
    Request {
        method: method.to_string(),
        target: target.to_string(),
        version: "HTTP/1.1".to_string(),
        path,
        query,
        headers: Vec::new(),
        body: Vec::new(),
    }
}

/// A `GET` through the route table.
pub fn get(target: &str) -> Response {
    route(&request("GET", target), &context())
}

pub fn body(response: &Response) -> String {
    String::from_utf8_lossy(&response.body).into_owned()
}

/// The response body, parsed as JSON.
pub fn json(response: &Response) -> Value {
    let text = body(response);
    serde_json::from_str(&text).unwrap_or_else(|error| panic!("not JSON ({error}): {text}"))
}

/// A JSON object's keys, in the order they were written.
pub fn keys(value: &Value) -> Vec<String> {
    value
        .as_object()
        .unwrap_or_else(|| panic!("not a JSON object: {value}"))
        .keys()
        .cloned()
        .collect()
}

/// The same keys, sorted, for comparing two sets without their order.
pub fn sorted_keys(value: &Value) -> Vec<String> {
    let mut keys = keys(value);
    keys.sort();
    keys
}

/// Asserts an object's keys are exactly `expected`, in that order.
pub fn assert_keys(value: &Value, expected: &[&str]) {
    let expected: Vec<String> = expected.iter().map(|key| key.to_string()).collect();
    assert_eq!(keys(value), expected);
}

/// Whether `keys` are a subset of `order`, in `order`'s relative order.
pub fn key_order_is(keys: &[String], order: &[&str]) -> bool {
    let mut position = 0;
    for key in keys {
        while position < order.len() && order[position] != key.as_str() {
            position += 1;
        }
        if position == order.len() {
            return false;
        }
        position += 1;
    }
    true
}

/// The live deployment `RECIBASE_LIVE_API` names, with a trailing slash; `None`
/// when it is unset, which skips the live specs.
pub fn live_base() -> Option<String> {
    std::env::var("RECIBASE_LIVE_API")
        .ok()
        .filter(|base| !base.trim().is_empty())
        .map(|base| {
            let base = base.trim().to_string();
            if base.ends_with('/') {
                base
            } else {
                format!("{base}/")
            }
        })
}

/// `GET <base><path>` over the network, as `(status, content-type, body)`.
pub fn fetch(base: &str, path: &str) -> (u16, String, String) {
    let url = format!("{base}{path}");
    let agent = ureq::Agent::config_builder()
        .timeout_global(Some(std::time::Duration::from_secs(20)))
        // A 4xx/5xx is a response to compare, not a transport error.
        .http_status_as_error(false)
        .build()
        .new_agent();
    let mut response = agent
        .get(&url)
        .call()
        .unwrap_or_else(|error| panic!("GET {url}: {error}"));
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
        .unwrap_or_else(|error| panic!("body of {url}: {error}"));
    (status, content_type, text)
}
