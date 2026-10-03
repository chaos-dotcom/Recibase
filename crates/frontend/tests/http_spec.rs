//! Unit tests for `http.rs`: the wire format, the request parser, and the two
//! target rewrites Werkzeug applies before routing.

mod support;

use std::io::Write;
use std::net::{Shutdown, TcpListener, TcpStream};

use chrono::DateTime;
use recibase_frontend::http::{
    Request, Response, http_date, merge_slashes, percent_decode, read_request, reason_phrase,
};
use support::{body_of, get, header, post_form, request, with_header};

/// Werkzeug's default `merge_slashes`: `//chicken-curry` matches the
/// `/chicken-curry` rule.
#[test]
fn merge_slashes_collapses_repeats() {
    assert_eq!(merge_slashes("//a///b"), "/a/b");
    assert_eq!(merge_slashes("/"), "/");
    assert_eq!(merge_slashes(""), "");
    assert_eq!(merge_slashes("/a/b"), "/a/b");
    assert_eq!(merge_slashes("/a//"), "/a/");
    assert_eq!(merge_slashes("//"), "/");
}

#[test]
fn percent_decode_decodes_escapes() {
    assert_eq!(percent_decode("/a%20b"), "/a b");
    assert_eq!(percent_decode("%2Fetc%2Fpasswd"), "/etc/passwd");
    assert_eq!(percent_decode("%E2%9C%93"), "✓");
    // Anything that is not a valid escape is left exactly as it was.
    assert_eq!(percent_decode("%zz"), "%zz");
    assert_eq!(percent_decode("100%"), "100%");
    assert_eq!(percent_decode("%4"), "%4");
    // `unquote` decodes `%XX` only; `+` is `form.percent_decode`'s job.
    assert_eq!(percent_decode("a+b"), "a+b");
}

/// `request.args.get(name)`: the first value of a repeated key wins.
#[test]
fn query_param_takes_the_first_value() {
    let request = request("GET", "/test-recipe?scale=2&scale=3&name=a%20b");
    assert_eq!(request.query_param("scale"), Some("2".to_string()));
    assert_eq!(request.query_param("name"), Some("a b".to_string()));
    assert_eq!(request.query_param("missing"), None);

    let no_query = get("/test-recipe");
    assert_eq!(no_query.query, "");
    assert_eq!(no_query.query_param("scale"), None);
}

#[test]
fn form_parses_urlencoded_bodies() {
    let request = post_form(
        "/contribute",
        &[("name", "A&B"), ("name", "second"), ("blank", ""), ("pct", "\"x\"")],
    );
    let form = request.form();
    assert_eq!(form.get("name"), Some("A&B"));
    assert_eq!(form.get_list("name"), vec!["A&B", "second"]);
    assert_eq!(form.get("blank"), Some(""));
    assert_eq!(form.get("pct"), Some("\"x\""));
    assert_eq!(form.get_or("missing", "fallback"), "fallback");
}

/// A browser always sends the form content type for this application's only
/// form, so anything else is an empty `request.form`.
#[test]
fn form_ignores_other_content_types() {
    let request = with_header(request("POST", "/contribute"), "Content-Type", "application/json");
    assert!(request.form().is_empty());
}

#[test]
fn keeps_alive_follows_the_connection_header() {
    assert!(request("GET", "/").keeps_alive());
    assert!(!with_header(request("GET", "/"), "Connection", "close").keeps_alive());
    assert!(with_header(request("GET", "/"), "Connection", "keep-alive").keeps_alive());

    let mut http_1_0 = request("GET", "/");
    http_1_0.version = "HTTP/1.0".to_string();
    assert!(!http_1_0.keeps_alive());
    let mut http_1_0_keep_alive = with_header(request("GET", "/"), "Connection", "keep-alive");
    http_1_0_keep_alive.version = "HTTP/1.0".to_string();
    assert!(http_1_0_keep_alive.keeps_alive());
}

/// `request.url_root` is built from the `Host` header.
#[test]
fn url_root_uses_the_host_header() {
    assert_eq!(get("/").url_root(), "http://127.0.0.1:8080/");
    let mut no_host = get("/");
    no_host.host = None;
    assert_eq!(no_host.url_root(), "http://localhost/");
}

#[test]
fn reason_phrases_are_werkzeugs() {
    assert_eq!(reason_phrase(200), "OK");
    assert_eq!(reason_phrase(206), "PARTIAL CONTENT");
    assert_eq!(reason_phrase(301), "MOVED PERMANENTLY");
    assert_eq!(reason_phrase(302), "FOUND");
    assert_eq!(reason_phrase(304), "NOT MODIFIED");
    assert_eq!(reason_phrase(400), "BAD REQUEST");
    assert_eq!(reason_phrase(404), "NOT FOUND");
    assert_eq!(reason_phrase(405), "METHOD NOT ALLOWED");
    assert_eq!(reason_phrase(416), "REQUESTED RANGE NOT SATISFIABLE");
    assert_eq!(reason_phrase(500), "INTERNAL SERVER ERROR");
    assert_eq!(reason_phrase(503), "SERVICE UNAVAILABLE");
    assert_eq!(reason_phrase(418), "UNKNOWN");
}

/// The header block, in order, with the `Content-Length` the body has.
#[test]
fn to_bytes_writes_the_wire_format() {
    let response = Response::html(200, "hi".to_string());
    let wire = String::from_utf8_lossy(&response.to_bytes(
        "Mon, 01 Jan 2024 00:00:00 GMT",
        "keep-alive",
        false,
    ))
    .into_owned();
    assert_eq!(
        wire,
        "HTTP/1.1 200 OK\r\n\
         Date: Mon, 01 Jan 2024 00:00:00 GMT\r\n\
         Content-Type: text/html; charset=utf-8\r\n\
         Content-Length: 2\r\n\
         Connection: keep-alive\r\n\
         \r\n\
         hi"
    );

    let head_only = String::from_utf8_lossy(&response.to_bytes(
        "Mon, 01 Jan 2024 00:00:00 GMT",
        "close",
        true,
    ))
    .into_owned();
    assert!(head_only.ends_with("Connection: close\r\n\r\n"), "{head_only}");
    assert_eq!(response.reason(), "OK");
}

#[test]
fn http_date_is_rfc_1123_gmt() {
    let epoch = DateTime::from_timestamp(0, 0).expect("the epoch");
    assert_eq!(http_date(epoch), "Thu, 01 Jan 1970 00:00:00 GMT");
}

/// `read_request`, over a real socket: the request line, the headers with
/// their case, the body by `Content-Length`, and the path rewrites.
#[test]
fn read_request_parses_the_wire() {
    read_one(
        "GET //a///b?x=1&y=%2F HTTP/1.1\r\nHost: example.test\r\nConnection: close\r\n\r\n",
        |request| {
            assert_eq!(request.method, "GET");
            assert_eq!(request.target, "//a///b?x=1&y=%2F");
            assert_eq!(request.version, "HTTP/1.1");
            assert_eq!(request.path, "/a/b");
            assert_eq!(request.query, "x=1&y=%2F");
            assert_eq!(request.host.as_deref(), Some("example.test"));
            assert_eq!(request.query_param("y"), Some("/".to_string()));
            assert!(!request.keeps_alive());
            assert!(!request.is_head());
            assert!(request.body.is_empty());
        },
    );

    read_one(
        "POST /contribute HTTP/1.1\r\nHost: h\r\nContent-Type: application/x-www-form-urlencoded\r\nContent-Length: 7\r\n\r\nname=ab",
        |request| {
            assert_eq!(request.method, "POST");
            assert_eq!(request.body, b"name=ab");
            assert_eq!(request.form().get("name"), Some("ab"));
            assert!(request.keeps_alive());
        },
    );

    read_one("HEAD / HTTP/1.0\r\n\r\n", |request| {
        assert!(request.is_head());
        assert_eq!(request.version, "HTTP/1.0");
        assert_eq!(request.path, "/");
        assert_eq!(request.host, None);
    });
}

/// Sends `raw` into a socket and hands the parsed request to `check`.
fn read_one(raw: &str, check: impl FnOnce(&Request)) {
    let listener = TcpListener::bind(("127.0.0.1", 0)).expect("bind a socket");
    let port = listener.local_addr().expect("the socket's address").port();
    let bytes = raw.as_bytes().to_vec();
    let sender = std::thread::spawn(move || {
        let mut stream = TcpStream::connect(("127.0.0.1", port)).expect("connect");
        stream.write_all(&bytes).expect("write the request");
        stream.shutdown(Shutdown::Write).expect("half close");
    });
    let (stream, _) = listener.accept().expect("accept");
    let request = read_request(&stream)
        .expect("read the request")
        .expect("a request, not an end of stream");
    check(&request);
    sender.join().expect("the sender finished");
}

/// The headers a response carries are the ones a caller can look up, in the
/// case the callers use.
#[test]
fn response_headers_are_case_insensitive() {
    let response = Response::new(404).header("Content-Type", "text/plain");
    assert_eq!(header(&response, "content-type"), Some("text/plain"));
    assert_eq!(header(&response, "Content-Length"), None);
    assert_eq!(body_of(&response), "");
}
