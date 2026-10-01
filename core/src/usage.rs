//! `se.reciba.api.usage.UsageData` and the meal-log CSV.
//!
//! OWNER: workstream W6.

use crate::meal::DatedNote;
use chrono::NaiveDate;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MealLogEntry {
    pub meal_name: String,
    pub date: NaiveDate,
    pub note: Option<String>,
    pub featured: bool,
}

impl MealLogEntry {
    pub fn new(meal_name: &str, date: NaiveDate, note: Option<String>, featured: bool) -> Self {
        MealLogEntry { meal_name: meal_name.to_string(), date, note, featured }
    }

    /// `MealLogEntry(mealName, rawDate, rawNote, rawFeature): Try[MealLogEntry]`.
    pub fn parse(
        meal_name: &str,
        raw_date: &str,
        raw_note: &str,
        raw_feature: &str,
    ) -> Result<MealLogEntry, String> {
        let date = parse_date(raw_date)?;
        let note = if raw_note.is_empty() { None } else { Some(raw_note.to_string()) };
        Ok(MealLogEntry {
            meal_name: meal_name.to_string(),
            date,
            note,
            featured: raw_feature == "TRUE",
        })
    }
}

/// `UsageData.parseDate`: the second comma-separated field as `d MMMM yy`.
pub fn parse_date(input: &str) -> Result<NaiveDate, String> {
    let raw = input
        .split(',')
        .nth(1)
        .ok_or_else(|| format!("no date in {:?}", input))?
        .trim();
    parse_day_month_year(raw).ok_or_else(|| format!("bad date {:?}", raw))
}

fn parse_day_month_year(raw: &str) -> Option<NaiveDate> {
    let mut parts = raw.split_whitespace();
    let day: u32 = parts.next()?.parse().ok()?;
    let month = month_number(parts.next()?)?;
    let year: i32 = parts.next()?.parse().ok()?;
    let year = if year < 100 { 2000 + year } else { year };
    NaiveDate::from_ymd_opt(year, month, day)
}

fn month_number(name: &str) -> Option<u32> {
    match name {
        "January" => Some(1),
        "February" => Some(2),
        "March" => Some(3),
        "April" => Some(4),
        "May" => Some(5),
        "June" => Some(6),
        "July" => Some(7),
        "August" => Some(8),
        "September" => Some(9),
        "October" => Some(10),
        "November" => Some(11),
        "December" => Some(12),
        _ => None,
    }
}

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
pub fn notes(entries: &[MealLogEntry]) -> HashMap<String, Vec<DatedNote>> {
    let mut out: HashMap<String, Vec<DatedNote>> = HashMap::new();
    for entry in entries {
        let bucket = out.entry(entry.meal_name.clone()).or_default();
        if let Some(note) = &entry.note {
            bucket.push(DatedNote { date: entry.date, note: note.clone() });
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

/// True when the CSV row should be dropped (blank meal name).
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
