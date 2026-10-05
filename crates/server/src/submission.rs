//! `RecipeSubmissionRoutes`: the contribute page's POST endpoint.

use crate::controllers;
use crate::http::{Request, Response};
use crate::routes::{Context, error_response};
use recibase_submit::config::RecipeSubmissionConfig;
use recibase_submit::recipe_source::{RecipeSource, SubmitRejection};
use recibase_submit::recipe_submission::{Passcode, RecipeSubmission};
use recibase_submit::turnstile::Turnstile;

const DEFAULT_MAX_BYTES: usize = 256 * 1024;

pub fn handle(request: &Request, context: &Context) -> Response {
    let env = &*context.env;
    let Some(config) = RecipeSubmissionConfig::from_env(env) else {
        return error_response(503, "recipe submission not configured");
    };

    let max_bytes: usize = env("RECIPE_SUBMIT_MAX_BYTES")
        .and_then(|v| v.parse().ok())
        .unwrap_or(DEFAULT_MAX_BYTES);
    if request.body.len() > max_bytes {
        return error_response(400, "submission is too large");
    }

    let text = String::from_utf8_lossy(&request.body).into_owned();
    let Ok(json) = serde_json::from_str::<serde_json::Value>(&text) else {
        return error_response(400, "invalid json");
    };

    let token = json
        .get("cf-turnstile-response")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    if !Turnstile::token_accepted(&token, &config.turnstile.hostnames) {
        return error_response(403, "forbidden");
    }

    let turnstile = recibase_submit::turnstile::CloudflareTurnstile::new(config.turnstile.clone());
    if !turnstile.allow(&token) {
        return error_response(403, "forbidden");
    }

    let passcode = request.bearer_passcode();
    if !Passcode::equal(&config.passcode, &passcode) {
        return Response::json_status(
            401,
            &recibase_core::json::obj(vec![(
                "error",
                serde_json::Value::String("invalid passcode".to_string()),
            )]),
        );
    }

    let Some(submission) = RecipeSubmission::from_json(&json) else {
        return error_response(400, "invalid json");
    };

    let existing: Vec<recibase_core::recipe::RecipeDef> =
        recibase_core::recipes::recipes().to_vec();
    match RecipeSource::generate(&submission, controllers::london_today(), &existing) {
        Err(SubmitRejection::InvalidSubmission(message)) => error_response(400, &message),
        Err(SubmitRejection::ConflictingSubmission(message)) => error_response(409, &message),
        Ok(generated) => {
            let client = recibase_submit::github_client::GithubClient::new(&config.github, None);
            let title = format!("Add {}", generated.name);
            match client.open(
                &generated,
                &title,
                &title,
                "Submitted from the contribute page.",
            ) {
                Err(recibase_submit::pull_requests::PullRequestFailure::BranchAlreadyExists) => {
                    error_response(409, "A submission for this recipe is already open")
                }
                Err(recibase_submit::pull_requests::PullRequestFailure::GithubRejected(_)) => {
                    error_response(502, "GitHub rejected the submission")
                }
                Ok(url) => Response::json(&recibase_core::json::obj(vec![(
                    "url",
                    serde_json::Value::String(url),
                )])),
            }
        }
    }
}
