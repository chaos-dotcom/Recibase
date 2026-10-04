//! Shared helpers for the route and unit specs.
//!
//! `conftest.py` mocks `requests` and drives the Flask test client. The
//! Rust specs do better: a stub recipe API runs on a loopback socket, an
//! [`App`] is pointed at it, and requests are handed to [`App::handle`]
//! directly. This module holds that stub, the request builders and the
//! response accessors.

#![allow(dead_code)]

use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use recibase_frontend::app::App;
use recibase_frontend::http::{Request, Response};
use recibase_frontend::statics::StaticFiles;

/// `SAMPLE_RECIPES` from `conftest.py`.
pub const SAMPLE_RECIPES: &str = r#"[{"permalink": "test-recipe", "name": "Test Recipe"}]"#;

/// `SAMPLE_RECIPE` from `conftest.py`, in the same key order.
pub fn sample_recipe() -> serde_json::Value {
    serde_json::json!({
        "name": "Test Recipe",
        "description": "A test recipe",
        "permalink": "test-recipe",
        "tagline": "Tasty",
        "image": null,
        "edit": "https://github.com/example/edit",
        "ingredients_blocks": [
            {
                "name": "Sauce",
                "ingredients": [
                    {"name": "Onion", "quantity": "2", "prep": "chopped", "notes": null},
                    {"name": "Butter", "quantity": "50g", "prep": null, "notes": null},
                ],
            },
            {
                "name": "Finish",
                "ingredients": [
                    {"name": "butter", "quantity": "150g", "prep": null, "notes": null},
                ],
            },
        ],
        "method": ["Chop onion", "Eat"],
        "notes": [],
        "dated_notes": [],
        "tags": [],
        "source": null,
    })
}

/// One canned reply the stub API sends.
#[derive(Clone, Debug)]
pub struct Answer {
    pub status: u16,
    pub content_type: String,
    pub body: String,
    /// Close the connection without answering, which is how a request to an
    /// unreachable API looks to the client.
    pub hangup: bool,
}

impl Answer {
    /// `200` with a JSON body.
    pub fn json(body: impl Into<String>) -> Answer {
        Answer {
            status: 200,
            content_type: "application/json".to_string(),
            body: body.into(),
            hangup: false,
        }
    }

    pub fn json_status(status: u16, body: impl Into<String>) -> Answer {
        Answer {
            status,
            content_type: "application/json".to_string(),
            body: body.into(),
            hangup: false,
        }
    }

    pub fn text(status: u16, content_type: impl Into<String>, body: impl Into<String>) -> Answer {
        Answer {
            status,
            content_type: content_type.into(),
            body: body.into(),
            hangup: false,
        }
    }

    /// Accept the connection and drop it without answering. The client sees
    /// a broken connection, the same thing it sees when nothing is listening
    /// on the port - but the stub's other routes keep working, which the
    /// templates need.
    pub fn hangup() -> Answer {
        Answer {
            status: 0,
            content_type: String::new(),
            body: String::new(),
            hangup: true,
        }
    }

    /// A status with a body that says nothing.
    pub fn empty(status: u16) -> Answer {
        Answer::text(status, "text/plain", "")
    }
}

/// One request the stub API received.
#[derive(Clone, Debug)]
pub struct Recorded {
    pub method: String,
    pub target: String,
    pub headers: Vec<(String, String)>,
    pub body: String,
}

impl Recorded {
    pub fn header(&self, name: &str) -> Option<&str> {
        self.headers
            .iter()
            .find(|(key, _)| key.eq_ignore_ascii_case(name))
            .map(|(_, value)| value.as_str())
    }

    /// The recorded body, parsed as JSON.
    pub fn json(&self) -> serde_json::Value {
        serde_json::from_str(&self.body)
            .unwrap_or_else(|error| panic!("request body is not JSON: {error}: {}", self.body))
    }
}

struct Shared {
    routes: HashMap<(String, String), Answer>,
    requests: Vec<Recorded>,
}

/// A stub of the recipe API on a loopback socket, one thread per connection.
///
/// It answers the routes `conftest.py` mocks, and any of them can be replaced
/// per test with [`StubApi::on`]. Every request it receives is recorded, so a
/// test can assert on what the frontend posted.
pub struct StubApi {
    base_url: String,
    shared: Arc<Mutex<Shared>>,
}

impl StubApi {
    pub fn start() -> StubApi {
        let listener = TcpListener::bind(("127.0.0.1", 0)).expect("bind the stub API");
        let port = listener
            .local_addr()
            .expect("the stub API's address")
            .port();
        let shared = Arc::new(Mutex::new(Shared {
            routes: default_routes(),
            requests: Vec::new(),
        }));
        let listener_state = Arc::clone(&shared);
        std::thread::spawn(move || {
            for stream in listener.incoming() {
                let Ok(stream) = stream else { continue };
                let state = Arc::clone(&listener_state);
                std::thread::spawn(move || serve(stream, state));
            }
        });
        StubApi {
            base_url: format!("http://127.0.0.1:{}/", port),
            shared,
        }
    }

    /// `http://127.0.0.1:<port>/`, the shape `app.py`'s `backendBaseUrl` has.
    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    /// Replaces what the stub answers for one route.
    pub fn on(&self, method: &str, path: &str, answer: Answer) {
        let mut shared = self.shared.lock().expect("the stub API's state");
        shared
            .routes
            .insert((method.to_uppercase(), path.to_string()), answer);
    }

    pub fn requests(&self) -> Vec<Recorded> {
        self.shared
            .lock()
            .expect("the stub API's state")
            .requests
            .clone()
    }

    pub fn requests_to(&self, method: &str, path: &str) -> Vec<Recorded> {
        let method = method.to_uppercase();
        self.requests()
            .into_iter()
            .filter(|request| request.method == method && request.target == path)
            .collect()
    }

    /// The single request for `method path`; panics unless there is exactly
    /// one, which is what the ported Python assertions assume.
    pub fn single_request(&self, method: &str, path: &str) -> Recorded {
        let found = self.requests_to(method, path);
        assert_eq!(
            found.len(),
            1,
            "expected one {method} {path}, got {}",
            found.len()
        );
        found.into_iter().next().expect("just checked the length")
    }
}

/// The routes `_mock_requests_get` and the contribute tests answer.
fn default_routes() -> HashMap<(String, String), Answer> {
    let mut routes = HashMap::new();
    routes.insert(
        ("GET".to_string(), "/recipes/".to_string()),
        Answer::json(SAMPLE_RECIPES),
    );
    routes.insert(
        ("GET".to_string(), "/manifest".to_string()),
        Answer::json(r#"{"version": "deadbeef"}"#),
    );
    routes.insert(
        ("GET".to_string(), "/recipes/test-recipe".to_string()),
        Answer::json(sample_recipe().to_string()),
    );
    // `_mock_requests_get` answers every other recipe with a 404, which the
    // route turns into the 404 page.
    routes.insert(
        ("GET".to_string(), "/recipes/missing-recipe".to_string()),
        Answer::text(404, "text/plain", "not found"),
    );
    routes.insert(
        ("POST".to_string(), "/recipe-submissions".to_string()),
        Answer::json(r#"{"url": "https://github.com/chaos-dotcom/Recibase/pull/12"}"#),
    );
    routes
}

fn serve(stream: TcpStream, shared: Arc<Mutex<Shared>>) {
    let Ok(mut writer) = stream.try_clone() else {
        return;
    };
    let mut reader = BufReader::new(stream);
    loop {
        let mut line = String::new();
        match reader.read_line(&mut line) {
            Ok(0) | Err(_) => return,
            Ok(_) => {}
        }
        if line.trim().is_empty() {
            continue;
        }
        let mut parts = line.split_whitespace();
        let method = parts.next().unwrap_or_default().to_ascii_uppercase();
        let target = parts.next().unwrap_or_default().to_string();

        let mut headers: Vec<(String, String)> = Vec::new();
        loop {
            line.clear();
            match reader.read_line(&mut line) {
                Ok(0) | Err(_) => return,
                Ok(_) => {}
            }
            if line.trim().is_empty() {
                break;
            }
            if let Some((name, value)) = line.trim_end().split_once(':') {
                headers.push((name.trim().to_string(), value.trim().to_string()));
            }
        }
        let length: usize = headers
            .iter()
            .find(|(name, _)| name.eq_ignore_ascii_case("content-length"))
            .and_then(|(_, value)| value.parse().ok())
            .unwrap_or(0);
        let mut body = vec![0u8; length];
        if length > 0 && reader.read_exact(&mut body).is_err() {
            return;
        }
        let wants_close = headers.iter().any(|(name, value)| {
            name.eq_ignore_ascii_case("connection") && value.to_ascii_lowercase().contains("close")
        });
        let path = target
            .split_once('?')
            .map(|(path, _)| path)
            .unwrap_or(&target)
            .to_string();

        let answer = {
            let mut state = shared.lock().expect("the stub API's state");
            state.requests.push(Recorded {
                method: method.clone(),
                target: target.clone(),
                headers,
                body: String::from_utf8_lossy(&body).into_owned(),
            });
            state.routes.get(&(method, path)).cloned()
        }
        .unwrap_or_else(|| Answer::empty(404));

        let wire = format!(
            "HTTP/1.1 {} {}\r\nContent-Type: {}\r\nContent-Length: {}\r\nConnection: {}\r\n\r\n{}",
            answer.status,
            reason(answer.status),
            answer.content_type,
            answer.body.len(),
            if wants_close { "close" } else { "keep-alive" },
            answer.body
        );
        if answer.hangup {
            return;
        }
        if writer.write_all(wire.as_bytes()).is_err() || writer.flush().is_err() {
            return;
        }
        if wants_close {
            return;
        }
    }
}

fn reason(status: u16) -> &'static str {
    match status {
        200 => "OK",
        400 => "BAD REQUEST",
        401 => "UNAUTHORIZED",
        404 => "NOT FOUND",
        500 => "INTERNAL SERVER ERROR",
        502 => "BAD GATEWAY",
        _ => "OK",
    }
}

/// A port on which nothing is listening: bind one, learn it, drop it.
pub fn closed_port() -> u16 {
    let listener = TcpListener::bind(("127.0.0.1", 0)).expect("bind a port to close");
    let port = listener
        .local_addr()
        .expect("the closed port's address")
        .port();
    drop(listener);
    port
}

/// The frontend, pointed at a backend base URL.
pub fn app(backend_url: &str, frontend_version: &str) -> App {
    App::new(backend_url.to_string(), frontend_version.to_string(), 8080)
}

/// The frontend, pointed at a stub API, deployed as `latest`.
pub fn app_on(stub: &StubApi) -> App {
    app(stub.base_url(), "latest").with_statics(source_statics())
}

/// `crates/frontend/static`, which is where the served files live.
pub fn static_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("static")
}

/// The frontend's static files, rooted at the source `static/` directory.
///
/// Supplied through [`App::with_statics`] and [`StaticFiles::at`] rather than
/// `STATIC_DIR`, because mutating the process environment is `unsafe` in
/// edition 2024 and would race the other tests running in parallel.
pub fn source_statics() -> StaticFiles {
    StaticFiles::at(static_dir())
}

// -- request builders -------------------------------------------------

/// A `GET`, with the `Host` header a browser would send.
pub fn get(target: &str) -> Request {
    request("GET", target)
}

pub fn head(target: &str) -> Request {
    request("HEAD", target)
}

/// One request, with `path` and `query` split the way `read_request` splits
/// them.
pub fn request(method: &str, target: &str) -> Request {
    let (raw_path, query) = match target.split_once('?') {
        Some((path, query)) => (path, query.to_string()),
        None => (target, String::new()),
    };
    let path =
        recibase_frontend::http::merge_slashes(&recibase_frontend::http::percent_decode(raw_path));
    Request {
        method: method.to_string(),
        target: target.to_string(),
        version: "HTTP/1.1".to_string(),
        path,
        query,
        headers: vec![("Host".to_string(), "127.0.0.1:8080".to_string())],
        body: Vec::new(),
        host: Some("127.0.0.1:8080".to_string()),
    }
}

pub fn with_header(mut request: Request, name: &str, value: &str) -> Request {
    request.headers.push((name.to_string(), value.to_string()));
    request
}

/// A form POST, `application/x-www-form-urlencoded`, as a browser sends it.
pub fn post_form(target: &str, pairs: &[(&str, &str)]) -> Request {
    let mut request = request("POST", target);
    request.headers.push((
        "Content-Type".to_string(),
        "application/x-www-form-urlencoded".to_string(),
    ));
    request.body = urlencode(pairs);
    request
}

/// `urllib.parse.urlencode` for the pairs the tests post.
pub fn urlencode(pairs: &[(&str, &str)]) -> Vec<u8> {
    let encode = |value: &str| -> String {
        let mut out = String::new();
        for byte in value.bytes() {
            match byte {
                b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                    out.push(byte as char)
                }
                b' ' => out.push('+'),
                byte => out.push_str(&format!("%{:02X}", byte)),
            }
        }
        out
    };
    pairs
        .iter()
        .map(|(key, value)| format!("{}={}", encode(key), encode(value)))
        .collect::<Vec<String>>()
        .join("&")
        .into_bytes()
}

// -- response accessors -----------------------------------------------

/// A response header, looked up case-insensitively.
pub fn header<'a>(response: &'a Response, name: &str) -> Option<&'a str> {
    response
        .headers
        .iter()
        .find(|(key, _)| key.eq_ignore_ascii_case(name))
        .map(|(_, value)| value.as_str())
}

/// A response header, or a panic naming the one that is missing.
pub fn header_or<'a>(response: &'a Response, name: &str) -> &'a str {
    header(response, name).unwrap_or_else(|| panic!("no {} header in {:?}", name, response.headers))
}

/// The body as text, the way a test would read the rendered page.
pub fn body_of(response: &Response) -> String {
    String::from_utf8_lossy(&response.body).into_owned()
}
