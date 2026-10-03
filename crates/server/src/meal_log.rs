//! Fetching and parsing the meal log CSV (`UsageData` in the Scala).
//!
//! The Scala reads `MEAL_LOG_CSV_URL` with `scala.io.Source.fromURL`, so both
//! `file:` and `http(s):` URLs work here.

use recibase_core::usage::{self, MealLogEntry};
use std::collections::HashMap;

/// Minimal RFC 4180 reader: quoted fields, doubled quotes, CRLF or LF.
pub fn read_rows(text: &str) -> Vec<HashMap<String, String>> {
    let mut records: Vec<Vec<String>> = Vec::new();
    let mut field = String::new();
    let mut record: Vec<String> = Vec::new();
    let mut in_quotes = false;
    let mut chars = text.chars().peekable();
    while let Some(c) = chars.next() {
        if in_quotes {
            if c == '"' {
                if chars.peek() == Some(&'"') {
                    field.push('"');
                    chars.next();
                } else {
                    in_quotes = false;
                }
            } else {
                field.push(c);
            }
        } else {
            match c {
                '"' => in_quotes = true,
                ',' => record.push(std::mem::take(&mut field)),
                '\r' => {}
                '\n' => {
                    record.push(std::mem::take(&mut field));
                    records.push(std::mem::take(&mut record));
                }
                _ => field.push(c),
            }
        }
    }
    if !field.is_empty() || !record.is_empty() {
        record.push(field);
        records.push(record);
    }

    let mut out = Vec::new();
    let mut iter = records.into_iter();
    let Some(header) = iter.next() else {
        return out;
    };
    let header: Vec<String> = header
        .into_iter()
        .map(|h| h.trim_start_matches('\u{feff}').to_string())
        .collect();
    for record in iter {
        if record.iter().all(|f| f.is_empty()) {
            continue;
        }
        let mut row = HashMap::new();
        for (i, name) in header.iter().enumerate() {
            row.insert(name.clone(), record.get(i).cloned().unwrap_or_default());
        }
        out.push(row);
    }
    out
}

pub fn fetch(url: &str) -> Result<String, String> {
    if let Some(path) = url.strip_prefix("file://") {
        std::fs::read_to_string(path).map_err(|e| e.to_string())
    } else {
        let response = ureq::get(url).call().map_err(|e| e.to_string())?;
        response
            .into_body()
            .read_to_string()
            .map_err(|e| e.to_string())
    }
}

/// The parsed log, or an empty set when `MEAL_LOG_CSV_URL` is unset (the Scala
/// maps `Option` to `Set.empty`).
pub fn load_from_env(env: &dyn Fn(&str) -> Option<String>) -> Vec<MealLogEntry> {
    match env("MEAL_LOG_CSV_URL") {
        None => Vec::new(),
        Some(url) => match fetch(&url) {
            Ok(text) => usage::parse_csv_rows(&read_rows(&text)),
            Err(error) => {
                eprintln!("failed to read {}: {}", url, error);
                Vec::new()
            }
        },
    }
}
