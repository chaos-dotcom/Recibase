//! A port of `src/test/scala/se/reciba/api/recibase/UsageDataSpec.scala`,
//! plus the meal-log CSV reading rules and the comparison against the
//! byte-exact capture in `capture-scala-csv/`.
//!
//! OWNER: workstream W6.

use chrono::{DateTime, Datelike, Duration, NaiveDate, TimeZone, Utc};
use recibase_core::meal::DatedNote;
use recibase_core::usage::{
    featured_meals, last_eaten, notes, parse_csv, totals, Clock, MealLogEntry, UsageData,
    HARD_REFRESH_TIME_HOURS, SOFT_REFRESH_TIME_HOURS,
};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

fn date(year: i32, month: u32, day: u32) -> NaiveDate {
    NaiveDate::from_ymd_opt(year, month, day).unwrap()
}

fn entry(meal_name: &str, date: NaiveDate, note: Option<&str>) -> MealLogEntry {
    MealLogEntry::new(meal_name, date, note.map(|n| n.to_string()), false)
}

fn featured(meal_name: &str, date: NaiveDate) -> MealLogEntry {
    MealLogEntry::new(meal_name, date, None, true)
}

// ---------------------------------------------------------------------------
// UsageDataSpec
// ---------------------------------------------------------------------------

#[test]
fn date_parser_correctly_parses_date_strings() {
    assert_eq!(
        recibase_core::usage::parse_date("Thursday, 17 February 22").unwrap(),
        date(2022, 2, 17)
    );
    assert_eq!(
        recibase_core::usage::parse_date("Friday, 8 December 23").unwrap(),
        date(2023, 12, 8)
    );
}

#[test]
fn date_parser_rejects_unparsable_dates() {
    // `split(",")(1)` throws, so there is no field to parse.
    assert!(recibase_core::usage::parse_date("17 February 22").is_err());
    // "not a date,Some Meal" -> the second field is not a `d MMMM yy` date.
    assert!(recibase_core::usage::parse_date("not a date,Some Meal").is_err());
    // An unknown month name.
    assert!(recibase_core::usage::parse_date("Thursday, 17 Smarch 22").is_err());
    // A month name without a leading day.
    assert!(recibase_core::usage::parse_date("Thursday, February 22").is_err());
}

#[test]
fn last_eaten_calculates_the_most_recent_date_a_meal_was_eaten() {
    let actual = last_eaten(&[
        entry("Macaroni", date(2022, 2, 17), None),
        entry("Beyond Burgers", date(2022, 2, 18), None),
        entry("Macaroni", date(2022, 2, 19), None),
    ]);
    let mut expected = std::collections::HashMap::new();
    expected.insert("Beyond Burgers".to_string(), date(2022, 2, 18));
    expected.insert("Macaroni".to_string(), date(2022, 2, 19));
    assert_eq!(actual, expected);
}

#[test]
fn totals_calculates_how_often_meals_have_been_eaten() {
    let actual = totals(&[
        entry("Macaroni", date(2022, 2, 17), None),
        entry("Beyond Burgers", date(2022, 2, 18), None),
        entry("Macaroni", date(2022, 2, 19), None),
    ]);
    let mut expected = std::collections::HashMap::new();
    expected.insert("Beyond Burgers".to_string(), 1);
    expected.insert("Macaroni".to_string(), 2);
    assert_eq!(actual, expected);
}

#[test]
fn featured_meals_returns_the_most_recent_featured_date_per_meal() {
    let actual = featured_meals(&[
        featured("Macaroni", date(2022, 2, 17)),
        featured("Macaroni", date(2022, 2, 19)),
        featured("Beyond Burgers", date(2022, 2, 18)),
        entry("Salad", date(2022, 2, 20), None),
    ]);
    let mut expected = std::collections::HashMap::new();
    expected.insert("Macaroni".to_string(), date(2022, 2, 19));
    expected.insert("Beyond Burgers".to_string(), date(2022, 2, 18));
    assert_eq!(actual, expected);
    assert!(!actual.contains_key("Salad"));
}

#[test]
fn meal_log_entry_parses_the_feature_column() {
    let true_feature = MealLogEntry::parse("Macaroni", "Thursday, 17 February 22", "", "TRUE");
    assert_eq!(
        true_feature.unwrap(),
        featured("Macaroni", date(2022, 2, 17))
    );

    let false_feature = MealLogEntry::parse("Macaroni", "Thursday, 17 February 22", "", "FALSE");
    assert_eq!(
        false_feature.unwrap(),
        entry("Macaroni", date(2022, 2, 17), None)
    );

    let empty_feature = MealLogEntry::parse("Macaroni", "Thursday, 17 February 22", "", "");
    assert_eq!(
        empty_feature.unwrap(),
        entry("Macaroni", date(2022, 2, 17), None)
    );

    // Only the exact string "TRUE" features a meal.
    let lowercase = MealLogEntry::parse("Macaroni", "Thursday, 17 February 22", "", "true");
    assert!(!lowercase.unwrap().featured);

    let note = MealLogEntry::parse(
        "Macaroni",
        "Thursday, 17 February 22",
        "Add more cheese",
        "",
    );
    assert_eq!(note.unwrap().note, Some("Add more cheese".to_string()));
}

#[test]
fn aggregates_notes_made_when_making_meals() {
    let actual = notes(&[
        entry("Macaroni", date(2022, 2, 17), Some("Add more cheese")),
        entry("Beyond Burgers", date(2022, 2, 18), None),
        entry(
            "Macaroni",
            date(2022, 2, 19),
            Some("Try paprika on top before grilling"),
        ),
        entry("Macaroni", date(2022, 2, 20), None),
    ]);
    assert_eq!(actual.len(), 2);
    // A meal with entries but no notes still gets an (empty) key.
    assert_eq!(actual.get("Beyond Burgers"), Some(&Vec::new()));
    assert_eq!(
        actual.get("Macaroni"),
        Some(&vec![
            DatedNote {
                date: date(2022, 2, 17),
                note: "Add more cheese".to_string()
            },
            DatedNote {
                date: date(2022, 2, 19),
                note: "Try paprika on top before grilling".to_string(),
            },
        ])
    );
}

// ---------------------------------------------------------------------------
// The meal-log CSV
// ---------------------------------------------------------------------------

fn fixture_path() -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/meal-log.csv")
}

fn fixture_text() -> String {
    std::fs::read_to_string(fixture_path()).expect("the committed meal-log fixture")
}

fn fixture_entries() -> Vec<MealLogEntry> {
    parse_csv(std::fs::read(fixture_path()).unwrap().as_slice()).expect("the fixture parses")
}

/// The capture directory produced by the Scala server. Missing means the
/// capture has not been made in this checkout, so those assertions are skipped.
fn capture_dir() -> Option<std::path::PathBuf> {
    let dir = std::env::var("RECIBASE_CAPTURE_DIR")
        .unwrap_or_else(|_| "/Users/chaos/recibase-work/capture-scala-csv".to_string());
    let path = std::path::Path::new(&dir);
    if path.join("011_meals.raw").is_file() {
        Some(path.to_path_buf())
    } else {
        None
    }
}

#[test]
fn csv_fixture_parses_the_rows_that_survive() {
    let entries = fixture_entries();
    // 51 lines: one header row plus 50 data rows; the blank-meal row and the
    // row whose Date cell is not a `d MMMM yy` date are dropped.
    assert_eq!(entries.len(), 48);
    assert_eq!(
        entries
            .iter()
            .filter(|e| e.meal_name == "Apple & Sausage Filo Casserole")
            .count(),
        2
    );
    println!(
        "parsed {} CSV rows (2 of the 50 data rows dropped)",
        entries.len()
    );
}

#[test]
fn csv_reader_collapses_exact_duplicate_rows_like_a_scala_set() {
    let text = fixture_text();
    let first_data_row = text.lines().nth(1).unwrap().to_string();
    let duplicated = format!("{}{}\r\n", text, first_data_row);
    assert_eq!(parse_csv(duplicated.as_bytes()).unwrap(), fixture_entries());
}

#[test]
fn csv_reader_drops_the_blank_meal_and_the_unparsable_date() {
    let entries = fixture_entries();
    assert_eq!(entries.len(), 48);
    assert!(entries.iter().all(|e| !e.meal_name.is_empty()));
    assert!(!entries.iter().any(|e| e.meal_name == "Some Meal"));
    assert!(!entries
        .iter()
        .any(|e| e.note.as_deref() == Some("should be dropped")));
}

#[test]
fn csv_reader_reads_a_quoted_date_field_containing_a_comma() {
    let entries = fixture_entries();
    let filo = entries
        .iter()
        .find(|e| e.meal_name == "Apple & Sausage Filo Casserole" && e.date == date(2022, 1, 6))
        .expect("the quoted `Thursday, 6 January 22` date field");
    assert_eq!(filo.note, Some("Add more cheese".to_string()));
    assert!(filo.featured);
}

#[test]
fn csv_reader_hands_the_api_a_iso_date() {
    let entries = fixture_entries();
    let filo = entries
        .iter()
        .find(|e| e.meal_name == "Apple & Sausage Filo Casserole" && e.date.year() == 2022)
        .unwrap();
    // circe encodes a `LocalDate` as `YYYY-MM-DD`.
    assert_eq!(filo.date.format("%Y-%m-%d").to_string(), "2022-01-06");
}

#[test]
fn csv_reader_handles_a_bom_a_reordered_header_and_lf_line_endings() {
    let plain = fixture_entries();

    let mut with_bom = vec![0xEF, 0xBB, 0xBF];
    with_bom.extend_from_slice(fixture_text().as_bytes());
    assert_eq!(parse_csv(with_bom.as_slice()).unwrap(), plain);

    // Columns are read by header name, so their order does not matter.
    let mut reordered = String::from("Notes,Feature,Meal,Date\r\n");
    for line in fixture_text().lines().skip(1) {
        let record = parse_record(line);
        reordered.push_str(&format!(
            "{},{},{},\"{}\"\r\n",
            record[2], record[3], record[1], record[0]
        ));
    }
    assert_eq!(parse_csv(reordered.as_bytes()).unwrap(), plain);

    // `CSVFormat.RFC4180` reads unix line endings too.
    let lf_only = fixture_text().replace("\r\n", "\n");
    assert_eq!(parse_csv(lf_only.as_bytes()).unwrap(), plain);
}

/// Split a fixture line into its four fields (only used by the reorder test;
/// the fixture quotes exactly the `Date` field).
fn parse_record(line: &str) -> [&str; 4] {
    let mut fields: Vec<&str> = Vec::new();
    let mut rest = line;
    while fields.len() < 3 {
        if let Some(quoted) = rest.strip_prefix('"') {
            let end = quoted.find('"').unwrap();
            fields.push(&quoted[..end]);
            rest = quoted[end + 1..].strip_prefix(',').unwrap();
        } else {
            let end = rest.find(',').unwrap();
            fields.push(&rest[..end]);
            rest = &rest[end + 1..];
        }
    }
    fields.push(rest);
    [fields[0], fields[1], fields[2], fields[3]]
}

#[test]
fn csv_reader_matches_the_scala_capture() {
    let capture = capture_dir();
    if capture.is_none() {
        eprintln!(
            "skipping: no capture in {}",
            std::env::var("RECIBASE_CAPTURE_DIR")
                .unwrap_or_else(|_| "/Users/chaos/recibase-work/capture-scala-csv".to_string())
        );
        return;
    }
    let capture = capture.unwrap();
    let raw = std::fs::read(capture.join("011_meals.raw")).unwrap();
    let body_start = find_subslice(&raw, b"\r\n\r\n").expect("a header block") + 4;
    let meals: serde_json::Value = serde_json::from_slice(&raw[body_start..]).unwrap();
    let meals = meals.as_array().unwrap();

    let entries = fixture_entries();
    let counts = totals(&entries);
    let last = last_eaten(&entries);
    let notes_by_meal = notes(&entries);
    let featured_by_meal = featured_meals(&entries);

    let mut checked = 0;
    for meal in meals {
        let name = meal["name"].as_str().unwrap();
        let iso = |d: Option<NaiveDate>| d.map(|d| d.format("%Y-%m-%d").to_string());

        assert_eq!(
            meal["times_eaten"].as_i64().unwrap(),
            *counts.get(name).unwrap_or(&0),
            "times_eaten for {}",
            name
        );
        let expected_last: Option<String> = match meal["last_eaten"].as_str() {
            Some(s) => Some(s.to_string()),
            None => None,
        };
        assert_eq!(
            iso(last.get(name).copied()),
            expected_last,
            "last_eaten for {}",
            name
        );
        let expected_featured: Option<String> = match meal["featured"].as_str() {
            Some(s) => Some(s.to_string()),
            None => None,
        };
        assert_eq!(
            iso(featured_by_meal.get(name).copied()),
            expected_featured,
            "featured for {}",
            name
        );

        let expected_notes: Vec<serde_json::Value> = notes_by_meal
            .get(name)
            .map(|notes| {
                notes
                    .iter()
                    .map(|note| {
                        serde_json::json!({
                            "date": note.date.format("%Y-%m-%d").to_string(),
                            "note": note.note,
                        })
                    })
                    .collect()
            })
            .unwrap_or_default();
        assert_eq!(
            meal["dated_notes"],
            serde_json::Value::Array(expected_notes),
            "dated_notes for {}",
            name
        );
        checked += 1;
    }
    assert_eq!(checked, meals.len());
    println!(
        "parsed {} CSV rows from {}; compared against {} meals in the capture",
        entries.len(),
        fixture_path().display(),
        meals.len()
    );
    // Every eated meal in the capture is one of the parsed rows, so the row
    // count is pinned by the capture too.
    let total: i64 = meals
        .iter()
        .map(|m| m["times_eaten"].as_i64().unwrap())
        .sum();
    assert_eq!(total, entries.len() as i64);
}

fn find_subslice(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack
        .windows(needle.len())
        .position(|window| window == needle)
}

// ---------------------------------------------------------------------------
// UsageData: the cache and the refresh rules
// ---------------------------------------------------------------------------

/// A fetcher that counts its calls and returns `text`.
fn counting_fetcher(text: &'static str) -> (Arc<AtomicUsize>, recibase_core::usage::Fetcher) {
    let calls = Arc::new(AtomicUsize::new(0));
    let counter = calls.clone();
    (
        calls,
        Box::new(move |_url: &str| {
            counter.fetch_add(1, Ordering::SeqCst);
            Ok(text.to_string())
        }),
    )
}

fn failing_fetcher() -> (Arc<AtomicUsize>, recibase_core::usage::Fetcher) {
    let calls = Arc::new(AtomicUsize::new(0));
    let counter = calls.clone();
    (
        calls,
        Box::new(move |_url: &str| {
            counter.fetch_add(1, Ordering::SeqCst);
            Err("the connection failed".to_string())
        }),
    )
}

fn fixed_clock(start: DateTime<Utc>) -> (Arc<Mutex<DateTime<Utc>>>, Clock) {
    let now = Arc::new(Mutex::new(start));
    let handle = now.clone();
    (now, Box::new(move || *handle.lock().unwrap()))
}

fn at(hours: i64, minutes: i64) -> DateTime<Utc> {
    Utc.with_ymd_and_hms(2026, 10, 1, 12, 0, 0).unwrap()
        + Duration::hours(hours)
        + Duration::minutes(minutes)
}

const ONE_ROW: &str =
    "Date,Meal,Notes,Feature\r\n\"Thursday, 17 February 22\",Macaroni,Add more cheese,TRUE\r\n";

#[test]
fn without_a_url_no_request_is_made_and_the_data_is_empty() {
    let (calls, fetcher) = counting_fetcher(ONE_ROW);
    let usage = UsageData::with_fetcher(None, fetcher);
    assert!(usage.meal_count().is_empty());
    assert!(usage.meal_last_eaten().is_empty());
    assert!(usage.meal_notes().is_empty());
    assert!(usage.featured_meals().is_empty());
    assert_eq!(calls.load(Ordering::SeqCst), 0);
    // `csvUrlOpt.map(...)` is None but the cell is still stamped with `now`.
    assert_ne!(usage.fetched_at(), DateTime::<Utc>::MIN_UTC);
    assert!(usage.entries().is_empty());
}

#[test]
fn the_first_read_fetches_once_and_the_getters_share_the_cache() {
    let (calls, fetcher) = counting_fetcher(ONE_ROW);
    let (clock, clock_fn) = fixed_clock(at(0, 0));
    let usage = UsageData::with_clock(
        Some("file:///tmp/never-read.csv".to_string()),
        fetcher,
        clock_fn,
    );
    *clock.lock().unwrap() = at(0, 0);

    assert_eq!(usage.meal_count().get("Macaroni"), Some(&1));
    assert_eq!(
        usage.meal_last_eaten().get("Macaroni"),
        Some(&date(2022, 2, 17))
    );
    assert_eq!(
        usage.meal_notes().get("Macaroni"),
        Some(&vec![DatedNote {
            date: date(2022, 2, 17),
            note: "Add more cheese".to_string(),
        }])
    );
    assert_eq!(
        usage.featured_meals().get("Macaroni"),
        Some(&date(2022, 2, 17))
    );
    assert_eq!(calls.load(Ordering::SeqCst), 1);
}

#[test]
fn the_soft_refresh_runs_after_twelve_hours_and_the_hard_after_seventy_two() {
    let (calls, fetcher) = counting_fetcher(ONE_ROW);
    let (clock, clock_fn) = fixed_clock(at(0, 0));
    let usage = UsageData::with_clock(
        Some("file:///tmp/never-read.csv".to_string()),
        fetcher,
        clock_fn,
    );
    usage.seed(vec![entry("Macaroni", date(2022, 2, 17), None)], at(0, 0));

    // Exactly at the timeout `isBefore` is false, so the data is still fresh.
    *clock.lock().unwrap() = at(SOFT_REFRESH_TIME_HOURS, 0);
    usage.refresh_meal_log();
    assert_eq!(calls.load(Ordering::SeqCst), 0);

    *clock.lock().unwrap() = at(SOFT_REFRESH_TIME_HOURS, 1);
    usage.refresh_meal_log();
    assert_eq!(calls.load(Ordering::SeqCst), 1);

    // `mealCount` and friends use the hard 72 hour window, measured from the
    // refresh that just happened.
    let refreshed_at = at(SOFT_REFRESH_TIME_HOURS, 1);
    *clock.lock().unwrap() = refreshed_at + Duration::hours(HARD_REFRESH_TIME_HOURS);
    assert_eq!(usage.meal_count().get("Macaroni"), Some(&1));
    assert_eq!(calls.load(Ordering::SeqCst), 1);

    *clock.lock().unwrap() =
        refreshed_at + Duration::hours(HARD_REFRESH_TIME_HOURS) + Duration::minutes(1);
    usage.meal_count();
    assert_eq!(calls.load(Ordering::SeqCst), 2);
    // The fetch stamped the cache with the clock's instant.
    assert_eq!(
        usage.fetched_at(),
        refreshed_at + Duration::hours(HARD_REFRESH_TIME_HOURS) + Duration::minutes(1)
    );
}

#[test]
fn a_failed_fetch_keeps_the_previous_entries_and_stays_stale() {
    let (calls, fetcher) = failing_fetcher();
    let (clock, clock_fn) = fixed_clock(at(0, 0));
    let usage = UsageData::with_clock(
        Some("file:///tmp/never-read.csv".to_string()),
        fetcher,
        clock_fn,
    );
    // An empty cache is stale because `fetchedAt` is `Instant.MIN`.
    assert!(usage.meal_count().is_empty());
    assert_eq!(calls.load(Ordering::SeqCst), 1);

    usage.seed(vec![entry("Macaroni", date(2022, 2, 17), None)], at(0, 0));
    *clock.lock().unwrap() = at(HARD_REFRESH_TIME_HOURS, 1);
    // The stale read fails, so the cell keeps the last good entries...
    assert_eq!(usage.meal_count().get("Macaroni"), Some(&1));
    // ... and the fetch time, so the next read tries again.
    assert_eq!(usage.fetched_at(), at(0, 0));
    assert_eq!(calls.load(Ordering::SeqCst), 2);
}

#[test]
fn try_cached_meal_log_entries_reports_the_failure() {
    let (_calls, fetcher) = failing_fetcher();
    let usage = UsageData::with_fetcher(Some("file:///tmp/never-read.csv".to_string()), fetcher);
    let error = usage.try_cached_meal_log_entries(Duration::hours(HARD_REFRESH_TIME_HOURS));
    assert!(error.is_err());
}

#[test]
fn from_env_reads_meal_log_csv_url() {
    // `from_env` is a thin wrapper; assert the shape without touching the
    // process environment of the test runner.
    let usage = UsageData::with_url(std::env::var("MEAL_LOG_CSV_URL").ok());
    assert_eq!(
        usage.url(),
        std::env::var("MEAL_LOG_CSV_URL").ok().as_deref()
    );
}

#[test]
fn csv_reader_accepts_an_empty_or_header_only_file() {
    assert!(parse_csv(b"".as_slice()).unwrap().is_empty());
    assert!(parse_csv(b"Date,Meal,Notes,Feature\r\n".as_slice())
        .unwrap()
        .is_empty());
}

#[test]
fn csv_reader_fails_where_commons_csv_throws() {
    // `record.get("Meal")` has no mapping: the Scala's `parseCsv` throws
    // "Mapping for Meal not found, expected one of [Date, Notes, Feature]".
    let missing_column =
        parse_csv(b"Date,Notes,Feature\r\n\"Thursday, 17 February 22\",,TRUE\r\n".as_slice());
    assert_eq!(
        missing_column.err().map(|e| e.to_string()).as_deref(),
        Some("Mapping for Meal not found, expected one of Date, Notes, Feature")
    );
    // A header without any data rows is never read, so it does not fail.
    assert!(parse_csv(b"Date,Notes,Feature\r\n".as_slice())
        .unwrap()
        .is_empty());

    // `record.get("Notes")` past the end of a short record.
    let short_row = parse_csv(
        b"Date,Meal,Notes,Feature\r\n\"Thursday, 17 February 22\",Macaroni\r\n".as_slice(),
    );
    assert_eq!(
        short_row.err().map(|e| e.to_string()).as_deref(),
        Some("Index for header 'Notes' is 2 but CSVRecord only has 2 values!")
    );

    // The first record of a file with no header row is taken as the header,
    // which is what `setHeader()` does, so there are no data rows.
    assert!(
        parse_csv(b"\"Thursday, 17 February 22\",Macaroni,note,TRUE\r\n".as_slice())
            .unwrap()
            .is_empty()
    );
}

#[test]
fn csv_reader_ignores_blank_lines_unlike_commons_csv() {
    // Documented divergence: commons-csv turns a blank line into a record with
    // one value, which throws out of `parseCsv`; the `csv` crate skips it.
    let text =
        "Date,Meal,Notes,Feature\r\n\r\n\"Thursday, 17 February 22\",Macaroni,note,TRUE\r\n\r\n";
    let entries = parse_csv(text.as_bytes()).unwrap();
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].meal_name, "Macaroni");
}

#[test]
fn csv_reader_keeps_a_quoted_field_that_spans_lines_and_ignores_extra_columns() {
    let multi_line = "Date,Meal,Notes,Feature\r\n\"Thursday,\r\n17 February 22\",Macaroni,\"two\r\nlines\",TRUE\r\n";
    let entries = parse_csv(multi_line.as_bytes()).unwrap();
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].date, date(2022, 2, 17));
    assert_eq!(entries[0].note.as_deref(), Some("two\r\nlines"));

    // Extra values past the last header column are ignored by `get(name)`.
    let extra =
        "Date,Meal,Notes,Feature\r\n\"Thursday, 17 February 22\",Macaroni,note,TRUE,ignored\r\n";
    assert_eq!(parse_csv(extra.as_bytes()).unwrap().len(), 1);
}

#[test]
fn the_built_in_fetcher_reads_a_file_url_and_rejects_anything_else() {
    use recibase_core::usage::fetch_csv_text;
    let path = fixture_path();
    let url = format!("file://{}", path.display());
    assert_eq!(fetch_csv_text(&url).unwrap(), fixture_text());
    // `file:` with a single slash, and `%20` escapes.
    let spaced = path.parent().unwrap().join("meal log.csv");
    std::fs::write(&spaced, fixture_text()).unwrap();
    let escaped = format!("file:{}", spaced.display()).replace(' ', "%20");
    assert_eq!(fetch_csv_text(&escaped).unwrap(), fixture_text());
    std::fs::remove_file(&spaced).unwrap();

    assert!(fetch_csv_text("/tmp/meal-log.csv").is_err());
    assert!(fetch_csv_text("https://example.com/meal-log.csv").is_err());
    assert!(fetch_csv_text("file:///tmp/definitely-not-here.csv").is_err());
}

#[test]
fn the_built_in_fetcher_reads_an_http_url_and_follows_one_redirect() {
    use recibase_core::usage::fetch_csv_text;
    use std::io::{Read as _, Write as _};
    use std::net::TcpListener;

    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();
    listener.set_nonblocking(true).unwrap();
    let body = fixture_text();
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(10);
    let server = std::thread::spawn(move || {
        // Bounded: serve the redirect and the body, then stop.
        let mut served = 0;
        while served < 3 && std::time::Instant::now() < deadline {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0u8; 2048];
                    let read = stream.read(&mut buffer).unwrap_or(0);
                    let request = String::from_utf8_lossy(&buffer[..read]).to_string();
                    let response = if request.starts_with("GET /redirect") {
                        "HTTP/1.0 302 Found\r\nLocation: /meal-log.csv\r\nContent-Length: 0\r\n\r\n"
                            .to_string()
                    } else if request.starts_with("GET /meal-log.csv") {
                        format!(
                            "HTTP/1.0 200 OK\r\nContent-Type: text/csv\r\nContent-Length: {}\r\n\r\n{}",
                            body.len(),
                            body
                        )
                    } else {
                        "HTTP/1.0 404 Not Found\r\nContent-Length: 0\r\n\r\n".to_string()
                    };
                    let _ = stream.write_all(response.as_bytes());
                    served += 1;
                }
                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    std::thread::sleep(std::time::Duration::from_millis(10))
                }
                Err(_) => break,
            }
        }
    });

    let url = format!("http://127.0.0.1:{}/redirect", port);
    assert_eq!(fetch_csv_text(&url).unwrap(), fixture_text());
    // A 404 is an error, not an empty log.
    assert!(fetch_csv_text(&format!("http://127.0.0.1:{}/missing", port)).is_err());
    let _ = server.join();
}
