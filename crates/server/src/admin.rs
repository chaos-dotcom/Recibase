//! The admin tracker: a server-rendered page that lists and records the meals
//! the owner has eaten, plus the routes that add and remove entries.

use crate::auth::{Admin, text};
use crate::db::{MealEntryRow, NewMealEntry, Session, User};
use crate::http::{Request, Response, parse_form};
use chrono::{Local, NaiveDate};
use recibase_core::meal_definitions::meal_stubs;

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
        if meal_name.is_empty() || !is_known_meal(meal_name) {
            return text(400, "choose a meal from the list");
        }
        let eaten_on = NaiveDate::parse_from_str(field("eaten_on"), "%Y-%m-%d")
            .unwrap_or_else(|_| Local::now().date_naive());
        let note = field("note").trim();
        let featured = matches!(field("featured"), "on" | "true" | "1");

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
}

/// Whether a name is one of the meals the corpus knows about.
fn is_known_meal(name: &str) -> bool {
    meal_stubs().iter().any(|meal| meal.name == name)
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
    use super::{escape, is_known_meal};

    #[test]
    fn html_special_characters_are_escaped() {
        assert_eq!(escape("<script>&\"'"), "&lt;script&gt;&amp;&quot;&#39;");
    }

    #[test]
    fn only_corpus_meals_are_known() {
        assert!(is_known_meal("Carbonara"));
        assert!(!is_known_meal("Definitely Not A Meal"));
    }
}
