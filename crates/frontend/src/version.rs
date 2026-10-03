//! `resolve_deployed_version` from `app.py`: the commit the frontend was
//! deployed from, or `latest`.
//!
//! The runtime environment and the `GIT_COMMIT` file come first. A deployment
//! that sets neither - the frontend image, which does not see the build
//! machine's checkout - falls back to the commit `build.rs` baked in.

use std::path::{Path, PathBuf};

/// The commit `build.rs` found when this binary was built, or `""`.
const BUILD_COMMIT: Option<&str> = option_env!("RECIBASE_BUILD_COMMIT");

/// Environment variables checked in order.
pub const COMMIT_ENV_KEYS: [&str; 3] = ["SOURCE_COMMIT", "GIT_COMMIT", "GITHUB_SHA"];

/// `^[0-9a-fA-F]{7,40}$`.
pub fn is_commit(value: &str) -> bool {
    (7..=40).contains(&value.len()) && value.bytes().all(|b| b.is_ascii_hexdigit())
}

/// The Python reads `GIT_COMMIT` from beside the module. The Rust binary
/// looks beside the executable first, then in the working directory.
pub fn git_commit_file() -> Option<PathBuf> {
    let mut candidates: Vec<PathBuf> = Vec::new();
    if let Ok(exe) = std::env::current_exe()
        && let Some(dir) = exe.parent()
    {
        candidates.push(dir.join("GIT_COMMIT"));
    }
    candidates.push(PathBuf::from("GIT_COMMIT"));
    candidates.into_iter().find(|path| path.is_file())
}

/// The Python's `resolve_deployed_version(environ)`; `git_commit` is the
/// `GIT_COMMIT` file, if there is one.
pub fn resolve_version(
    environ: &dyn Fn(&str) -> Option<String>,
    git_commit: Option<&Path>,
) -> String {
    resolve_version_with_build(environ, git_commit, None)
}

/// [`resolve_version`], with the commit baked in at build time as the last
/// resort before `latest`: environment, then file, then build, then `latest`.
pub fn resolve_version_with_build(
    environ: &dyn Fn(&str) -> Option<String>,
    git_commit: Option<&Path>,
    build_commit: Option<&str>,
) -> String {
    for key in COMMIT_ENV_KEYS {
        let value = environ(key).unwrap_or_default().trim().to_string();
        if is_commit(&value) {
            return value;
        }
    }
    if let Some(contents) = git_commit.and_then(|path| std::fs::read_to_string(path).ok()) {
        let value = contents.trim();
        if is_commit(value) {
            return value.to_string();
        }
    }
    if let Some(value) = build_commit {
        let value = value.trim();
        if is_commit(value) {
            return value.to_string();
        }
    }
    "latest".to_string()
}

/// The deployed version of this process.
pub fn resolve_deployed_version() -> String {
    let environ = |key: &str| std::env::var(key).ok();
    resolve_version_with_build(&environ, git_commit_file().as_deref(), BUILD_COMMIT)
}
