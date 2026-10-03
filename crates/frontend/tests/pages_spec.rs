//! `pages.rs`: the three pages Werkzeug renders itself.

use recibase_frontend::pages::{method_not_allowed, range_not_satisfiable, redirect};
use support::{body_of, header_or};

mod support;

#[test]
fn redirect_is_werkzeugs() {
    let response = redirect(302, "test-recipe");
    assert_eq!(response.status, 302);
    assert_eq!(response.reason(), "FOUND");
    assert_eq!(header_or(&response, "Location"), "test-recipe");
    assert_eq!(header_or(&response, "Content-Type"), "text/html; charset=utf-8");
    let body = body_of(&response);
    assert!(body.contains("<title>Redirecting...</title>"), "{body}");
    assert!(body.contains("href=\"test-recipe\""), "{body}");
    assert_eq!(
        header_or(&response, "Content-Length"),
        body.len().to_string(),
    );
}

/// The escaping in the body is MarkupSafe's, and the header keeps the raw
/// location the client is sent to.
#[test]
fn redirect_escapes_the_body_but_not_the_location() {
    let location = "/a?b=<script>&y=\"z\"";
    let response = redirect(301, location);
    assert_eq!(response.status, 301);
    assert_eq!(response.reason(), "MOVED PERMANENTLY");
    assert_eq!(header_or(&response, "Location"), location);
    let body = body_of(&response);
    assert!(body.contains("&lt;script&gt;"), "{body}");
    assert!(body.contains("&#34;z&#34;"), "{body}");
    assert!(!body.contains("<script>"), "{body}");
}

#[test]
fn method_not_allowed_lists_the_methods() {
    let response = method_not_allowed(&["HEAD", "POST", "GET", "OPTIONS"]);
    assert_eq!(response.status, 405);
    assert_eq!(response.reason(), "METHOD NOT ALLOWED");
    assert_eq!(header_or(&response, "Allow"), "HEAD, POST, GET, OPTIONS");
    assert!(body_of(&response).contains("405 Method Not Allowed"));
}

#[test]
fn range_not_satisfiable_reports_the_size() {
    let response = range_not_satisfiable(1234);
    assert_eq!(response.status, 416);
    assert_eq!(response.reason(), "REQUESTED RANGE NOT SATISFIABLE");
    assert_eq!(header_or(&response, "Content-Range"), "bytes */1234");
    assert!(body_of(&response).contains("416 Requested Range Not Satisfiable"));
}
