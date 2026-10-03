//! Bakes the deployed commit into the binary, so the footer reads `v<hash>`
//! wherever nothing sets `SOURCE_COMMIT` / `GIT_COMMIT` / `GITHUB_SHA` at
//! runtime - which is every deployment of the frontend image.
//!
//! `.git/` is not in the Docker build context, so there the commit arrives as
//! a build argument (see the `frontend` service's `build.args` in
//! `compose.yaml`); a local `cargo build` reads it from the checkout instead.

use std::path::{Path, PathBuf};
use std::process::Command;

/// `^[0-9a-fA-F]{7,40}$`, the shape `version.rs` accepts.
fn is_commit(value: &str) -> bool {
    (7..=40).contains(&value.len()) && value.bytes().all(|byte| byte.is_ascii_hexdigit())
}

/// The commit a CI passes at build time, read the way the runtime reads it.
fn from_environment() -> Option<String> {
    ["SOURCE_COMMIT", "GIT_COMMIT", "GITHUB_SHA"]
        .into_iter()
        .find_map(|key| {
            println!("cargo:rerun-if-env-changed={key}");
            let value = std::env::var(key).ok()?;
            let value = value.trim().to_string();
            is_commit(&value).then_some(value)
        })
}

/// `git rev-parse HEAD` in the checkout, which is where local builds run.
fn from_git() -> Option<String> {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let output = Command::new("git")
        .arg("-C")
        .arg(manifest_dir)
        .args(["rev-parse", "HEAD"])
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    let value = String::from_utf8(output.stdout).ok()?.trim().to_string();
    is_commit(&value).then_some(value)
}

/// The `.git` directory at or above the crate, so a new commit reruns this.
fn git_dir() -> Option<PathBuf> {
    let mut dir: Option<&Path> = Some(Path::new(env!("CARGO_MANIFEST_DIR")));
    while let Some(current) = dir {
        let candidate = current.join(".git");
        if candidate.is_dir() {
            return Some(candidate);
        }
        dir = current.parent();
    }
    None
}

fn main() {
    // A directory is fingerprinted recursively, so a checkout or a commit -
    // both of which rewrite a ref - reruns this script.
    if let Some(git_dir) = git_dir() {
        println!("cargo:rerun-if-changed={}", git_dir.display());
    }
    let commit = from_environment().or_else(from_git).unwrap_or_default();
    println!("cargo:rustc-env=RECIBASE_BUILD_COMMIT={commit}");
}
