//! `request.form`: an ordered, multi-valued map.
//!
//! Werkzeug's `MultiDict` keeps every value for a key in arrival order;
//! `form.get(k)` returns the first and `form.getlist(k)` returns all of
//! them. `contribute.rs` depends on both behaviours.

/// An ordered multi-valued form, as sent by a browser in the body of a
/// `application/x-www-form-urlencoded` POST.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Form {
    entries: Vec<(String, String)>,
}

impl Form {
    pub fn new() -> Form {
        Form {
            entries: Vec::new(),
        }
    }

    pub fn from_pairs(pairs: Vec<(String, String)>) -> Form {
        Form { entries: pairs }
    }

    /// Parses `a=1&b=2&a=3`. An empty body is an empty form.
    pub fn parse(body: &[u8]) -> Form {
        let text = String::from_utf8_lossy(body);
        let mut entries = Vec::new();
        for pair in text.split('&') {
            if pair.is_empty() {
                continue;
            }
            let (key, value) = match pair.split_once('=') {
                Some((k, v)) => (k, v),
                None => (pair, ""),
            };
            entries.push((percent_decode(key), percent_decode(value)));
        }
        Form { entries }
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn pairs(&self) -> &[(String, String)] {
        &self.entries
    }

    /// The first value for `key`, like `MultiDict.get`.
    pub fn get(&self, key: &str) -> Option<&str> {
        self.entries
            .iter()
            .find(|(k, _)| k == key)
            .map(|(_, v)| v.as_str())
    }

    /// `form.get(key, default)` with a `str` default.
    pub fn get_or<'a>(&'a self, key: &str, default: &'a str) -> &'a str {
        self.get(key).unwrap_or(default)
    }

    /// Every value for `key`, like `MultiDict.getlist`.
    pub fn get_list(&self, key: &str) -> Vec<&str> {
        self.entries
            .iter()
            .filter(|(k, _)| k == key)
            .map(|(_, v)| v.as_str())
            .collect()
    }
}

/// `urllib.parse.unquote_plus`: `%XX` escapes and `+` as space.
pub fn percent_decode(input: &str) -> String {
    let bytes = input.as_bytes();
    let mut out: Vec<u8> = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        match bytes[i] {
            b'%' if i + 2 < bytes.len() => {
                let hex = std::str::from_utf8(&bytes[i + 1..i + 3]).unwrap_or("");
                match u8::from_str_radix(hex, 16) {
                    Ok(byte) => {
                        out.push(byte);
                        i += 3;
                    }
                    Err(_) => {
                        out.push(bytes[i]);
                        i += 1;
                    }
                }
            }
            b'+' => {
                out.push(b' ');
                i += 1;
            }
            byte => {
                out.push(byte);
                i += 1;
            }
        }
    }
    String::from_utf8_lossy(&out).into_owned()
}
