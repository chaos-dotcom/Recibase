//! The admin tracker routes when the tracker is not configured.
//!
//! With no `DATABASE_URL`/OIDC in the environment the routes still match, so a
//! misconfigured deployment fails loudly with a 503 rather than 404, and every
//! existing read-only route is untouched. The JSON API and its credentialed
//! CORS are exercised here too, since they need no database.

mod support;

use recibase_server::http::Request;
use support::{context, request};

fn with_origin(mut request: Request, origin: &str) -> Request {
    request
        .headers
        .push(("Origin".to_string(), origin.to_string()));
    request
}

#[test]
fn tracker_routes_are_unavailable_without_configuration() {
    let cases = [
        ("GET", "/auth/login"),
        ("GET", "/auth/callback"),
        ("GET", "/admin"),
        ("GET", "/admin/api/session"),
        ("GET", "/admin/api/entries"),
        ("POST", "/auth/logout"),
        ("POST", "/admin/entries"),
        ("POST", "/admin/entries/delete"),
        ("POST", "/admin/api/entries"),
        ("POST", "/admin/api/entries/delete"),
        ("POST", "/admin/api/entries/update"),
    ];
    for (method, path) in cases {
        let response = recibase_server::route(&request(method, path), &context());
        assert_eq!(response.status, 503, "{method} {path}");
        let body = support::body(&response);
        assert!(body.contains("not configured"), "{method} {path}: {body}");
    }
}

#[test]
fn the_json_api_reflects_the_origin_with_credentials() {
    let origin = "https://recibase.example";
    let response = recibase_server::route(
        &with_origin(request("GET", "/admin/api/session"), origin),
        &context(),
    );
    let headers = &response.headers_before_length;
    assert!(
        headers
            .iter()
            .any(|(name, value)| *name == "Access-Control-Allow-Origin" && value == origin),
        "origin is reflected: {headers:?}"
    );
    assert!(
        headers
            .iter()
            .any(|(name, value)| *name == "Access-Control-Allow-Credentials" && value == "true"),
        "credentials are allowed: {headers:?}"
    );
}

#[test]
fn the_json_api_preflight_is_credentialed() {
    let origin = "https://recibase.example";
    let mut request = with_origin(request("OPTIONS", "/admin/api/entries"), origin);
    request.headers.push((
        "Access-Control-Request-Method".to_string(),
        "POST".to_string(),
    ));
    request.headers.push((
        "Access-Control-Request-Headers".to_string(),
        "content-type, x-csrf-token".to_string(),
    ));
    let response = recibase_server::route(&request, &context());
    assert_eq!(response.status, 200);
    let headers = &response.headers_before_length;
    assert!(
        headers
            .iter()
            .any(|(name, value)| *name == "Access-Control-Allow-Origin" && value == origin),
        "origin is reflected: {headers:?}"
    );
    assert!(
        headers.iter().any(|(name, _)| *name == "Access-Control-Allow-Credentials"),
        "credentials are allowed: {headers:?}"
    );
    assert!(
        headers.iter().any(|(name, value)| {
            *name == "Access-Control-Allow-Headers" && value == "content-type, x-csrf-token"
        }),
        "requested headers are echoed: {headers:?}"
    );
}

#[test]
fn read_only_routes_keep_the_wildcard_cors() {
    let mut request = with_origin(request("GET", "/meals/"), "https://recibase.example");
    request.method = "GET".to_string();
    let response = recibase_server::route(&request, &context());
    assert_eq!(response.status, 200);
    assert!(
        response
            .headers_after_length
            .iter()
            .any(|(name, value)| *name == "Access-Control-Allow-Origin" && *value == "*"),
        "the wildcard is unchanged: {:?}",
        response.headers_after_length
    );
    assert!(
        response
            .headers_before_length
            .iter()
            .all(|(name, _)| *name != "Access-Control-Allow-Credentials"),
        "no credentials on the public API"
    );
}
