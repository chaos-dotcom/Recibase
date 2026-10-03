//! `test_cached_backend.py`'s two cases, the TTL behaviour around them, and
//! `version.rs` - the `resolve_deployed_version` cases from `test_routes.py`.
//!
//! `resolve_version` takes the environment as a closure and the `GIT_COMMIT`
//! file as a `Option<&Path>`, so none of these touch the process environment.

use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

use recibase_frontend::cached_backend::{BackendUnavailable, CachedBackendCall, TTL_SECONDS};
use recibase_frontend::version::{resolve_version, resolve_version_with_build};

// -- `cached_backend.py` -----------------------------------------------

/// `test_fetch_data_calls_function_once`.
#[test]
fn fetch_data_calls_function_once() {
    let calls = Arc::new(AtomicUsize::new(0));
    let cached = CachedBackendCall::new({
        let calls = Arc::clone(&calls);
        move || {
            calls.fetch_add(1, Ordering::SeqCst);
            Ok(vec!["ok"])
        }
    });

    assert_eq!(cached.fetch_data(), Ok(vec!["ok"]));
    assert_eq!(cached.fetch_data(), Ok(vec!["ok"]));
    assert_eq!(calls.load(Ordering::SeqCst), 1);
}

/// `test_fetch_data_fails_fast`: a failing fetch propagates and is not
/// cached.
#[test]
fn fetch_data_fails_fast() {
    let calls = Arc::new(AtomicUsize::new(0));
    let cached: CachedBackendCall<Vec<&str>> = CachedBackendCall::new({
        let calls = Arc::clone(&calls);
        move || {
            calls.fetch_add(1, Ordering::SeqCst);
            Err(BackendUnavailable)
        }
    });

    assert_eq!(cached.fetch_data(), Err(BackendUnavailable));
    assert_eq!(calls.load(Ordering::SeqCst), 1);
}

/// `CachedBackendCall.ttl_seconds`.
#[test]
fn ttl_seconds_is_fifteen_minutes() {
    assert_eq!(TTL_SECONDS, 15 * 60);
}

/// The other half of the cache: once the TTL has passed the fetch function
/// runs again, and a failed refetch leaves the cached value alone - the next
/// call fetches a fresh one rather than reporting the old value as current.
#[test]
fn fetch_data_refetches_after_the_ttl() {
    let calls = Arc::new(AtomicUsize::new(0));
    let cached = CachedBackendCall::new({
        let calls = Arc::clone(&calls);
        move || {
            let call = calls.fetch_add(1, Ordering::SeqCst);
            match call {
                0 => Ok("first".to_string()),
                1 => Err(BackendUnavailable),
                _ => Ok("second".to_string()),
            }
        }
    })
    .with_ttl(Duration::ZERO);

    assert_eq!(cached.fetch_data(), Ok("first".to_string()));
    assert_eq!(cached.fetch_data(), Err(BackendUnavailable));
    assert_eq!(cached.fetch_data(), Ok("second".to_string()));
    assert_eq!(calls.load(Ordering::SeqCst), 3);
}

// -- `resolve_deployed_version` ----------------------------------------

fn environ<'a>(pairs: &'a [(&'a str, &'a str)]) -> impl Fn(&str) -> Option<String> + 'a {
    move |key: &str| {
        pairs
            .iter()
            .find(|(name, _)| *name == key)
            .map(|(_, value)| (*value).to_string())
    }
}

/// `test_resolve_deployed_version_prefers_source_commit`.
#[test]
fn resolve_deployed_version_prefers_source_commit() {
    let vars = environ(&[
        ("SOURCE_COMMIT", "abcdef1234567890"),
        ("GIT_COMMIT", "deadbeef"),
    ]);
    assert_eq!(resolve_version(&vars, None), "abcdef1234567890");
}

/// `test_resolve_deployed_version_ignores_head_and_latest`.
#[test]
fn resolve_deployed_version_ignores_head_and_latest() {
    let vars = environ(&[
        ("SOURCE_COMMIT", "HEAD"),
        ("GIT_COMMIT", "latest"),
        ("GITHUB_SHA", "cafeba6"),
    ]);
    assert_eq!(resolve_version(&vars, None), "cafeba6");
}

/// `test_resolve_deployed_version_falls_back_to_latest`.
#[test]
fn resolve_deployed_version_falls_back_to_latest() {
    let vars = environ(&[]);
    assert_eq!(resolve_version(&vars, None), "latest");
}

/// The `GIT_COMMIT` file is read when the environment says nothing, and a
/// file that is not a commit is no better than no file.
#[test]
fn resolve_deployed_version_reads_the_commit_file() {
    let empty = environ(&[]);

    let commit = temp_file("resolve-version-commit", "  abcdef1\n");
    assert_eq!(resolve_version(&empty, Some(commit.as_path())), "abcdef1");

    let nonsense = temp_file("resolve-version-nonsense", "HEAD\n");
    assert_eq!(resolve_version(&empty, Some(nonsense.as_path())), "latest");

    let missing = PathBuf::from("/nonexistent/recibase/GIT_COMMIT");
    assert_eq!(resolve_version(&empty, Some(missing.as_path())), "latest");
}

/// The environment wins over the file, which is what `app.py` does: it only
/// opens `GIT_COMMIT` when no variable holds a commit.
#[test]
fn resolve_deployed_version_prefers_the_environment_over_the_file() {
    let vars = environ(&[("GIT_COMMIT", "abcdef1")]);
    let file = temp_file("resolve-version-both", "0123456\n");
    assert_eq!(resolve_version(&vars, Some(file.as_path())), "abcdef1");
}

/// The commit `build.rs` baked in is the last resort before `latest`: it is
/// used only when neither the environment nor the `GIT_COMMIT` file names one,
/// and a value that is not a commit is no better than none.
#[test]
fn resolve_deployed_version_falls_back_to_the_build_commit() {
    let empty = environ(&[]);
    assert_eq!(
        resolve_version_with_build(&empty, None, Some("cafebabe")),
        "cafebabe"
    );
    assert_eq!(
        resolve_version_with_build(&empty, None, Some("latest")),
        "latest"
    );
    assert_eq!(resolve_version_with_build(&empty, None, Some("")), "latest");
}

/// The environment and the file still win over the build-time commit.
#[test]
fn resolve_deployed_version_prefers_the_environment_and_file_over_the_build_commit() {
    let vars = environ(&[("GIT_COMMIT", "abcdef1")]);
    let file = temp_file("resolve-version-build-file", "0123456\n");
    assert_eq!(
        resolve_version_with_build(&vars, None, Some("cafebabe")),
        "abcdef1"
    );
    assert_eq!(
        resolve_version_with_build(&environ(&[]), Some(file.as_path()), Some("cafebabe")),
        "0123456"
    );
}

/// A file of `contents` in the temporary directory, cleaned up by the caller
/// via the returned path's parent (the file itself is left to the OS).
fn temp_file(name: &str, contents: &str) -> PathBuf {
    let path = std::env::temp_dir().join(format!("recibase-{}-{}", name, std::process::id()));
    std::fs::write(&path, contents).expect("write the temporary file");
    path
}
