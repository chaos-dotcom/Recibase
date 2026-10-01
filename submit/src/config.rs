//! `se.reciba.api.submit.RecipeSubmissionConfig`.

use crate::log;
use std::collections::HashSet;

pub use crate::turnstile::TurnstileSettings;

/// `case class GithubSettings(token, repository, baseBranch)`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GithubSettings {
    pub token: String,
    pub repository: String,
    pub base_branch: String,
}

/// `case class RecipeSubmissionConfig(passcode, github, turnstile)`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecipeSubmissionConfig {
    pub passcode: String,
    pub github: GithubSettings,
    pub turnstile: TurnstileSettings,
}

pub const DEFAULT_REPOSITORY: &str = "The-Silverwood-Institute/Recibase";
pub const DEFAULT_BRANCH: &str = "master";

impl RecipeSubmissionConfig {
    /// `fromEnv`, with the environment read through `env` so tests can supply
    /// their own map. The variable names are the Scala's.
    pub fn from_env(env: &dyn Fn(&str) -> Option<String>) -> Option<RecipeSubmissionConfig> {
        let passcode = env("RECIPE_SUBMIT_PASSCODE");
        let token = env("GITHUB_TOKEN");
        let repository = env("GITHUB_REPOSITORY");
        let base_branch = env("GITHUB_BASE_BRANCH");
        let turnstile_secret = env("TURNSTILE_SECRET");
        let turnstile_hostnames = env("TURNSTILE_HOSTNAMES");

        let configured = present(token.as_deref())
            || present(passcode.as_deref())
            || present(turnstile_secret.as_deref());
        let config = RecipeSubmissionConfig::from(
            passcode,
            token,
            repository,
            base_branch,
            turnstile_secret,
            turnstile_hostnames,
        );
        if configured && config.is_none() {
            log::warn("recipe submission env is set but invalid");
        }
        config
    }

    /// `fromEnv` reading the real process environment.
    pub fn from_process_env() -> Option<RecipeSubmissionConfig> {
        RecipeSubmissionConfig::from_env(&|key| std::env::var(key).ok())
    }

    /// `from`: the passcode, the GitHub token, the Turnstile secret and at
    /// least one hostname are all required, and an empty value counts as
    /// absent.
    pub fn from(
        passcode: Option<String>,
        token: Option<String>,
        repository: Option<String>,
        base_branch: Option<String>,
        turnstile_secret: Option<String>,
        turnstile_hostnames: Option<String>,
    ) -> Option<RecipeSubmissionConfig> {
        let hostnames: HashSet<String> = turnstile_hostnames
            .unwrap_or_default()
            .split(',')
            .map(|hostname| hostname.trim().to_string())
            .filter(|hostname| !hostname.is_empty())
            .collect();

        let passcode = non_empty(passcode)?;
        let token = non_empty(token)?;
        let secret = non_empty(turnstile_secret)?;
        if hostnames.is_empty() {
            return None;
        }

        let repository = non_empty(repository).unwrap_or_else(|| DEFAULT_REPOSITORY.to_string());
        let base_branch = non_empty(base_branch).unwrap_or_else(|| DEFAULT_BRANCH.to_string());
        if !repository_pattern_matches(&repository) || !branch_pattern_matches(&base_branch) {
            return None;
        }

        Some(RecipeSubmissionConfig {
            passcode,
            github: GithubSettings { token, repository, base_branch },
            turnstile: TurnstileSettings { secret, hostnames },
        })
    }
}

fn present(value: Option<&str>) -> bool {
    value.map(|text| !text.is_empty()).unwrap_or(false)
}

fn non_empty(value: Option<String>) -> Option<String> {
    value.filter(|text| !text.is_empty())
}

/// `^[A-Za-z0-9_.-]+/[A-Za-z0-9_.-]+$`, as `Regex.matches` anchors it.
fn repository_pattern_matches(repository: &str) -> bool {
    let mut parts = repository.split('/');
    match (parts.next(), parts.next(), parts.next()) {
        (Some(owner), Some(name), None) => {
            !owner.is_empty()
                && !name.is_empty()
                && owner.chars().all(name_character)
                && name.chars().all(name_character)
        }
        _ => false,
    }
}

/// `^[A-Za-z0-9._-]+$`.
fn branch_pattern_matches(branch: &str) -> bool {
    !branch.is_empty() && branch.chars().all(name_character)
}

fn name_character(character: char) -> bool {
    character.is_ascii_alphanumeric()
        || character == '_'
        || character == '.'
        || character == '-'
}
