//! `RecibaseRoutes` and `RecipeSubmissionRoutes`.

use crate::auth::Admin;
use crate::controllers::{self, Usage};
use crate::db::{Db, MealEntryRow};
use crate::http::{Request, Response};
use crate::meal_log;
use recibase_core::json::obj;
use recibase_core::usage::MealLogEntry;
use serde_json::Value;

/// The process environment, injectable so the harness and the tests can supply
/// their own map.
pub type Env = Box<dyn Fn(&str) -> Option<String> + Send + Sync>;

pub struct Context {
    pub env: Env,
    /// The date the request is served on, pinned by `RECIBASE_TODAY`.
    pub today: chrono::NaiveDate,
    /// The CSV meal log, loaded once at start-up; the fallback usage store.
    csv_entries: Vec<MealLogEntry>,
    /// The Postgres store, when the tracker is configured.
    db: Option<Db>,
    /// The tracker routes, present only when the database and OIDC are
    /// configured.
    admin: Option<Admin>,
}

impl Context {
    pub fn new(env: Env) -> Context {
        let today = controllers::today(&*env);
        let csv_entries = meal_log::load_from_env(&*env);
        let db = crate::config::database_url(&*env).and_then(|url| match Db::connect(&url) {
            Ok(db) => Some(db),
            Err(error) => {
                eprintln!("database unavailable ({error}); admin tracker disabled");
                None
            }
        });
        let admin = db.as_ref().and_then(|db| {
            crate::config::AdminConfig::from_env(&*env).map(|config| Admin::new(db.clone(), config))
        });
        Context {
            env,
            today,
            csv_entries,
            db,
            admin,
        }
    }

    /// Usage data for the read-only endpoints. The database is the source of
    /// truth when it is configured; otherwise the start-up CSV log is used, so
    /// an unconfigured deployment behaves exactly as before.
    pub fn usage(&self) -> Usage {
        Usage {
            entries: self.current_entries(),
            today: self.today,
        }
    }

    fn current_entries(&self) -> Vec<MealLogEntry> {
        // The owner whose entries count as usage: pinned by the environment, or
        // adopted at first login. Until there is one, fall back to the CSV.
        let subject = (self.env)("RECIBASE_OWNER_SUBJECT").or_else(|| {
            self.admin
                .as_ref()
                .and_then(|admin| admin.owner_subject())
        });
        let (Some(db), Some(subject)) = (&self.db, subject) else {
            return self.csv_entries.clone();
        };
        match db.entries_for_subject(&subject) {
            Ok(rows) => rows.into_iter().map(entry_from_row).collect(),
            Err(error) => {
                eprintln!("usage query failed ({error}); using the CSV log");
                self.csv_entries.clone()
            }
        }
    }
}

fn entry_from_row(row: MealEntryRow) -> MealLogEntry {
    MealLogEntry::new(&row.meal_name, row.eaten_on, row.note, row.featured)
}

fn error_json(message: &str) -> Value {
    obj(vec![("error", Value::String(message.to_string()))])
}

/// `RecibaseRoutes.routes` for a single request, wrapped in the CORS
/// middleware the Scala server installs around the route set.
///
/// Measured behaviour: an `OPTIONS` request that carries both `Origin` and
/// `Access-Control-Request-Method` is answered with an empty preflight 200;
/// any other request with an `Origin` gets `Access-Control-Allow-Origin: *`
/// appended to the response its route produced. A request that matches no
/// route at all is answered by `orNotFound` outside the middleware, so it
/// carries no CORS header.
///
/// The admin JSON API is the exception: the browser calls it cross-origin with
/// credentials, so those responses reflect the caller's origin and allow
/// credentials instead of using the wildcard.
pub fn route(request: &Request, context: &Context) -> Response {
    let credentialed = request.path.starts_with("/admin/api/");
    let origin = request.header("Origin").map(str::to_string);
    if request.method == "OPTIONS" {
        let requested_method = request.header("Access-Control-Request-Method").is_some();
        if let Some(origin) = &origin
            && requested_method
        {
            // http4s echoes the requested header list with each name trimmed
            // and re-joined with ", ".
            let requested = request
                .header("Access-Control-Request-Headers")
                .unwrap_or_default();
            let echoed = requested
                .split(',')
                .map(str::trim)
                .filter(|name| !name.is_empty())
                .collect::<Vec<_>>()
                .join(", ");
            if credentialed {
                return Response::credentialed_preflight(origin, &echoed);
            }
            return Response::preflight(&echoed);
        }
    }
    let (mut response, matched) = dispatch(request, context);
    if matched && let Some(origin) = &origin {
        if credentialed {
            response
                .headers_before_length
                .push(("Access-Control-Allow-Origin", origin.clone()));
            response.headers_before_length.push((
                "Access-Control-Allow-Credentials",
                "true".to_string(),
            ));
            response
                .headers_before_length
                .push(("Vary", "Origin".to_string()));
        } else {
            response
                .headers_after_length
                .push(("Access-Control-Allow-Origin", "*"));
        }
    }
    response
}

/// Returns the response and whether a route matched (so the CORS layer knows
/// whether to decorate it).
fn dispatch(request: &Request, context: &Context) -> (Response, bool) {
    if request.method == "GET" {
        let path = request.path.as_str();
        if path == "/" {
            return (Response::json(&controllers::docs()), true);
        }
        if path == "/health" {
            return (Response::plain("ok"), true);
        }
        if path == "/manifest" {
            return (
                Response::json(&controllers::manifest_json(&*context.env)),
                true,
            );
        }
        if path == "/recipes/" {
            let ingredient = request.query_param("hasIngredient");
            let with_revision = request.query_param("withRevision").as_deref() == Some("true");
            let with_tags = request.query_param("withTags").as_deref() == Some("true");
            return (
                Response::json(&controllers::recipes_json(
                    ingredient.as_deref(),
                    with_revision,
                    with_tags,
                )),
                true,
            );
        }
        if let Some(permalink) = path.strip_prefix("/recipes/") {
            if !permalink.is_empty() && !permalink.contains('/') {
                return match controllers::recipe_json(permalink, &context.usage()) {
                    Some(json) => (Response::json(&json), true),
                    None => (
                        Response::status(
                            404,
                            "text/plain; charset=UTF-8",
                            b"Recipe not found".to_vec(),
                        ),
                        true,
                    ),
                };
            }
            return (Response::not_found(), false);
        }
        if path == "/meals/" {
            return (
                Response::json(&controllers::meals_json(&context.usage())),
                true,
            );
        }
        if path == "/meals/raw" {
            return (Response::plain(&controllers::meal_names()), true);
        }
        if path == "/meals/usage" {
            return (
                Response::json(&controllers::meals_usage(&context.usage())),
                true,
            );
        }
        if let Some(response) = admin_get(request, context) {
            return (response, true);
        }
        return (Response::not_found(), false);
    }

    if request.method == "POST" {
        if request.path == "/recipe-submissions" {
            return (crate::submission::handle(request, context), true);
        }
        if let Some(response) = admin_post(request, context) {
            return (response, true);
        }
    }

    (Response::not_found(), false)
}

/// The tracker's `GET` routes, or `None` when the path is not one of them.
fn admin_get(request: &Request, context: &Context) -> Option<Response> {
    match request.path.as_str() {
        "/auth/login" => Some(match &context.admin {
            Some(admin) => admin.login(),
            None => disabled(),
        }),
        "/auth/callback" => Some(match &context.admin {
            Some(admin) => admin.callback(request),
            None => disabled(),
        }),
        "/admin" => Some(match &context.admin {
            Some(admin) => match admin.session(request) {
                Some((session, user)) => admin.page(&session, &user),
                None => Response::redirect(302, "/auth/login"),
            },
            None => disabled(),
        }),
        // The JSON API the frontend's JavaScript calls; a missing session is a
        // 401 the script renders a login prompt for, not a redirect.
        "/admin/api/session" => Some(match &context.admin {
            Some(admin) => admin.session_json(request),
            None => disabled(),
        }),
        "/admin/api/entries" => Some(match &context.admin {
            Some(admin) => match admin.session(request) {
                Some((_session, user)) => admin.entries_json(request, &user),
                None => error_response(401, "not signed in"),
            },
            None => disabled(),
        }),
        _ => None,
    }
}

/// The tracker's `POST` routes, or `None` when the path is not one of them.
fn admin_post(request: &Request, context: &Context) -> Option<Response> {
    let path = request.path.as_str();
    let is_form = path == "/admin/entries" || path == "/admin/entries/delete";
    let is_api = path == "/admin/api/entries"
        || path == "/admin/api/entries/delete"
        || path == "/admin/api/entries/update";
    if path != "/auth/logout" && !is_form && !is_api {
        return None;
    }
    let Some(admin) = &context.admin else {
        return Some(disabled());
    };
    if path == "/auth/logout" {
        return Some(admin.logout(request));
    }
    let not_signed_in = || {
        if is_api {
            error_response(401, "not signed in")
        } else {
            Response::redirect(303, "/auth/login")
        }
    };
    Some(match admin.session(request) {
        Some((session, user)) => match path {
            "/admin/entries" => admin.add_entry(request, &session, &user),
            "/admin/entries/delete" => admin.delete_entry(request, &session, &user),
            "/admin/api/entries" => admin.add_entry_json(request, &session, &user),
            "/admin/api/entries/update" => admin.update_entry_json(request, &session, &user),
            _ => admin.delete_entry_json(request, &session, &user),
        },
        None => not_signed_in(),
    })
}

/// The answer when the tracker routes are reached but not configured.
fn disabled() -> Response {
    error_response(503, "admin tracker not configured")
}

pub fn error_response(status: u16, message: &str) -> Response {
    Response::json_status(status, &error_json(message))
}

impl Response {
    /// A JSON response with a non-200 status.
    pub fn json_status(status: u16, value: &Value) -> Response {
        Response::status(
            status,
            "application/json",
            serde_json::to_vec(value).expect("JSON serialisation cannot fail"),
        )
    }
}
