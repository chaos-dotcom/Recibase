//! `RecipeSubmissionConfig.from` and `fromEnv`.

use recibase_submit::config::{RecipeSubmissionConfig, DEFAULT_BRANCH, DEFAULT_REPOSITORY};

fn from(
    passcode: Option<&str>,
    token: Option<&str>,
    repository: Option<&str>,
    base_branch: Option<&str>,
    turnstile_secret: Option<&str>,
    turnstile_hostnames: Option<&str>,
) -> Option<RecipeSubmissionConfig> {
    RecipeSubmissionConfig::from(
        passcode.map(|v| v.to_string()),
        token.map(|v| v.to_string()),
        repository.map(|v| v.to_string()),
        base_branch.map(|v| v.to_string()),
        turnstile_secret.map(|v| v.to_string()),
        turnstile_hostnames.map(|v| v.to_string()),
    )
}

#[test]
fn builds_a_config_when_every_required_value_is_present() {
    let config = from(
        Some("passcode"),
        Some("token"),
        Some("owner/repo"),
        Some("main"),
        Some("secret"),
        Some("recipes.example"),
    )
    .expect("configured");

    assert_eq!(config.passcode, "passcode");
    assert_eq!(config.github.token, "token");
    assert_eq!(config.github.repository, "owner/repo");
    assert_eq!(config.github.base_branch, "main");
    assert_eq!(config.turnstile.secret, "secret");
    assert_eq!(config.turnstile.hostnames.len(), 1);
    assert!(config.turnstile.hostnames.contains("recipes.example"));
}

#[test]
fn falls_back_to_the_default_repository_and_branch() {
    let config = from(
        Some("passcode"),
        Some("token"),
        None,
        None,
        Some("secret"),
        Some("a.example, b.example"),
    )
    .expect("configured");
    assert_eq!(config.github.repository, DEFAULT_REPOSITORY);
    assert_eq!(config.github.base_branch, DEFAULT_BRANCH);
    assert_eq!(config.turnstile.hostnames.len(), 2);
    assert!(config.turnstile.hostnames.contains("a.example"));
    assert!(config.turnstile.hostnames.contains("b.example"));
}

#[test]
fn an_empty_value_counts_as_absent() {
    for (passcode, token, secret, hostnames) in [
        (Some(""), Some("token"), Some("secret"), Some("a.example")),
        (Some("passcode"), Some(""), Some("secret"), Some("a.example")),
        (Some("passcode"), Some("token"), Some(""), Some("a.example")),
        (Some("passcode"), Some("token"), Some("secret"), Some("")),
        (Some("passcode"), Some("token"), Some("secret"), Some(" , , ")),
    ] {
        assert!(from(passcode, token, None, None, secret, hostnames).is_none());
    }
}

#[test]
fn the_repository_and_the_branch_must_match_the_patterns() {
    let bad_repositories = ["nope", "a/b/c", "/repo", "owner/", "owner/rep o", "owner/repo\n"];
    for repository in bad_repositories {
        assert!(
            from(Some("p"), Some("t"), Some(repository), Some("master"), Some("s"), Some("h")).is_none(),
            "{}",
            repository
        );
    }

    let bad_branches = ["master branch", "ma/ster", "master\n"];
    for branch in bad_branches {
        assert!(
            from(Some("p"), Some("t"), Some("owner/repo"), Some(branch), Some("s"), Some("h")).is_none(),
            "{}",
            branch
        );
    }

    // An empty repository or branch falls back to the defaults.
    let empty = from(Some("p"), Some("t"), Some(""), Some(""), Some("s"), Some("h")).expect("configured");
    assert_eq!(empty.github.repository, DEFAULT_REPOSITORY);
    assert_eq!(empty.github.base_branch, DEFAULT_BRANCH);

    for repository in ["owner/repo", "A._-b/C9", "chaos-dotcom/Recibase"] {
        assert!(
            from(Some("p"), Some("t"), Some(repository), Some("release-1.0"), Some("s"), Some("h")).is_some(),
            "{}",
            repository
        );
    }
}

#[test]
fn from_env_reads_the_six_variables() {
    let env = |key: &str| match key {
        "RECIPE_SUBMIT_PASSCODE" => Some("passcode".to_string()),
        "GITHUB_TOKEN" => Some("token".to_string()),
        "TURNSTILE_SECRET" => Some("secret".to_string()),
        "TURNSTILE_HOSTNAMES" => Some("recipes.example,www.example".to_string()),
        _ => None,
    };
    let config = RecipeSubmissionConfig::from_env(&env).expect("configured");
    assert_eq!(config.passcode, "passcode");
    assert_eq!(config.github.repository, DEFAULT_REPOSITORY);
    assert_eq!(config.github.base_branch, DEFAULT_BRANCH);
    assert_eq!(config.turnstile.hostnames.len(), 2);

    let nothing = |_: &str| None;
    assert!(RecipeSubmissionConfig::from_env(&nothing).is_none());
    // Env set but unusable: no config.
    let invalid = |key: &str| match key {
        "GITHUB_TOKEN" => Some("token".to_string()),
        _ => None,
    };
    assert!(RecipeSubmissionConfig::from_env(&invalid).is_none());
}

#[test]
fn settings_are_cloneable() {
    let config = from(
        Some("p"),
        Some("t"),
        None,
        None,
        Some("s"),
        Some("h.example"),
    )
    .expect("configured");
    let copy = config.clone();
    assert_eq!(copy, config);
}
