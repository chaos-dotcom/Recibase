
//! `/static/<filename>`, the Rust spelling of Flask's `send_from_directory`.
//!
//! Flask serves static files through Werkzeug's `send_file`, which sets a
//! `Content-Disposition`, a `Last-Modified`, a `Cache-Control: no-cache`, an
//! `ETag` of the form `"<mtime>-<size>-<adler32 of the path>"` and
//! `Accept-Ranges: bytes`, then makes the response conditional: a matching
//! `If-None-Match` or `If-Modified-Since` answers `304`, and a `Range`
//! answers `206`, or `416` when the range cannot be satisfied.

use std::path::{Path, PathBuf};

use crate::http::{Request, Response};
use crate::pages;

/// Whether the request was served from the filesystem.
pub enum Outcome {
    Served(Response),
    NotFound,
}

pub struct StaticFiles {
    root: PathBuf,
}

impl StaticFiles {
    /// The directory `/static/` is served from.
    ///
    /// `STATIC_DIR` wins. Otherwise `static` is looked for beside the
    /// executable - the shape of a deployment that copies the assets next to the
    /// binary - and then, for a source checkout, in `crates/frontend/static`
    /// above the executable (`target/release/`) and above the working directory
    /// (`cargo run`).
    pub fn from_env() -> StaticFiles {
        if let Ok(dir) = std::env::var("STATIC_DIR") {
            return StaticFiles { root: PathBuf::from(dir) };
        }
        let mut candidates: Vec<PathBuf> = Vec::new();
        let mut collect = |base: Option<&Path>| {
            if let Some(base) = base {
                candidates.push(base.join("static"));
                if let Some(repo) = base.parent().and_then(Path::parent) {
                    candidates.push(repo.join("static"));
                    candidates.push(repo.join("crates").join("frontend").join("static"));
                }
            }
        };
        let exe_dir = std::env::current_exe()
            .ok()
            .and_then(|exe| exe.parent().map(Path::to_path_buf));
        collect(exe_dir.as_deref());
        collect(std::env::current_dir().ok().as_deref());
        candidates
            .iter()
            .find(|dir| dir.is_dir())
            .cloned()
            .map(|root| StaticFiles { root })
            .unwrap_or_else(|| StaticFiles { root: PathBuf::from("static") })
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    pub fn serve(&self, request: &Request, filename: &str) -> Outcome {
        if filename.is_empty() || filename.starts_with('/') {
            return Outcome::NotFound;
        }
        if filename.split('/').any(|segment| segment == ".." || segment == ".") {
            return Outcome::NotFound;
        }
        let path = self.root.join(filename);
        if !path.is_file() {
            return Outcome::NotFound;
        }
        let Ok(metadata) = std::fs::metadata(&path) else {
            return Outcome::NotFound;
        };
        let Ok(contents) = std::fs::read(&path) else {
            return Outcome::NotFound;
        };

        let size = contents.len();
        let modified = metadata
            .modified()
            .ok()
            .and_then(|time| time.duration_since(std::time::UNIX_EPOCH).ok())
            .map(|delta| delta.as_secs_f64())
            .unwrap_or(0.0);
        let etag = format!(
            "\"{}-{}-{}\"",
            python_float_repr(modified),
            size,
            adler32(path.to_string_lossy().as_bytes())
        );
        let last_modified = crate::http::http_date(
            chrono::DateTime::from_timestamp(modified as i64, 0)
                .unwrap_or_else(|| chrono::DateTime::from_timestamp(0, 0).expect("epoch")),
        );
        let basename = filename.rsplit('/').next().unwrap_or(filename);
        let content_type = mime_type(basename);

        let entity_headers = |response: Response| {
            response
                .header("Content-Disposition", format!("inline; filename={}", basename))
                .header("Content-Type", format!("{}; charset=utf-8", content_type))
        };
        let full_headers = |response: Response, length: Option<usize>| {
            let response = entity_headers(response);
            let response = match length {
                Some(length) => response.header("Content-Length", length.to_string()),
                None => response,
            };
            response
                .header("Last-Modified", last_modified.clone())
                .header("Cache-Control", "no-cache")
                .header("ETag", etag.clone())
                .header("Accept-Ranges", "bytes")
        };

        let conditional = request.method.eq_ignore_ascii_case("GET")
            || request.method.eq_ignore_ascii_case("HEAD");
        let mut served_range = None;
        if conditional
            && let Some(range_header) = request.header("Range")
                && range_is_processable(request, &etag, &last_modified) {
                    match parse_range(range_header, size) {
                        Some(range) => served_range = Some(range),
                        None => return Outcome::Served(pages::range_not_satisfiable(size)),
                    }
                }

        if let Some((start, end)) = served_range {
            let body = contents[start..end].to_vec();
            let response = full_headers(Response::new(206), Some(body.len()))
                .header("Content-Range", format!("bytes {}-{}/{}", start, end - 1, size))
                .body(body);
            return Outcome::Served(response);
        }

        if conditional && !is_resource_modified(request, &etag, modified) {
            // A failed `If-Match` is a 412 that keeps the whole response -
            // headers and body - and changes only the status.
            if ParsedEtags::parse(request.header("If-Match").unwrap_or_default()).is_present() {
                let response = full_headers(Response::new(412), Some(size)).body(contents);
                return Outcome::Served(response);
            }
            let response = Response::new(304)
                .header("Content-Disposition", format!("inline; filename={}", basename))
                .header("Cache-Control", "no-cache")
                .header("ETag", etag.clone())
                .header("Accept-Ranges", "bytes");
            return Outcome::Served(response);
        }

        let response = full_headers(Response::new(200), Some(size)).body(contents);
        Outcome::Served(response)
    }
}

/// Werkzeug derives the ETag from the *path* of the file, not its contents.
fn adler32(data: &[u8]) -> u32 {
    let mut a: u32 = 1;
    let mut b: u32 = 0;
    for byte in data {
        a = (a + u32::from(*byte)) % 65521;
        b = (b + a) % 65521;
    }
    (b << 16) | a
}

/// `repr(float)` in Python, which is what the ETag quotes: `1790891551.0`,
/// where Rust's own `Display` would write `1790891551`.
pub fn python_float_repr(value: f64) -> String {
    let text = format!("{}", value);
    if text.contains('.') || text.contains('e') || text.contains("inf") || text.contains("NaN") {
        text
    } else {
        format!("{}.0", text)
    }
}

fn mime_type(name: &str) -> &'static str {
    let extension = name.rsplit('.').next().unwrap_or("").to_ascii_lowercase();
    match extension.as_str() {
        "css" => "text/css",
        "js" => "text/javascript",
        "json" => "application/json",
        "html" | "htm" => "text/html",
        "xml" => "application/xml",
        "txt" => "text/plain",
        "svg" => "image/svg+xml",
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "gif" => "image/gif",
        "ico" => "image/vnd.microsoft.icon",
        "webp" => "image/webp",
        "woff" => "font/woff",
        "woff2" => "font/woff2",
        _ => "application/octet-stream",
    }
}

/// `bytes=10-19` -> `(10, 20)`; `bytes=10-` -> `(10, size)`; `bytes=-5` ->
/// `(size - 5, size)`. `None` means the range cannot be satisfied, which
/// Werkzeug answers with a 416.
fn parse_range(header: &str, size: usize) -> Option<(usize, usize)> {
    let spec = header.trim().strip_prefix("bytes=")?;
    let (first, rest) = spec.split_once(',').unwrap_or((spec, ""));
    if !rest.is_empty() {
        // Werkzeug rejects multi-range requests rather than answering the
        // first range.
        return None;
    }
    let (start_text, end_text) = first.trim().split_once('-')?;
    let start_text = start_text.trim();
    let end_text = end_text.trim();
    if start_text.is_empty() {
        let suffix: usize = end_text.parse().ok()?;
        if suffix == 0 {
            return None;
        }
        let start = size.saturating_sub(suffix);
        return Some((start, size));
    }
    let start: usize = start_text.parse().ok()?;
    if start >= size {
        return None;
    }
    if end_text.is_empty() {
        return Some((start, size));
    }
    let end: usize = end_text.parse().ok()?;
    if end < start {
        return None;
    }
    Some((start, (end + 1).min(size)))
}

fn range_is_processable(request: &Request, etag: &str, last_modified: &str) -> bool {
    if request.header("Range").is_none() {
        return false;
    }
    match request.header("If-Range") {
        None => true,
        Some(if_range) => {
            // The range is processed only when the resource has *not* changed
            // since the validator in `If-Range`.
            let value = if_range.trim();
            if value.starts_with('"') || value.starts_with("W/") {
                etag_matches(value, etag)
            } else {
                match (parse_http_date(value), parse_http_date(last_modified)) {
                    (Some(if_range_date), Some(modified)) => modified <= if_range_date,
                    _ => true,
                }
            }
        }
    }
}


/// An `ETag` header, parsed the way Werkzeug's `parse_etags` does.
#[derive(Debug, Default)]
struct ParsedEtags {
    star: bool,
    strong: Vec<String>,
    weak: Vec<String>,
}

impl ParsedEtags {
    fn parse(value: &str) -> ParsedEtags {
        let value = value.trim();
        if value.is_empty() {
            return ParsedEtags::default();
        }
        if value == "*" {
            return ParsedEtags { star: true, ..ParsedEtags::default() };
        }
        let mut parsed = ParsedEtags::default();
        for item in value.split(',') {
            let item = item.trim();
            if item.is_empty() {
                continue;
            }
            let (weak, rest) = match item.strip_prefix("W/").or_else(|| item.strip_prefix("w/")) {
                Some(rest) => (true, rest.trim()),
                None => (false, item),
            };
            let tag = match rest.strip_prefix('"').and_then(|r| r.strip_suffix('"')) {
                Some(tag) => tag.to_string(),
                // Werkzeug accepts an unquoted value too, and calls it an
                // "invalid unquoted" tag.
                None => rest.to_string(),
            };
            if weak {
                parsed.weak.push(tag);
            } else {
                parsed.strong.push(tag);
            }
        }
        parsed
    }

    /// `if if_none_match:` - an empty or unparseable header is skipped.
    fn is_present(&self) -> bool {
        self.star || !self.strong.is_empty() || !self.weak.is_empty()
    }

    fn contains(&self, etag: &str) -> bool {
        self.star || self.strong.iter().any(|tag| tag == etag)
    }

    /// `contains_weak`: the weak and the strong tags together.
    fn contains_weak(&self, etag: &str) -> bool {
        self.weak.iter().any(|tag| tag == etag) || self.contains(etag)
    }

    /// `is_strong`. A `*` tag is not strong: `ETags.__init__` leaves the strong
    /// set empty when `star_tag` is set, which is why `If-Match: *` is answered
    /// with a 412.
    fn is_strong(&self, etag: &str) -> bool {
        self.strong.iter().any(|tag| tag == etag)
    }
}

/// The tag without its quotes and its `W/` marker.
fn unquote_etag(value: &str) -> String {
    let value = value.trim();
    let value = value.strip_prefix("W/").unwrap_or(value).trim();
    value
        .strip_prefix('"')
        .and_then(|rest| rest.strip_suffix('"'))
        .unwrap_or(value)
        .to_string()
}

/// Werkzeug's `is_resource_modified`: `false` means "not modified", which the
/// caller answers with a 304 - or with a 412 when the request carried an
/// `If-Match`.
///
/// `If-Unmodified-Since` is deliberately not consulted, because Werkzeug's
/// `is_resource_modified` does not take it as an argument either.
fn is_resource_modified(request: &Request, etag_quoted: &str, modified: f64) -> bool {
    let etag = unquote_etag(etag_quoted);
    let mut unmodified = false;

    if let Some(value) = request.header("If-Modified-Since")
        && let Some(timestamp) = parse_http_date(value)
            && modified <= timestamp as f64 {
                unmodified = true;
            }

    if !etag.is_empty() {
        if let Some(value) = request.header("If-None-Match") {
            let parsed = ParsedEtags::parse(value);
            if parsed.is_present() {
                unmodified = parsed.contains_weak(&etag);
            }
        }
        // `If-Match` is applied after `If-None-Match`, and overwrites it.
        if let Some(value) = request.header("If-Match") {
            let parsed = ParsedEtags::parse(value);
            if parsed.is_present() {
                unmodified = !parsed.is_strong(&etag);
            }
        }
    }

    !unmodified
}

fn etag_matches(header: &str, etag: &str) -> bool {
    let wanted = etag.trim().trim_start_matches("W/");
    header.split(',').any(|candidate| {
        let candidate = candidate.trim();
        candidate == "*" || candidate.trim_start_matches("W/") == wanted
    })
}

/// The RFC 1123 form, which is the only one this application ever sends.
fn parse_http_date(value: &str) -> Option<i64> {
    chrono::DateTime::parse_from_rfc2822(value.trim())
        .ok()
        .map(|parsed| parsed.timestamp())
}
