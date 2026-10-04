//! `RecibaseRoutes` and `RecipeSubmissionRoutes`.

use crate::controllers::{self, Usage};
use crate::http::{Request, Response};
use crate::meal_log;
use recibase_core::json::obj;
use serde_json::Value;

/// The process environment, injectable so the harness and the tests can supply
/// their own map.
pub type Env = Box<dyn Fn(&str) -> Option<String> + Send + Sync>;

pub struct Context {
    pub usage: Usage,
    pub env: Env,
}

impl Context {
    pub fn new(env: Env) -> Context {
        let entries = meal_log::load_from_env(&*env);
        let today = controllers::today(&*env);
        Context {
            usage: Usage { entries, today },
            env,
        }
    }
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
pub fn route(request: &Request, context: &Context) -> Response {
    if request.method == "OPTIONS" {
        let origin = request.header("Origin").is_some();
        let requested_method = request.header("Access-Control-Request-Method").is_some();
        if origin && requested_method {
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
            return Response::preflight(&echoed);
        }
    }
    let origin = request.header("Origin").is_some();
    let (mut response, matched) = dispatch(request, context);
    if matched && origin {
        response
            .headers_after_length
            .push(("Access-Control-Allow-Origin", "*"));
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
                return match controllers::recipe_json(permalink, &context.usage) {
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
                Response::json(&controllers::meals_json(&context.usage)),
                true,
            );
        }
        if path == "/meals/raw" {
            return (Response::plain(&controllers::meal_names()), true);
        }
        return (Response::not_found(), false);
    }

    if request.method == "POST" && request.path == "/recipe-submissions" {
        return (crate::submission::handle(request, context), true);
    }

    (Response::not_found(), false)
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
