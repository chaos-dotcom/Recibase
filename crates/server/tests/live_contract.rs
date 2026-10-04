//! Opt-in conformance against another live Recibase deployment: is our API's
//! *declaration* the same as theirs, whatever either of us happens to serve?
//!
//! The comparison is the shape of each response - the routes, the JSON keys and
//! their order, the status codes - not the recipes, tags, meals or manifest
//! values. It runs only when `RECIBASE_LIVE_API` names a deployment (e.g.
//! `RECIBASE_LIVE_API=https://api.reciba.se/`), so CI stays offline and this is
//! never the only gate. When it is set but the deployment cannot be reached,
//! the spec fails rather than skipping: opting in is a request to compare.

mod support;

use serde_json::Value;
use support::{fetch, get, json, keys, live_base, sorted_keys};

use recibase_core::recipes;

#[test]
fn live_docs_declares_the_same_routes() {
    let Some(base) = live_base() else { return };
    let (status, _, text) = fetch(&base, "");
    assert_eq!(status, 200);
    let theirs = serde_json::from_str::<Value>(&text).expect("their docs are JSON");
    assert_eq!(
        sorted_keys(&json(&get("/"))),
        sorted_keys(&theirs),
        "the docs map declares the same routes"
    );
}

#[test]
fn live_manifest_declares_the_same_fields() {
    let Some(base) = live_base() else { return };
    let (status, _, text) = fetch(&base, "manifest");
    assert_eq!(status, 200);
    let theirs = serde_json::from_str::<Value>(&text).expect("their manifest is JSON");
    assert_eq!(
        keys(&json(&get("/manifest"))),
        keys(&theirs),
        "the manifest declares the same fields in the same order"
    );
}

#[test]
fn live_health_is_ok() {
    let Some(base) = live_base() else { return };
    let (status, _, text) = fetch(&base, "health");
    assert_eq!(status, 200);
    assert_eq!(text, "ok");
}

#[test]
fn live_recipe_list_entries_are_name_then_permalink() {
    let Some(base) = live_base() else { return };
    let (status, _, text) = fetch(&base, "recipes/");
    assert_eq!(status, 200);
    let theirs = serde_json::from_str::<Value>(&text).expect("their list is JSON");
    let theirs_first = theirs
        .as_array()
        .and_then(|entries| entries.first())
        .expect("their list has an entry");
    let ours = json(&get("/recipes/"));
    let ours_first = ours
        .as_array()
        .and_then(|entries| entries.first())
        .expect("our list has an entry");
    // `ours` is a port addition to the entry and is not part of the base
    // declaration, so strip it from both sides before comparing.
    let base_keys = |entry: &Value| -> Vec<String> {
        keys(entry)
            .into_iter()
            .filter(|key| key != "ours")
            .collect()
    };
    assert_eq!(
        base_keys(ours_first),
        base_keys(theirs_first),
        "the base entry declares the same fields in the same order"
    );
}

#[test]
fn live_recipe_declares_the_same_fields() {
    let Some(base) = live_base() else { return };
    let (status, _, text) = fetch(&base, "recipes/");
    assert_eq!(status, 200);
    let theirs = serde_json::from_str::<Value>(&text).expect("their list is JSON");
    let shared = theirs
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(|entry| entry["permalink"].as_str())
        .find(|permalink| {
            recipes::recipes()
                .iter()
                .any(|recipe| recipe.permalink() == *permalink)
        });
    let Some(permalink) = shared else {
        eprintln!("skipping: no recipe both deployments host");
        return;
    };
    let (status, _, text) = fetch(&base, &format!("recipes/{permalink}"));
    assert_eq!(status, 200);
    let theirs = serde_json::from_str::<Value>(&text).expect("their recipe is JSON");
    assert_eq!(
        keys(&json(&get(&format!("/recipes/{permalink}")))),
        keys(&theirs),
        "the recipe declares the same fields in the same order ({permalink})"
    );
}

#[test]
fn live_meals_declare_the_same_fields() {
    let Some(base) = live_base() else { return };
    let (status, _, text) = fetch(&base, "meals/");
    assert_eq!(status, 200);
    let theirs = serde_json::from_str::<Value>(&text).expect("their meals are JSON");
    let theirs_first = theirs
        .as_array()
        .and_then(|meals| meals.first())
        .expect("their meals have an entry");
    let ours = json(&get("/meals/"));
    let ours_first = ours
        .as_array()
        .and_then(|meals| meals.first())
        .expect("our meals have an entry");
    assert_eq!(
        keys(ours_first),
        keys(theirs_first),
        "a meal declares the same fields in the same order"
    );
}
