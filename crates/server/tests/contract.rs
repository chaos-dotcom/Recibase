//! The API's contract: the routes it declares and the shape of every response.
//!
//! This is the "declaration" half of the port's compatibility - the endpoints,
//! the JSON keys and their order, the status codes, the content types - as
//! opposed to the "manifest" half (the recipes, tags, meals and manifest
//! values), which belongs to the deployment. It never names a recipe: it takes
//! one from the registry and checks the shape it declares, so changing what we
//! host does not change what this asserts.

mod support;

use serde_json::Value;
use support::{assert_keys, body, get, json, key_order_is, keys};

use recibase_core::recipes;

#[test]
fn docs_declares_every_route() {
    let response = get("/");
    assert_eq!(response.status, 200);
    assert_eq!(response.content_type, Some("application/json"));
    let docs = json(&response);
    let declared = [
        ("docs", "/"),
        ("recipes_list", "/recipes/?hasIngredient={ingredient_name}"),
        ("recipe", "/recipes/{recipe_permalink}"),
        ("meals_list", "/meals/"),
        ("meal_names_text", "/meals/raw"),
        ("service_info", "/manifest"),
        ("health", "/health"),
    ];
    for (key, value) in declared {
        assert_eq!(
            docs.get(key).and_then(Value::as_str),
            Some(value),
            "docs[{key}]"
        );
    }
    assert_eq!(
        keys(&docs).len(),
        declared.len(),
        "docs declares exactly these routes"
    );
}

#[test]
fn health_is_plain_ok() {
    let response = get("/health");
    assert_eq!(response.status, 200);
    assert_eq!(response.content_type, Some("text/plain; charset=UTF-8"));
    assert_eq!(body(&response), "ok");
}

#[test]
fn manifest_declares_the_multi_tenancy_fields_in_order() {
    let response = get("/manifest");
    assert_eq!(response.status, 200);
    assert_eq!(response.content_type, Some("application/json"));
    let manifest = json(&response);
    assert_keys(
        &manifest,
        &["version", "name", "source_url", "base_commit_url"],
    );
    for key in ["version", "name", "source_url", "base_commit_url"] {
        assert!(manifest[key].is_string(), "{key} is a string");
    }
    assert!(
        !manifest["version"].as_str().unwrap_or_default().is_empty(),
        "version is set"
    );
}

#[test]
fn recipe_list_entries_are_name_then_permalink() {
    let response = get("/recipes/");
    assert_eq!(response.status, 200);
    assert_eq!(response.content_type, Some("application/json"));
    let parsed = json(&response);
    let entries = parsed.as_array().expect("a JSON array");
    assert!(!entries.is_empty(), "the list is not empty");
    for entry in entries {
        let entry_keys = keys(entry);
        assert_eq!(entry_keys.first().map(String::as_str), Some("name"));
        assert_eq!(entry_keys.get(1).map(String::as_str), Some("permalink"));
        assert!(
            entry_keys
                .iter()
                .all(|key| ["name", "permalink", "ours"].contains(&key.as_str())),
            "no opt-in keys without a query: {entry_keys:?}"
        );
        if let Some(ours) = entry.get("ours") {
            assert!(ours.is_boolean(), "ours is a boolean");
        }
    }
    // `ours` is the deployment's own marking, so it appears at least once.
    assert!(
        entries.iter().any(|entry| entry.get("ours").is_some()),
        "our own recipes are marked"
    );
}

#[test]
fn recipe_list_revision_and_tags_are_opt_in_but_well_shaped() {
    let response = get("/recipes/?withRevision=true");
    assert_eq!(response.status, 200);
    for entry in json(&response).as_array().expect("a JSON array") {
        let revision = entry["revision"].as_str().expect("revision is present");
        assert!(
            revision.len() == 16 && revision.bytes().all(|byte| byte.is_ascii_hexdigit()),
            "revision is a 16-hex-digit digest: {revision}"
        );
        assert!(
            key_order_is(&keys(entry), &["name", "permalink", "ours", "revision"]),
            "revision comes after name, permalink and ours"
        );
    }

    let response = get("/recipes/?withTags=true");
    assert_eq!(response.status, 200);
    for entry in json(&response).as_array().expect("a JSON array") {
        assert!(
            entry.get("revision").is_none(),
            "withTags alone adds no revision"
        );
        if let Some(tags) = entry.get("tags") {
            assert!(
                tags.as_array()
                    .expect("tags is an array")
                    .iter()
                    .all(Value::is_string),
                "tags are strings"
            );
        }
    }
}

#[test]
fn a_recipe_declares_its_fields_in_order() {
    let permalink = recipes::recipes()[0].permalink();
    let response = get(&format!("/recipes/{permalink}"));
    assert_eq!(response.status, 200);
    assert_eq!(response.content_type, Some("application/json"));
    assert_keys(
        &json(&response),
        &[
            "name",
            "permalink",
            "edit",
            "source",
            "description",
            "tagline",
            "notes",
            "dated_notes",
            "tags",
            "inherited_tags",
            "image",
            "ingredients_blocks",
            "method",
        ],
    );
}

#[test]
fn a_meal_declares_its_fields_in_order() {
    let response = get("/meals/");
    assert_eq!(response.status, 200);
    assert_eq!(response.content_type, Some("application/json"));
    let parsed = json(&response);
    let meals = parsed.as_array().expect("a JSON array");
    assert!(!meals.is_empty(), "there are meals");
    for meal in meals {
        assert_keys(
            meal,
            &[
                "name",
                "tags",
                "inherited_tags",
                "source",
                "dated_notes",
                "last_eaten",
                "times_eaten",
                "featured",
            ],
        );
    }
}

#[test]
fn meal_names_are_sorted_plain_text() {
    let response = get("/meals/raw");
    assert_eq!(response.status, 200);
    assert_eq!(response.content_type, Some("text/plain; charset=UTF-8"));
    let text = body(&response);
    let names: Vec<&str> = text.split('\n').collect();
    assert!(names.iter().all(|name| !name.is_empty()), "no blank lines");
    let mut sorted = names.clone();
    sorted.sort();
    assert_eq!(names, sorted, "names are sorted");
}

#[test]
fn not_found_bodies_are_the_declarations() {
    let unknown = get("/nope");
    assert_eq!(unknown.status, 404);
    assert_eq!(unknown.content_type, Some("text/plain; charset=UTF-8"));
    assert_eq!(body(&unknown), "Not found");

    let missing = get("/recipes/i-do-not-exist");
    assert_eq!(missing.status, 404);
    assert_eq!(missing.content_type, Some("text/plain; charset=UTF-8"));
    assert_eq!(body(&missing), "Recipe not found");
}
