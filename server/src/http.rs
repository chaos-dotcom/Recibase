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
            400 => "Bad Request",
            401 => "Unauthorized",
            403 => "Forbidden",
            404 => "Not Found",
            409 => "Conflict",
            413 => "Payload Too Large",
            422 => "Unprocessable Entity",
            500 => "Internal Server Error",
            502 => "Bad Gateway",
            503 => "Service Unavailable",
            _ => "OK",
        }
    }

    /// The exact bytes on the wire.
    pub fn to_bytes(&self, close: bool, date: &str) -> Vec<u8> {
        let mut out = Vec::with_capacity(self.body.len() + 128);
        out.extend_from_slice(
            format!("HTTP/1.1 {} {}\r\n", self.status, Response::reason(self.status)).as_bytes(),
        );
        out.extend_from_slice(format!("Date: {}\r\n", date).as_bytes());
        if close {
            out.extend_from_slice(b"Connection: close\r\n");
        }
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

    pub fn write_to(&self, stream: &mut TcpStream, close: bool, date: &str) -> std::io::Result<()> {
        stream.write_all(&self.to_bytes(close, date))?;
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

    Ok(Some(Request { method, target, path, query, headers, body }))
}

/// `EEE, dd MMM yyyy HH:mm:ss GMT`, the format java.time emits for HTTP dates.
pub fn http_date(now: chrono::DateTime<chrono::Utc>) -> String {
    now.format("%a, %d %b %Y %H:%M:%S GMT").to_string()
}
