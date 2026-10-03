//! Port of `se.reciba.api.GithubClientSpec`, plus the two source-shape guards
//! that the Scala client checks before it contacts GitHub.
//!
//! The fake GitHub is a `std::net::TcpListener` on port 0, like the Scala
//! spec's `com.sun.net.httpserver.HttpServer`: it records every call and
//! answers the first rule that matches the method and the raw path.

use recibase_submit::config::GithubSettings;
use recibase_submit::github_client::GithubClient;
use recibase_submit::pull_requests::PullRequestFailure;
use recibase_submit::recipe_source::GeneratedRecipe;
use serde_json::Value;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::Duration;

#[derive(Debug, Clone)]
struct GithubCall {
    method: String,
    path: String,
    authorization: Option<String>,
    body: String,
}

impl GithubCall {
    fn json(&self) -> Value {
        serde_json::from_str(&self.body).expect("json body")
    }
}

type Matcher = Box<dyn Fn(&str) -> bool + Send + 'static>;

struct Rule {
    method: String,
    matcher: Matcher,
    status: u16,
    body: String,
}

struct FakeGithub {
    port: u16,
    calls: Arc<Mutex<Vec<GithubCall>>>,
    rules: Arc<Mutex<Vec<Rule>>>,
    stop: Arc<AtomicBool>,
    handle: Option<JoinHandle<()>>,
}

impl FakeGithub {
    fn new() -> FakeGithub {
        let listener = TcpListener::bind("127.0.0.1:0").expect("bind");
        let port = listener.local_addr().expect("addr").port();
        listener.set_nonblocking(true).expect("nonblocking");
        let calls = Arc::new(Mutex::new(Vec::new()));
        let rules = Arc::new(Mutex::new(Vec::new()));
        let stop = Arc::new(AtomicBool::new(false));
        let thread_calls = Arc::clone(&calls);
        let thread_rules = Arc::clone(&rules);
        let thread_stop = Arc::clone(&stop);
        let handle = thread::spawn(move || {
            // Bounded: the listener is never left running, and a transient
            // accept error never stops the server.
            let deadline = std::time::Instant::now() + Duration::from_secs(30);
            while !thread_stop.load(Ordering::SeqCst) && std::time::Instant::now() < deadline {
                match listener.accept() {
                    Ok((stream, _)) => serve(stream, &thread_calls, &thread_rules),
                    Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
                        thread::sleep(Duration::from_millis(2));
                    }
                    Err(error) if error.kind() == std::io::ErrorKind::Interrupted => {}
                    Err(error) => {
                        assert!(error.kind() != std::io::ErrorKind::WouldBlock);
                        thread::sleep(Duration::from_millis(2));
                    }
                }
            }
        });
        FakeGithub { port, calls, rules, stop, handle: Some(handle) }
    }

    fn base(&self) -> String {
        format!("http://127.0.0.1:{}", self.port)
    }

    fn respond(
        &self,
        method: &str,
        matcher: impl Fn(&str) -> bool + Send + 'static,
        status: u16,
        body: &str,
    ) {
        self.rules.lock().expect("rules").push(Rule {
            method: method.to_string(),
            matcher: Box::new(matcher),
            status,
            body: body.to_string(),
        });
    }

    fn recorded(&self) -> Vec<GithubCall> {
        self.calls.lock().expect("calls").clone()
    }

    fn close(&mut self) {
        self.stop.store(true, Ordering::SeqCst);
        if let Some(handle) = self.handle.take() {
            let _ = handle.join();
        }
    }
}

fn serve(mut stream: TcpStream, calls: &Arc<Mutex<Vec<GithubCall>>>, rules: &Arc<Mutex<Vec<Rule>>>) {
    stream.set_read_timeout(Some(Duration::from_secs(5))).ok();
    let mut reader = BufReader::new(match stream.try_clone() {
        Ok(clone) => clone,
        Err(_) => {
            let _ = respond(&mut stream, 400, "{}");
            return;
        }
    });

    let mut request_line = String::new();
    loop {
        match reader.read_line(&mut request_line) {
            Ok(0) => return,
            Ok(_) => break,
            Err(ref error) if error.kind() == std::io::ErrorKind::WouldBlock => {
                thread::sleep(Duration::from_millis(2));
                continue;
            }
            Err(_) => {
                let _ = respond(&mut stream, 400, "{}");
                return;
            }
        }
    }
    let mut parts = request_line.split_whitespace();
    let method = parts.next().unwrap_or_default().to_string();
    let path = parts.next().unwrap_or_default().to_string();

    let mut content_length = 0usize;
    let mut authorization = None;
    loop {
        let mut line = String::new();
        match reader.read_line(&mut line) {
            Ok(0) => break,
            Ok(_) => {}
            // The listener is non-blocking, so the accepted socket may be too:
            // wait for the bytes instead of treating WouldBlock as EOF.
            Err(ref error) if error.kind() == std::io::ErrorKind::WouldBlock => {
                thread::sleep(Duration::from_millis(2));
                continue;
            }
            Err(_) => break,
        }
        let header = line.trim_end_matches(['\r', '\n']);
        if header.is_empty() {
            break;
        }
        if let Some((name, value)) = header.split_once(':') {
            let name = name.trim().to_ascii_lowercase();
            let value = value.trim().to_string();
            if name == "content-length" {
                content_length = value.parse().unwrap_or(0);
            }
            if name == "authorization" {
                authorization = Some(value);
            }
        }
    }

    let mut body = vec![0u8; content_length];
    if content_length > 0 && reader.read_exact(&mut body).is_err() {
        let _ = respond(&mut stream, 400, "{}");
        return;
    }
    calls.lock().expect("calls").push(GithubCall {
        method: method.clone(),
        path: path.clone(),
        authorization,
        body: String::from_utf8_lossy(&body).into_owned(),
    });

    let (status, response_body) = {
        let rules = rules.lock().expect("rules");
        match rules
            .iter()
            .find(|rule| rule.method == method && (rule.matcher)(&path))
        {
            Some(rule) => (rule.status, rule.body.clone()),
            None => (404, "{\"message\":\"unexpected\"}".to_string()),
        }
    };

    let _ = respond(&mut stream, status, &response_body);
}

/// Writes one response and closes the connection, the way the Scala fake's
/// `HttpServer` handler does.
fn respond(stream: &mut TcpStream, status: u16, body: &str) -> std::io::Result<()> {
    let bytes = body.as_bytes();
    let reason = match status {
        200 => "OK",
        201 => "Created",
        204 => "No Content",
        400 => "Bad Request",
        404 => "Not Found",
        422 => "Unprocessable Entity",
        500 => "Internal Server Error",
        _ => "Unknown",
    };
    let head = if status == 204 {
        format!(
            "HTTP/1.1 {} {}\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
            status, reason
        )
    } else {
        format!(
            "HTTP/1.1 {} {}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
            status,
            reason,
            bytes.len()
        )
    };

    stream.write_all(head.as_bytes())?;
    if status != 204 {
        stream.write_all(bytes)?;
    }
    stream.flush()?;
    let _ = stream.shutdown(Shutdown::Both);
    Ok(())
}

fn settings() -> GithubSettings {
    GithubSettings {
        token: "test-token".to_string(),
        repository: "The-Silverwood-Institute/Recibase".to_string(),
        base_branch: "master".to_string(),
    }
}

const SHA: &str = r#"{"object":{"sha":"abc123"}}"#;

fn recipe() -> GeneratedRecipe {
    GeneratedRecipe {
        name: "Phone Test Soup".to_string(),
        object_name: "PhoneTestSoup".to_string(),
        permalink: "phone-test-soup".to_string(),
        branch: "recipe/phone-test-soup".to_string(),
        path: "src/main/scala/se/reciba/api/recibase/recipes/PhoneTestSoup.scala".to_string(),
        source: "package se.reciba.api.recipes\n".to_string(),
    }
}

#[test]
fn creates_a_branch_commits_the_file_and_opens_a_pull_request() {
    let mut github = FakeGithub::new();
    github.respond("GET", |path| path.contains("/git/ref/heads/master"), 200, SHA);
    github.respond("POST", |path| path.ends_with("/git/refs"), 201, "{}");
    github.respond("PUT", |path| path.contains("/contents/"), 201, "{}");
    github.respond(
        "POST",
        |path| path.ends_with("/pulls"),
        201,
        r#"{"html_url":"https://github.com/The-Silverwood-Institute/Recibase/pull/4"}"#,
    );

    let client = GithubClient::new(&settings(), Some(&github.base()));
    let result = client.open(
        &recipe(),
        "Add Phone Test Soup",
        "Add Phone Test Soup",
        "Submitted from the contribute page.",
    );
    assert_eq!(
        result,
        Ok("https://github.com/The-Silverwood-Institute/Recibase/pull/4".to_string())
    );

    let recorded = github.recorded();
    let calls: Vec<String> = recorded
        .iter()
        .map(|call| format!("{} {}", call.method, call.path))
        .collect();
    assert_eq!(
        calls,
        vec![
            "GET /repos/The-Silverwood-Institute/Recibase/git/ref/heads/master".to_string(),
            "POST /repos/The-Silverwood-Institute/Recibase/git/refs".to_string(),
            "PUT /repos/The-Silverwood-Institute/Recibase/contents/src/main/scala/se/reciba/api/recibase/recipes/PhoneTestSoup.scala".to_string(),
            "POST /repos/The-Silverwood-Institute/Recibase/pulls".to_string(),
        ]
    );
    for call in &recorded {
        assert_eq!(call.authorization.as_deref(), Some("Bearer test-token"));
    }

    let file = recorded[2].json();
    assert_eq!(file.get("message").and_then(Value::as_str), Some("Add Phone Test Soup"));
    assert_eq!(file.get("branch").and_then(Value::as_str), Some("recipe/phone-test-soup"));
    let encoded = file.get("content").and_then(Value::as_str).expect("content");
    assert_eq!(
        String::from_utf8(base64_decode(encoded)).expect("utf8"),
        recipe().source
    );

    let pull = recorded[3].json();
    assert_eq!(pull.get("head").and_then(Value::as_str), Some("recipe/phone-test-soup"));
    assert_eq!(pull.get("base").and_then(Value::as_str), Some("master"));
    assert_eq!(pull.get("draft"), Some(&Value::Bool(true)));
    assert_eq!(
        pull.get("body").and_then(Value::as_str),
        Some("Submitted from the contribute page.")
    );

    github.close();
}

#[test]
fn returns_a_conflict_when_the_branch_already_exists() {
    let mut github = FakeGithub::new();
    github.respond("GET", |path| path.contains("/git/ref/heads/master"), 200, SHA);
    github.respond(
        "POST",
        |path| path.ends_with("/git/refs"),
        422,
        r#"{"message":"Reference already exists"}"#,
    );

    let client = GithubClient::new(&settings(), Some(&github.base()));
    let result = client.open(&recipe(), "Add", "Add", "body");
    assert_eq!(result, Err(PullRequestFailure::BranchAlreadyExists));

    let methods: Vec<String> = github.recorded().iter().map(|call| call.method.clone()).collect();
    assert_eq!(methods, vec!["GET".to_string(), "POST".to_string()]);

    github.close();
}

#[test]
fn deletes_the_branch_when_the_file_upload_fails() {
    let mut github = FakeGithub::new();
    github.respond("GET", |path| path.contains("/git/ref/heads/master"), 200, SHA);
    github.respond("POST", |path| path.ends_with("/git/refs"), 201, "{}");
    github.respond("PUT", |path| path.contains("/contents/"), 500, r#"{"message":"nope"}"#);
    github.respond("DELETE", |path| path.contains("/git/refs/heads/"), 204, "");

    let client = GithubClient::new(&settings(), Some(&github.base()));
    let result = client.open(&recipe(), "Add", "Add", "body");
    match result {
        Err(PullRequestFailure::GithubRejected(message)) => {
            assert!(message.contains("500"), "message was {}", message);
        }
        other => panic!("expected GithubRejected, got {:?}", other),
    }

    let recorded = github.recorded();
    let methods: Vec<String> = recorded.iter().map(|call| call.method.clone()).collect();
    assert_eq!(
        methods,
        vec!["GET".to_string(), "POST".to_string(), "PUT".to_string(), "DELETE".to_string()]
    );
    assert!(recorded
        .last()
        .expect("a delete call")
        .path
        .contains("heads/recipe%2Fphone-test-soup"));

    github.close();
}

#[test]
fn deletes_the_branch_when_the_pull_request_creation_fails() {
    let mut github = FakeGithub::new();
    github.respond("GET", |path| path.contains("/git/ref/heads/master"), 200, SHA);
    github.respond("POST", |path| path.ends_with("/git/refs"), 201, "{}");
    github.respond("PUT", |path| path.contains("/contents/"), 201, "{}");
    github.respond("POST", |path| path.ends_with("/pulls"), 422, r#"{"message":"Validation Failed"}"#);
    github.respond("DELETE", |path| path.contains("/git/refs/heads/"), 204, "");

    let client = GithubClient::new(&settings(), Some(&github.base()));
    let result = client.open(&recipe(), "Add", "Add", "body");
    match result {
        Err(PullRequestFailure::GithubRejected(message)) => {
            assert_eq!(message, "GitHub returned HTTP 422: Validation Failed");
        }
        other => panic!("expected GithubRejected, got {:?}", other),
    }

    let methods: Vec<String> = github.recorded().iter().map(|call| call.method.clone()).collect();
    assert_eq!(methods, vec!["GET", "POST", "PUT", "POST", "DELETE"]);

    github.close();
}

#[test]
fn rejects_a_branch_or_path_that_does_not_match_the_patterns() {
    let client = GithubClient::new(&settings(), Some("http://127.0.0.1:1"));

    let bad_branch = GeneratedRecipe { branch: "recipe/Phone-Test".to_string(), ..recipe() };
    assert_eq!(
        client.open(&bad_branch, "Add", "Add", "body"),
        Err(PullRequestFailure::GithubRejected("invalid branch".to_string()))
    );

    let bad_path = GeneratedRecipe { path: "src/main/scala/recipes/PhoneTestSoup.scala".to_string(), ..recipe() };
    assert_eq!(
        client.open(&bad_path, "Add", "Add", "body"),
        Err(PullRequestFailure::GithubRejected("invalid path".to_string()))
    );
}

fn base64_decode(input: &str) -> Vec<u8> {
    const ALPHABET: &[u8; 64] =
        b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut lookup = [255u8; 256];
    for (index, byte) in ALPHABET.iter().enumerate() {
        lookup[*byte as usize] = index as u8;
    }
    let mut out = Vec::new();
    let mut buffer: u32 = 0;
    let mut bits: u32 = 0;
    for byte in input.bytes() {
        if byte == b'=' {
            break;
        }
        let value = lookup[byte as usize];
        assert!(value != 255, "invalid base64 input");
        buffer = (buffer << 6) | value as u32;
        bits += 6;
        if bits >= 8 {
            bits -= 8;
            out.push(((buffer >> bits) & 0xff) as u8);
        }
    }
    out
}
