//! `se.reciba.api.submit.GithubClient` - branch, file and pull request calls on
//! the GitHub REST API.

use crate::config::GithubSettings;
use crate::java::{replace_literal, take_utf16, url_encode_form};
use crate::log;
use crate::net;
use crate::pull_requests::{PullRequestFailure, RecipePullRequests};
use crate::recipe_source::GeneratedRecipe;
use recibase_core::json;
use serde_json::Value;
use std::time::Duration;

pub const DEFAULT_API_BASE: &str = "https://api.github.com";

/// `class GithubClient(settings, apiBase = "https://api.github.com")`.
#[derive(Debug, Clone)]
pub struct GithubClient {
    settings: GithubSettings,
    api_base: String,
}

impl GithubClient {
    pub fn new(settings: &GithubSettings, api_base: Option<&str>) -> Self {
        GithubClient {
            settings: settings.clone(),
            api_base: api_base.unwrap_or(DEFAULT_API_BASE).to_string(),
        }
    }

    pub fn settings(&self) -> &GithubSettings {
        &self.settings
    }

    pub fn api_base(&self) -> &str {
        &self.api_base
    }

    /// `open`: check the branch and path shapes, then read the base sha, create
    /// the branch, commit the file and open the draft pull request. A failure
    /// after the branch exists deletes the branch first.
    pub fn open(
        &self,
        recipe: &GeneratedRecipe,
        commit_message: &str,
        pull_title: &str,
        pull_body: &str,
    ) -> Result<String, PullRequestFailure> {
        if !branch_pattern_matches(&recipe.branch) {
            return Err(PullRequestFailure::GithubRejected(
                "invalid branch".to_string(),
            ));
        }
        if !path_pattern_matches(&recipe.path) {
            return Err(PullRequestFailure::GithubRejected(
                "invalid path".to_string(),
            ));
        }

        let http = net::agent(Duration::from_secs(15), Duration::from_secs(30));
        let sha = self.base_sha(&http)?;
        self.create_branch(&http, &recipe.branch, &sha)?;
        if let Err(failure) = self.put_file(&http, recipe, commit_message) {
            self.delete_branch(&http, &recipe.branch)?;
            return Err(failure);
        }
        match self.create_pull(&http, &recipe.branch, pull_title, pull_body) {
            Err(failure) => {
                self.delete_branch(&http, &recipe.branch)?;
                Err(failure)
            }
            Ok(url) => {
                log::info(&format!("opened recipe pull request {}", url));
                Ok(url)
            }
        }
    }

    fn base_sha(&self, http: &ureq::Agent) -> Result<String, PullRequestFailure> {
        let path = format!(
            "/repos/{}/git/ref/heads/{}",
            self.settings.repository, self.settings.base_branch
        );
        let (status, body) = self.get(http, &path)?;
        if status != 200 {
            return Err(self.rejected(status, &body));
        }
        match json_sha(&body) {
            Some(sha) => Ok(sha),
            None => Err(PullRequestFailure::GithubRejected(format!(
                "GitHub returned HTTP {} without a sha",
                status
            ))),
        }
    }

    fn create_branch(
        &self,
        http: &ureq::Agent,
        branch: &str,
        sha: &str,
    ) -> Result<(), PullRequestFailure> {
        let payload = json::obj(vec![
            ("ref", Value::String(format!("refs/heads/{}", branch))),
            ("sha", Value::String(sha.to_string())),
        ]);
        let path = format!("/repos/{}/git/refs", self.settings.repository);
        let (status, body) = self.post(http, &path, &payload)?;
        if status == 200 || status == 201 {
            Ok(())
        } else if status == 422
            && github_message(&body)
                .to_lowercase()
                .contains("already exists")
        {
            log::info(&format!("recipe branch already exists: {}", branch));
            Err(PullRequestFailure::BranchAlreadyExists)
        } else {
            Err(self.rejected(status, &body))
        }
    }

    fn put_file(
        &self,
        http: &ureq::Agent,
        recipe: &GeneratedRecipe,
        commit_message: &str,
    ) -> Result<(), PullRequestFailure> {
        let encoded = base64_encode(recipe.source.as_bytes());
        let payload = json::obj(vec![
            ("message", Value::String(commit_message.to_string())),
            ("content", Value::String(encoded)),
            ("branch", Value::String(recipe.branch.clone())),
        ]);
        let path = format!(
            "/repos/{}/contents/{}",
            self.settings.repository, recipe.path
        );
        let (status, body) = self.put(http, &path, &payload)?;
        if status == 200 || status == 201 {
            Ok(())
        } else {
            Err(self.rejected(status, &body))
        }
    }

    fn create_pull(
        &self,
        http: &ureq::Agent,
        branch: &str,
        title: &str,
        pull_body: &str,
    ) -> Result<String, PullRequestFailure> {
        let payload = json::obj(vec![
            ("title", Value::String(title.to_string())),
            ("head", Value::String(branch.to_string())),
            ("base", Value::String(self.settings.base_branch.clone())),
            ("body", Value::String(pull_body.to_string())),
            ("draft", Value::Bool(true)),
        ]);
        let path = format!("/repos/{}/pulls", self.settings.repository);
        let (status, body) = self.post(http, &path, &payload)?;
        if status != 200 && status != 201 {
            return Err(self.rejected(status, &body));
        }
        match json_pull_url(&body) {
            Some(url) => Ok(url),
            None => Err(PullRequestFailure::GithubRejected(format!(
                "GitHub returned HTTP {} without a pull request url",
                status
            ))),
        }
    }

    fn delete_branch(&self, http: &ureq::Agent, branch: &str) -> Result<(), PullRequestFailure> {
        let encoded = url_encode_form(branch);
        let path = format!(
            "/repos/{}/git/refs/heads/{}",
            self.settings.repository, encoded
        );
        let (status, body) = self.delete(http, &path)?;
        if status != 204 && status != 200 {
            log::warn(&format!(
                "failed to delete branch {}: {} {}",
                branch,
                status,
                self.sanitize(&github_message(&body))
            ));
        }
        Ok(())
    }

    fn rejected(&self, status: u16, body: &str) -> PullRequestFailure {
        let detail = self.sanitize(&github_message(body));
        let message = if detail.is_empty() {
            format!("GitHub returned HTTP {}", status)
        } else {
            format!("GitHub returned HTTP {}: {}", status, detail)
        };
        log::warn(&format!("github recipe submission failed: {}", message));
        PullRequestFailure::GithubRejected(message)
    }

    fn get(&self, http: &ureq::Agent, path: &str) -> Result<(u16, String), PullRequestFailure> {
        self.send(http, "GET", path, None)
    }

    fn delete(&self, http: &ureq::Agent, path: &str) -> Result<(u16, String), PullRequestFailure> {
        self.send(http, "DELETE", path, None)
    }

    fn post(
        &self,
        http: &ureq::Agent,
        path: &str,
        payload: &Value,
    ) -> Result<(u16, String), PullRequestFailure> {
        self.send(http, "POST", path, Some(payload))
    }

    fn put(
        &self,
        http: &ureq::Agent,
        path: &str,
        payload: &Value,
    ) -> Result<(u16, String), PullRequestFailure> {
        self.send(http, "PUT", path, Some(payload))
    }

    fn send(
        &self,
        http: &ureq::Agent,
        method: &str,
        path: &str,
        payload: Option<&Value>,
    ) -> Result<(u16, String), PullRequestFailure> {
        let url = format!("{}{}", self.api_base, path);
        let auth = format!("Bearer {}", self.settings.token);
        macro_rules! headers {
            ($builder:expr) => {
                $builder
                    .header("Authorization", auth.as_str())
                    .header("Accept", "application/vnd.github+json")
                    .header("User-Agent", "recibase")
                    .header("X-GitHub-Api-Version", "2022-11-28")
            };
        }
        let response = match (method, payload) {
            ("GET", None) => headers!(http.get(url.as_str())).call(),
            ("DELETE", None) => headers!(http.delete(url.as_str())).call(),
            ("POST", Some(body)) => headers!(http.post(url.as_str()))
                .header("Content-Type", "application/json")
                .send(json::to_string(body).as_str()),
            ("PUT", Some(body)) => headers!(http.put(url.as_str()))
                .header("Content-Type", "application/json")
                .send(json::to_string(body).as_str()),
            _ => unreachable!("GithubClient sends GET/DELETE without a body and POST/PUT with one"),
        };
        match response {
            Ok(response) => {
                let status = response.status().as_u16();
                Ok((status, net::read_body(response)))
            }
            Err(error) => {
                let message = self.sanitize(&error_message(&error));
                log::warn(&format!("github recipe submission failed: {}", message));
                Err(PullRequestFailure::GithubRejected(message))
            }
        }
    }

    /// `message.replace(settings.token, "***")`.
    fn sanitize(&self, message: &str) -> String {
        replace_literal(message, &self.settings.token, "***")
    }
}

impl RecipePullRequests for GithubClient {
    fn open(
        &self,
        recipe: &GeneratedRecipe,
        commit_message: &str,
        pull_title: &str,
        pull_body: &str,
    ) -> Result<String, PullRequestFailure> {
        GithubClient::open(self, recipe, commit_message, pull_title, pull_body)
    }
}

/// `Option(error.getMessage).getOrElse(error.getClass.getSimpleName)`.
fn error_message(error: &ureq::Error) -> String {
    let message = error.to_string();
    if message.is_empty() {
        net::error_name(error).to_string()
    } else {
        message
    }
}

/// `json.hcursor.downField("object").get[String]("sha")`.
fn json_sha(body: &str) -> Option<String> {
    let json: Value = serde_json::from_str(body).ok()?;
    json.get("object")?
        .get("sha")?
        .as_str()
        .map(|sha| sha.to_string())
}

/// `json.hcursor.get[String]("html_url")`.
fn json_pull_url(body: &str) -> Option<String> {
    let json: Value = serde_json::from_str(body).ok()?;
    json.get("html_url")
        .and_then(Value::as_str)
        .map(|url| url.to_string())
}

/// `jawn.parse(body)` then `.get[String]("message").getOrElse("").take(200)`.
fn github_message(body: &str) -> String {
    match serde_json::from_str::<Value>(body) {
        Ok(json) => json
            .get("message")
            .and_then(Value::as_str)
            .map(|message| take_utf16(message, 200))
            .unwrap_or_default(),
        Err(_) => String::new(),
    }
}

/// `^recipe/[a-z0-9]+(-[a-z0-9]+)*$`.
fn branch_pattern_matches(branch: &str) -> bool {
    match branch.strip_prefix("recipe/") {
        Some(rest) => {
            !rest.is_empty()
                && rest.split('-').all(|part| {
                    !part.is_empty()
                        && part
                            .chars()
                            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit())
                })
        }
        None => false,
    }
}

/// `^src/main/scala/se/reciba/api/recibase/recipes/[A-Z][A-Za-z0-9]*\.scala$`.
fn path_pattern_matches(path: &str) -> bool {
    const PREFIX: &str = "src/main/scala/se/reciba/api/recibase/recipes/";
    match path
        .strip_prefix(PREFIX)
        .and_then(|rest| rest.strip_suffix(".scala"))
    {
        Some(name) => {
            let mut characters = name.chars();
            match characters.next() {
                Some(first) if first.is_ascii_uppercase() => {
                    characters.all(|c| c.is_ascii_alphanumeric())
                }
                _ => false,
            }
        }
        None => false,
    }
}

const BASE64_ALPHABET: &[u8; 64] =
    b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

/// `java.util.Base64.getEncoder.encodeToString`, including the padding.
fn base64_encode(input: &[u8]) -> String {
    let mut out = String::with_capacity(input.len().div_ceil(3) * 4);
    for chunk in input.chunks(3) {
        let first = chunk[0] as u32;
        let second = chunk.get(1).copied().unwrap_or(0) as u32;
        let third = chunk.get(2).copied().unwrap_or(0) as u32;
        let triple = (first << 16) | (second << 8) | third;
        out.push(BASE64_ALPHABET[((triple >> 18) & 0x3f) as usize] as char);
        out.push(BASE64_ALPHABET[((triple >> 12) & 0x3f) as usize] as char);
        if chunk.len() > 1 {
            out.push(BASE64_ALPHABET[((triple >> 6) & 0x3f) as usize] as char);
        } else {
            out.push('=');
        }
        if chunk.len() > 2 {
            out.push(BASE64_ALPHABET[(triple & 0x3f) as usize] as char);
        } else {
            out.push('=');
        }
    }
    out
}
