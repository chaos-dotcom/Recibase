//! Every recipe's JSON encoding must equal the Scala capture byte for byte.
//!
//! The captures are the byte-exact HTTP responses in `capture-scala/` (and
//! `capture-scala-csv/`); a recipe is matched to its capture through the `edit`
//! field, which ends with `/<ObjectName>.scala`. Both `tags` and
//! `inherited_tags` are Scala `Set`s, so this also proves the CHAMP iteration
//! order reproduced by `core::scala_hash`.

use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;

use recibase_core as core;
use recibase_core::recipe::CHAOS_TAG;

fn capture_dir() -> PathBuf {
    if let Ok(dir) = std::env::var("RECIBASE_CAPTURE_DIR") {
        return PathBuf::from(dir);
    }
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let candidates = [
        manifest
            .join("..")
            .join("..")
            .join("..")
            .join("capture-scala"),
        // The repository keeps its own copy of the captures under tools/.
        manifest
            .join("..")
            .join("..")
            .join("tools")
            .join("harness")
            .join("capture-scala"),
        PathBuf::from("/Users/chaos/recibase-work/capture-scala"),
    ];
    candidates
        .iter()
        .find(|dir| dir.is_dir())
        .cloned()
        .unwrap_or_else(|| panic!("no capture-scala directory; set RECIBASE_CAPTURE_DIR"))
}

/// The captured response body of every recipe, keyed by object name.
fn captured_bodies() -> BTreeMap<String, String> {
    let mut out = BTreeMap::new();
    let mut paths: Vec<PathBuf> = fs::read_dir(capture_dir())
        .expect("read the capture directory")
        .map(|entry| entry.expect("directory entry").path())
        .filter(|path| path.extension().is_some_and(|ext| ext == "raw"))
        .collect();
    paths.sort();
    for path in paths {
        let raw = fs::read(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
        let split = raw
            .windows(4)
            .position(|window| window == b"\r\n\r\n")
            .unwrap_or_else(|| panic!("no header terminator in {}", path.display()));
        let body = String::from_utf8(raw[split + 4..].to_vec())
            .unwrap_or_else(|e| panic!("body of {} is not UTF-8: {e}", path.display()));
        // Only recipe responses carry an `edit` field.
        let Ok(json) = serde_json::from_str::<serde_json::Value>(&body) else {
            continue;
        };
        let Some(edit) = json.get("edit").and_then(|v| v.as_str()) else {
            continue;
        };
        let Some(object_name) = edit
            .strip_suffix(".scala")
            .and_then(|stem| stem.rsplit('/').next())
        else {
            continue;
        };
        // `/recipes/vegetable-primavera` is captured twice; both copies are
        // byte-identical, so the later one simply wins.
        out.insert(object_name.to_string(), body);
    }
    assert!(
        !out.is_empty(),
        "no recipe capture found in {}",
        capture_dir().display()
    );
    out
}

/// `crates/core/src/recipes`, where the one-file-per-recipe modules live.
fn recipes_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src").join("recipes")
}

/// Mirror of `tools/gen_recipes.py`'s `snake`: an `_` before every interior
/// capital, lowercased. Maps a recipe's `object_name` to its module file stem.
fn module_stem(object_name: &str) -> String {
    let mut out = String::new();
    for (i, c) in object_name.chars().enumerate() {
        if i > 0 && c.is_ascii_uppercase() {
            out.push('_');
        }
        out.extend(c.to_lowercase());
    }
    out
}

/// A recipe is ours (not Kit's or Alex's ported corpus) when its module carries
/// the chaos tag. The byte-for-byte comparison ignores those recipes, so they do
/// not need a Scala capture.
fn is_ours(object_name: &str) -> bool {
    let path = recipes_dir().join(format!("{}.rs", module_stem(object_name)));
    fs::read_to_string(path).is_ok_and(|src| src.contains(CHAOS_TAG))
}

#[test]
fn every_recipe_equals_its_capture_byte_for_byte() {
    let captures = captured_bodies();
    let recipes = core::recipes::recipes();
    assert!(!recipes.is_empty(), "the recipe registry is empty");

    let mut checked = 0usize;
    for recipe in recipes {
        let name = &recipe.object_name;
        // Our own (chaos-tagged) recipes are not part of the upstream corpus and
        // have no Scala capture; the byte-for-byte comparison ignores them.
        if is_ours(name) {
            continue;
        }
        let expected = captures
            .get(name)
            .unwrap_or_else(|| panic!("no capture whose edit ends with /{name}.scala"));
        let actual = core::json::to_string(&recipe.to_json_with_usage(&[]));
        assert_eq!(
            actual, *expected,
            "{name}: the JSON body differs from the Scala capture"
        );
        checked += 1;
    }
    assert_eq!(
        checked,
        captures.len(),
        "every capture must belong to a recipe in the registry"
    );
}
