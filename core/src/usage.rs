//! `se.reciba.api.usage.UsageData` and the meal-log CSV.
//!
//! OWNER: workstream W6.

use crate::meal::DatedNote;
use chrono::{DateTime, Duration, NaiveDate, SecondsFormat, Utc};
use std::collections::{HashMap, HashSet};
use std::io::{Read, Write};
use std::sync::Mutex;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MealLogEntry {
    pub meal_name: String,
    pub date: NaiveDate,
    pub note: Option<String>,
    pub featured: bool,
}

impl MealLogEntry {
    pub fn new(meal_name: &str, date: NaiveDate, note: Option<String>, featured: bool) -> Self {
        MealLogEntry {
            meal_name: meal_name.to_string(),
            date,
            note,
            featured,
        }
    }

    /// `MealLogEntry(mealName, rawDate, rawNote, rawFeature): Try[MealLogEntry]`.
    pub fn parse(
        meal_name: &str,
        raw_date: &str,
        raw_note: &str,
        raw_feature: &str,
    ) -> Result<MealLogEntry, String> {
        let date = parse_date(raw_date)?;
        let note = if raw_note.is_empty() {
            None
        } else {
            Some(raw_note.to_string())
        };
        Ok(MealLogEntry {
            meal_name: meal_name.to_string(),
            date,
            note,
            featured: raw_feature == "TRUE",
        })
    }
}

/// `UsageData.parseDate`: the second comma-separated field of the `Date` cell,
/// parsed with the `DateTimeFormatter` pattern `d MMMM yy`.
///
/// The Scala parses the *field value*, not the raw line, so the comma inside
/// `"Thursday, 6 January 22"` is consumed by the `split(",")`.
pub fn parse_date(input: &str) -> Result<NaiveDate, String> {
    let raw = input
        .split(',')
        .nth(1)
        .ok_or_else(|| format!("no date in {:?}", input))?
        .trim();
    parse_day_month_year(raw).ok_or_else(|| format!("bad date {:?}", raw))
}

/// `d MMMM yy` in `ResolverStyle.SMART`: `d` and `yy` accept at most two digits
/// (a longer number leaves unparsed text, which is a parse error), the month is
/// matched against the full English name or its three-letter form, and a
/// two-digit year is resolved against the base year 2000.
fn parse_day_month_year(raw: &str) -> Option<NaiveDate> {
    let mut parts = raw.split_whitespace();
    let day = number_field(parts.next()?, 2)?;
    let month = month_number(parts.next()?)?;
    let year = number_field(parts.next()?, 2)?;
    if parts.next().is_some() {
        return None;
    }
    NaiveDate::from_ymd_opt(2000 + year as i32, month, day)
}

/// A `DateTimeFormatter` number field of at most `max_width` digits.
fn number_field(token: &str, max_width: usize) -> Option<u32> {
    if token.is_empty() || token.len() > max_width {
        return None;
    }
    if !token.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    token.parse().ok()
}

/// Month number for a full or three-letter English month name. Java's text
/// parsing is case-insensitive and `SMART` mode also accepts a shorter text.
fn month_number(name: &str) -> Option<u32> {
    let lower = name.to_lowercase();
    for (index, full) in MONTHS.iter().enumerate() {
        if lower == *full || lower == full[..3].to_string() {
            return Some(index as u32 + 1);
        }
    }
    None
}

const MONTHS: [&str; 12] = [
    "january",
    "february",
    "march",
    "april",
    "may",
    "june",
    "july",
    "august",
    "september",
    "october",
    "november",
    "december",
];

/// `UsageData.lastEaten`.
pub fn last_eaten(entries: &[MealLogEntry]) -> HashMap<String, NaiveDate> {
    let mut out: HashMap<String, NaiveDate> = HashMap::new();
    for entry in entries {
        out.entry(entry.meal_name.clone())
            .and_modify(|d| {
                if entry.date > *d {
                    *d = entry.date;
                }
            })
            .or_insert(entry.date);
    }
    out
}

/// `UsageData.totals`.
pub fn totals(entries: &[MealLogEntry]) -> HashMap<String, i64> {
    let mut out: HashMap<String, i64> = HashMap::new();
    for entry in entries {
        *out.entry(entry.meal_name.clone()).or_insert(0) += 1;
    }
    out
}

/// `UsageData.notes`, sorted by date.
///
/// The Scala sorts a `List` built by walking a `Set`, so entries that share a
/// date keep the `Set` iteration order; this port keeps the CSV order for those
/// ties (the two orders can only differ when one meal has two notes on the same
/// date).
pub fn notes(entries: &[MealLogEntry]) -> HashMap<String, Vec<DatedNote>> {
    let mut out: HashMap<String, Vec<DatedNote>> = HashMap::new();
    for entry in entries {
        let bucket = out.entry(entry.meal_name.clone()).or_default();
        if let Some(note) = &entry.note {
            bucket.push(DatedNote {
                date: entry.date,
                note: note.clone(),
            });
        }
    }
    for bucket in out.values_mut() {
        bucket.sort_by_key(|n| n.date);
    }
    out
}

/// `UsageData.featuredMeals`.
pub fn featured_meals(entries: &[MealLogEntry]) -> HashMap<String, NaiveDate> {
    let mut out: HashMap<String, NaiveDate> = HashMap::new();
    for entry in entries.iter().filter(|e| e.featured) {
        out.entry(entry.meal_name.clone())
            .and_modify(|d| {
                if entry.date > *d {
                    *d = entry.date;
                }
            })
            .or_insert(entry.date);
    }
    out
}

/// True when the CSV row should be dropped (blank meal name):
/// `case (_, "", _, _) => None`.
pub fn skip_row(meal_name: &str) -> bool {
    meal_name.is_empty()
}

/// `MealLogEntry` values from CSV records, dropping unparsable rows.
pub fn parse_csv_rows(rows: &[HashMap<String, String>]) -> Vec<MealLogEntry> {
    let mut out = Vec::new();
    for row in rows {
        let meal = row.get("Meal").cloned().unwrap_or_default();
        if skip_row(&meal) {
            continue;
        }
        let date = row.get("Date").cloned().unwrap_or_default();
        let note = row.get("Notes").cloned().unwrap_or_default();
        let feature = row.get("Feature").cloned().unwrap_or_default();
        if let Ok(entry) = MealLogEntry::parse(&meal, &date, &note, &feature) {
            out.push(entry);
        }
    }
    out
}

// ---------------------------------------------------------------------------
// CSV reading
// ---------------------------------------------------------------------------

/// A failure while reading the CSV stream. A row whose `Date` cell cannot be
/// parsed is not an error - it is dropped, as `toOption` does in the Scala.
#[derive(Debug)]
pub enum CsvError {
    Io(std::io::Error),
    Csv(csv::Error),
    /// commons-csv: "Mapping for %s not found, expected one of %s". The Scala
    /// reads `Date`, `Meal`, `Notes` and `Feature` by name, and a missing
    /// column throws out of `parseCsv`.
    MissingHeader {
        name: String,
        headers: Vec<String>,
    },
    /// commons-csv: "Index for header '%s' is %d but CSVRecord only has %d
    /// values!". A record with fewer values than a column it is read by name
    /// also throws out of `parseCsv`.
    ShortRow {
        name: String,
        index: usize,
        values: usize,
    },
}

impl std::fmt::Display for CsvError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CsvError::Io(e) => write!(f, "{}", e),
            CsvError::Csv(e) => write!(f, "{}", e),
            CsvError::MissingHeader { name, headers } => write!(
                f,
                "Mapping for {} not found, expected one of {}",
                name,
                headers.join(", ")
            ),
            CsvError::ShortRow {
                name,
                index,
                values,
            } => write!(
                f,
                "Index for header '{}' is {} but CSVRecord only has {} values!",
                name, index, values
            ),
        }
    }
}

impl std::error::Error for CsvError {}

impl From<std::io::Error> for CsvError {
    fn from(e: std::io::Error) -> Self {
        CsvError::Io(e)
    }
}

impl From<csv::Error> for CsvError {
    fn from(e: csv::Error) -> Self {
        CsvError::Csv(e)
    }
}

/// The header names the Scala reads with `record.get(...)`.
pub const CSV_HEADERS: [&str; 4] = ["Date", "Meal", "Notes", "Feature"];

/// `UsageData.parseCsv` over
/// `CSVFormat.RFC4180.builder().setHeader().setSkipHeaderRecord(true)`:
///
/// * the first record is the header and is not a data row;
/// * columns are looked up by header name, not by position;
/// * a row whose `Meal` cell is empty is dropped;
/// * a row whose `Date` cell does not parse as `d MMMM yy` is dropped
///   (`toOption`);
/// * the surviving entries go into a `Set`, so exact duplicates collapse.
///
/// A leading UTF-8 BOM is skipped so a BOM-prefixed header still exposes a
/// `Date` column. (Apache commons-csv 1.14.1 does *not* do this: the header of
/// a BOM-prefixed file is named `"\u{FEFF}Date"`, so the Scala fails the whole
/// request. The BOM is stripped here on purpose.)
///
/// One divergence that is left in place: commons-csv turns a blank line into a
/// one-value record whose `get("Meal")` throws, while the `csv` crate skips
/// blank lines. A meal log with blank lines therefore fails on the Scala and
/// parses here.
pub fn parse_csv<R: Read>(reader: R) -> Result<Vec<MealLogEntry>, CsvError> {
    let mut bytes = Vec::new();
    let mut reader = reader;
    reader.read_to_end(&mut bytes)?;
    let body: &[u8] = match bytes.strip_prefix(&[0xEF, 0xBB, 0xBF]) {
        Some(rest) => rest,
        None => &bytes,
    };

    let mut csv_reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .flexible(true)
        .from_reader(body);
    let headers = csv_reader.headers()?.clone();
    let column = |name: &str| -> Option<usize> { headers.iter().position(|h| h == name) };
    let (date_column, meal_column, notes_column, feature_column) = (
        column(CSV_HEADERS[0]),
        column(CSV_HEADERS[1]),
        column(CSV_HEADERS[2]),
        column(CSV_HEADERS[3]),
    );

    let mut out: Vec<MealLogEntry> = Vec::new();
    let mut seen: HashSet<(String, NaiveDate, Option<String>, bool)> = HashSet::new();
    for record in csv_reader.records() {
        let record = record?;
        // The Scala builds `(record.get("Date"), record.get("Meal"),
        // record.get("Notes"), Option(record.get("Feature")))` for every
        // record, before the blank-meal match, and `CSVRecord.get` throws for
        // a column the header does not declare and for a record that has fewer
        // values than that column's index. Both failures escape `parseCsv`.
        for (name, column) in [
            (CSV_HEADERS[0], date_column),
            (CSV_HEADERS[1], meal_column),
            (CSV_HEADERS[2], notes_column),
            (CSV_HEADERS[3], feature_column),
        ] {
            match column {
                None => {
                    return Err(CsvError::MissingHeader {
                        name: name.to_string(),
                        headers: headers.iter().map(|h| h.to_string()).collect(),
                    })
                }
                Some(index) if index >= record.len() => {
                    return Err(CsvError::ShortRow {
                        name: name.to_string(),
                        index,
                        values: record.len(),
                    })
                }
                Some(_) => {}
            }
        }
        let field =
            |index: Option<usize>| -> &str { index.and_then(|i| record.get(i)).unwrap_or("") };
        let meal_name = field(meal_column);
        if skip_row(meal_name) {
            continue;
        }
        let entry = MealLogEntry::parse(
            meal_name,
            field(date_column),
            field(notes_column),
            field(feature_column),
        );
        if let Ok(entry) = entry {
            let key = (
                entry.meal_name.clone(),
                entry.date,
                entry.note.clone(),
                entry.featured,
            );
            if seen.insert(key) {
                out.push(entry);
            }
        }
    }
    Ok(out)
}

/// `parse_csv` over CSV text that has already been read.
pub fn parse_csv_text(text: &str) -> Result<Vec<MealLogEntry>, CsvError> {
    parse_csv(text.as_bytes())
}

// ---------------------------------------------------------------------------
// Loading `MEAL_LOG_CSV_URL`
// ---------------------------------------------------------------------------

/// The Scala's `scala.io.Source.fromURL(url).bufferedReader()`.
///
/// `file:` and `http:` URLs are handled here; `https:` needs a TLS client, so
/// pass a fetcher of your own to [`UsageData::with_fetcher`] instead.
pub fn fetch_csv_text(url: &str) -> Result<String, String> {
    if let Some(rest) = url.strip_prefix("file:") {
        let path = file_url_path(rest);
        let bytes = std::fs::read(&path).map_err(|e| format!("{}: {}", url, e))?;
        // `Source.fromURL` decodes with the default charset and replaces
        // malformed input, like `String::from_utf8_lossy`.
        return Ok(String::from_utf8_lossy(&bytes).into_owned());
    }
    if url.starts_with("http://") {
        let bytes = http_get(url)?;
        return Ok(String::from_utf8_lossy(&bytes).into_owned());
    }
    if url.starts_with("https://") {
        return Err(format!(
            "{}: https is not supported by the built-in fetcher, pass a custom \
             fetcher to UsageData::with_fetcher",
            url
        ));
    }
    // `new java.net.URL(url)`: "no protocol".
    Err(format!("{}: no protocol", url))
}

/// The filesystem path of a `file:` URL: `file:/a`, `file:///a` and
/// `file://host/a` all name `/a`; `%XX` escapes are decoded.
fn file_url_path(rest: &str) -> String {
    let after_authority = match rest.strip_prefix("//") {
        Some(authority_and_path) => match authority_and_path.find('/') {
            Some(index) => &authority_and_path[index..],
            None => "/",
        },
        None => rest,
    };
    percent_decode(after_authority)
}

fn percent_decode(input: &str) -> String {
    let bytes = input.as_bytes();
    let mut out: Vec<u8> = Vec::with_capacity(bytes.len());
    let mut index = 0;
    while index < bytes.len() {
        if bytes[index] == b'%' && index + 2 < bytes.len() {
            let hex = std::str::from_utf8(&bytes[index + 1..index + 3]).ok();
            if let Some(value) = hex.and_then(|h| u8::from_str_radix(h, 16).ok()) {
                out.push(value);
                index += 3;
                continue;
            }
        }
        out.push(bytes[index]);
        index += 1;
    }
    String::from_utf8_lossy(&out).into_owned()
}

/// Host, port and request target of an `http://` URL.
fn split_http_url(url: &str) -> Result<(String, u16, String), String> {
    let rest = url
        .strip_prefix("http://")
        .ok_or_else(|| format!("{}: not an http URL", url))?;
    let (authority, target) = match rest.find('/') {
        Some(index) => (&rest[..index], &rest[index..]),
        None => (rest, "/"),
    };
    let (host, port) = match authority.rsplit_once(':') {
        Some((host, port)) => (
            host.to_string(),
            port.parse().map_err(|_| format!("{}: bad port", url))?,
        ),
        None => (authority.to_string(), 80),
    };
    if host.is_empty() {
        return Err(format!("{}: no host", url));
    }
    Ok((host, port, target.to_string()))
}

/// A blocking `GET`, following up to five redirects.
fn http_get(url: &str) -> Result<Vec<u8>, String> {
    let mut current = url.to_string();
    for _ in 0..6 {
        let (host, port, target) = split_http_url(&current)?;
        let host_header = if port == 80 {
            host.clone()
        } else {
            format!("{}:{}", host, port)
        };
        let mut stream = std::net::TcpStream::connect((host.as_str(), port))
            .map_err(|e| format!("{}: {}", current, e))?;
        let timeout = Some(std::time::Duration::from_secs(30));
        stream
            .set_read_timeout(timeout)
            .map_err(|e| e.to_string())?;
        stream
            .set_write_timeout(timeout)
            .map_err(|e| e.to_string())?;
        // HTTP/1.0 keeps the body close-delimited, so no chunked framing from
        // a conforming server.
        let request = format!(
            "GET {} HTTP/1.0\r\nHost: {}\r\nConnection: close\r\n\r\n",
            target, host_header
        );
        stream
            .write_all(request.as_bytes())
            .map_err(|e| format!("{}: {}", current, e))?;
        stream.flush().map_err(|e| format!("{}: {}", current, e))?;
        let mut raw = Vec::new();
        stream
            .read_to_end(&mut raw)
            .map_err(|e| format!("{}: {}", current, e))?;

        let head_end = find_subslice(&raw, b"\r\n\r\n")
            .or_else(|| find_subslice(&raw, b"\n\n"))
            .map(|index| {
                if raw[index] == b'\r' {
                    index + 4
                } else {
                    index + 2
                }
            })
            .ok_or_else(|| format!("{}: no HTTP header in the response", current))?;
        let head = String::from_utf8_lossy(&raw[..head_end]).into_owned();
        let mut lines = head.lines();
        let status_line = lines.next().unwrap_or("");
        let status: u16 = status_line
            .split_whitespace()
            .nth(1)
            .and_then(|code| code.parse().ok())
            .ok_or_else(|| format!("{}: bad status line {:?}", current, status_line))?;
        let mut headers: HashMap<String, String> = HashMap::new();
        for line in lines {
            if let Some((name, value)) = line.split_once(':') {
                headers
                    .entry(name.trim().to_lowercase())
                    .or_insert_with(|| value.trim().to_string());
            }
        }
        let chunked = headers
            .get("transfer-encoding")
            .map(|value| value.to_lowercase().contains("chunked"))
            .unwrap_or(false);
        let body = if chunked {
            dechunk(&raw[head_end..]).map_err(|e| format!("{}: {}", current, e))?
        } else {
            raw[head_end..].to_vec()
        };

        if matches!(status, 301 | 302 | 303 | 307 | 308) {
            let location = headers
                .get("location")
                .ok_or_else(|| format!("{}: HTTP {} without Location", current, status))?;
            current = resolve_redirect(&current, location)?;
            continue;
        }
        if !(200..300).contains(&status) {
            return Err(format!("{}: HTTP {}", current, status));
        }
        return Ok(body);
    }
    Err(format!("{}: too many redirects", url))
}

fn resolve_redirect(current: &str, location: &str) -> Result<String, String> {
    if location.starts_with("http://") || location.starts_with("https://") {
        return Ok(location.to_string());
    }
    let rest = current
        .strip_prefix("http://")
        .ok_or_else(|| format!("{}: bad base URL", current))?;
    let authority = match rest.find('/') {
        Some(index) => &rest[..index],
        None => rest,
    };
    if let Some(path) = location.strip_prefix('/') {
        return Ok(format!("http://{}/{}", authority, path));
    }
    let base_path = match rest.find('/') {
        Some(index) => &rest[index + 1..],
        None => "",
    };
    let directory = match base_path.rfind('/') {
        Some(index) => &base_path[..index + 1],
        None => "",
    };
    Ok(format!(
        "http://{}/{}",
        authority,
        format!("{}{}", directory, location)
    ))
}

fn dechunk(body: &[u8]) -> Result<Vec<u8>, String> {
    let mut out = Vec::new();
    let mut index = 0;
    loop {
        let line_end = find_subslice(&body[index..], b"\r\n")
            .ok_or_else(|| "truncated chunk header".to_string())?;
        let size_text = String::from_utf8_lossy(&body[index..index + line_end]).into_owned();
        let size_text = size_text.split(';').next().unwrap_or("").trim().to_string();
        let size = usize::from_str_radix(&size_text, 16)
            .map_err(|_| format!("bad chunk size {:?}", size_text))?;
        index += line_end + 2;
        if size == 0 {
            return Ok(out);
        }
        if index + size > body.len() {
            return Err("truncated chunk".to_string());
        }
        out.extend_from_slice(&body[index..index + size]);
        index += size + 2;
    }
}

fn find_subslice(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack
        .windows(needle.len())
        .position(|window| window == needle)
}

// ---------------------------------------------------------------------------
// UsageData
// ---------------------------------------------------------------------------

/// `case class FetchedMealLogEntries(entries: Set[MealLogEntry], fetchedAt: Instant)`.
#[derive(Debug, Clone)]
pub struct FetchedMealLogEntries {
    pub entries: Vec<MealLogEntry>,
    pub fetched_at: DateTime<Utc>,
}

impl FetchedMealLogEntries {
    /// `FetchedMealLogEntries.empty`: `Instant.MIN`, i.e. always stale.
    pub fn empty() -> Self {
        FetchedMealLogEntries {
            entries: Vec::new(),
            fetched_at: DateTime::<Utc>::MIN_UTC,
        }
    }
}

/// `UsageData.softRefreshTime`.
pub const SOFT_REFRESH_TIME_HOURS: i64 = 12;
/// `UsageData.hardRefreshTime`.
pub const HARD_REFRESH_TIME_HOURS: i64 = 72;

/// Reads the CSV text a `MEAL_LOG_CSV_URL` points at.
pub type Fetcher = Box<dyn Fn(&str) -> Result<String, String> + Send + Sync>;
/// The current instant, injectable so the refresh rules can be tested.
pub type Clock = Box<dyn Fn() -> DateTime<Utc> + Send + Sync>;

/// `class UsageData`, with the `cats.effect.std.AtomicCell` replaced by a
/// `Mutex` (the Scala's `evalUpdateAndGet` holds the cell for the whole
/// update, so the two serialise callers the same way).
pub struct UsageData {
    url: Option<String>,
    fetcher: Fetcher,
    clock: Clock,
    soft_refresh_time: Duration,
    hard_refresh_time: Duration,
    cell: Mutex<FetchedMealLogEntries>,
}

impl UsageData {
    /// `UsageData.apply`, with `csvUrlOpt = sys.env.get("MEAL_LOG_CSV_URL")`.
    pub fn from_env() -> UsageData {
        UsageData::with_url(std::env::var("MEAL_LOG_CSV_URL").ok())
    }

    /// `UsageData` for one URL (`None` = the environment variable is unset).
    pub fn with_url(url: Option<String>) -> UsageData {
        UsageData::with_fetcher(url, Box::new(fetch_csv_text))
    }

    pub fn with_fetcher(url: Option<String>, fetcher: Fetcher) -> UsageData {
        UsageData::with_clock(url, fetcher, Box::new(|| Utc::now()))
    }

    pub fn with_clock(url: Option<String>, fetcher: Fetcher, clock: Clock) -> UsageData {
        UsageData {
            url,
            fetcher,
            clock,
            soft_refresh_time: Duration::hours(SOFT_REFRESH_TIME_HOURS),
            hard_refresh_time: Duration::hours(HARD_REFRESH_TIME_HOURS),
            cell: Mutex::new(FetchedMealLogEntries::empty()),
        }
    }

    pub fn url(&self) -> Option<&str> {
        self.url.as_deref()
    }

    fn lock(&self) -> std::sync::MutexGuard<'_, FetchedMealLogEntries> {
        // A panic in another thread must not poison the cache for everyone
        // else, which is what `AtomicCell` would be indifferent to.
        self.cell
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
    }

    /// `cachedMealLogEntries`. A failed fetch leaves the cache untouched, as a
    /// failed `evalUpdateAndGet` leaves the `AtomicCell` untouched.
    pub fn try_cached_meal_log_entries(
        &self,
        timeout: Duration,
    ) -> Result<Vec<MealLogEntry>, String> {
        let mut cell = self.lock();
        let now = (self.clock)();
        let data_is_stale = cell.fetched_at < now - timeout;
        if data_is_stale {
            println!(
                "Refreshing stale usage data: {} vs now {}, exceeds {} hour timeout",
                java_instant(cell.fetched_at),
                java_instant(now),
                timeout.num_hours()
            );
            match &self.url {
                // `csvUrlOpt.map(...)`: no URL, no request, empty set.
                None => {
                    *cell = FetchedMealLogEntries {
                        entries: Vec::new(),
                        fetched_at: now,
                    };
                }
                Some(url) => {
                    let text = (self.fetcher)(url)?;
                    let entries = parse_csv_text(&text)
                        .map_err(|e| format!("Failed to parse meal log CSV {}: {}", url, e))?;
                    *cell = FetchedMealLogEntries {
                        entries,
                        fetched_at: now,
                    };
                }
            }
        }
        Ok(cell.entries.clone())
    }

    /// `cachedMealLogEntries`, reporting a failed fetch on stderr and serving
    /// the last good entries.
    pub fn cached_meal_log_entries(&self, timeout: Duration) -> Vec<MealLogEntry> {
        match self.try_cached_meal_log_entries(timeout) {
            Ok(entries) => entries,
            Err(message) => {
                eprintln!("{}", message);
                self.lock().entries.clone()
            }
        }
    }

    /// `refreshMealLog`.
    pub fn refresh_meal_log(&self) {
        let _ = self.cached_meal_log_entries(self.soft_refresh_time);
    }

    /// `mealCount`.
    pub fn meal_count(&self) -> HashMap<String, i64> {
        totals(&self.cached_meal_log_entries(self.hard_refresh_time))
    }

    /// `mealLastEaten`.
    pub fn meal_last_eaten(&self) -> HashMap<String, NaiveDate> {
        last_eaten(&self.cached_meal_log_entries(self.hard_refresh_time))
    }

    /// `mealNotes`.
    pub fn meal_notes(&self) -> HashMap<String, Vec<DatedNote>> {
        notes(&self.cached_meal_log_entries(self.hard_refresh_time))
    }

    /// `featuredMeals`.
    pub fn featured_meals(&self) -> HashMap<String, NaiveDate> {
        featured_meals(&self.cached_meal_log_entries(self.hard_refresh_time))
    }

    /// The entries currently in the cache.
    pub fn entries(&self) -> Vec<MealLogEntry> {
        self.lock().entries.clone()
    }

    /// The fetch time currently in the cache.
    pub fn fetched_at(&self) -> DateTime<Utc> {
        self.lock().fetched_at
    }

    /// Put entries into the cache as if they had been fetched at `fetched_at`.
    pub fn seed(&self, entries: Vec<MealLogEntry>, fetched_at: DateTime<Utc>) {
        *self.lock() = FetchedMealLogEntries {
            entries,
            fetched_at,
        };
    }
}

/// `Instant.toString`, which is what the Scala prints when the data is stale.
pub fn java_instant(instant: DateTime<Utc>) -> String {
    instant.to_rfc3339_opts(SecondsFormat::AutoSi, true)
}
