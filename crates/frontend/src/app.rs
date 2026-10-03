//! `app.py`: the routes.
//!
//! | route | method | page |
//! |---|---|---|
//! | `/` | GET | `home.html` |
//! | `/manifest.json` | GET | the frontend version and the API URL |
//! | `/sitemap.xml` | GET | `sitemap.xml` |
//! | `/random` | GET | 302 to a recipe chosen at random |
//! | `/contribute` | GET, POST | `contribute.html` |
//! | `/static/<filename>` | GET | the file, conditionally |
//! | `/<name>` | GET | `recipe.html`, 301 if the name is not lower case |
//!
//! Anything else is the 404 page; an unmatched method on a matched path is
//! Werkzeug's 405 with its `Allow` header; and a failure to reach the API is
//! the 503 page.

use std::sync::Arc;

use minijinja::value::Value as TemplateValue;
use serde_json::{Map, Value, json};

use crate::backend::BackendClient;
use crate::cached_backend::{BackendUnavailable, CachedBackendCall};
use crate::http::{Request, Response};
use crate::peer::Peer;
use crate::statics::{Outcome, StaticFiles};
use crate::templates::{Templates, is_backend_unavailable};
use crate::{contribute, pages, scaler};

pub enum RouteError {
    BackendUnavailable,
    InternalError,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Route {
    Home,
    Manifest,
    Sitemap,
    Random,
    Contribute,
    Static(String),
    Recipe(String),
}

impl Route {
    pub fn match_path(path: &str) -> Option<Route> {
        match path {
            "/" => return Some(Route::Home),
            "/manifest.json" => return Some(Route::Manifest),
            "/sitemap.xml" => return Some(Route::Sitemap),
            "/random" => return Some(Route::Random),
            "/contribute" => return Some(Route::Contribute),
            _ => {}
        }
        if let Some(filename) = path.strip_prefix("/static/") {
            return Some(Route::Static(filename.to_string()));
        }
        if let Some(name) = path.strip_prefix('/')
            && !name.is_empty()
            && !name.contains('/')
        {
            return Some(Route::Recipe(name.to_string()));
        }
        None
    }

    /// Werkzeug lists `HEAD` and `OPTIONS` for every `GET` rule.
    pub fn methods(&self) -> &'static [&'static str] {
        match self {
            Route::Contribute => &["HEAD", "POST", "GET", "OPTIONS"],
            _ => &["HEAD", "GET", "OPTIONS"],
        }
    }

    pub fn allows(&self, method: &str) -> bool {
        self.methods().contains(&method)
    }
}

pub struct App {
    pub backend: Arc<BackendClient>,
    pub backend_url: String,
    pub frontend_version: String,
    pub templates: Templates,
    pub statics: StaticFiles,
    pub recipe_list: Arc<CachedBackendCall<serde_json::Value>>,
    pub api_version: Arc<CachedBackendCall<String>>,
    /// Other deployments whose recipes we list alongside ours.
    pub peers: Vec<Peer>,
    pub fallback_host: String,
}

impl App {
    pub fn new(backend_url: String, frontend_version: String, port: u16) -> App {
        // `PEER_BACKENDS`, e.g.
        // `Kit & Alex|https://api.reciba.se/|https://reciba.se`.
        let peers =
            crate::peer::parse_peer_backends(&std::env::var("PEER_BACKENDS").unwrap_or_default());
        App::with_peers(backend_url, frontend_version, port, peers)
    }

    /// [`App::new`], with the peers supplied rather than read from the
    /// environment (the tests use this).
    pub fn with_peers(
        backend_url: String,
        frontend_version: String,
        port: u16,
        peers: Vec<Peer>,
    ) -> App {
        let backend = Arc::new(BackendClient::new(backend_url.clone()));
        let recipe_list = Arc::new(CachedBackendCall::new({
            let backend = Arc::clone(&backend);
            move || backend.get_json("recipes/")
        }));
        let api_version = Arc::new(CachedBackendCall::new({
            let backend = Arc::clone(&backend);
            move || -> Result<String, BackendUnavailable> {
                let manifest = backend.get_json("manifest")?;
                manifest
                    .get("version")
                    .and_then(|version| version.as_str())
                    .map(|version| version.to_string())
                    .ok_or(BackendUnavailable)
            }
        }));
        let templates = Templates::with_peers(
            Arc::clone(&recipe_list),
            Arc::clone(&api_version),
            &frontend_version,
            peers.clone(),
        );
        App {
            backend,
            backend_url,
            frontend_version,
            templates,
            statics: StaticFiles::from_env(),
            recipe_list,
            api_version,
            peers,
            fallback_host: format!("localhost:{}", port),
        }
    }

    pub fn handle(&self, request: &Request) -> Response {
        match self.dispatch(request) {
            Ok(response) => response,
            Err(RouteError::BackendUnavailable) => self.backend_unavailable(),
            Err(RouteError::InternalError) => self.internal_error(),
        }
    }

    fn dispatch(&self, request: &Request) -> Result<Response, RouteError> {
        let method = request.method.to_ascii_uppercase();
        let route = match Route::match_path(&request.path) {
            Some(route) => route,
            None => return self.not_found(),
        };
        if method == "OPTIONS" {
            return Ok(Response::new(200)
                .header("Content-Type", "text/html; charset=utf-8")
                .header("Allow", route.methods().join(", "))
                .header("Content-Length", "0"));
        }
        if !route.allows(&method) {
            return Ok(pages::method_not_allowed(route.methods()));
        }
        match route {
            Route::Home => self.render("home.html", empty_context()),
            Route::Manifest => Ok(self.manifest(request)),
            Route::Sitemap => self.sitemap(request),
            Route::Random => self.random(request),
            Route::Contribute => self.contribute(request),
            Route::Static(filename) => self.static_file(request, &filename),
            Route::Recipe(name) => self.recipe(request, &name),
        }
    }

    fn render(&self, name: &str, context: TemplateValue) -> Result<Response, RouteError> {
        match self.templates.render(name, context) {
            Ok(rendered) => Ok(Response::html(200, rendered)),
            Err(error) if is_backend_unavailable(&error) => Err(RouteError::BackendUnavailable),
            Err(_) => Err(RouteError::InternalError),
        }
    }

    fn not_found(&self) -> Result<Response, RouteError> {
        match self.templates.render("notfound.html", empty_context()) {
            Ok(rendered) => Ok(Response::html(404, rendered)),
            Err(error) if is_backend_unavailable(&error) => Err(RouteError::BackendUnavailable),
            Err(_) => Err(RouteError::InternalError),
        }
    }

    fn internal_error(&self) -> Response {
        match self.templates.render("internalerror.html", empty_context()) {
            Ok(rendered) => Response::html(500, rendered),
            Err(_) => Response::html(500, "<h1>500 - Internal Error</h1>".to_string()),
        }
    }

    fn backend_unavailable(&self) -> Response {
        match self
            .templates
            .render("backendunavailable.html", empty_context())
        {
            Ok(rendered) => Response::html(503, rendered),
            Err(_) => Response::html(503, "<h1>503 - Backend Unavailable</h1>".to_string()),
        }
    }

    /// `json.dumps({'version': ..., 'apiUrl': ...})` with the `text/json`
    /// content type `app.py` sets.
    fn manifest(&self, _request: &Request) -> Response {
        let body = format!(
            "{{\"version\": {}, \"apiUrl\": {}}}",
            json_string(&self.frontend_version),
            json_string(&self.backend_url)
        );
        Response::new(200)
            .header("Content-Type", "text/json")
            .header("Content-Length", body.len().to_string())
            .body(body.into_bytes())
    }

    fn sitemap(&self, request: &Request) -> Result<Response, RouteError> {
        let base_url = request.url_root().trim_end_matches('/').to_string();
        let context = TemplateValue::from_serialize(json!({ "baseUrl": base_url }));
        self.render("sitemap.xml", context)
    }

    /// `redirect(random.choice(fetchRecipeList())['permalink'], 302)`. The
    /// target carries no leading slash, so the `Location` header is relative.
    ///
    /// `?onlyOurs=true` restricts the draw to the recipes the API marks
    /// `ours`, which is what the drawer's "only ours" toggle asks for.
    fn random(&self, request: &Request) -> Result<Response, RouteError> {
        let recipes = self
            .recipe_list
            .fetch_data()
            .map_err(|_| RouteError::BackendUnavailable)?;
        let Some(recipes) = recipes.as_array() else {
            return Err(RouteError::InternalError);
        };
        let only_ours = request.query_param("onlyOurs").as_deref() == Some("true");
        let candidates: Vec<&Value> = recipes
            .iter()
            .filter(|recipe| {
                !only_ours || recipe.get("ours").and_then(Value::as_bool) == Some(true)
            })
            .collect();
        if candidates.is_empty() {
            // `random.choice([])` raises IndexError, which is a 500.
            return Err(RouteError::InternalError);
        }
        let index = random_index(candidates.len());
        let Some(permalink) = candidates[index]
            .get("permalink")
            .and_then(|version| version.as_str())
        else {
            return Err(RouteError::InternalError);
        };
        Ok(pages::redirect(302, permalink))
    }

    fn static_file(&self, request: &Request, filename: &str) -> Result<Response, RouteError> {
        match self.statics.serve(request, filename) {
            Outcome::Served(response) => Ok(response),
            Outcome::NotFound => self.not_found(),
        }
    }

    /// `render_contribute` from `app.py`.
    fn render_contribute(
        &self,
        form: Option<&crate::form::Form>,
        error: Option<&str>,
        pr_url: Option<&str>,
    ) -> Result<Response, RouteError> {
        let context = TemplateValue::from_serialize(json!({
            "recipeUrl": "contribute",
            "tag_groups": contribute::TAG_GROUPS
                .iter()
                .map(|(name, tags)| {
                    json!([name, tags.iter().map(|(v, l)| json!([v, l])).collect::<Vec<_>>()])
                })
                .collect::<Vec<_>>(),
            "diet_checkboxes": contribute::DIET_CHECKBOXES
                .iter()
                .map(|(v, l)| json!([v, l]))
                .collect::<Vec<_>>(),
            "form": contribute::page_state(form),
            "error": error,
            "pr_url": pr_url,
        }));
        self.render("contribute.html", context)
    }

    fn contribute(&self, request: &Request) -> Result<Response, RouteError> {
        if request.method.eq_ignore_ascii_case("GET") || request.method.eq_ignore_ascii_case("HEAD")
        {
            return self.render_contribute(None, None, None);
        }

        let form = request.form();
        let payload = contribute::submission_payload(&form);
        let authorization = contribute::authorization_header(&form);
        let response = match self
            .backend
            .post_json("recipe-submissions", &payload, &authorization)
        {
            Ok(response) => response,
            Err(_) => {
                return self.render_contribute(
                    Some(&form),
                    Some("Could not reach the recipe API. Nothing was submitted."),
                    None,
                );
            }
        };

        if response.status == 200 {
            let body: Option<serde_json::Value> = serde_json::from_str(&response.text).ok();
            let url = body.as_ref().and_then(contribute::pull_request_url);
            return match url {
                Some(url) => self.render_contribute(None, None, Some(&url)),
                None => self.render_contribute(
                    Some(&form),
                    Some("The recipe API did not return a pull request link."),
                    None,
                ),
            };
        }

        let message =
            contribute::failure_message(response.status, &response.content_type, &response.text);
        self.render_contribute(Some(&form), Some(&message), None)
    }

    fn recipe(&self, request: &Request, name: &str) -> Result<Response, RouteError> {
        if name.to_lowercase() != name {
            return Ok(pages::redirect(301, &format!("/{}", name.to_lowercase())));
        }

        let response = self
            .backend
            .get(&format!("recipes/{}", name))
            .map_err(|_| RouteError::BackendUnavailable)?;
        if response.status == 404 {
            return self.not_found();
        }
        if !(200..300).contains(&response.status) {
            return Err(RouteError::BackendUnavailable);
        }
        let mut recipe: serde_json::Value =
            serde_json::from_str(&response.text).map_err(|_| RouteError::BackendUnavailable)?;

        let scale_factor = match scaler::get_scale_factor(request.query_param("scale").as_deref()) {
            Some(factor) => {
                scale_blocks(&mut recipe, factor);
                factor
            }
            None => 1.0,
        };

        let mut combined_notes: Vec<serde_json::Value> = recipe
            .get("notes")
            .and_then(Value::as_array)
            .cloned()
            .unwrap_or_default();
        if let Some(dated) = recipe.get("dated_notes").and_then(Value::as_array) {
            for note in dated {
                let date = note
                    .get("date")
                    .and_then(|version| version.as_str())
                    .unwrap_or_default();
                let text = note
                    .get("note")
                    .and_then(|version| version.as_str())
                    .unwrap_or_default();
                combined_notes.push(serde_json::Value::from(format!("{}: {}", date, text)));
            }
        }

        let empty = serde_json::Value::Array(Vec::new());
        let blocks = recipe.get("ingredients_blocks").unwrap_or(&empty);
        let copy_ingredients = scaler::ingredients_copy_text(blocks);

        let also = self.peer_matches(recipe.get("name").and_then(Value::as_str));
        let context = TemplateValue::from_serialize(json!({
            "recipe": recipe,
            "scale_factor": scale_factor,
            "combined_notes": combined_notes,
            "copy_ingredients": copy_ingredients,
            "also": also,
        }));
        self.render("recipe.html", context)
    }

    /// The peers that list a recipe with this name, as `{label, url}` for the
    /// "also on" hint on the recipe page. A peer that cannot be reached is
    /// skipped.
    fn peer_matches(&self, name: Option<&str>) -> Vec<Value> {
        let Some(name) = name else {
            return Vec::new();
        };
        let mut matches = Vec::new();
        for peer in &self.peers {
            let Ok(list) = peer.recipes.fetch_data() else {
                continue;
            };
            let Some(entries) = list.as_array() else {
                continue;
            };
            if let Some(entry) = entries
                .iter()
                .find(|entry| entry.get("name").and_then(Value::as_str) == Some(name))
            {
                let permalink = entry
                    .get("permalink")
                    .and_then(Value::as_str)
                    .unwrap_or_default();
                matches.push(json!({ "label": peer.label, "url": peer.recipe_url(permalink) }));
            }
        }
        matches
    }
}

/// `list(map(lambda b: dict(name=b['name'], ingredients=list(map(...))), ...))`:
/// every block is replaced by an object with exactly those two keys, and
/// each ingredient is scaled in place.
fn scale_blocks(recipe: &mut serde_json::Value, factor: f64) {
    let Some(blocks) = recipe
        .get_mut("ingredients_blocks")
        .and_then(Value::as_array_mut)
    else {
        return;
    };
    for block in blocks.iter_mut() {
        let name = block
            .get("name")
            .cloned()
            .unwrap_or(serde_json::Value::Null);
        if let Some(ingredients) = block.get_mut("ingredients").and_then(Value::as_array_mut) {
            for ingredient in ingredients.iter_mut() {
                scaler::scale_ingredient(ingredient, factor);
            }
        }
        let ingredients = block
            .get("ingredients")
            .cloned()
            .unwrap_or(Value::Array(Vec::new()));
        let mut replacement = Map::new();
        replacement.insert("name".to_string(), name);
        replacement.insert("ingredients".to_string(), ingredients);
        *block = serde_json::Value::Object(replacement);
    }
}

fn empty_context() -> TemplateValue {
    TemplateValue::from_serialize(json!({}))
}

/// `json.dumps` of a `str`, which escapes non-ASCII the way Python's
/// `ensure_ascii=True` does.
fn json_string(value: &str) -> String {
    let mut out = String::with_capacity(value.len() + 2);
    out.push('"');
    for ch in value.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            '\u{08}' => out.push_str("\\b"),
            '\u{0c}' => out.push_str("\\f"),
            ch if (ch as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", ch as u32)),
            ch if (ch as u32) < 0x7f => out.push(ch),
            ch => {
                let code = ch as u32;
                if code > 0xffff {
                    let code = code - 0x10000;
                    out.push_str(&format!(
                        "\\u{:04x}\\u{:04x}",
                        0xd800 + (code >> 10),
                        0xdc00 + (code & 0x3ff)
                    ));
                } else {
                    out.push_str(&format!("\\u{:04x}", code));
                }
            }
        }
    }
    out.push('"');
    out
}

/// `random.choice`: one uniformly chosen index, from the operating system's
/// entropy, which is all `std` offers without a dependency.
fn random_index(len: usize) -> usize {
    use std::hash::{BuildHasher, Hasher};
    use std::time::{SystemTime, UNIX_EPOCH};

    let mut hasher = std::collections::hash_map::RandomState::new().build_hasher();
    hasher.write_u128(
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|delta| delta.as_nanos())
            .unwrap_or(0),
    );
    hasher.write_usize(len);
    hasher.write_u64(std::process::id() as u64);
    (hasher.finish() % len as u64) as usize
}
