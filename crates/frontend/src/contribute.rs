//! `contribute.py`: the form state and the JSON body for
//! `POST /recipe-submissions`.
//!
//! The API owns validation, Scala generation, and opening the pull request,
//! so this module only turns the posted form into the shapes the route
//! needs: the state the template redisplays, the body it posts, the
//! `Authorization` header it sends, the pull request URL it accepts back,
//! and the message it shows when the API refuses.

use crate::form::Form;
use serde_json::{Value, json};

/// The tag groups the template renders, in order.
pub const TAG_GROUPS: &[(&str, &[(&str, &str)])] = &[
    (
        "Meal",
        &[
            ("Christmas", "Christmas"),
            ("Pudding", "Pudding"),
            ("Lunch", "Lunch"),
            ("Baking", "Baking"),
            ("NonMeal", "Not a Meal"),
            ("Soup", "Soup"),
        ],
    ),
    (
        "Diet",
        &[
            ("Vegan", "Vegan"),
            ("VeganIsh", "Vegan-ish"),
            ("Vegetarian", "Vegetarian"),
            ("VegetarianIsh", "Vegetarian-ish"),
            ("Pescatarian", "Pescatarian"),
        ],
    ),
    (
        "Vibe",
        &[
            ("ColdWeather", "Cold Weather"),
            ("HotWeather", "Hot Weather"),
            ("Stodge", "Stodge"),
            ("Spicy", "Spicy"),
        ],
    ),
    (
        "Effort",
        &[
            ("Slow", "Slow"),
            ("Quick", "Quick"),
            ("Scales", "Scales"),
            ("HighEffort", "High Effort"),
            ("LowEffort", "Low Effort"),
        ],
    ),
    (
        "Good to know",
        &[("Freezes", "Freezes"), ("BetterNextDay", "Better Next Day")],
    ),
];

/// The free-standing diet checkboxes, rendered under the `Diet` group.
pub const DIET_CHECKBOXES: &[(&str, &str)] = &[("GlutenFree", "Gluten-Free")];

/// What the API answers when it does not explain itself.
const FALLBACK_ERRORS: &[(u16, &str)] = &[
    (401, "Invalid passcode."),
    (409, "A recipe with this name already exists."),
    (403, "Could not verify this submission."),
    (502, "Could not open the pull request. Try again."),
    (503, "Recipe submission is not available."),
];

const UNKNOWN_ERROR: &str = "Could not submit the recipe.";

/// Python's `str.strip` treats these as whitespace: the Unicode `White_Space`
/// property, which Rust's `char::is_whitespace` also has, plus the four C0
/// separators `\x1c`-`\x1f` that only Python's `str` counts.
fn py_is_space(ch: char) -> bool {
    ch.is_whitespace() || ('\u{1c}'..='\u{1f}').contains(&ch)
}

/// `str.strip()`.
fn py_strip(value: &str) -> &str {
    value.trim_matches(py_is_space)
}

/// `str.splitlines()`, over Python's line boundary set: `\n`, `\r`, `\r\n`,
/// `\v`, `\f`, `\x1c`, `\x1d`, `\x1e`, `\x85`, `\u2028`, `\u2029`. A
/// trailing boundary does not produce a final empty line.
fn py_splitlines(value: &str) -> Vec<&str> {
    let mut lines = Vec::new();
    let mut start = 0;
    let mut index = 0;
    while index < value.len() {
        let ch = value[index..].chars().next().expect("char boundary");
        let mut next = index + ch.len_utf8();
        let boundary = match ch {
            '\r' => {
                if value[next..].starts_with('\n') {
                    next += 1;
                }
                true
            }
            '\n' | '\u{b}' | '\u{c}' | '\u{1c}' | '\u{1d}' | '\u{1e}' | '\u{85}' | '\u{2028}'
            | '\u{2029}' => true,
            _ => false,
        };
        if boundary {
            lines.push(&value[start..index]);
            start = next;
        }
        index = next;
    }
    if start < value.len() {
        lines.push(&value[start..]);
    }
    lines
}

/// `_clean`: `None` and `None`-like absent values become `""`, anything else
/// is stripped.
fn clean(value: Option<&str>) -> String {
    value
        .map(|value| py_strip(value).to_string())
        .unwrap_or_default()
}

/// `_or_none`: `_clean(value) or None`.
fn or_none(value: Option<&str>) -> Option<String> {
    let cleaned = clean(value);
    if cleaned.is_empty() {
        None
    } else {
        Some(cleaned)
    }
}

/// `_lines`: one entry per non-blank line, stripped.
fn lines(value: Option<&str>) -> Vec<String> {
    py_splitlines(&clean(value))
        .into_iter()
        .map(|line| py_strip(line).to_string())
        .filter(|line| !line.is_empty())
        .collect()
}

/// One ingredient row, as `ingredient_rows` returns it: the raw values from
/// the form, so the template can re-render exactly what was typed.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct IngredientRow {
    name: String,
    quantity: String,
    prep: String,
    notes: String,
}

impl IngredientRow {
    /// The row as a JSON object, in the Python dict literal's key order.
    fn to_value(&self) -> Value {
        json!({
            "name": self.name,
            "quantity": self.quantity,
            "prep": self.prep,
            "notes": self.notes,
        })
    }
}

/// `ingredient_rows`: one row per position of the longest of the four
/// lists, with `""` where a list is shorter. An empty form gives a single
/// blank row, which is what the page starts with.
fn ingredient_rows(form: &Form) -> Vec<IngredientRow> {
    let names = form.get_list("ingredient_name");
    let quantities = form.get_list("ingredient_quantity");
    let preps = form.get_list("ingredient_prep");
    let notes = form.get_list("ingredient_notes");
    let count = names
        .len()
        .max(quantities.len())
        .max(preps.len())
        .max(notes.len());
    if count == 0 {
        return vec![IngredientRow::default()];
    }
    let at = |values: &[&str], index: usize| values.get(index).copied().unwrap_or("").to_string();
    (0..count)
        .map(|index| IngredientRow {
            name: at(&names, index),
            quantity: at(&quantities, index),
            prep: at(&preps, index),
            notes: at(&notes, index),
        })
        .collect()
}

/// `page_state`: what the template redisplays. Values are verbatim (no
/// stripping) so a refused submission shows the form exactly as posted.
pub fn page_state(form: Option<&Form>) -> Value {
    let (passcode, name, source, description, notes, method, tags, ingredients) = match form {
        None => (
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            Vec::new(),
            vec![IngredientRow::default()],
        ),
        Some(form) => (
            form.get_or("passcode", "").to_string(),
            form.get_or("name", "").to_string(),
            form.get_or("source", "").to_string(),
            form.get_or("description", "").to_string(),
            form.get_or("notes", "").to_string(),
            form.get_or("method", "").to_string(),
            form.get_list("tags")
                .iter()
                .map(|tag| tag.to_string())
                .collect(),
            ingredient_rows(form),
        ),
    };
    json!({
        "passcode": passcode,
        "name": name,
        "source": source,
        "description": description,
        "notes": notes,
        "method": method,
        "tags": tags,
        "ingredients": ingredients.iter().map(IngredientRow::to_value).collect::<Vec<Value>>(),
    })
}

/// The `Authorization` header for the API call.
pub fn authorization_header(form: &Form) -> String {
    format!("Bearer {}", clean(form.get("passcode")))
}

/// The JSON body posted to the API, with the Python's key order.
pub fn submission_payload(form: &Form) -> Value {
    let mut ingredients: Vec<Value> = Vec::new();
    for row in ingredient_rows(form) {
        let name = clean(Some(&row.name));
        let quantity = or_none(Some(&row.quantity));
        let prep = or_none(Some(&row.prep));
        let notes = or_none(Some(&row.notes));
        if name.is_empty() && quantity.is_none() && prep.is_none() && notes.is_none() {
            continue;
        }
        ingredients.push(json!({
            "name": name,
            "quantity": quantity,
            "prep": prep,
            "notes": notes,
        }));
    }
    let tags: Vec<String> = form
        .get_list("tags")
        .iter()
        .map(|tag| py_strip(tag))
        .filter(|tag| !tag.is_empty())
        .map(|tag| tag.to_string())
        .collect();
    json!({
        "name": clean(form.get("name")),
        "source": or_none(form.get("source")),
        "description": or_none(form.get("description")),
        "notes": lines(form.get("notes")),
        "tags": tags,
        "ingredients": ingredients,
        "method": lines(form.get("method")),
        "cf-turnstile-response": clean(form.get("cf-turnstile-response")),
    })
}

/// `pull_request_url`: the API's reply is only trusted when it carries a
/// whitespace-free `https://github.com/` URL, so a hostile or broken reply
/// cannot put a link on the page.
pub fn pull_request_url(payload: &Value) -> Option<String> {
    let url = payload.as_object()?.get("url")?.as_str()?;
    if url.chars().any(py_is_space) {
        return None;
    }
    if !url.starts_with("https://github.com/") {
        return None;
    }
    Some(url.to_string())
}

/// `_FALLBACK_ERRORS.get(status, "Could not submit the recipe.")`.
fn fallback_error(status: u16) -> String {
    FALLBACK_ERRORS
        .iter()
        .find(|(code, _)| *code == status)
        .map(|(_, message)| (*message).to_string())
        .unwrap_or_else(|| UNKNOWN_ERROR.to_string())
}

/// `failure_message`, taking the response's status, its `Content-Type`
/// header value (`""` when absent) and its body decoded as UTF-8. The body
/// is parsed as JSON when the content type mentions JSON or the text looks
/// like an object, because that is what `requests.Response.json` is called
/// into doing.
pub fn failure_message(status: u16, content_type: &str, text: &str) -> String {
    let text = py_strip(text);
    let parsed: Option<Value> =
        if content_type.to_lowercase().contains("json") || text.starts_with('{') {
            serde_json::from_str(text).ok()
        } else {
            None
        };
    if let Some(Value::Object(object)) = &parsed {
        for key in ["error", "message"] {
            if let Some(Value::String(value)) = object.get(key) {
                let value = py_strip(value);
                if !value.is_empty() {
                    return value.chars().take(500).collect();
                }
            }
        }
    }
    if !text.is_empty() && !text.starts_with('<') && !text.starts_with('{') {
        return text.chars().take(500).collect();
    }
    fallback_error(status)
}
