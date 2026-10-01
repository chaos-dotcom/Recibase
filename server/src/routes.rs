//! `RecibaseRoutes` and `RecipeSubmissionRoutes`.

use crate::controllers::{self, Usage};
use crate::http::{Request, Response};
use crate::meal_log;
use recibase_core::json::obj;
use serde_json::Value;

pub struct Context {
    pub usage: Usage,
    pub env: Box<dyn Fn(&str) -> Option<String> + Send + Sync>,
}

impl Context {
    pub fn new(env: Box<dyn Fn(&str) -> Option<String> + Send + Sync>) -> Context {
        let entries = meal_log::load_from_env(&*env);
        let today = controllers::today(&*env);
        Context { usage: Usage { entries, today }, env }
    }
}

fn error_json(message: &str) -> Value {
    obj(vec![("error", Value::String(message.to_string()))])
}

/// `RecibaseRoutes.routes` for a single request.
pub fn route(request: &Request, context: &Context) -> Response {
    if request.method == "GET" {
        let path = request.path.as_str();
        if path == "/" {
            return Response::json(&controllers::docs());
        }
        if path == "/health" {
            return Response::plain("ok");
        }
        if path == "/manifest" {
            return Response::json(&controllers::manifest_json(&*context.env));
        }
        if path == "/recipes/" {
            let ingredient = request.query_param("hasIngredient");
            return Response::json(&controllers::recipes_json(ingredient.as_deref()));
        }
        if let Some(permalink) = path.strip_prefix("/recipes/") {
            if !permalink.is_empty() && !permalink.contains('/') {
                return match controllers::recipe_json(permalink, &context.usage) {
                    Some(json) => Response::json(&json),
                    None => Response::status(404, "text/plain; charset=UTF-8", b"Recipe not found".to_vec()),
                };
            }
            return Response::not_found();
        }
        if path == "/meals/" {
            return Response::json(&controllers::meals_json(&context.usage));
        }
        if path == "/meals/raw" {
            return Response::plain(&controllers::meal_names());
        }
        return Response::not_found();
    }

    if request.method == "POST" && request.path == "/recipe-submissions" {
        return crate::submission::handle(request, context);
    }

    Response::not_found()
}

pub fn error_response(status: u16, message: &str) -> Response {
    Response::json_status(status, &error_json(message))
}

impl Response {
    /// A JSON response with a non-200 status.
    pub fn json_status(status: u16, value: &Value) -> Response {
        Response {
            status,
            content_type: "application/json",
            body: serde_json::to_vec(value).expect("JSON serialisation cannot fail"),
        }
    }
}
