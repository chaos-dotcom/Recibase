//! `statics.rs`: `/static/<filename>`, the `ETag` and the conditional
//! requests Werkzeug's `send_file` answers.

mod support;

use std::path::{Path, PathBuf};

use chrono::DateTime;
use recibase_frontend::http::http_date;
use recibase_frontend::statics::{Outcome, StaticFiles, python_float_repr};
use support::{
    StubApi, app_on, body_of, get, header, header_or, root_statics_at_source, static_dir,
    with_header,
};

/// `repr(float)` is what the ETag quotes: `1790891551.0`, not
/// `1790891551`.
#[test]
fn python_float_repr_writes_a_float() {
    assert_eq!(python_float_repr(1790891551.0), "1790891551.0");
    assert_eq!(python_float_repr(0.0), "0.0");
    assert_eq!(python_float_repr(900.0), "900.0");
    assert_eq!(python_float_repr(2.5), "2.5");
    assert_eq!(python_float_repr(1790891551.1234567), "1790891551.1234567");
}

/// The file the serving tests use.
fn styles() -> PathBuf {
    static_dir().join("styles.css")
}

#[test]
fn static_files_root_is_the_environment_variable() {
    let root = root_statics_at_source();
    assert_eq!(StaticFiles::from_env().root(), root.as_path());
}

#[test]
fn a_static_file_carries_werkzeugs_headers() {
    root_statics_at_source();
    let response = StaticFiles::from_env()
        .serve(&get("/static/styles.css"), "styles.css");
    let Outcome::Served(response) = response else {
        panic!("styles.css is not being served");
    };
    assert_eq!(response.status, 200);
    assert_eq!(header_or(&response, "Content-Type"), "text/css; charset=utf-8");
    assert_eq!(
        header_or(&response, "Content-Disposition"),
        "inline; filename=styles.css",
    );
    assert_eq!(header_or(&response, "Cache-Control"), "no-cache");
    assert_eq!(header_or(&response, "Accept-Ranges"), "bytes");

    let contents = std::fs::read(styles()).expect("static/styles.css");
    assert_eq!(response.body, contents);
    assert_eq!(
        header_or(&response, "Content-Length"),
        contents.len().to_string(),
    );

    // The ETag is `"<mtime as Python writes it>-<size>-<adler32 of the path>"`.
    let modified = std::fs::metadata(styles())
        .expect("styles.css's metadata")
        .modified()
        .expect("styles.css's mtime")
        .duration_since(std::time::UNIX_EPOCH)
        .expect("an mtime after the epoch")
        .as_secs_f64();
    let expected = format!(
        "\"{}-{}-{}\"",
        python_float_repr(modified),
        contents.len(),
        adler32(styles().to_string_lossy().as_bytes()),
    );
    assert_eq!(header_or(&response, "ETag"), expected);

    // `Last-Modified` is the same instant in RFC 1123.
    let last_modified = DateTime::parse_from_rfc2822(header_or(&response, "Last-Modified"))
        .expect("an RFC 1123 Last-Modified");
    assert_eq!(last_modified.timestamp(), modified as i64);
    assert_eq!(
        header_or(&response, "Last-Modified"),
        http_date(DateTime::from_timestamp(modified as i64, 0).expect("a timestamp")),
    );
}

/// A matching `If-None-Match` is a 304 with no body, and an `If-Modified-Since`
/// at the file's own `Last-Modified` is the same.
#[test]
fn a_matching_conditional_request_is_not_modified() {
    root_statics_at_source();
    let files = StaticFiles::from_env();
    let fresh = files.serve(&get("/static/styles.css"), "styles.css");
    let Outcome::Served(fresh) = fresh else { panic!("styles.css is not being served") };
    let etag = header_or(&fresh, "ETag").to_string();
    let last_modified = header_or(&fresh, "Last-Modified").to_string();

    let request = with_header(get("/static/styles.css"), "If-None-Match", &etag);
    let Outcome::Served(response) = files.serve(&request, "styles.css") else {
        panic!("styles.css is not being served")
    };
    assert_eq!(response.status, 304);
    assert!(response.body.is_empty());
    assert_eq!(header_or(&response, "ETag"), etag.as_str());
    assert_eq!(header(&response, "Content-Type"), None);

    let request =
        with_header(get("/static/styles.css"), "If-Modified-Since", &last_modified);
    let Outcome::Served(response) = files.serve(&request, "styles.css") else {
        panic!("styles.css is not being served")
    };
    assert_eq!(response.status, 304);

    // A stale `If-None-Match` is the whole file.
    let request =
        with_header(get("/static/styles.css"), "If-None-Match", "\"something-else\"");
    let Outcome::Served(response) = files.serve(&request, "styles.css") else {
        panic!("styles.css is not being served")
    };
    assert_eq!(response.status, 200);
    assert!(!response.body.is_empty());
}

#[test]
fn a_range_request_is_partial_content() {
    root_statics_at_source();
    let files = StaticFiles::from_env();
    let contents = std::fs::read(styles()).expect("static/styles.css");

    let request = with_header(get("/static/styles.css"), "Range", "bytes=0-9");
    let Outcome::Served(response) = files.serve(&request, "styles.css") else {
        panic!("styles.css is not being served")
    };
    assert_eq!(response.status, 206);
    assert_eq!(response.body, contents[..10]);
    assert_eq!(
        header_or(&response, "Content-Range"),
        format!("bytes 0-9/{}", contents.len()),
    );
    assert_eq!(header_or(&response, "Content-Length"), "10");

    // `bytes=-10` is the last ten bytes.
    let request = with_header(get("/static/styles.css"), "Range", "bytes=-10");
    let Outcome::Served(response) = files.serve(&request, "styles.css") else {
        panic!("styles.css is not being served")
    };
    assert_eq!(response.status, 206);
    assert_eq!(response.body, contents[contents.len() - 10..]);

    // `bytes=10-` runs to the end of the file.
    let request = with_header(get("/static/styles.css"), "Range", "bytes=10-");
    let Outcome::Served(response) = files.serve(&request, "styles.css") else {
        panic!("styles.css is not being served")
    };
    assert_eq!(response.status, 206);
    assert_eq!(response.body, contents[10..]);
}

/// An unsatisfiable range is Werkzeug's 416, with the file's size.
#[test]
fn an_unsatisfiable_range_is_416() {
    root_statics_at_source();
    let files = StaticFiles::from_env();
    let size = std::fs::read(styles()).expect("static/styles.css").len();

    for range in ["bytes=999999999-", "bytes=0-1,4-5", "items=0-9", "bytes=5-4"] {
        let request = with_header(get("/static/styles.css"), "Range", range);
        let Outcome::Served(response) = files.serve(&request, "styles.css") else {
            panic!("styles.css is not being served")
        };
        assert_eq!(response.status, 416, "Range: {range}");
        assert_eq!(
            header_or(&response, "Content-Range"),
            format!("bytes */{}", size),
            "Range: {range}",
        );
    }
}

#[test]
fn missing_and_unreachable_files_are_not_found() {
    root_statics_at_source();
    let files = StaticFiles::from_env();
    for filename in ["nope.css", "", "/styles.css", "../Cargo.toml", "./styles.css", "sub/../styles.css"] {
        assert!(
            matches!(files.serve(&get("/static/styles.css"), filename), Outcome::NotFound),
            "{filename:?} should not be served",
        );
    }
}

/// The route turns `NotFound` into the 404 page, for a file that does not
/// exist as well as for a traversal attempt.
#[test]
fn an_unknown_static_file_is_the_404_page() {
    root_statics_at_source();
    let stub = StubApi::start();
    let app = app_on(&stub);
    let response = app.handle(&get("/static/nope.css"));
    assert_eq!(response.status, 404);
    assert!(body_of(&response).contains("404 - Page not Found"));
}

/// Werkzeug's `ETag`: `zlib.adler32` of the file's path, in lower case hex
/// here only because the format has no leading zeroes.
fn adler32(data: &[u8]) -> u32 {
    let mut a: u32 = 1;
    let mut b: u32 = 0;
    for byte in data {
        a = (a + u32::from(*byte)) % 65521;
        b = (b + a) % 65521;
    }
    (b << 16) | a
}

/// `Path::strip_prefix` keeps the relative path's separator handling honest:
/// the served file is the one under the root.
#[test]
fn the_root_joins_the_filename() {
    let root = root_statics_at_source();
    assert!(root.join("styles.css").is_file());
    assert_eq!(root.file_name().and_then(|name| name.to_str()), Some("static"));
    assert_eq!(Path::new(env!("CARGO_MANIFEST_DIR")).join("static"), root);
}

/// A file whose timestamp has a sub-second part must still answer `304` to an
/// `If-Modified-Since` that matches its `Last-Modified`.
///
/// HTTP dates have no fractions, so Werkzeug drops the microseconds from the
/// file's timestamp before comparing. Without that, a file modified at
/// 21:52:31.75 looks newer than an `If-Modified-Since` of 21:52:31 and the
/// answer is a `200`. This is pinned here rather than left to the real
/// `styles.css`, whose own timestamp happens to be a whole second.
#[test]
fn a_fractional_timestamp_still_matches_if_modified_since() {
    fn served(files: &StaticFiles, request: &recibase_frontend::http::Request) -> recibase_frontend::http::Response {
        match files.serve(request, "styles.css") {
            Outcome::Served(response) => response,
            Outcome::NotFound => panic!("styles.css is not being served"),
        }
    }

    let dir = std::env::temp_dir().join("recibase-statics-fractional");
    std::fs::create_dir_all(&dir).expect("temp static dir");
    let file = dir.join("styles.css");
    std::fs::copy(styles(), &file).expect("copy styles.css");
    // 2026-10-01 21:52:31 UTC, with 750 ms on top.
    let when = std::time::UNIX_EPOCH + std::time::Duration::from_millis(1790891551750);
    let handle = std::fs::File::options().write(true).open(&file).expect("open");
    handle
        .set_times(std::fs::FileTimes::new().set_modified(when))
        .expect("set mtime");
    drop(handle);

    unsafe { std::env::set_var("STATIC_DIR", &dir) };
    let files = StaticFiles::from_env();

    let fresh = served(&files, &get("/static/styles.css"));
    assert_eq!(fresh.status, 200);
    assert_eq!(
        header_or(&fresh, "Last-Modified"),
        "Thu, 01 Oct 2026 21:52:31 GMT",
    );
    assert!(
        header_or(&fresh, "ETag").contains("1790891551.75"),
        "the ETag keeps the fraction: {}",
        header_or(&fresh, "ETag"),
    );

    // The date does not, so this matches.
    let request = with_header(
        get("/static/styles.css"),
        "If-Modified-Since",
        "Thu, 01 Oct 2026 21:52:31 GMT",
    );
    assert_eq!(
        served(&files, &request).status,
        304,
        "the sub-second part must not count",
    );

    // One second earlier is a real change.
    let request = with_header(
        get("/static/styles.css"),
        "If-Modified-Since",
        "Thu, 01 Oct 2026 21:52:30 GMT",
    );
    let response = served(&files, &request);
    assert_eq!(response.status, 200);
    assert!(!response.body.is_empty());

    unsafe { std::env::set_var("STATIC_DIR", static_dir()) };
}
