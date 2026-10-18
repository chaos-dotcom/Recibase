//! The admin tracker: a server-rendered page that lists and records the meals
//! the owner has eaten, plus the routes that add and remove entries.

use crate::auth::{Admin, text};
use crate::db::{MealEntryRow, NewMealEntry, Session, User};
use crate::http::{Request, Response, parse_form};
use chrono::{Local, NaiveDate};
use recibase_core::json::obj;
use recibase_core::meal_definitions::meal_stubs;
use serde_json::Value;

impl Admin {
    /// `GET /admin`: the tracker page for a signed-in owner.
    pub fn page(&self, session: &Session, user: &User) -> Response {
        let entries = match self.db.entries_for_user(user.id) {
            Ok(entries) => entries,
            Err(error) => {
                eprintln!("could not load entries: {error}");
                return text(500, "could not load your meals");
            }
        };
        Response::html(render_page(user, session, &entries))
    }

    /// `POST /admin/entries`: record one meal.
    pub fn add_entry(&self, request: &Request, session: &Session, user: &User) -> Response {
        if !self.origin_ok(request) || !self.csrf_ok(request, session) {
            return text(403, "forbidden");
        }
        let form = parse_form(&request.body);
        let field = |name: &str| {
            form.iter()
                .find(|(key, _)| key == name)
                .map(|(_, value)| value.as_str())
                .unwrap_or("")
        };

        let meal_name = field("meal_name").trim();
        if !valid_meal_name(meal_name) {
            return text(400, "choose a meal");
        }
        let eaten_on = NaiveDate::parse_from_str(field("eaten_on"), "%Y-%m-%d")
            .unwrap_or_else(|_| Local::now().date_naive());
        let note = field("note").trim();
        let featured = matches!(field("featured"), "on" | "true" | "1");

        if let Err(response) = self.reject_duplicate(user.id, meal_name, eaten_on, None) {
            return response;
        }
        let entry = NewMealEntry {
            user_id: user.id,
            meal_name,
            eaten_on,
            note: (!note.is_empty()).then_some(note),
            featured,
        };
        match self.db.add_entry(&entry) {
            Ok(_) => Response::redirect(303, "/admin"),
            Err(error) => {
                eprintln!("could not record a meal: {error}");
                text(500, "could not record that meal")
            }
        }
    }

    /// `POST /admin/entries/delete`: remove one entry the owner recorded.
    pub fn delete_entry(&self, request: &Request, session: &Session, user: &User) -> Response {
        if !self.origin_ok(request) || !self.csrf_ok(request, session) {
            return text(403, "forbidden");
        }
        let form = parse_form(&request.body);
        let id = form
            .iter()
            .find(|(key, _)| key == "id")
            .and_then(|(_, value)| value.parse::<i32>().ok());
        let Some(id) = id else {
            return text(400, "missing entry id");
        };
        match self.db.delete_entry(id, user.id) {
            Ok(_) => Response::redirect(303, "/admin"),
            Err(error) => {
                eprintln!("could not delete an entry: {error}");
                text(500, "could not delete that entry")
            }
        }
    }

    /// `GET /admin/api/session`: the login state and the CSRF token the
    /// frontend's JavaScript needs. Always 200, so the page can render a login
    /// prompt without special-casing an error.
    pub fn session_json(&self, request: &Request) -> Response {
        match self.session(request) {
            Some((session, user)) => Response::json(&obj(vec![
                ("authenticated", Value::Bool(true)),
                ("email", opt_value(user.email)),
                ("csrf", Value::String(session.csrf_token)),
            ])),
            None => Response::json(&obj(vec![
                ("authenticated", Value::Bool(false)),
                ("email", Value::Null),
                ("csrf", Value::Null),
            ])),
        }
    }

    /// `GET /admin/api/entries`: every entry the owner recorded, newest first.
    pub fn entries_json(&self, _request: &Request, user: &User) -> Response {
        match self.db.entries_for_user(user.id) {
            Ok(rows) => Response::json(&Value::Array(rows.iter().map(entry_to_json).collect())),
            Err(error) => {
                eprintln!("could not load entries: {error}");
                text(500, "could not load your meals")
            }
        }
    }

    /// `POST /admin/api/entries`: record one meal from the frontend's JSON.
    pub fn add_entry_json(&self, request: &Request, session: &Session, user: &User) -> Response {
        if !self.origin_ok(request) || !self.csrf_header_ok(request, session) {
            return text(403, "forbidden");
        }
        let Ok(body) = serde_json::from_slice::<Value>(&request.body) else {
            return text(400, "invalid json");
        };
        let meal_name = body
            .get("meal_name")
            .and_then(Value::as_str)
            .unwrap_or("")
            .trim();
        if !valid_meal_name(meal_name) {
            return text(400, "choose a meal");
        }
        let eaten_on = body
            .get("eaten_on")
            .and_then(Value::as_str)
            .and_then(|value| NaiveDate::parse_from_str(value, "%Y-%m-%d").ok())
            .unwrap_or_else(|| Local::now().date_naive());
        let note = body
            .get("note")
            .and_then(Value::as_str)
            .map(str::trim)
            .filter(|value| !value.is_empty());
        let featured = body
            .get("featured")
            .and_then(Value::as_bool)
            .unwrap_or(false);

        if let Err(response) = self.reject_duplicate(user.id, meal_name, eaten_on, None) {
            return response;
        }
        let new = NewMealEntry {
            user_id: user.id,
            meal_name,
            eaten_on,
            note,
            featured,
        };
        match self.db.add_entry(&new) {
            Ok(row) => Response::json_status(201, &entry_to_json(&row)),
            Err(error) => {
                eprintln!("could not record a meal: {error}");
                text(500, "could not record that meal")
            }
        }
    }

    /// `POST /admin/api/entries/update`: change an entry's date, note and
    /// featured flag. A change that would collide with another entry of the same
    /// meal on the same day is refused.
    pub fn update_entry_json(&self, request: &Request, session: &Session, user: &User) -> Response {
        if !self.origin_ok(request) || !self.csrf_header_ok(request, session) {
            return text(403, "forbidden");
        }
        let Ok(body) = serde_json::from_slice::<Value>(&request.body) else {
            return text(400, "invalid json");
        };
        let Some(id) = body.get("id").and_then(Value::as_i64).map(|id| id as i32) else {
            return text(400, "missing entry id");
        };
        let eaten_on = body
            .get("eaten_on")
            .and_then(Value::as_str)
            .and_then(|value| NaiveDate::parse_from_str(value, "%Y-%m-%d").ok())
            .unwrap_or_else(|| Local::now().date_naive());
        let note = body
            .get("note")
            .and_then(Value::as_str)
            .map(str::trim)
            .filter(|value| !value.is_empty());
        let featured = body
            .get("featured")
            .and_then(Value::as_bool)
            .unwrap_or(false);

        let existing = match self.db.entry(user.id, id) {
            Ok(Some(entry)) => entry,
            Ok(None) => return text(404, "no such entry"),
            Err(error) => {
                eprintln!("could not load an entry: {error}");
                return text(500, "could not save that entry");
            }
        };
        if let Err(response) =
            self.reject_duplicate(user.id, &existing.meal_name, eaten_on, Some(id))
        {
            return response;
        }
        match self.db.update_entry(user.id, id, eaten_on, note, featured) {
            Ok(_) => Response::json(&obj(vec![("ok", Value::Bool(true))])),
            Err(error) => {
                eprintln!("could not update an entry: {error}");
                text(500, "could not save that entry")
            }
        }
    }

    /// Refuses an entry that would duplicate the same meal on the same day.
    fn reject_duplicate(
        &self,
        user_id: i32,
        meal_name: &str,
        eaten_on: NaiveDate,
        exclude_id: Option<i32>,
    ) -> Result<(), Response> {
        match self.db.entry_exists(user_id, meal_name, eaten_on, exclude_id) {
            Ok(true) => Err(text(409, "That meal is already logged for that day")),
            Ok(false) => Ok(()),
            Err(error) => {
                eprintln!("could not check for duplicates: {error}");
                Err(text(500, "could not save that entry"))
            }
        }
    }

    /// `POST /admin/api/entries/delete`: remove one entry by id.
    pub fn delete_entry_json(&self, request: &Request, session: &Session, user: &User) -> Response {
        if !self.origin_ok(request) || !self.csrf_header_ok(request, session) {
            return text(403, "forbidden");
        }
        let Ok(body) = serde_json::from_slice::<Value>(&request.body) else {
            return text(400, "invalid json");
        };
        let Some(id) = body.get("id").and_then(Value::as_i64).map(|id| id as i32) else {
            return text(400, "missing entry id");
        };
        match self.db.delete_entry(id, user.id) {
            Ok(_) => Response::json(&obj(vec![("ok", Value::Bool(true))])),
            Err(error) => {
                eprintln!("could not delete an entry: {error}");
                text(500, "could not delete that entry")
            }
        }
    }

    /// Whether the `X-CSRF-Token` header carries the session's token, the
    /// header form the frontend's JavaScript uses.
    pub fn csrf_header_ok(&self, request: &Request, session: &Session) -> bool {
        request
            .header("X-CSRF-Token")
            .map(|token| token == session.csrf_token)
            .unwrap_or(false)
    }
}

/// One entry, in the shape the frontend's JavaScript consumes.
fn entry_to_json(entry: &MealEntryRow) -> Value {
    obj(vec![
        ("id", Value::from(entry.id)),
        ("meal_name", Value::String(entry.meal_name.clone())),
        (
            "eaten_on",
            Value::String(entry.eaten_on.format("%Y-%m-%d").to_string()),
        ),
        ("note", opt_value(entry.note.clone())),
        ("featured", Value::Bool(entry.featured)),
    ])
}

fn opt_value(value: Option<String>) -> Value {
    value.map(Value::String).unwrap_or(Value::Null)
}

/// Whether a name may be recorded.
///
/// The corpus constrains the form's suggestions, but a meal a peer's server
/// serves is not in our corpus, so any reasonable non-empty name is accepted;
/// a personal tracker should be able to count it too.
fn valid_meal_name(name: &str) -> bool {
    let name = name.trim();
    !name.is_empty() && name.chars().count() <= 120
}

/// Every meal name, sorted, for the entry form's `datalist`.
fn known_meal_names() -> Vec<String> {
    let mut names: Vec<String> = meal_stubs().iter().map(|meal| meal.name.clone()).collect();
    names.sort();
    names.dedup();
    names
}

fn render_page(user: &User, session: &Session, entries: &[MealEntryRow]) -> String {
    let who = user.email.as_deref().unwrap_or(&user.subject);
    let today = Local::now().date_naive();

    let options = known_meal_names()
        .iter()
        .map(|name| format!("<option value=\"{}\">", escape(name)))
        .collect::<Vec<_>>()
        .join("\n      ");

    let rows = entries
        .iter()
        .map(|entry| {
            format!(
                "<tr><td>{date}</td><td>{meal}</td><td>{note}</td><td>{featured}</td>\
<td><form method=\"post\" action=\"/admin/entries/delete\">\
<input type=\"hidden\" name=\"csrf\" value=\"{csrf}\">\
<input type=\"hidden\" name=\"id\" value=\"{id}\">\
<button type=\"submit\">Delete</button></form></td></tr>",
                date = entry.eaten_on.format("%Y-%m-%d"),
                meal = escape(&entry.meal_name),
                note = escape(entry.note.as_deref().unwrap_or("")),
                featured = if entry.featured { "yes" } else { "" },
                csrf = escape(&session.csrf_token),
                id = entry.id,
            )
        })
        .collect::<Vec<_>>()
        .join("\n      ");

    let empty_note = if entries.is_empty() {
        "<tr><td colspan=\"5\">No meals logged yet.</td></tr>"
    } else {
        ""
    };

    format!(
        r#"<!doctype html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>Recibase tracker</title>
<style>
  body {{ font-family: system-ui, sans-serif; max-width: 52rem; margin: 2rem auto; padding: 0 1rem; }}
  header {{ display: flex; justify-content: space-between; align-items: baseline; }}
  form.log {{ display: grid; gap: .6rem; max-width: 28rem; margin-block-end: 2rem; }}
  label {{ display: grid; gap: .2rem; }}
  table {{ border-collapse: collapse; width: 100%; }}
  th, td {{ text-align: left; padding: .35rem .5rem; border-bottom: 1px solid #ddd; }}
  input, textarea, button {{ font: inherit; padding: .35rem; }}
</style>
</head>
<body>
<header>
  <h1>Recibase tracker</h1>
  <form method="post" action="/auth/logout"><button type="submit">Log out</button></form>
</header>
<p>Signed in as <code>{who}</code>.</p>

<h2>Log a meal</h2>
<form class="log" method="post" action="/admin/entries">
  <input type="hidden" name="csrf" value="{csrf}">
  <label>Meal
    <input name="meal_name" list="meals" autocomplete="off" required>
    <datalist id="meals">
      {options}
    </datalist>
  </label>
  <label>Date
    <input type="date" name="eaten_on" value="{today}">
  </label>
  <label>Note
    <textarea name="note" rows="2"></textarea>
  </label>
  <label><input type="checkbox" name="featured"> Featured</label>
  <button type="submit">Add meal</button>
</form>

<h2>History ({count})</h2>
<table>
  <thead><tr><th>Date</th><th>Meal</th><th>Note</th><th>Featured</th><th></th></tr></thead>
  <tbody>
      {empty_note}
      {rows}
  </tbody>
</table>
</body>
</html>
"#,
        who = escape(who),
        csrf = escape(&session.csrf_token),
        options = options,
        today = today.format("%Y-%m-%d"),
        count = entries.len(),
        empty_note = empty_note,
        rows = rows,
    )
}

/// HTML-escapes text that reaches the page.
fn escape(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    for character in input.chars() {
        match character {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&#39;"),
            other => out.push(other),
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::{escape, valid_meal_name};

    #[test]
    fn html_special_characters_are_escaped() {
        assert_eq!(escape("<script>&\"'"), "&lt;script&gt;&amp;&quot;&#39;");
    }

    #[test]
    fn any_reasonable_name_is_accepted() {
        assert!(valid_meal_name("Carbonara"));
        // A meal a peer's server serves is not in our corpus but is allowed.
        assert!(valid_meal_name("Kit's Reci-Verse Curry"));
        assert!(!valid_meal_name("   "));
        assert!(!valid_meal_name(&"x".repeat(121)));
    }
}
