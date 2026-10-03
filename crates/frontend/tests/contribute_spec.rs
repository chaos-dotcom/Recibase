//! The `contribute.py` port: the ported `test_routes.py` contribute cases,
//! checked against the same functions the route calls, plus the golden data
//! generated from the Python by `tools/golden/gen_contribute_golden.py`.
//!
//! The Flask tests asserted on rendered HTML. The router and the template
//! engine are other people's files, so the assertions here are made where
//! the module's own behaviour lives: the form state it hands the template,
//! the payload it posts, and the message it shows.

use std::path::{Path, PathBuf};

use recibase_frontend::contribute::{
    DIET_CHECKBOXES, TAG_GROUPS, authorization_header, failure_message, page_state,
    pull_request_url, submission_payload,
};
use recibase_frontend::form::Form;
use serde_json::{Value, json};

/// `contribute_forms.json`.
const FORM_CASES: usize = 14;
/// `failure_messages.json`.
const FAILURE_CASES: usize = 16;
/// `pull_request_urls.json`.
const URL_CASES: usize = 14;
/// `(value, label)` pairs in `TAG_GROUPS`.
const TAG_COUNT: usize = 25;

fn golden_path(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests").join("golden").join(name)
}

/// Reads a golden file, or fails loudly: a missing file is a broken build,
/// not a skipped test.
fn load_golden(name: &str) -> Value {
    let path = golden_path(name);
    let text = std::fs::read_to_string(&path).unwrap_or_else(|error| {
        panic!("golden file {} is missing or unreadable: {error}", path.display())
    });
    serde_json::from_str(&text).unwrap_or_else(|error| {
        panic!("golden file {} is not valid JSON: {error}", path.display())
    })
}

/// The cases of a golden array, checked against the count in the brief.
fn golden_cases(name: &str, expected: usize) -> Vec<Value> {
    let value = load_golden(name);
    let cases = value
        .as_array()
        .unwrap_or_else(|| panic!("golden file {name} is not a JSON array"));
    assert_eq!(
        cases.len(),
        expected,
        "golden file {name} has {} cases, expected {expected}",
        cases.len()
    );
    cases.clone()
}

/// A `Form` from a golden `[[key, value], ...]` list, order preserved.
fn form_from_pairs(pairs: &Value) -> Form {
    let pairs = pairs.as_array().expect("form pairs");
    Form::from_pairs(
        pairs
            .iter()
            .map(|pair| {
                let pair = pair.as_array().expect("[key, value]");
                assert_eq!(pair.len(), 2, "[key, value]");
                (
                    pair[0].as_str().expect("key").to_string(),
                    pair[1].as_str().expect("value").to_string(),
                )
            })
            .collect(),
    )
}

/// The `Content-Type` of a response, looked up case-insensitively, `""` when
/// the header is absent.
fn content_type(headers: &Value) -> String {
    headers
        .as_object()
        .and_then(|headers| {
            headers
                .iter()
                .find(|(name, _)| name.eq_ignore_ascii_case("content-type"))
        })
        .and_then(|(_, value)| value.as_str())
        .unwrap_or("")
        .to_string()
}

#[test]
fn golden_forms_match_page_state_payload_and_header() {
    let cases = golden_cases("contribute_forms.json", FORM_CASES);
    for (index, case) in cases.iter().enumerate() {
        let form = form_from_pairs(&case["form"]);
        assert_eq!(page_state(Some(&form)), case["page_state"], "case {index}: page_state");
        assert_eq!(submission_payload(&form), case["payload"], "case {index}: payload");
        assert_eq!(
            authorization_header(&form),
            case["auth"].as_str().expect("auth"),
            "case {index}: authorization_header"
        );
    }
}

#[test]
fn golden_empty_form_is_the_page_default() {
    let cases = golden_cases("contribute_forms.json", FORM_CASES);
    let blank = &cases[0]["page_state"];
    assert_eq!(page_state(None), *blank, "an unposted page and an empty form agree");
    assert_eq!(page_state(Some(&Form::new())), *blank);
}

#[test]
fn golden_tags_match_the_template_data() {
    let golden = load_golden("tag_groups.json");
    let groups = golden["tag_groups"].as_array().expect("tag_groups");
    assert_eq!(groups.len(), TAG_GROUPS.len(), "tag group count");
    let mut tags = 0;
    for (group, expected) in TAG_GROUPS.iter().zip(groups) {
        assert_eq!(expected[0].as_str(), Some(group.0), "group name");
        let pairs = expected[1].as_array().expect("(value, label) pairs");
        assert_eq!(pairs.len(), group.1.len(), "{} pairs", group.0);
        for (tag, expected) in group.1.iter().zip(pairs) {
            assert_eq!(expected[0].as_str(), Some(tag.0), "{}: value", group.0);
            assert_eq!(expected[1].as_str(), Some(tag.1), "{}: label", group.0);
            tags += 1;
        }
    }
    assert_eq!(tags, TAG_COUNT, "total tags");

    let checkboxes = golden["diet_checkboxes"].as_array().expect("diet_checkboxes");
    assert_eq!(checkboxes.len(), DIET_CHECKBOXES.len(), "diet checkbox count");
    for (checkbox, expected) in DIET_CHECKBOXES.iter().zip(checkboxes) {
        assert_eq!(expected[0].as_str(), Some(checkbox.0), "diet value");
        assert_eq!(expected[1].as_str(), Some(checkbox.1), "diet label");
    }
}

#[test]
fn golden_failure_messages() {
    let cases = golden_cases("failure_messages.json", FAILURE_CASES);
    for (index, case) in cases.iter().enumerate() {
        let status = case["status"].as_u64().expect("status") as u16;
        let message = failure_message(
            status,
            &content_type(&case["headers"]),
            case["text"].as_str().expect("text"),
        );
        assert_eq!(
            message,
            case["message"].as_str().expect("message"),
            "case {index}: status {status}"
        );
    }
}

#[test]
fn golden_pull_request_urls() {
    let cases = golden_cases("pull_request_urls.json", URL_CASES);
    for (index, case) in cases.iter().enumerate() {
        let text = case["payload"].as_str().expect("payload");
        let parsed: Value = serde_json::from_str(text).expect("payload is JSON");
        let expected = case["url"].as_str().map(str::to_string);
        assert_eq!(pull_request_url(&parsed), expected, "case {index}: {text}");
    }
}

// -- the ported `test_routes.py` cases --------------------------------

/// `test_contribute_submits_recipe`: the body the route posts for the
/// Chilli con Carne form, and the header it posts it with.
#[test]
fn contribute_submits_recipe() {
    let form = Form::from_pairs(vec![
        ("passcode", " secret "),
        ("name", " Chilli con Carne "),
        ("source", "Kit's Dad"),
        ("description", " Weeknight "),
        ("notes", "Rest overnight\n\n"),
        ("tags", "Spicy"),
        ("tags", "Scales"),
        ("ingredient_name", "Mince"),
        ("ingredient_name", "  "),
        ("ingredient_name", "Garlic"),
        ("ingredient_quantity", "500g"),
        ("ingredient_quantity", ""),
        ("ingredient_quantity", ""),
        ("ingredient_prep", ""),
        ("ingredient_prep", ""),
        ("ingredient_prep", "crushed"),
        ("ingredient_notes", ""),
        ("ingredient_notes", ""),
        ("ingredient_notes", ""),
        ("method", "Brown the mince.\n\nServe."),
        ("cf-turnstile-response", "token"),
    ]
    .into_iter()
    .map(|(key, value)| (key.to_string(), value.to_string()))
    .collect());

    assert_eq!(authorization_header(&form), "Bearer secret");
    assert_eq!(
        submission_payload(&form),
        json!({
            "name": "Chilli con Carne",
            "source": "Kit's Dad",
            "description": "Weeknight",
            "notes": ["Rest overnight"],
            "tags": ["Spicy", "Scales"],
            "ingredients": [
                {"name": "Mince", "quantity": "500g", "prep": null, "notes": null},
                {"name": "Garlic", "quantity": null, "prep": "crushed", "notes": null},
            ],
            "method": ["Brown the mince.", "Serve."],
            "cf-turnstile-response": "token",
        })
    );

    // The page the form came from redisplays what was typed, unstripped.
    let state = page_state(Some(&form));
    assert_eq!(state["passcode"], " secret ");
    assert_eq!(state["name"], " Chilli con Carne ");
    assert_eq!(state["notes"], "Rest overnight\n\n");
    assert_eq!(state["tags"], json!(["Spicy", "Scales"]));
    assert_eq!(state["ingredients"][1]["name"], "  ");
}

/// `test_contribute_keeps_form_on_api_error`: a refused submission keeps
/// every value the form was posted with, so nothing has to be retyped.
#[test]
fn contribute_keeps_form_on_api_error() {
    let form = Form::from_pairs(
        [
            ("passcode", "nope"),
            ("name", "Soup"),
            ("source", ""),
            ("description", ""),
            ("notes", ""),
            ("tags", "Spicy"),
            ("ingredient_name", "Onion"),
            ("ingredient_quantity", "1"),
            ("ingredient_prep", ""),
            ("ingredient_notes", "Optional"),
            ("method", "Simmer."),
        ]
        .into_iter()
        .map(|(key, value)| (key.to_string(), value.to_string()))
        .collect(),
    );

    assert_eq!(failure_message(401, "text/plain", "invalid passcode"), "invalid passcode");

    let state = page_state(Some(&form));
    assert_eq!(state["name"], "Soup");
    assert_eq!(state["ingredients"][0]["name"], "Onion");
    assert_eq!(state["ingredients"][0]["notes"], "Optional");
    assert_eq!(state["tags"], json!(["Spicy"]));

    let payload = submission_payload(&form);
    assert_eq!(payload["tags"], json!(["Spicy"]));
    assert_eq!(
        payload["ingredients"],
        json!([{"name": "Onion", "quantity": "1", "prep": null, "notes": "Optional"}])
    );
}

/// `test_contribute_shows_json_error`: the API's own explanation is shown.
#[test]
fn contribute_shows_json_error() {
    let message = failure_message(
        400,
        "application/json",
        r#"{"error": "Add at least one ingredient."}"#,
    );
    assert_eq!(message, "Add at least one ingredient.");
}

/// `test_contribute_rejects_unexpected_pull_request_url`: a reply that is not
/// a GitHub pull request link is refused, and the form comes back.
#[test]
fn contribute_rejects_unexpected_pull_request_url() {
    let reply = json!({"url": "javascript:alert(1)"});
    assert_eq!(pull_request_url(&reply), None);
    let accepted = json!({"url": "https://github.com/chaos-dotcom/Recibase/pull/12"});
    assert_eq!(
        pull_request_url(&accepted).as_deref(),
        Some("https://github.com/chaos-dotcom/Recibase/pull/12")
    );

    let form = Form::from_pairs(
        [
            ("passcode", "secret"),
            ("name", "Soup"),
            ("ingredient_name", "Onion"),
            ("ingredient_quantity", "1"),
            ("ingredient_prep", ""),
            ("ingredient_notes", ""),
            ("method", "Simmer."),
        ]
        .into_iter()
        .map(|(key, value)| (key.to_string(), value.to_string()))
        .collect(),
    );
    assert_eq!(page_state(Some(&form))["name"], "Soup");
}

/// `test_contribute_escapes_redisplayed_values`: the value reaches the
/// template verbatim, and the API's plain-text reason is shown as-is.
#[test]
fn contribute_escapes_redisplayed_values() {
    let form = Form::from_pairs(
        [
            ("passcode", "secret"),
            ("name", "<script>alert(1)</script>"),
            ("ingredient_name", "Onion"),
            ("ingredient_quantity", "1"),
            ("ingredient_prep", ""),
            ("ingredient_notes", ""),
            ("method", "Simmer."),
        ]
        .into_iter()
        .map(|(key, value)| (key.to_string(), value.to_string()))
        .collect(),
    );
    assert_eq!(page_state(Some(&form))["name"], "<script>alert(1)</script>");
    assert_eq!(failure_message(502, "text/plain", "upstream failed"), "upstream failed");
}

/// `test_contribute_reports_unreachable_api`: the route has no response to
/// pass to `failure_message`, so the message is the route's; what this module
/// owns is that the form still comes back.
#[test]
fn contribute_reports_unreachable_api() {
    let form = Form::from_pairs(
        [("passcode", "secret"), ("name", "Soup"), ("method", "Simmer.")]
            .into_iter()
            .map(|(key, value)| (key.to_string(), value.to_string()))
            .collect(),
    );
    let state = page_state(Some(&form));
    assert_eq!(state["name"], "Soup");
    assert_eq!(state["method"], "Simmer.");
    assert_eq!(state["ingredients"][0]["name"], "");
}

// -- the details the port has to keep exactly -------------------------

/// `ingredient_rows`: one row per position of the longest of the four
/// lists; an empty form still shows one blank row.
#[test]
fn ingredient_rows_follow_the_longest_list() {
    let form = Form::from_pairs(
        [("ingredient_notes", "a"), ("ingredient_notes", "b"), ("ingredient_notes", "c")]
            .into_iter()
            .map(|(key, value)| (key.to_string(), value.to_string()))
            .collect(),
    );
    let rows = &page_state(Some(&form))["ingredients"];
    assert_eq!(rows.as_array().map(Vec::len), Some(3));
    assert_eq!(rows[0], json!({"name": "", "quantity": "", "prep": "", "notes": "a"}));
    assert_eq!(rows[2]["notes"], "c");

    let blank = &page_state(None)["ingredients"];
    assert_eq!(blank.as_array().map(Vec::len), Some(1));
    assert_eq!(blank[0], json!({"name": "", "quantity": "", "prep": "", "notes": ""}));

    // A row of four empty values is dropped from the payload.
    let empty_row = Form::from_pairs(vec![
        ("ingredient_name".to_string(), "".to_string()),
        ("ingredient_quantity".to_string(), "".to_string()),
        ("ingredient_prep".to_string(), "".to_string()),
        ("ingredient_notes".to_string(), "".to_string()),
    ]);
    assert_eq!(submission_payload(&empty_row)["ingredients"], json!([]));
}

/// `_lines` uses Python's line boundaries, not just `\n`.
#[test]
fn lines_split_on_every_python_line_boundary() {
    let cases = [
        ("A\nB", vec!["A", "B"]),
        ("\r\nA\r\nB", vec!["A", "B"]),
        ("A\rB", vec!["A", "B"]),
        ("A\u{b}B", vec!["A", "B"]),
        ("A\u{c}B", vec!["A", "B"]),
        ("A\u{1c}B\u{1d}C\u{1e}D", vec!["A", "B", "C", "D"]),
        ("A\u{85}B", vec!["A", "B"]),
        ("A\u{2028}B\u{2029}C", vec!["A", "B", "C"]),
        ("  \n  \n x \n", vec!["x"]),
        ("\n\n", Vec::new()),
        ("", Vec::new()),
        ("  Rest overnight  ", vec!["Rest overnight"]),
    ];
    for (value, expected) in cases {
        let form = Form::from_pairs(vec![("method".to_string(), value.to_string())]);
        assert_eq!(submission_payload(&form)["method"], json!(expected), "method {value:?}");

        let form = Form::from_pairs(vec![("notes".to_string(), value.to_string())]);
        assert_eq!(submission_payload(&form)["notes"], json!(expected), "notes {value:?}");
    }
}

/// `_clean` strips the C0 separators Python counts as whitespace.
#[test]
fn clean_strips_python_whitespace() {
    let form = Form::from_pairs(
        [
            ("passcode", "\u{1c} p \u{1f}"),
            ("name", "\u{85}n\u{a0}"),
            ("source", "\u{2028} s \u{2029}"),
            ("cf-turnstile-response", "  "),
        ]
        .into_iter()
        .map(|(key, value)| (key.to_string(), value.to_string()))
        .collect(),
    );
    assert_eq!(authorization_header(&form), "Bearer p");
    let payload = submission_payload(&form);
    assert_eq!(payload["name"], "n");
    assert_eq!(payload["source"], "s");
    assert_eq!(payload["cf-turnstile-response"], "");
}

/// `failure_message`: the JSON branch, the plain-text branch, and truncation
/// all follow `requests`' behaviour.
#[test]
fn failure_message_branches() {
    // A JSON body is parsed even without a JSON content type.
    assert_eq!(failure_message(400, "text/plain", r#"{"error": "oops"}"#), "oops");
    // `error` is preferred; a blank or non-string one falls through to `message`.
    assert_eq!(
        failure_message(400, "application/json", r#"{"error": "   ", "message": "second"}"#),
        "second"
    );
    assert_eq!(failure_message(400, "application/json", r#"{"error": 5}"#),
        "Could not submit the recipe.");
    // An unparseable body is treated as unparsed.
    assert_eq!(failure_message(400, "text/plain", "{not json}"), "Could not submit the recipe.");
    // A non-object JSON body is not a parsed response.
    assert_eq!(failure_message(400, "text/plain", "[1]"), "[1]");
    // HTML is never shown to the user.
    assert_eq!(failure_message(200, "text/html", "<html>nope</html>"),
        "Could not submit the recipe.");
    // Whitespace-only and empty bodies fall back.
    assert_eq!(failure_message(400, "text/plain", "   "), "Could not submit the recipe.");
    assert_eq!(failure_message(409, "", ""), "A recipe with this name already exists.");
    // 500 characters, not bytes.
    let long = "é".repeat(600);
    assert_eq!(failure_message(400, "text/plain", &long).chars().count(), 500);
    let long_json = format!(r#"{{"error": "{}"}}"#, "x".repeat(600));
    assert_eq!(failure_message(400, "application/json", &long_json).chars().count(), 500);
}

// -- the page, rendered through the real template ----------------------

/// The crate renders templates with MiniJinja and the layout calls two
/// backend helpers; neither is part of this module, so the test stands in for
/// them.
fn template_environment() -> minijinja::Environment<'static> {
    let mut environment = minijinja::Environment::new();
    environment.set_loader(minijinja::path_loader(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("templates"),
    ));
    environment.add_function("fetchRecipeList", || minijinja::Value::from(Vec::<minijinja::Value>::new()));
    environment.add_function("fetchApiVersion", || "1234567890abcdef");
    environment
}

fn render_page(
    environment: &minijinja::Environment<'_>,
    form: Value,
    error: Option<&str>,
    pr_url: Option<&str>,
) -> String {
    let tag_groups: Vec<(String, Vec<(String, String)>)> = TAG_GROUPS
        .iter()
        .map(|(name, tags)| {
            (
                (*name).to_string(),
                tags.iter().map(|(value, label)| ((*value).to_string(), (*label).to_string())).collect(),
            )
        })
        .collect();
    environment
        .get_template("contribute.html")
        .expect("contribute.html")
        .render(minijinja::context! {
            pageName => "Add a recipe",
            recipeUrl => "contribute",
            config => minijinja::context! { frontendVersion => "1234567890abcdef" },
            pr_url => pr_url,
            error => error,
            form => form,
            tag_groups => tag_groups,
            diet_checkboxes => DIET_CHECKBOXES,
        })
        .expect("contribute.html renders")
}

/// `test_contribute_page`: the page the module's default state produces.
#[test]
fn contribute_page_renders_the_default_state() {
    let environment = template_environment();
    let page = render_page(&environment, page_state(None), None, None);
    assert!(page.contains("Add a recipe"), "heading");
    assert!(page.contains(r#"name="passcode""#));
    assert!(page.contains(r#"value="VegetarianIsh""#));
    assert!(page.contains(r#"value="GlutenFree""#));
    assert!(page.contains("Gluten-Free"));
    assert!(!page.contains(r#"value="NeverEaten""#));
    assert!(!page.contains(r#"value="Popular""#));
    assert!(!page.contains(r#"value="New""#));
    assert!(page.contains(r#"class="cf-turnstile""#));
    assert!(page.contains(r#"data-sitekey="0x4AAAAAAFFMifPD-G1JDoui""#));
    assert!(page.contains(r#"data-action="contribute""#));
    assert!(page.contains("https://challenges.cloudflare.com/turnstile/v0/api.js"));
    assert!(page.contains("Drafts saved for 7 days using a cookie, or until submitted."));
    assert!(page.contains("Save Draft"));
    assert!(page.contains("Delete Draft"));
    assert!(page.contains("mdl-navigation__link add-recipe is-current"));
}

/// The tags reach the page in `TAG_GROUPS` order, and the Diet group is the
/// select the template switches on.
#[test]
fn contribute_page_renders_tags_in_order() {
    let environment = template_environment();
    let page = render_page(&environment, page_state(None), None, None);
    let mut cursor = 0;
    for (group, tags) in TAG_GROUPS {
        let heading = format!("<h3>{group}</h3>");
        let at = page[cursor..]
            .find(&heading)
            .unwrap_or_else(|| panic!("{group} heading after byte {cursor}"));
        cursor += at + heading.len();
        for (value, label) in *tags {
            let tag = format!(r#"value="{value}""#);
            let at = page[cursor..].find(&tag).unwrap_or_else(|| panic!("{value} after {group}"));
            cursor += at + tag.len();
            assert!(page.contains(label), "{label} is rendered");
        }
    }
}

/// `test_contribute_keeps_form_on_api_error` and
/// `test_contribute_escapes_redisplayed_values` as the page shows them.
#[test]
fn contribute_page_redisplays_and_escapes() {
    let environment = template_environment();
    let form = Form::from_pairs(
        [
            ("passcode", "secret"),
            ("name", "<script>alert(1)</script>"),
            ("tags", "Spicy"),
            ("ingredient_name", "Onion"),
            ("ingredient_quantity", "1"),
            ("ingredient_prep", ""),
            ("ingredient_notes", "Optional"),
            ("method", "Simmer."),
        ]
        .into_iter()
        .map(|(key, value)| (key.to_string(), value.to_string()))
        .collect(),
    );
    let failed = failure_message(502, "text/plain", "upstream failed");
    let page = render_page(&environment, page_state(Some(&form)), Some(&failed), None);

    assert!(!page.contains("<script>alert(1)</script>"), "the value is escaped");
    // MiniJinja escapes `/` as `&#x2f;` where Jinja2 leaves it alone, so the
    // closing tag is checked up to the slash.
    assert!(page.contains("&lt;script&gt;alert(1)&lt;"));
    assert!(page.contains("script&gt;"));
    assert!(page.contains("upstream failed"));
    assert!(page.contains(r#"value="Spicy" checked"#));
    assert!(page.contains(r#"value="Onion""#));
    assert!(page.contains(r#"value="Optional""#));
    assert!(!page.contains("Pull request opened"));

    // A successful reply replaces the form with the link.
    let url = "https://github.com/chaos-dotcom/Recibase/pull/12";
    let accepted = json!({ "url": url });
    let opened = render_page(&environment, page_state(None), None, pull_request_url(&accepted).as_deref());
    assert!(opened.contains("Pull request opened"));
    assert!(!opened.contains(r#"name="passcode""#), "the form is replaced");
    // The module hands the template the URL unchanged; MiniJinja escapes the
    // slashes in an interpolated value where Jinja2 only escapes `<`, `>`,
    // `&`, `"` and `'`, so the href is checked in MiniJinja's spelling.
    assert!(opened.contains(&url.replace('/', "&#x2f;")));
}
