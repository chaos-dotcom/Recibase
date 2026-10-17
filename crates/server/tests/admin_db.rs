//! The tracker's Postgres round trip.
//!
//! Gated on `DATABASE_URL`, so `cargo test` skips it unless a database is
//! provided - point it at any scratch database, since the test applies the
//! migrations and writes under its own subject.

mod support;

use chrono::{Duration, NaiveDate, Utc};
use recibase_server::Context;
use recibase_server::db::{Db, NewMealEntry};
use support::request;

fn database_url() -> Option<String> {
    std::env::var("DATABASE_URL")
        .ok()
        .filter(|url| !url.trim().is_empty())
}

#[test]
fn entries_sessions_and_users_round_trip() {
    let Some(url) = database_url() else {
        eprintln!("skipping: DATABASE_URL is unset");
        return;
    };
    let db = Db::connect(&url).expect("connect and migrate");

    let subject = format!("test-subject-{}", std::process::id());
    let user = db
        .upsert_user(&subject, Some("owner@example.com"))
        .expect("upsert user");
    let again = db
        .upsert_user(&subject, Some("owner@example.com"))
        .expect("upsert is idempotent");
    assert_eq!(again.id, user.id);

    let now = Utc::now();
    let session_id = db
        .create_session(user.id, "csrf-token", now + Duration::days(1))
        .expect("create session");
    assert!(
        db.session(&session_id, now).expect("lookup").is_some(),
        "a fresh session is valid"
    );
    assert!(
        db.session(&session_id, now + Duration::days(2))
            .expect("lookup")
            .is_none(),
        "an expired session is not"
    );

    let entry = db
        .add_entry(&NewMealEntry {
            user_id: user.id,
            meal_name: "Carbonara",
            eaten_on: NaiveDate::from_ymd_opt(2024, 1, 2).unwrap(),
            note: Some("tasty"),
            featured: true,
        })
        .expect("add entry");

    let rows = db.entries_for_subject(&subject).expect("entries");
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].meal_name, "Carbonara");
    assert!(rows[0].featured);

    assert!(db.delete_entry(entry.id, user.id).expect("delete"));
    assert!(db.entries_for_subject(&subject).expect("entries").is_empty());

    db.delete_session(&session_id).expect("delete session");
}

#[test]
fn the_database_drives_meals_usage() {
    let Some(url) = database_url() else {
        eprintln!("skipping: DATABASE_URL is unset");
        return;
    };
    let subject = format!("usage-subject-{}", std::process::id());

    {
        let db = Db::connect(&url).expect("connect and migrate");
        let user = db.upsert_user(&subject, None).expect("upsert user");
        db.add_entry(&NewMealEntry {
            user_id: user.id,
            meal_name: "Carbonara",
            eaten_on: NaiveDate::from_ymd_opt(2024, 3, 1).unwrap(),
            note: Some("first"),
            featured: false,
        })
        .expect("add entry");
    }

    // A context whose usage comes from the database for this subject.
    let env_url = url.clone();
    let env_subject = subject.clone();
    let context = Context::new(Box::new(move |key: &str| match key {
        "DATABASE_URL" => Some(env_url.clone()),
        "RECIBASE_OWNER_SUBJECT" => Some(env_subject.clone()),
        _ => None,
    }));

    let response = recibase_server::route(&request("GET", "/meals/"), &context);
    let value = support::json(&response);
    let carbonara = value
        .as_array()
        .expect("a JSON array")
        .iter()
        .find(|meal| meal.get("name").and_then(|n| n.as_str()) == Some("Carbonara"))
        .expect("Carbonara is in the meal list");
    assert_eq!(carbonara.get("times_eaten").and_then(|n| n.as_i64()), Some(1));
}
