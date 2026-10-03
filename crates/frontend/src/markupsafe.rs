
//! HTML escaping, byte-compatible with Jinja2's.
//!
//! Jinja2 escapes through MarkupSafe, which turns five characters into
//! entities:
//!
//! ```text
//! &  ->  &amp;      <  ->  &lt;      >  ->  &gt;
//! "  ->  &#34;      '  ->  &#39;
//! ```
//!
//! MiniJinja - a Jinja2 implementation, but a different one - escapes six,
//! with different spellings for two of them and `/` as well:
//!
//! ```text
//! "  ->  &quot;     '  ->  &#x27;    /  ->  &#x2f;
//! ```
//!
//! The rendered page is part of what this port is compared against, so the
//! MiniJinja spellings are rewritten back to MarkupSafe's after rendering.
//! `/` is simply unescaped. This is safe because escaping is applied to the
//! whole value when it is printed: a literal `&` in a value becomes `&amp;`,
//! so a value can never produce the byte sequence `&#x27;` or `&quot;` for
//! this rewriting to corrupt. `template_sources_are_free_of_the_rewritten_entities`
//! in the test suite checks the other half of that argument - that no
//! template contains those sequences literally.

const REWRITES: [(&str, &str); 3] = [
    ("&#x27;", "&#39;"),
    ("&quot;", "&#34;"),
    ("&#x2f;", "/"),
];

/// Rewrites MiniJinja's entity spellings to MarkupSafe's.
pub fn markupsafe_compat(rendered: &str) -> String {
    let mut out = rendered.to_string();
    for (from, to) in REWRITES {
        if out.contains(from) {
            out = out.replace(from, to);
        }
    }
    out
}

/// MarkupSafe's `escape`, for the places that escape by hand rather than
/// through the template engine.
pub fn escape(value: &str) -> String {
    let mut out = String::with_capacity(value.len());
    for ch in value.chars() {
        match ch {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&#34;"),
            '\'' => out.push_str("&#39;"),
            _ => out.push(ch),
        }
    }
    out
}
