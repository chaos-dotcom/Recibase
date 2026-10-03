
//! `cached_backend.py`: a value that is refetched once its TTL expires.

use std::sync::Mutex;
use std::time::{Duration, Instant};

/// The Python's `ttl_seconds = 15 * 60`.
pub const TTL_SECONDS: u64 = 15 * 60;

/// Caches one backend response. `fetch_data` calls the fetch function when
/// there is no cached value yet, or when the cached one is older than the
/// TTL; a failing fetch propagates and leaves the previous value in place,
/// exactly as the Python does.
pub struct CachedBackendCall<T> {
    ttl: Duration,
    fetch_function: Box<dyn Fn() -> Result<T, BackendUnavailable> + Send + Sync>,
    state: Mutex<Option<(T, Instant)>>,
}

/// The Python raises `BackendUnavailable` when the API cannot be reached.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BackendUnavailable;

impl std::fmt::Display for BackendUnavailable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("backend unavailable")
    }
}

impl std::error::Error for BackendUnavailable {}

impl<T: Clone> CachedBackendCall<T> {
    pub fn new(
        fetch_function: impl Fn() -> Result<T, BackendUnavailable> + Send + Sync + 'static,
    ) -> Self {
        CachedBackendCall {
            ttl: Duration::from_secs(TTL_SECONDS),
            fetch_function: Box::new(fetch_function),
            state: Mutex::new(None),
        }
    }

    pub fn with_ttl(mut self, ttl: Duration) -> Self {
        self.ttl = ttl;
        self
    }

    pub fn fetch_data(&self) -> Result<T, BackendUnavailable> {
        let mut state = self.state.lock().expect("cache mutex poisoned");
        let fresh = match state.as_ref() {
            Some((_, fetched_at)) => fetched_at.elapsed() <= self.ttl,
            None => false,
        };
        if !fresh {
            let value = (self.fetch_function)()?;
            *state = Some((value, Instant::now()));
        }
        Ok(state.as_ref().expect("just filled").0.clone())
    }
}
