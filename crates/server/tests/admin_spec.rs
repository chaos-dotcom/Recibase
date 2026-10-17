//! The admin tracker routes when the tracker is not configured.
//!
//! With no `DATABASE_URL`/OIDC in the environment the routes still match, so a
//! misconfigured deployment fails loudly with a 503 rather than 404, and every
//! existing read-only route is untouched.

mod support;

use support::{context, request};

#[test]
fn tracker_routes_are_unavailable_without_configuration() {
    let cases = [
        ("GET", "/auth/login"),
        ("GET", "/auth/callback"),
        ("GET", "/admin"),
        ("POST", "/auth/logout"),
        ("POST", "/admin/entries"),
        ("POST", "/admin/entries/delete"),
    ];
    for (method, path) in cases {
        let response = recibase_server::route(&request(method, path), &context());
        assert_eq!(response.status, 503, "{method} {path}");
        let body = support::body(&response);
        assert!(body.contains("not configured"), "{method} {path}: {body}");
    }
}
