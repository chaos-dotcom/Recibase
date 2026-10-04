//! `test_routes.py`, ported for the routes that are not `contribute`'s.
//!
//! `conftest.py` mocks `requests` and drives the Flask test client. Here a
//! real stub API runs on a loopback socket, an `App` is pointed at it and
//! requests go straight to `App::handle`, so the assertions cover the route,
//! the template engine and the HTTP client together.
//!
//! `contribute.rs` has its own suite (`contribute_spec.rs`); the contribute
//! cases here are thin on purpose - what they check is that the *route* wires
//! the module to the form, to the API and to the template.

mod support;

use serde_json::json;
use support::*;

use recibase_frontend::app::App;
use recibase_frontend::peer::{Peer, ServerIdentity};

// -- the ported `test_routes.py` cases --------------------------------

/// `test_homepage`.
#[test]
fn homepage() {
    let stub = StubApi::start();
    let app = app_on(&stub);
    let response = app.handle(&get("/"));
    assert_eq!(response.status, 200);
    assert!(body_of(&response).contains("Recibase"));
}

/// `test_manifest`. The Python asserts `app.frontendVersion` and the
/// configured backend URL; here those are what the test passed to `App::new`.
#[test]
fn manifest() {
    let stub = StubApi::start();
    let app = app_on(&stub);
    let response = app.handle(&get("/manifest.json"));
    assert_eq!(response.status, 200);
    assert_eq!(header_or(&response, "Content-Type"), "text/json");
    let body: serde_json::Value =
        serde_json::from_str(&body_of(&response)).expect("the manifest is JSON");
    assert_eq!(
        body,
        json!({"version": app.frontend_version, "apiUrl": stub.base_url()}),
    );
}

/// `test_sitemap`.
#[test]
fn sitemap() {
    let stub = StubApi::start();
    let app = app_on(&stub);
    let response = app.handle(&get("/sitemap.xml"));
    assert_eq!(response.status, 200);
    let body = body_of(&response);
    assert!(body.contains("<urlset"), "{body}");
    assert!(body.contains("test-recipe"), "{body}");
}

/// `test_random_recipe_redirects`. `random.choice` of a one-recipe list is
/// that recipe, and `app.py` redirects to a relative location.
#[test]
fn random_recipe_redirects() {
    let stub = StubApi::start();
    let app = app_on(&stub);
    let response = app.handle(&get("/random"));
    assert_eq!(response.status, 302);
    assert!(header_or(&response, "Location").ends_with("test-recipe"));
}

/// The drawer's reci-verse toggle, unticked, keeps to our own recipes and sends
/// `?onlyOurs=true`; the draw must stay within the recipes the API marks `ours`.
#[test]
fn random_recipe_redirects_respects_only_ours() {
    let stub = StubApi::start();
    stub.on(
        "GET",
        "/recipes/",
        Answer::json(
            r#"[{"permalink": "our-recipe", "name": "Our Recipe", "ours": true},
                 {"permalink": "their-recipe", "name": "Their Recipe"}]"#,
        ),
    );
    let app = app_on(&stub);
    for _ in 0..20 {
        let response = app.handle(&get("/random?onlyOurs=true"));
        assert_eq!(response.status, 302);
        assert!(header_or(&response, "Location").ends_with("our-recipe"));
    }
}

/// A peer's recipes appear in the drawer, labelled with their name and linked
/// to *our* page for them; a recipe of ours with the same name wins and gains
/// the "also on" hint.
#[test]
fn peer_recipes_appear_in_the_drawer() {
    let stub = StubApi::start();
    let peer = StubApi::start();
    peer.on(
        "GET",
        "/recipes/",
        Answer::json(
            r#"[{"name": "Their Recipe", "permalink": "their-recipe"},
                 {"name": "Test Recipe", "permalink": "test-recipe"}]"#,
        ),
    );
    let app = App::with_peers(
        stub.base_url().to_string(),
        "latest".to_string(),
        8080,
        vec![Peer::new(
            "Kit & Alex".to_string(),
            peer.base_url().to_string(),
            "https://reciba.se".to_string(),
        )],
    );

    let body = body_of(&app.handle(&get("/")));
    assert!(body.contains("Their Recipe"), "{body}");
    assert!(body.contains("recipe-source\">Kit &amp; Alex"), "{body}");
    // The link stays on this frontend: a local permalink, not their site.
    assert!(body.contains("href=\"their-recipe\""), "{body}");
    assert!(!body.contains("https://reciba.se/their-recipe"), "{body}");
    // Our same-named recipe wins, with the hint pointing at theirs.
    assert!(body.contains("title=\"Also on Kit &amp; Alex\""), "{body}");
}

/// Clicking a peer's recipe keeps you here: the page is rendered from the
/// peer's API, credited to them in the caption, and carries no "also on" hint
/// (the peer *is* the other place).
#[test]
fn a_peer_recipe_is_served_by_this_frontend() {
    let stub = StubApi::start();
    let peer = StubApi::start();
    peer.on(
        "GET",
        "/recipes/",
        Answer::json(r#"[{"name": "Their Recipe", "permalink": "their-recipe"}]"#),
    );
    let mut recipe = sample_recipe();
    recipe["name"] = json!("Their Recipe");
    recipe["permalink"] = json!("their-recipe");
    peer.on(
        "GET",
        "/recipes/their-recipe",
        Answer::json(recipe.to_string()),
    );
    let app = App::with_peers(
        stub.base_url().to_string(),
        "latest".to_string(),
        8080,
        vec![Peer::new(
            "Kit & Alex".to_string(),
            peer.base_url().to_string(),
            "https://reciba.se".to_string(),
        )],
    );

    let response = app.handle(&get("/their-recipe"));
    assert_eq!(response.status, 200);
    let body = body_of(&response);
    assert!(body.contains("Their Recipe"), "{body}");
    assert!(
        body.contains("Found on <a href=\"https://reciba.se\">Kit &amp; Alex</a> across the"),
        "{body}"
    );
    assert!(!body.contains("class=\"recipe-also\""), "{body}");
}

/// A permalink no one holds is still the 404 page.
#[test]
fn an_unknown_recipe_is_not_found_even_with_peers() {
    let stub = StubApi::start();
    let peer = StubApi::start();
    peer.on(
        "GET",
        "/recipes/",
        Answer::json(r#"[{"name": "Their Recipe", "permalink": "their-recipe"}]"#),
    );
    let app = App::with_peers(
        stub.base_url().to_string(),
        "latest".to_string(),
        8080,
        vec![Peer::new(
            "Kit & Alex".to_string(),
            peer.base_url().to_string(),
            "https://reciba.se".to_string(),
        )],
    );

    let response = app.handle(&get("/nobody-has-this"));
    assert_eq!(response.status, 404);
    assert!(body_of(&response).contains("404"), "{}", body_of(&response));
}

/// The recipe page names and links this deployment's own server when
/// `SERVER_IDENTITY` is set, instead of the generic "this server".
#[test]
fn the_recipe_page_names_and_links_our_server() {
    let stub = StubApi::start();
    let app = App::with_peers(
        stub.base_url().to_string(),
        "latest".to_string(),
        8080,
        Vec::new(),
    )
    .with_server(Some(ServerIdentity {
        label: "Kit & Alex".to_string(),
        site_url: "https://reciba.se".to_string(),
    }));

    let body = body_of(&app.handle(&get("/test-recipe")));
    assert!(
        body.contains("Found on <a href=\"https://reciba.se\">Kit &amp; Alex</a> across the <span class=\"reci-verse__term\">reci-verse</span>."),
        "{body}"
    );
}

/// With no own server configured, the caption keeps its generic wording.
#[test]
fn the_recipe_page_falls_back_to_this_server() {
    let stub = StubApi::start();
    let app = app_on(&stub);
    let body = body_of(&app.handle(&get("/test-recipe")));
    assert!(
        body.contains(
            "Found on this server across the <span class=\"reci-verse__term\">reci-verse</span>."
        ),
        "{body}"
    );
}

/// `test_recipe_page`.
#[test]
fn recipe_page() {
    let stub = StubApi::start();
    let app = app_on(&stub);
    let response = app.handle(&get("/test-recipe"));
    assert_eq!(response.status, 200);
    let body = body_of(&response);
    assert!(body.contains("Test Recipe"), "{body}");
    assert!(body.contains("Chop onion"), "{body}");
}

/// `test_recipe_lowercase_redirect`.
#[test]
fn recipe_lowercase_redirect() {
    let stub = StubApi::start();
    let app = app_on(&stub);
    let response = app.handle(&get("/Test-Recipe"));
    assert_eq!(response.status, 301);
    assert!(header_or(&response, "Location").ends_with("/test-recipe"));
}

/// `test_recipe_not_found`.
#[test]
fn recipe_not_found() {
    let stub = StubApi::start();
    let app = app_on(&stub);
    let response = app.handle(&get("/missing-recipe"));
    assert_eq!(response.status, 404);
    assert!(body_of(&response).contains("404"));
}

/// `test_recipe_scaling`.
#[test]
fn recipe_scaling() {
    let stub = StubApi::start();
    let app = app_on(&stub);
    let response = app.handle(&get("/test-recipe?scale=2"));
    assert_eq!(response.status, 200);
    let body = body_of(&response);
    assert!(body.contains('4'), "{body}");
    // The Python only looks for the digit; this says where it comes from.
    assert!(body.contains("Onion: 4"), "{body}");
}

/// `test_homepage_backend_unavailable`. The Python patches
/// `app.fetchRecipeList` to raise; here the API is not reachable at all.
#[test]
fn homepage_backend_unavailable() {
    let app = app(&format!("http://127.0.0.1:{}/", closed_port()), "latest");
    let response = app.handle(&get("/"));
    assert_eq!(response.status, 503);
    assert!(body_of(&response).contains("Backend Unavailable"));
}

/// `test_recipe_backend_unavailable`.
#[test]
fn recipe_backend_unavailable() {
    let app = app(&format!("http://127.0.0.1:{}/", closed_port()), "latest");
    let response = app.handle(&get("/test-recipe"));
    assert_eq!(response.status, 503);
    assert!(body_of(&response).contains("Backend Unavailable"));
}

/// The "backend error" companion of `test_recipe_backend_unavailable`: the
/// API answers, but it answers 500, which `app.py` turns into the 503 page.
#[test]
fn recipe_backend_error_is_503() {
    let stub = StubApi::start();
    stub.on("GET", "/recipes/test-recipe", Answer::empty(500));
    let app = app_on(&stub);
    let response = app.handle(&get("/test-recipe"));
    assert_eq!(response.status, 503);
    assert!(body_of(&response).contains("Backend Unavailable"));
}

/// `test_contribute_page`: the tags, the Turnstile widget and the draft
/// buttons the drawer link's page carries.
#[test]
fn contribute_page() {
    let stub = StubApi::start();
    let app = app_on(&stub);
    let response = app.handle(&get("/contribute"));
    assert_eq!(response.status, 200);
    let body = body_of(&response);
    for expected in [
        "Add a recipe",
        "name=\"passcode\"",
        "value=\"VegetarianIsh\"",
        "value=\"GlutenFree\"",
        "Gluten-Free",
        "class=\"cf-turnstile\"",
        "data-sitekey=\"0x4AAAAAAFFMifPD-G1JDoui\"",
        "data-action=\"contribute\"",
        "https://challenges.cloudflare.com/turnstile/v0/api.js",
        "Drafts saved for 7 days using a cookie, or until submitted.",
        "Save Draft",
        "Delete Draft",
        "mdl-navigation__link add-recipe is-current",
    ] {
        assert!(body.contains(expected), "missing {expected} in {body}");
    }
    for unexpected in ["value=\"NeverEaten\"", "value=\"Popular\"", "value=\"New\""] {
        assert!(
            !body.contains(unexpected),
            "unexpected {unexpected} in {body}"
        );
    }
}

/// `test_contribute_link_in_drawer`: the drawer carries the link everywhere,
/// and only marks it current on the contribute page.
#[test]
fn contribute_link_in_drawer() {
    let stub = StubApi::start();
    let app = app_on(&stub);
    let response = app.handle(&get("/"));
    assert_eq!(response.status, 200);
    let body = body_of(&response);
    assert!(body.contains("href=\"/contribute\""), "{body}");
    assert!(!body.contains("add-recipe is-current"), "{body}");
}

/// `test_contribute_submits_recipe`: the route posts the module's payload and
/// header to `recipe-submissions` and shows the pull request link.
///
/// `requests.post(..., timeout=30)` has no on-the-wire equivalent: the Rust
/// client's timeout is configured in `backend.rs` and is not observable from
/// the test, so it is not asserted here.
#[test]
fn contribute_submits_recipe() {
    let stub = StubApi::start();
    let app = app_on(&stub);
    let response = app.handle(&post_form(
        "/contribute",
        &[
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
        ],
    ));
    assert_eq!(response.status, 200);
    let body = body_of(&response);
    assert!(
        body.contains("https://github.com/chaos-dotcom/Recibase/pull/12"),
        "{body}"
    );
    assert!(!body.contains("name=\"passcode\""), "{body}");

    let posted = stub.single_request("POST", "/recipe-submissions");
    assert!(posted.target.ends_with("recipe-submissions"));
    assert_eq!(posted.header("Content-Type"), Some("application/json"));
    assert_eq!(posted.header("Authorization"), Some("Bearer secret"));
    assert_eq!(
        posted.json(),
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
        }),
    );
}

/// `test_contribute_keeps_form_on_api_error`: a plain-text refusal is shown
/// and the form is redisplayed as posted.
#[test]
fn contribute_keeps_form_on_api_error() {
    let stub = StubApi::start();
    stub.on(
        "POST",
        "/recipe-submissions",
        Answer::text(401, "text/plain", "invalid passcode"),
    );
    let app = app_on(&stub);
    let response = app.handle(&post_form(
        "/contribute",
        &[
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
        ],
    ));
    assert_eq!(response.status, 200);
    let body = body_of(&response);
    for expected in [
        "invalid passcode",
        "value=\"Soup\"",
        "value=\"Onion\"",
        "value=\"Optional\"",
        "value=\"Spicy\" checked",
    ] {
        assert!(body.contains(expected), "missing {expected} in {body}");
    }
    assert!(!body.contains("Pull request opened"), "{body}");
}

/// `test_contribute_shows_json_error`.
#[test]
fn contribute_shows_json_error() {
    let stub = StubApi::start();
    stub.on(
        "POST",
        "/recipe-submissions",
        Answer::json_status(400, r#"{"error": "Add at least one ingredient."}"#),
    );
    let app = app_on(&stub);
    let response = app.handle(&post_form(
        "/contribute",
        &[
            ("passcode", "secret"),
            ("name", "Empty"),
            ("method", "Stir."),
        ],
    ));
    assert_eq!(response.status, 200);
    assert!(body_of(&response).contains("Add at least one ingredient."));
}

/// `test_contribute_rejects_unexpected_pull_request_url`: a URL the module
/// does not trust is neither shown nor linked, and the form comes back.
#[test]
fn contribute_rejects_unexpected_pull_request_url() {
    let stub = StubApi::start();
    stub.on(
        "POST",
        "/recipe-submissions",
        Answer::json(r#"{"url": "javascript:alert(1)"}"#),
    );
    let app = app_on(&stub);
    let response = app.handle(&post_form(
        "/contribute",
        &[
            ("passcode", "secret"),
            ("name", "Soup"),
            ("ingredient_name", "Onion"),
            ("ingredient_quantity", "1"),
            ("ingredient_prep", ""),
            ("ingredient_notes", ""),
            ("method", "Simmer."),
        ],
    ));
    let body = body_of(&response);
    assert!(!body.contains("javascript:alert(1)"), "{body}");
    assert!(
        body.contains("did not return a pull request link"),
        "{body}"
    );
    assert!(body.contains("value=\"Soup\""), "{body}");
}

/// `test_contribute_escapes_redisplayed_values`: a hostile recipe name is
/// escaped when the form is redisplayed, and the API's message still shows.
#[test]
fn contribute_escapes_redisplayed_values() {
    let stub = StubApi::start();
    stub.on(
        "POST",
        "/recipe-submissions",
        Answer::text(502, "text/plain", "upstream failed"),
    );
    let app = app_on(&stub);
    let response = app.handle(&post_form(
        "/contribute",
        &[
            ("passcode", "secret"),
            ("name", "<script>alert(1)</script>"),
            ("ingredient_name", "Onion"),
            ("ingredient_quantity", "1"),
            ("ingredient_prep", ""),
            ("ingredient_notes", ""),
            ("method", "Simmer."),
        ],
    ));
    let body = body_of(&response);
    assert!(!body.contains("<script>alert(1)</script>"), "{body}");
    assert!(
        body.contains("&lt;script&gt;alert(1)&lt;/script&gt;"),
        "{body}"
    );
    assert!(body.contains("upstream failed"), "{body}");
}

/// `test_contribute_reports_unreachable_api`: an unreachable API is the
/// form's message, not the 503 page.
///
/// The Python test patches only `requests.post`, so `fetchRecipeList` still
/// answers and the layout can be rendered. The stub does the same by dropping
/// the connection on `recipe-submissions` alone. (Point an `App` at a dead
/// port and the *page* cannot be rendered either, so the answer is the 503
/// page - which is what Flask itself would do with a dead backend.)
#[test]
fn contribute_reports_unreachable_api() {
    let stub = StubApi::start();
    stub.on("POST", "/recipe-submissions", Answer::hangup());
    let app = app_on(&stub);
    let response = app.handle(&post_form(
        "/contribute",
        &[
            ("passcode", "secret"),
            ("name", "Soup"),
            ("method", "Simmer."),
        ],
    ));
    assert_eq!(response.status, 200);
    let body = body_of(&response);
    assert!(body.contains("Could not reach the recipe API"), "{body}");
    assert!(body.contains("value=\"Soup\""), "{body}");
    assert!(!body.contains("Backend Unavailable"), "{body}");
}

/// `test_recipe_copy_ingredients_are_premerged`: the clipboard text is
/// merged before it reaches the page, and the old per-ingredient attributes
/// are gone.
#[test]
fn recipe_copy_ingredients_are_premerged() {
    let stub = StubApi::start();
    let app = app_on(&stub);
    let response = app.handle(&get("/test-recipe"));
    assert_eq!(response.status, 200);
    let body = body_of(&response);
    assert!(!body.contains("x-quantity"), "{body}");
    assert!(!body.contains("x-ingredient"), "{body}");
    assert!(body.contains("data-ingredients="), "{body}");
    assert!(body.contains("200g Butter"), "{body}");
    assert!(body.contains("2 Onion"), "{body}");
}

// -- the method and static-file cases the brief asks for -------------

/// `HEAD /` carries the `Content-Length` of the `GET` and sends no body.
#[test]
fn head_matches_get_without_a_body() {
    let stub = StubApi::start();
    let app = app_on(&stub);
    let get_response = app.handle(&get("/"));
    let head_response = app.handle(&head("/"));
    assert_eq!(head_response.status, 200);
    assert_eq!(
        header_or(&head_response, "Content-Length"),
        header_or(&get_response, "Content-Length"),
    );

    // `to_bytes` is what the server writes, and `head_only` is how a HEAD
    // request is written: headers and `Content-Length`, no body.
    let request = head("/");
    let bytes = head_response.to_bytes("Mon, 01 Jan 2024 00:00:00 GMT", "keep-alive", true);
    let text = String::from_utf8_lossy(&bytes).into_owned();
    let (headers, body) = text.split_once("\r\n\r\n").expect("a header block");
    assert!(headers.contains("Content-Length: "), "{headers}");
    assert!(body.is_empty(), "{body}");
    assert!(request.is_head());
}

/// `PUT /contribute` is a 405 that lists the methods the route does allow.
#[test]
fn put_contribute_is_405_with_allow() {
    let stub = StubApi::start();
    let app = app_on(&stub);
    let response = app.handle(&request("PUT", "/contribute"));
    assert_eq!(response.status, 405);
    assert_eq!(header_or(&response, "Allow"), "HEAD, POST, GET, OPTIONS");
    assert!(body_of(&response).contains("405 Method Not Allowed"));
}

/// `OPTIONS /` is a 200 with no body and the `Allow` header.
#[test]
fn options_home_is_empty_200_with_allow() {
    let stub = StubApi::start();
    let app = app_on(&stub);
    let response = app.handle(&request("OPTIONS", "/"));
    assert_eq!(response.status, 200);
    assert!(response.body.is_empty());
    assert_eq!(header_or(&response, "Allow"), "HEAD, GET, OPTIONS");
}

/// `/static/styles.css` is served with an `ETag`, answers 304 to a matching
/// `If-None-Match`, and answers 206 to `Range: bytes=0-9`.
#[test]
fn static_styles_css_is_served_conditionally() {
    root_statics_at_source();
    let stub = StubApi::start();
    let app = app_on(&stub);

    let response = app.handle(&get("/static/styles.css"));
    assert_eq!(response.status, 200);
    let etag = header_or(&response, "ETag").to_string();
    assert!(
        etag.starts_with('"') && etag.ends_with('"') && etag.len() > 2,
        "{etag}"
    );
    assert_eq!(header_or(&response, "Accept-Ranges"), "bytes");
    assert_eq!(
        header_or(&response, "Content-Type"),
        "text/css; charset=utf-8"
    );
    let on_disk = std::fs::read(static_dir().join("styles.css")).expect("static/styles.css");
    assert_eq!(response.body, on_disk);

    let conditional = app.handle(&with_header(
        get("/static/styles.css"),
        "If-None-Match",
        &etag,
    ));
    assert_eq!(conditional.status, 304);
    assert!(conditional.body.is_empty());
    assert_eq!(header_or(&conditional, "ETag"), etag.as_str());

    let ranged = app.handle(&with_header(
        get("/static/styles.css"),
        "Range",
        "bytes=0-9",
    ));
    assert_eq!(ranged.status, 206);
    assert_eq!(ranged.body, on_disk[..10]);
    assert_eq!(
        header_or(&ranged, "Content-Range"),
        format!("bytes 0-9/{}", on_disk.len()),
    );
}
