
//! A minimal HTTP/1.1 server.
//!
//! The Flask application is served by two different servers in production
//! (`gunicorn`, per the `Procfile`) and in development (Werkzeug's
//! development server). Both are synchronous WSGI servers, so this port
//! uses the same shape: one thread per connection, a blocking read of one
//! request, one response.
//!
//! The response bytes are written by hand because the header set, the
//! header order and the reason phrase are part of what the frontend is
//! compared against:
//!
//! ```text
//! HTTP/1.1 <status> <REASON>\r\nDate: <rfc1123>\r\n<headers>\r\nConnection: <c>\r\n\r\n<body>
//! ```

use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpStream;

use crate::form::Form;

#[derive(Debug, Clone)]
pub struct Request {
    pub method: String,
    pub target: String,
    /// The HTTP version from the request line, e.g. `HTTP/1.1`.
    pub version: String,
    /// The path with duplicate slashes merged, percent escapes decoded.
    pub path: String,
    pub query: String,
    pub headers: Vec<(String, String)>,
    pub body: Vec<u8>,
    /// The value of the `Host` header, used to build `url_root`.
    pub host: Option<String>,
}

impl Request {
    pub fn header(&self, name: &str) -> Option<&str> {
        self.headers
            .iter()
            .find(|(k, _)| k.eq_ignore_ascii_case(name))
            .map(|(_, v)| v.as_str())
    }

    pub fn is_head(&self) -> bool {
        self.method.eq_ignore_ascii_case("HEAD")
    }

    /// Werkzeug's default `merge_slashes`: `//a///b` is the path `/a/b`.
    pub fn normalised_path(&self) -> &str {
        &self.path
    }

    /// `scheme://host/`, the shape `request.url_root` has. The Python reads
    /// `wsgi.url_scheme` (always `http` unless a WSGI server says otherwise)
    /// and the `Host` header; so does this.
    pub fn url_root(&self) -> String {
        let host = self
            .host
            .clone()
            .unwrap_or_else(|| "localhost".to_string());
        format!("http://{}/", host)
    }

    /// http4s/Ember-style connection handling, which is also what Werkzeug
    /// and gunicorn do for an HTTP/1.1 request: keep the connection unless
    /// the client asked for `close`.
    pub fn keeps_alive(&self) -> bool {
        let connection = self
            .header("Connection")
            .unwrap_or_default()
            .to_ascii_lowercase();
        if connection.split(',').any(|token| token.trim() == "close") {
            return false;
        }
        if connection.split(',').any(|token| token.trim() == "keep-alive") {
            return true;
        }
        self.version.eq_ignore_ascii_case("HTTP/1.1")
    }

    /// `request.args.get(name)` - the first value wins.
    pub fn query_param(&self, name: &str) -> Option<String> {
        let query = self.query.strip_prefix('?').unwrap_or(&self.query);
        for pair in query.split('&') {
            if pair.is_empty() {
                continue;
            }
            let (key, value) = match pair.split_once('=') {
                Some((k, v)) => (k, v),
                None => (pair, ""),
            };
            if percent_decode(key) == name {
                return Some(percent_decode(value));
            }
        }
        None
    }

    /// `request.form`: only the `application/x-www-form-urlencoded` content
    /// type is parsed. The contributor form has no file inputs, so a browser
    /// always sends that one.
    pub fn form(&self) -> Form {
        let content_type = self.header("Content-Type").unwrap_or_default();
        if !content_type
            .to_ascii_lowercase()
            .starts_with("application/x-www-form-urlencoded")
        {
            return Form::new();
        }
        Form::parse(&self.body)
    }
}

pub fn percent_decode(input: &str) -> String {
    let bytes = input.as_bytes();
    let mut out: Vec<u8> = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        match bytes[i] {
            b'%' if i + 2 < bytes.len() => {
                let hex = std::str::from_utf8(&bytes[i + 1..i + 3]).unwrap_or("");
                match u8::from_str_radix(hex, 16) {
                    Ok(byte) => {
                        out.push(byte);
                        i += 3;
                    }
                    Err(_) => {
                        out.push(bytes[i]);
                        i += 1;
                    }
                }
            }
            byte => {
                out.push(byte);
                i += 1;
            }
        }
    }
    String::from_utf8_lossy(&out).into_owned()
}

/// A response, as the headers that must be written in this order.
#[derive(Debug, Clone)]
pub struct Response {
    pub status: u16,
    pub headers: Vec<(String, String)>,
    pub body: Vec<u8>,
}

impl Response {
    pub fn new(status: u16) -> Response {
        Response { status, headers: Vec::new(), body: Vec::new() }
    }

    pub fn html(status: u16, body: String) -> Response {
        Response::new(status)
            .header("Content-Type", "text/html; charset=utf-8")
            .header("Content-Length", body.len().to_string())
            .body(body.into_bytes())
    }

    pub fn header(mut self, name: &str, value: impl Into<String>) -> Response {
        self.headers.push((name.to_string(), value.into()));
        self
    }

    pub fn body(mut self, body: Vec<u8>) -> Response {
        self.body = body;
        self
    }

    pub fn reason(&self) -> &'static str {
        reason_phrase(self.status)
    }

    /// The exact bytes on the wire. `head_only` is set for a `HEAD` request,
    /// which keeps the `Content-Length` but sends no body - what both
    /// Werkzeug and gunicorn do.
    pub fn to_bytes(&self, date: &str, connection: &str, head_only: bool) -> Vec<u8> {
        let mut out = Vec::with_capacity(self.body.len() + 256);
        out.extend_from_slice(
            format!("HTTP/1.1 {} {}\r\n", self.status, self.reason()).as_bytes(),
        );
        out.extend_from_slice(format!("Date: {}\r\n", date).as_bytes());
        for (name, value) in &self.headers {
            out.extend_from_slice(format!("{}: {}\r\n", name, value).as_bytes());
        }
        out.extend_from_slice(format!("Connection: {}\r\n", connection).as_bytes());
        out.extend_from_slice(b"\r\n");
        if !head_only {
            out.extend_from_slice(&self.body);
        }
        out
    }
}

/// Werkzeug writes the reason phrase in upper case and with its own
/// spellings; these are the ones this frontend can produce.
pub fn reason_phrase(status: u16) -> &'static str {
    match status {
        200 => "OK",
        206 => "PARTIAL CONTENT",
        301 => "MOVED PERMANENTLY",
        302 => "FOUND",
        304 => "NOT MODIFIED",
        400 => "BAD REQUEST",
        404 => "NOT FOUND",
        405 => "METHOD NOT ALLOWED",
        412 => "PRECONDITION FAILED",
        416 => "REQUESTED RANGE NOT SATISFIABLE",
        500 => "INTERNAL SERVER ERROR",
        503 => "SERVICE UNAVAILABLE",
        _ => "UNKNOWN",
    }
}

/// Reads one request. Returns `None` on a clean end of stream.
pub fn read_request(stream: &TcpStream) -> std::io::Result<Option<Request>> {
    let mut reader = BufReader::new(stream);
    let mut line = String::new();
    loop {
        line.clear();
        let n = reader.read_line(&mut line)?;
        if n == 0 {
            return Ok(None);
        }
        if !line.trim().is_empty() {
            break;
        }
    }
    let mut parts = line.trim_end().split(' ');
    let method = parts.next().unwrap_or_default().to_string();
    let target = parts.next().unwrap_or_default().to_string();
    let version = parts.next().unwrap_or("HTTP/1.0").to_string();
    let (raw_path, query) = match target.split_once('?') {
        Some((p, q)) => (p.to_string(), q.to_string()),
        None => (target.clone(), String::new()),
    };

    let mut headers: Vec<(String, String)> = Vec::new();
    loop {
        line.clear();
        let n = reader.read_line(&mut line)?;
        if n == 0 || line.trim().is_empty() {
            break;
        }
        if let Some((name, value)) = line.trim_end().split_once(':') {
            headers.push((name.trim().to_string(), value.trim().to_string()));
        }
    }

    let content_length: usize = headers
        .iter()
        .find(|(k, _)| k.eq_ignore_ascii_case("content-length"))
        .and_then(|(_, v)| v.parse().ok())
        .unwrap_or(0);
    let mut body = vec![0u8; content_length];
    if content_length > 0 {
        reader.read_exact(&mut body)?;
    }

    let decoded = percent_decode(&raw_path);
    let path = merge_slashes(&decoded);
    let host = headers
        .iter()
        .find(|(k, _)| k.eq_ignore_ascii_case("host"))
        .map(|(_, v)| v.clone());

    Ok(Some(Request { method, target, version, path, query, headers, body, host }))
}

/// Werkzeug's `merge_slashes`, on by default: the request target
/// `//chicken-curry` matches the `/chicken-curry` rule.
pub fn merge_slashes(path: &str) -> String {
    let mut out = String::with_capacity(path.len());
    let mut last_was_slash = false;
    for ch in path.chars() {
        if ch == '/' {
            if !last_was_slash {
                out.push(ch);
            }
            last_was_slash = true;
        } else {
            out.push(ch);
            last_was_slash = false;
        }
    }
    out
}

/// `EEE, dd MMM yyyy HH:mm:ss GMT`, the format both java.time and Werkzeug
/// emit for HTTP dates.
pub fn http_date(now: chrono::DateTime<chrono::Utc>) -> String {
    now.format("%a, %d %b %Y %H:%M:%S GMT").to_string()
}

pub fn write_response(
    stream: &mut TcpStream,
    response: &Response,
    request: &Request,
) -> std::io::Result<()> {
    let connection = if request.keeps_alive() { "keep-alive" } else { "close" };
    let now = chrono::Utc::now();
    stream.write_all(&response.to_bytes(&http_date(now), connection, request.is_head()))?;
    stream.flush()
}
