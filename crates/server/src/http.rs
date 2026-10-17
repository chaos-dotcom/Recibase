//! A minimal, deliberately literal HTTP/1.1 server.
//!
//! The Scala implementation serves through http4s' Ember backend. The response
//! bytes on the wire are part of the API, so this module writes them by hand:
//!
//! ```text
//! HTTP/1.1 <status>\r\nDate: <rfc1123>\r\n[Connection: close\r\n]Content-Type: <ct>\r\nContent-Length: <n>\r\n\r\n<body>
//! ```

use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpStream;

#[derive(Debug)]
pub struct Request {
    pub method: String,
    pub target: String,
    /// The HTTP version from the request line, e.g. `HTTP/1.1`.
    pub version: String,
    pub path: String,
    pub query: String,
    pub headers: Vec<(String, String)>,
    pub body: Vec<u8>,
}

impl Request {
    pub fn header(&self, name: &str) -> Option<&str> {
        self.headers
            .iter()
            .find(|(k, _)| k.eq_ignore_ascii_case(name))
            .map(|(_, v)| v.as_str())
    }

    /// `Authorization: Bearer <x>` -> `<x>` (case-insensitive prefix, as the Scala).
    pub fn bearer_passcode(&self) -> String {
        match self.header("Authorization") {
            Some(value) if value.len() >= 7 && value[..7].eq_ignore_ascii_case("Bearer ") => {
                value[7..].to_string()
            }
            _ => String::new(),
        }
    }

    /// http4s/Ember's connection handling: a client that asks for
    /// `keep-alive` gets it, an HTTP/1.1 request without a `Connection` header
    /// keeps the connection, and anything else (an explicit `close`, or
    /// HTTP/1.0 without `keep-alive`) closes it.
    pub fn keeps_alive(&self) -> bool {
        let connection = self
            .header("Connection")
            .unwrap_or_default()
            .to_ascii_lowercase();
        if connection.split(',').any(|token| token.trim() == "close") {
            return false;
        }
        if connection
            .split(',')
            .any(|token| token.trim() == "keep-alive")
        {
            return true;
        }
        self.version.eq_ignore_ascii_case("HTTP/1.1")
    }

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

    /// The value of a cookie from the `Cookie` header, by name.
    pub fn cookie(&self, name: &str) -> Option<String> {
        for (key, value) in &self.headers {
            if !key.eq_ignore_ascii_case("cookie") {
                continue;
            }
            for pair in value.split(';') {
                if let Some((cookie_name, cookie_value)) = pair.split_once('=')
                    && cookie_name.trim() == name
                {
                    return Some(cookie_value.trim().to_string());
                }
            }
        }
        None
    }
}

/// An `application/x-www-form-urlencoded` body as ordered name/value pairs,
/// with `+` and `%XX` decoded. Duplicate names are kept.
pub fn parse_form(body: &[u8]) -> Vec<(String, String)> {
    let text = String::from_utf8_lossy(body);
    let mut out = Vec::new();
    for pair in text.split('&') {
        if pair.is_empty() {
            continue;
        }
        let (key, value) = pair.split_once('=').unwrap_or((pair, ""));
        out.push((percent_decode(key), percent_decode(value)));
    }
    out
}

fn percent_decode(input: &str) -> String {
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
            b'+' => {
                out.push(b' ');
                i += 1;
            }
            byte => {
                out.push(byte);
                i += 1;
            }
        }
    }
    String::from_utf8_lossy(&out).into_owned()
}

#[derive(Debug, Clone)]
pub struct Response {
    pub status: u16,
    /// `None` for a CORS preflight, which http4s answers without a body type.
    pub content_type: Option<&'static str>,
    pub body: Vec<u8>,
    /// Headers written before `Content-Length` (the CORS preflight ones).
    pub headers_before_length: Vec<(&'static str, String)>,
    /// Headers written after `Content-Length` (`Access-Control-Allow-Origin`).
    pub headers_after_length: Vec<(&'static str, &'static str)>,
}

impl Response {
    pub fn json(value: &serde_json::Value) -> Response {
        Response::status(
            200,
            "application/json",
            serde_json::to_vec(value).expect("JSON serialisation cannot fail"),
        )
    }

    pub fn plain(body: &str) -> Response {
        Response::status(200, "text/plain; charset=UTF-8", body.as_bytes().to_vec())
    }

    pub fn html(body: String) -> Response {
        Response::status(200, "text/html; charset=UTF-8", body.into_bytes())
    }

    /// A redirect with an empty body and only a `Location` header. `status` is
    /// normally 303 (`See Other`) after a POST, or 302 for a temporary GET
    /// bounce to the identity provider.
    pub fn redirect(status: u16, location: &str) -> Response {
        Response {
            status,
            content_type: None,
            body: Vec::new(),
            headers_before_length: vec![("Location", location.to_string())],
            headers_after_length: Vec::new(),
        }
    }

    /// Appends a `Set-Cookie` response header.
    pub fn set_cookie(&mut self, value: String) {
        self.headers_before_length.push(("Set-Cookie", value));
    }

    pub fn status(status: u16, content_type: &'static str, body: Vec<u8>) -> Response {
        Response {
            status,
            content_type: Some(content_type),
            body,
            headers_before_length: Vec::new(),
            headers_after_length: Vec::new(),
        }
    }

    /// http4s' CORS preflight answer: an empty 200 with the allow headers and
    /// no content type.
    pub fn preflight(requested_headers: &str) -> Response {
        Response {
            status: 200,
            content_type: None,
            body: Vec::new(),
            headers_before_length: vec![
                ("Access-Control-Allow-Origin", "*".to_string()),
                (
                    "Access-Control-Allow-Methods",
                    "PATCH, HEAD, QUERY, PUT, GET, POST, DELETE".to_string(),
                ),
                (
                    "Access-Control-Allow-Headers",
                    requested_headers.to_string(),
                ),
                (
                    "Vary",
                    "Access-Control-Request-Method, Access-Control-Request-Headers".to_string(),
                ),
            ],
            headers_after_length: Vec::new(),
        }
    }

    /// http4s' default "route not found" response.
    pub fn not_found() -> Response {
        Response::status(404, "text/plain; charset=UTF-8", b"Not found".to_vec())
    }

    fn reason(status: u16) -> &'static str {
        match status {
            200 => "OK",
            302 => "Found",
            303 => "See Other",
            307 => "Temporary Redirect",
            308 => "Permanent Redirect",
            400 => "Bad Request",
            401 => "Unauthorized",
            403 => "Forbidden",
            404 => "Not Found",
            405 => "Method Not Allowed",
            409 => "Conflict",
            413 => "Payload Too Large",
            422 => "Unprocessable Entity",
            429 => "Too Many Requests",
            500 => "Internal Server Error",
            502 => "Bad Gateway",
            503 => "Service Unavailable",
            _ => "OK",
        }
    }

    /// The exact bytes on the wire. `connection` is the value of the
    /// `Connection` header ("close" or "keep-alive").
    pub fn to_bytes(&self, connection: &str, date: &str) -> Vec<u8> {
        let mut out = Vec::with_capacity(self.body.len() + 128);
        out.extend_from_slice(
            format!(
                "HTTP/1.1 {} {}\r\n",
                self.status,
                Response::reason(self.status)
            )
            .as_bytes(),
        );
        out.extend_from_slice(format!("Date: {}\r\n", date).as_bytes());
        out.extend_from_slice(format!("Connection: {}\r\n", connection).as_bytes());
        if let Some(content_type) = self.content_type {
            out.extend_from_slice(format!("Content-Type: {}\r\n", content_type).as_bytes());
        }
        for (name, value) in &self.headers_before_length {
            out.extend_from_slice(format!("{}: {}\r\n", name, value).as_bytes());
        }
        out.extend_from_slice(format!("Content-Length: {}\r\n", self.body.len()).as_bytes());
        for (name, value) in &self.headers_after_length {
            out.extend_from_slice(format!("{}: {}\r\n", name, value).as_bytes());
        }
        out.extend_from_slice(b"\r\n");
        out.extend_from_slice(&self.body);
        out
    }

    pub fn write_to(
        &self,
        stream: &mut TcpStream,
        connection: &str,
        date: &str,
    ) -> std::io::Result<()> {
        stream.write_all(&self.to_bytes(connection, date))?;
        stream.flush()
    }
}

/// Reads one request. Returns None on a clean end of stream.
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
    let (path, query) = match target.split_once('?') {
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

    Ok(Some(Request {
        method,
        target,
        version,
        path,
        query,
        headers,
        body,
    }))
}

/// `EEE, dd MMM yyyy HH:mm:ss GMT`, the format java.time emits for HTTP dates.
pub fn http_date(now: chrono::DateTime<chrono::Utc>) -> String {
    now.format("%a, %d %b %Y %H:%M:%S GMT").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn request(headers: &[(&str, &str)], body: &str) -> Request {
        Request {
            method: "POST".to_string(),
            target: "/admin/entries".to_string(),
            version: "HTTP/1.1".to_string(),
            path: "/admin/entries".to_string(),
            query: String::new(),
            headers: headers
                .iter()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect(),
            body: body.as_bytes().to_vec(),
        }
    }

    #[test]
    fn a_cookie_is_found_by_name() {
        let request = request(&[("Cookie", "a=1; recibase_session=abc123; b=2")], "");
        assert_eq!(request.cookie("recibase_session").as_deref(), Some("abc123"));
        assert_eq!(request.cookie("missing"), None);
    }

    #[test]
    fn forms_decode_plus_and_percent() {
        let pairs = parse_form(b"meal_name=Sweet+Potato&note=a%26b&featured=on");
        assert_eq!(pairs[0], ("meal_name".to_string(), "Sweet Potato".to_string()));
        assert_eq!(pairs[1], ("note".to_string(), "a&b".to_string()));
        assert_eq!(pairs[2], ("featured".to_string(), "on".to_string()));
    }

    #[test]
    fn a_redirect_writes_its_location_and_reason() {
        let mut response = Response::redirect(303, "/admin");
        response.set_cookie("recibase_session=x; HttpOnly".to_string());
        let bytes = String::from_utf8(response.to_bytes("close", "now")).unwrap();
        assert!(bytes.starts_with("HTTP/1.1 303 See Other\r\n"), "{bytes}");
        assert!(bytes.contains("\r\nLocation: /admin\r\n"), "{bytes}");
        assert!(bytes.contains("\r\nSet-Cookie: recibase_session=x; HttpOnly\r\n"), "{bytes}");
    }
}
