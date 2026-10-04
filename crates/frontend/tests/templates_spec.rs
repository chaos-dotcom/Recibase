//! `templates.rs`: the eight embedded templates, the two globals `app.py`
//! injects into all of them, and the 503 the failed one produces.

mod support;

use std::sync::Arc;

use minijinja::value::Value as TemplateValue;
use serde_json::json;

use recibase_frontend::cached_backend::{BackendUnavailable, CachedBackendCall};
use recibase_frontend::contribute;
use recibase_frontend::templates::{TEMPLATES, Templates, is_backend_unavailable};
use support::{SAMPLE_RECIPES, sample_recipe};

/// The templates, with an API that answers.
fn working_templates(frontend_version: &str) -> Templates {
    let list: Result<serde_json::Value, BackendUnavailable> =
        Ok(serde_json::from_str(SAMPLE_RECIPES).expect("SAMPLE_RECIPES is JSON"));
    let version: Result<String, BackendUnavailable> = Ok("deadbeef".to_string());
    Templates::new(
        Arc::new(CachedBackendCall::new(move || list.clone())),
        Arc::new(CachedBackendCall::new(move || version.clone())),
        frontend_version,
    )
}

/// The templates, with the given recipe list.
fn templates_with_list(list: serde_json::Value) -> Templates {
    let list: Result<serde_json::Value, BackendUnavailable> = Ok(list);
    let version: Result<String, BackendUnavailable> = Ok("deadbeef".to_string());
    Templates::new(
        Arc::new(CachedBackendCall::new(move || list.clone())),
        Arc::new(CachedBackendCall::new(move || version.clone())),
        "latest",
    )
}

/// The templates, with an API that cannot be reached.
fn broken_templates() -> Templates {
    Templates::new(
        Arc::new(CachedBackendCall::new(
            || -> Result<serde_json::Value, BackendUnavailable> { Err(BackendUnavailable) },
        )),
        Arc::new(CachedBackendCall::new(
            || -> Result<String, BackendUnavailable> { Err(BackendUnavailable) },
        )),
        "latest",
    )
}

fn render(name: &str, context: serde_json::Value) -> String {
    working_templates("latest")
        .render(name, TemplateValue::from_serialize(&context))
        .unwrap_or_else(|error| panic!("{name} did not render: {error}"))
}

#[test]
fn every_template_is_embedded_and_renders() {
    assert_eq!(TEMPLATES.len(), 8, "the eight templates of templates/");

    let home = render("home.html", json!({}));
    assert!(home.contains("Recibase"), "{home}");
    assert!(
        home.contains("Welcome to Chaos' Reciebase Server"),
        "{home}"
    );
    // The drawer lists the recipes the API returned.
    assert!(home.contains("href=\"test-recipe\""), "{home}");

    let not_found = render("notfound.html", json!({}));
    assert!(not_found.contains("404 - Page not Found"), "{not_found}");

    let internal_error = render("internalerror.html", json!({}));
    assert!(
        internal_error.contains("500 - Internal Error"),
        "{internal_error}"
    );

    let unavailable = render("backendunavailable.html", json!({}));
    assert!(
        unavailable.contains("503 - Backend Unavailable"),
        "{unavailable}"
    );

    let sitemap = working_templates("latest")
        .render(
            "sitemap.xml",
            TemplateValue::from_serialize(json!({"baseUrl": "https://reciba.se"})),
        )
        .expect("sitemap.xml renders");
    assert!(sitemap.contains("<urlset"), "{sitemap}");
    assert!(
        sitemap.contains("<loc>https://reciba.se/</loc>"),
        "{sitemap}"
    );
    assert!(
        sitemap.contains("<loc>https://reciba.se/test-recipe</loc>"),
        "{sitemap}"
    );

    let recipe = render(
        "recipe.html",
        json!({
            "recipe": sample_recipe(),
            "scale_factor": 1,
            "combined_notes": [],
            "copy_ingredients": "2 Onion\n200g Butter",
        }),
    );
    assert!(recipe.contains("<h2>Test Recipe</h2>"), "{recipe}");
    assert!(recipe.contains("Chop onion"), "{recipe}");
    assert!(recipe.contains("data-ingredients=\"2 Onion"), "{recipe}");

    let contribute = render(
        "contribute.html",
        json!({
            "recipeUrl": "contribute",
            "tag_groups": [],
            "diet_checkboxes": [],
            "form": contribute::page_state(None),
            "error": "Could not submit the recipe.",
            "pr_url": null,
        }),
    );
    assert!(contribute.contains("Add a recipe"), "{contribute}");
    assert!(
        contribute.contains("Could not submit the recipe."),
        "{contribute}"
    );
    assert!(contribute.contains("name=\"passcode\""), "{contribute}");
}

/// The temporary "only ours" toggle: our recipes carry `data-ours="true"`, the
/// rest `"false"`, and the filter script is wired into the layout.
#[test]
fn the_drawer_marks_our_recipes_for_the_only_ours_toggle() {
    let home = templates_with_list(json!([
        {"name": "Our Recipe", "permalink": "our-recipe", "ours": true},
        {"name": "Their Recipe", "permalink": "their-recipe"},
    ]))
    .render("home.html", TemplateValue::from_serialize(json!({})))
    .expect("home.html renders");

    assert!(home.contains("data-ours=\"true\""), "{home}");
    assert!(home.contains("data-ours=\"false\""), "{home}");
    assert!(home.contains("id=\"onlyOurs\""), "{home}");
    assert!(home.contains("/static/filters.js"), "{home}");
    // The random link is rewritten by filters.js to carry the toggle state.
    assert!(home.contains("href=\"/random\""), "{home}");
}

/// The footer carries the frontend version the `App` was deployed as, and
/// the API version `fetchApiVersion` returns.
#[test]
fn the_footer_carries_both_versions() {
    let home = working_templates("abcdef1234567890")
        .render("home.html", TemplateValue::from_serialize(json!({})))
        .expect("home.html renders");
    assert!(home.contains("Recibase/commit/abcdef1234567890"), "{home}");
    assert!(home.contains(">abcdef<"), "{home}");
    assert!(home.contains("Recibase/commit/deadbeef"), "{home}");
    assert!(home.contains(">deadbe<"), "{home}");
}

/// A request whose `fetchRecipeList()` fails is the 503 page, not a 500:
/// the error is the one `app.rs` recognises.
#[test]
fn a_failing_recipe_list_is_the_backend_unavailable_error() {
    let error = broken_templates()
        .render("home.html", TemplateValue::from_serialize(json!({})))
        .expect_err("home.html cannot render without the recipe list");
    assert!(is_backend_unavailable(&error), "{error}");
}
