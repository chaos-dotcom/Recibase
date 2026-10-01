//! Small model types: `Manifest` and `MenuEntry`.

use crate::json::obj;
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Manifest {
    pub version: String,
}

impl Manifest {
    pub fn to_json(&self) -> Value {
        obj(vec![("version", Value::String(self.version.clone()))])
    }

    /// `Manifest.deployedVersion(env)`.
    ///
    /// GIT_COMMIT, then SOURCE_COMMIT, then GITHUB_SHA; the first value that
    /// trimmed matches `[0-9a-fA-F]{7,40}` wins, otherwise "latest".
    pub fn deployed_version(env: &dyn Fn(&str) -> Option<String>) -> String {
        for key in ["GIT_COMMIT", "SOURCE_COMMIT", "GITHUB_SHA"] {
            if let Some(value) = env(key) {
                let trimmed = value.trim().to_string();
                if is_commit(&trimmed) {
                    return trimmed;
                }
            }
        }
        "latest".to_string()
    }
}

fn is_commit(value: &str) -> bool {
    let len = value.len();
    (7..=40).contains(&len) && value.chars().all(|c| c.is_ascii_hexdigit())
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MenuEntry {
    pub name: String,
    pub permalink: String,
}

impl MenuEntry {
    pub fn new(name: &str, permalink: &str) -> Self {
        MenuEntry { name: name.to_string(), permalink: permalink.to_string() }
    }

    pub fn to_json(&self) -> Value {
        obj(vec![
            ("name", Value::String(self.name.clone())),
            ("permalink", Value::String(self.permalink.clone())),
        ])
    }
}

/// The `/` docs map, in the Scala `Map` iteration order.
pub fn docs_entries() -> Vec<(&'static str, &'static str)> {
    vec![
        ("docs", "/"),
        ("recipes_list", "/recipes/?hasIngredient={ingredient_name}"),
        ("recipe", "/recipes/{recipe_permalink}"),
        ("meals_list", "/meals/"),
        ("meal_names_text", "/meals/raw"),
        ("service_info", "/manifest"),
        ("health", "/health"),
    ]
}

/// The docs map as JSON. Scala's `Map[String, String]` literal is a `HashMap`,
/// so the field order is the hash-trie order, not the source order.
pub fn docs_json() -> Value {
    let entries = docs_entries();
    let pairs: Vec<(String, String)> = entries
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect();
    let order = crate::scala_hash::map_order(&pairs);
    let mut fields = Vec::new();
    for i in order {
        fields.push((entries[i].0, Value::String(entries[i].1.to_string())));
    }
    obj(fields)
}
