//! The CORS layer, byte-for-byte against the Scala server's behaviour.
//!
//! The Scala wraps the route set in `CORS.policy.withAllowOriginAll`, so an
//! `OPTIONS` preflight is answered by the middleware and any other request that
//! matches a route gets `Access-Control-Allow-Origin: *` appended. A request
//! that matches no route is answered by `orNotFound` outside the middleware.
//! Every expectation here was measured against the running Scala server
//! (`capture-scala-cors*`).

use recibase_server::http::{Request, Response};
use recibase_server::routes::{Context, route};

const DATE: &str = "Thu, 01 Oct 2026 21:04:30 GMT";

fn context() -> Context {
    Context::new(Box::new(|_| None))
}

fn request(method: &str, target: &str, headers: &[(&str, &str)]) -> Request {
    let (path, query) = match target.split_once('?') {
        Some((p, q)) => (p.to_string(), q.to_string()),
        None => (target.to_string(), String::new()),
    };
    Request {
        method: method.to_string(),
        target: target.to_string(),
        version: "HTTP/1.1".to_string(),
        path,
        query,
        headers: headers
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect(),
        body: Vec::new(),
    }
}

fn wire(response: &Response) -> String {
    String::from_utf8(response.to_bytes("close", DATE)).expect("UTF-8 response")
}

const ORIGIN: &str = "https://c.reciba.se";

#[test]
fn preflight_is_answered_by_the_middleware_for_any_path() {
    for target in [
        "/recipes/",
        "/recipes/vegetable-primavera",
        "/health",
        "/meals/",
        "/nope",
    ] {
        let response = route(
            &request(
                "OPTIONS",
                target,
                &[
                    ("Origin", ORIGIN),
                    ("Access-Control-Request-Method", "GET"),
                    ("Access-Control-Request-Headers", "authorization"),
                ],
            ),
            &context(),
        );
        assert_eq!(
            wire(&response),
            concat!(
                "HTTP/1.1 200 OK\r\n",
                "Date: Thu, 01 Oct 2026 21:04:30 GMT\r\n",
                "Connection: close\r\n",
                "Access-Control-Allow-Origin: *\r\n",
                "Access-Control-Allow-Methods: PATCH, HEAD, QUERY, PUT, GET, POST, DELETE\r\n",
                "Access-Control-Allow-Headers: authorization\r\n",
                "Vary: Access-Control-Request-Method, Access-Control-Request-Headers\r\n",
                "Content-Length: 0\r\n\r\n",
            ),
            "{target}"
        );
    }
}

#[test]
fn preflight_echoes_the_requested_headers_trimmed_and_space_separated() {
    let response = route(
        &request(
            "OPTIONS",
            "/recipe-submissions",
            &[
                ("Origin", ORIGIN),
                ("Access-Control-Request-Method", "POST"),
                (
                    "Access-Control-Request-Headers",
                    "authorization,content-type",
                ),
            ],
        ),
        &context(),
    );
    assert!(
        wire(&response).contains("Access-Control-Allow-Headers: authorization, content-type\r\n")
    );

    // No requested headers at all still produces the header, empty.
    let response = route(
        &request(
            "OPTIONS",
            "/recipes/",
            &[("Origin", ORIGIN), ("Access-Control-Request-Method", "GET")],
        ),
        &context(),
    );
    assert!(wire(&response).contains("Access-Control-Allow-Headers: \r\n"));
}

#[test]
fn options_without_an_origin_or_a_requested_method_is_not_found() {
    for headers in [
        vec![("Origin", ORIGIN)],
        vec![("Access-Control-Request-Method", "GET")],
    ] {
        let response = route(&request("OPTIONS", "/recipes/", &headers), &context());
        assert_eq!(response.status, 404);
        assert!(!wire(&response).contains("Access-Control"));
    }
}

#[test]
fn a_matched_route_gets_the_allow_origin_header_after_content_length() {
    let response = route(
        &request(
            "GET",
            "/recipes/baked-rigatoni-aubergine",
            &[("Origin", ORIGIN)],
        ),
        &context(),
    );
    let text = wire(&response);
    let (head, body) = text.split_once("\r\n\r\n").expect("header terminator");
    assert!(head.contains("Content-Type: application/json\r\n"));
    assert!(head.contains("Content-Length: "));
    // The allow-origin header is written after Content-Length, as http4s does.
    assert!(head.ends_with("Access-Control-Allow-Origin: *"), "{head}");
    assert!(body.starts_with("{\""), "the body is unchanged");
}

#[test]
fn a_404_from_a_matched_route_keeps_the_cors_header() {
    let response = route(
        &request("GET", "/recipes/i-do-not-exist", &[("Origin", ORIGIN)]),
        &context(),
    );
    assert_eq!(response.status, 404);
    assert!(wire(&response).contains("Access-Control-Allow-Origin: *\r\n"));
}

#[test]
fn an_unmatched_request_gets_no_cors_header() {
    for (method, target) in [
        ("GET", "/nope"),
        ("GET", "/nope"),
        ("PUT", "/recipes/"),
        ("HEAD", "/health"),
        ("HEAD", "/recipes/"),
    ] {
        let response = route(&request(method, target, &[("Origin", ORIGIN)]), &context());
        assert_eq!(response.status, 404, "{method} {target}");
        assert_eq!(
            wire(&response),
            concat!(
                "HTTP/1.1 404 Not Found\r\n",
                "Date: Thu, 01 Oct 2026 21:04:30 GMT\r\n",
                "Connection: close\r\n",
                "Content-Type: text/plain; charset=UTF-8\r\n",
                "Content-Length: 9\r\n\r\nNot found",
            ),
            "{method} {target}"
        );
    }
}

#[test]
fn without_an_origin_the_response_is_unchanged() {
    let with = wire(&route(
        &request("GET", "/manifest", &[("Origin", ORIGIN)]),
        &context(),
    ));
    let without = wire(&route(&request("GET", "/manifest", &[]), &context()));
    assert!(with.contains("Access-Control-Allow-Origin: *"));
    assert!(!without.contains("Access-Control"));
}
