
//! `resolve_deployed_version` from `app.py`: the commit the frontend was
//! deployed from, or `latest`.

use std::path::{Path, PathBuf};

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
        && let Some(dir) = exe.parent() {
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
    for key in COMMIT_ENV_KEYS {
        let value = environ(key).unwrap_or_default().trim().to_string();
        if is_commit(&value) {
            return value;
        }
    }
    let contents = match git_commit.and_then(|path| std::fs::read_to_string(path).ok()) {
        Some(contents) => contents,
        None => return "latest".to_string(),
    };
    let value = contents.trim();
    if is_commit(value) {
        value.to_string()
    } else {
        "latest".to_string()
    }
}

/// The deployed version of this process.
pub fn resolve_deployed_version() -> String {
    let environ = |key: &str| std::env::var(key).ok();
    resolve_version(&environ, git_commit_file().as_deref())
}
