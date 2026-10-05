//! Port of `se.reciba.api.TurnstileSpec`.

use recibase_submit::turnstile::{Turnstile, TurnstileSettings};
use serde_json::Value;
use std::collections::HashSet;

fn hostnames() -> HashSet<String> {
    let mut set = HashSet::new();
    set.insert("recipes.example".to_string());
    set
}

fn json(text: &str) -> Value {
    serde_json::from_str(text).expect("valid json")
}

#[test]
fn siteverify_acceptance_requires_success_action_and_hostname() {
    let body = json(r#"{"success":true,"action":"contribute","hostname":"recipes.example"}"#);
    assert!(Turnstile::accepted(&body, &hostnames()));

    let wrong_action = json(r#"{"success":true,"action":"login","hostname":"recipes.example"}"#);
    let wrong_host = json(r#"{"success":true,"action":"contribute","hostname":"evil.example"}"#);
    assert!(!Turnstile::accepted(&wrong_action, &hostnames()));
    assert!(!Turnstile::accepted(&wrong_host, &hostnames()));
}

#[test]
fn siteverify_acceptance_rejects_every_other_shape() {
    let missing = json(r#"{"action":"contribute","hostname":"recipes.example"}"#);
    assert!(!Turnstile::accepted(&missing, &hostnames()));

    let string_success =
        json(r#"{"success":"true","action":"contribute","hostname":"recipes.example"}"#);
    assert!(!Turnstile::accepted(&string_success, &hostnames()));

    let not_an_object = json(r#"[]"#);
    assert!(!Turnstile::accepted(&not_an_object, &hostnames()));

    let empty_hostnames =
        json(r#"{"success":true,"action":"contribute","hostname":"recipes.example"}"#);
    assert!(!Turnstile::accepted(&empty_hostnames, &HashSet::new()));
}

#[test]
fn token_shape_rejects_an_empty_or_oversized_token_and_an_empty_hostname_list() {
    assert!(!Turnstile::token_accepted("", &hostnames()));
    assert!(!Turnstile::token_accepted(&"x".repeat(2049), &hostnames()));
    assert!(!Turnstile::token_accepted("token", &HashSet::new()));
    assert!(Turnstile::token_accepted("token", &hostnames()));
    assert!(Turnstile::token_accepted(&"x".repeat(2048), &hostnames()));
}

#[test]
fn settings_are_cloneable_and_defaultable() {
    let settings = TurnstileSettings {
        secret: "s".to_string(),
        hostnames: hostnames(),
    };
    let copy = settings.clone();
    assert_eq!(copy.secret, "s");
    assert_eq!(TurnstileSettings::default().secret, "");
}
