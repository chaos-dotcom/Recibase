//! The pages Werkzeug itself produces: the redirect, the 405 and the 416.
//!
//! Flask hands a redirect to Werkzeug's `redirect()`, an unmatched method to
//! `MethodNotAllowed`, and an unsatisfiable range to
//! `RequestedRangeNotSatisfiable`; each renders a page from Werkzeug's
//! templates. They are reproduced here so that the port answers the same
//! bytes.

use crate::http::Response;
use crate::markupsafe::escape;

/// `werkzeug.utils.redirect`.
pub fn redirect(status: u16, location: &str) -> Response {
    let escaped = escape(location);
    let body = format!(
        "<!doctype html>\n<html lang=en>\n<title>Redirecting...</title>\n\
         <h1>Redirecting...</h1>\n<p>You should be redirected automatically to \
         the target URL: <a href=\"{escaped}\">{escaped}</a>. If not, click the link.\n"
    );
    Response::html(status, body).header("Location", location)
}

/// `werkzeug.exceptions.MethodNotAllowed`.
pub fn method_not_allowed(allowed: &[&str]) -> Response {
    let body = "<!doctype html>\n<html lang=en>\n<title>405 Method Not Allowed</title>\n\
                <h1>Method Not Allowed</h1>\n\
                <p>The method is not allowed for the requested URL.</p>\n"
        .to_string();
    // Werkzeug writes `Allow` between the content type and the length.
    Response::new(405)
        .header("Content-Type", "text/html; charset=utf-8")
        .header("Allow", allowed.join(", "))
        .header("Content-Length", body.len().to_string())
        .body(body.into_bytes())
}

/// `werkzeug.exceptions.RequestedRangeNotSatisfiable`.
pub fn range_not_satisfiable(size: usize) -> Response {
    let body = "<!doctype html>\n<html lang=en>\n\
                <title>416 Requested Range Not Satisfiable</title>\n\
                <h1>Requested Range Not Satisfiable</h1>\n\
                <p>The server cannot provide the requested range.</p>\n"
        .to_string();
    Response::new(416)
        .header("Content-Type", "text/html; charset=utf-8")
        .header("Content-Range", format!("bytes */{}", size))
        .header("Content-Length", body.len().to_string())
        .body(body.into_bytes())
}
