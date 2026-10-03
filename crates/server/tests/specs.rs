//! Ports of the Scala specs that exercise core, the server and its routes:
//! PermalinkSpec, ManifestSpec, RecipesSpec and RecipeSubmissionRoutesSpec.

use recibase_core::json::to_string;
use recibase_core::misc::Manifest;
use recibase_core::permalink::from_raw_string;
use recibase_server::http::Request;
use recibase_server::routes::{route, Context};

fn context() -> Context {
    Context::new(Box::new(|_| None))
}

fn request(method: &str, target: &str) -> Request {
    let (path, query) = match target.split_once('?') {
        Some((p, q)) => (p.to_string(), q.to_string()),
        None => (target.to_string(), String::new()),
    };
    Request {
        method: method.to_string(),
        target: target.to_string(),
        version: "HTTP/1.1".to_string(),
        path,
        query,
        headers: Vec::new(),
        body: Vec::new(),
    }
}

fn body(response: &recibase_server::http::Response) -> String {
    String::from_utf8_lossy(&response.body).into_owned()
}

// ---------------------------------------------------------------- PermalinkSpec

#[test]
fn permalink_lowercases_the_input() {
    assert_eq!(from_raw_string("FooBar"), "foobar");
}

#[test]
fn permalink_replaces_spaces_with_dashes() {
    assert_eq!(from_raw_string("foo bar"), "foo-bar");
}

#[test]
fn permalink_trims_whitespace() {
    assert_eq!(from_raw_string(" foo "), "foo");
}

#[test]
fn permalink_removes_acutes() {
    assert_eq!(from_raw_string("fôôbär"), "foobar");
}

#[test]
fn permalink_removes_non_latin_characters() {
    assert_eq!(from_raw_string(".f$^b@r!"), "fbr");
}

#[test]
fn permalink_removes_duplicate_whitespace() {
    assert_eq!(from_raw_string("foo $ bar"), "foo-bar");
}

#[test]
fn permalink_removes_stop_words() {
    assert_eq!(from_raw_string("and bar with"), "bar");
}

#[test]
fn permalink_preserves_pre_existing_dashes() {
    assert_eq!(from_raw_string("foo-bar"), "foo-bar");
}

#[test]
fn permalink_removes_adjectives() {
    assert_eq!(from_raw_string("creamy foo and bar with"), "foo-bar");
}

// ----------------------------------------------------------------- ManifestSpec

#[test]
fn manifest_json_has_the_multi_tenancy_fields() {
    let json = Manifest::new("cafeba6".to_string()).to_json();
    assert_eq!(json["name"], Manifest::NAME);
    assert_eq!(json["source_url"], Manifest::SOURCE_URL);
    assert_eq!(json["base_commit_url"], Manifest::BASE_COMMIT_URL);
    assert_eq!(json["version"], "cafeba6");
}

#[test]
fn manifest_json_matches_the_scala_field_order() {
    let json = to_string(&Manifest::new("cafeba6".to_string()).to_json());
    assert_eq!(
        json,
        concat!(
            "{\"version\":\"cafeba6\",\"name\":\"Recibase\",",
            "\"source_url\":\"https://github.com/chaos-dotcom/Recibase\",",
            "\"base_commit_url\":\"https://github.com/chaos-dotcom/Recibase/commit/\"}"
        )
    );
}

#[test]
fn deployed_version_prefers_git_commit() {
    let env = |key: &str| match key {
        "GIT_COMMIT" => Some("abcdef1234567890".to_string()),
        "SOURCE_COMMIT" => Some("ffffffffffffffff".to_string()),
        _ => None,
    };
    assert_eq!(Manifest::deployed_version(&env), "abcdef1234567890");
}

#[test]
fn deployed_version_ignores_placeholders() {
    let env = |key: &str| match key {
        "SOURCE_COMMIT" => Some("HEAD".to_string()),
        "GIT_COMMIT" => Some("latest".to_string()),
        "GITHUB_SHA" => Some("cafeba6".to_string()),
        _ => None,
    };
    assert_eq!(Manifest::deployed_version(&env), "cafeba6");
}

#[test]
fn deployed_version_falls_back_to_latest() {
    assert_eq!(Manifest::deployed_version(&|_| None), "latest");
}

// ------------------------------------------------------------------- RecipesSpec

#[test]
fn recipes_list_returns_200() {
    let response = route(&request("GET", "/recipes/"), &context());
    assert_eq!(response.status, 200);
}

#[test]
fn recipes_list_links_to_each_recipe() {
    let response = route(&request("GET", "/recipes/"), &context());
    assert!(body(&response)
        .contains("{\"name\":\"Vegetable Primavera\",\"permalink\":\"vegetable-primavera\"}"));
}

#[test]
fn recipes_list_marks_our_recipes_with_ours() {
    let response = route(&request("GET", "/recipes/"), &context());
    let entries: Vec<serde_json::Value> =
        serde_json::from_str(&body(&response)).expect("the recipe list is JSON");

    let marked = entries.iter().filter(|entry| entry["ours"] == true).count();
    assert_eq!(
        marked,
        recibase_core::recipes::chaos_recipes().len(),
        "exactly our (chaos-tagged) recipes are marked"
    );

    // `ours` is serialised only when set, so Kit's and Alex's entries stay
    // byte-identical to before.
    let theirs = entries
        .iter()
        .find(|entry| entry["permalink"] == "vegetable-primavera")
        .expect("Vegetable Primavera is in the list");
    assert!(theirs.get("ours").is_none(), "their entry must not carry ours");
}

#[test]
fn filtered_list_returns_200() {
    let response = route(&request("GET", "/recipes/?hasIngredient=Thyme"), &context());
    assert_eq!(response.status, 200);
}

#[test]
fn filtered_list_includes_recipes_with_thyme() {
    let response = route(&request("GET", "/recipes/?hasIngredient=Thyme"), &context());
    assert!(body(&response).contains("beetroot-risotto"));
}

#[test]
fn filtered_list_excludes_recipes_without_thyme() {
    let response = route(&request("GET", "/recipes/?hasIngredient=Thyme"), &context());
    assert!(!body(&response).contains("baked-rigatoni-aubergine"));
}

#[test]
fn health_returns_200_and_ok() {
    let response = route(&request("GET", "/health"), &context());
    assert_eq!(response.status, 200);
    assert_eq!(body(&response), "ok");
}

#[test]
fn missing_recipe_returns_404() {
    let response = route(&request("GET", "/recipes/i-do-not-exist"), &context());
    assert_eq!(response.status, 404);
    assert_eq!(body(&response), "Recipe not found");
}

#[test]
fn existing_recipe_returns_200_and_json() {
    let response = route(&request("GET", "/recipes/vegetable-primavera"), &context());
    assert_eq!(response.status, 200);
    let json: serde_json::Value = serde_json::from_str(&body(&response)).unwrap();
    assert_eq!(json["name"], "Vegetable Primavera");
}

// ------------------------------------------------- RecipeSubmissionRoutesSpec

fn post(payload: &str, passcode: Option<&str>) -> recibase_server::http::Response {
    let mut request = request("POST", "/recipe-submissions");
    request.body = payload.as_bytes().to_vec();
    if let Some(passcode) = passcode {
        request
            .headers
            .push(("Authorization".to_string(), format!("Bearer {}", passcode)));
    }
    route(&request, &context())
}

fn error_of(response: &recibase_server::http::Response) -> String {
    let json: serde_json::Value = serde_json::from_str(&body(response)).unwrap();
    json["error"].as_str().unwrap().to_string()
}

#[test]
fn unconfigured_submission_returns_503() {
    let response = post("{}", None);
    assert_eq!(response.status, 503);
    assert_eq!(error_of(&response), "recipe submission not configured");
}

#[test]
fn config_requires_both_secrets() {
    use recibase_submit::config::RecipeSubmissionConfig;
    assert!(RecipeSubmissionConfig::from(None, Some("token".into()), None, None, None, None).is_none());
    assert!(RecipeSubmissionConfig::from(Some("secret".into()), None, None, None, None, None).is_none());
}

#[test]
fn config_defaults_repository_and_branch() {
    use recibase_submit::config::RecipeSubmissionConfig;
    let config = RecipeSubmissionConfig::from(
        Some("secret".into()),
        Some("token".into()),
        None,
        None,
        Some("turnstile-secret".into()),
        Some("recipes.example, www.example".into()),
    )
    .expect("configured");
    assert_eq!(config.passcode, "secret");
    assert_eq!(config.github.repository, "chaos-dotcom/Recibase");
    assert_eq!(config.github.base_branch, "master");
    assert!(config.turnstile.hostnames.contains("recipes.example"));
    assert!(config.turnstile.hostnames.contains("www.example"));
}

#[test]
fn config_rejects_a_repository_that_is_not_owner_name() {
    use recibase_submit::config::RecipeSubmissionConfig;
    assert!(RecipeSubmissionConfig::from(
        Some("secret".into()),
        Some("token".into()),
        Some("not a repo".into()),
        None,
        Some("turnstile-secret".into()),
        Some("recipes.example".into()),
    )
    .is_none());
}

#[test]
fn config_requires_a_turnstile_secret_and_hostname() {
    use recibase_submit::config::RecipeSubmissionConfig;
    assert!(RecipeSubmissionConfig::from(
        Some("secret".into()),
        Some("token".into()),
        None,
        None,
        None,
        Some("recipes.example".into()),
    )
    .is_none());
    assert!(RecipeSubmissionConfig::from(
        Some("secret".into()),
        Some("token".into()),
        None,
        None,
        Some("turnstile-secret".into()),
        Some("  ".into()),
    )
    .is_none());
}

#[test]
fn turnstile_rejects_a_missing_token() {
    use recibase_submit::turnstile::Turnstile;
    let hostnames: std::collections::HashSet<String> =
        ["recipes.example".to_string()].into_iter().collect();
    assert!(!Turnstile::token_accepted("", &hostnames));
    assert!(!Turnstile::token_accepted(&"x".repeat(2049), &hostnames));
    assert!(!Turnstile::token_accepted("token", &std::collections::HashSet::new()));
    assert!(Turnstile::token_accepted("token", &hostnames));
}

#[test]
fn docs_map_lists_every_endpoint() {
    let response = route(&request("GET", "/"), &context());
    assert_eq!(response.status, 200);
    let json = body(&response);
    assert!(json.contains("\"health\":\"/health\""));
    assert!(json.contains("\"meal_names_text\":\"/meals/raw\""));
}

#[test]
fn unknown_paths_are_not_found() {
    for target in ["/nope", "/recipes/a/b", "/meals", "/recipes", "/recipe-submissions"] {
        let response = route(&request("GET", target), &context());
        assert_eq!(response.status, 404, "{}", target);
        assert_eq!(body(&response), "Not found", "{}", target);
    }
    let response = route(&request("POST", "/health"), &context());
    assert_eq!(response.status, 404);
}

#[test]
fn meals_raw_is_sorted_and_filtered_to_dinners() {
    let response = route(&request("GET", "/meals/raw"), &context());
    assert_eq!(response.status, 200);
    let text = body(&response);
    let names: Vec<&str> = text.split('\n').collect();
    let mut sorted = names.clone();
    sorted.sort();
    assert_eq!(names, sorted);
    // Non-dinner meals (puddings, lunches, baking, non-meals) are filtered out.
    assert!(!names.contains(&"Birthday Cake"));
    assert!(names.contains(&"Vegetable Primavera"));
    let _ = to_string;
}
