//! Acceptance test for workstream W2 (`core/src/scala_hash.rs`).
//!
//! Three checks, all against byte-exact captures of the running Scala server:
//!
//! 1. every `/recipes/<permalink>` capture: the `tags` and `inherited_tags`
//!    arrays, rebuilt from the `val tags = Set(Tag.X, ...)` list of the Scala
//!    recipe source (source order) through `scala_hash`, equal the capture;
//! 2. the `/` docs map (`core::misc::docs_json`) field order equals the capture;
//! 3. the `/meals/` array order: every captured entry is rebuilt as a
//!    `MealStubWithUsageData` with the captured field values and
//!    `scala_hash::set_order` must reproduce the captured order exactly, for
//!    both `capture-scala/` (no usage data) and `capture-scala-csv/`.
//!
//! The test needs the workspace around this worktree. It locates it from
//! `CARGO_MANIFEST_DIR` (../../) or from `RECIBASE_WORK`.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use recibase_core::meal::{DatedNote, MealStubWithUsageData, Source};
use recibase_core::scala_hash::{
    case_object_hash, java_string_hash, product_hash, scala_set_flat_map, seq_hash, set_hash,
    set_order, set_order_distinct, Slot, ScalaHash,
};
use recibase_core::tag::Tag;
use serde_json::Value;

// --------------------------------------------------------------------------
// workspace locations
// --------------------------------------------------------------------------

fn work_dir() -> PathBuf {
    if let Ok(dir) = std::env::var("RECIBASE_WORK") {
        return PathBuf::from(dir);
    }
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("core/ lives two levels below the workspace")
        .to_path_buf()
}

fn scala_recipes_dir() -> PathBuf {
    work_dir().join("Recibase/src/main/scala/se/reciba/api/recibase/recipes")
}

/// The capture directories to check, in order. `capture-scala` is required.
fn capture_dirs() -> Vec<PathBuf> {
    let work = work_dir();
    let mut dirs = Vec::new();
    for name in ["capture-scala", "capture-scala-csv"] {
        let dir = work.join(name);
        assert!(dir.is_dir(), "missing capture directory {}", dir.display());
        dirs.push(dir);
    }
    dirs
}

fn read_capture(dir: &Path, needle: &str) -> Vec<(String, Value)> {
    let mut out = Vec::new();
    for entry in std::fs::read_dir(dir).expect("capture dir") {
        let path = entry.expect("dir entry").path();
        let name = path.file_name().unwrap().to_string_lossy().to_string();
        if !name.contains(needle) || !name.ends_with(".raw") {
            continue;
        }
        let raw = std::fs::read(&path).expect("read capture");
        let body_start = raw
            .windows(4)
            .position(|w| w == b"\r\n\r\n")
            .expect("HTTP header terminator")
            + 4;
        let body = &raw[body_start..];
        let value: Value = serde_json::from_slice(body)
            .unwrap_or_else(|e| panic!("{}: not JSON: {e}", path.display()));
        out.push((name, value));
    }
    out.sort_by(|a, b| a.0.cmp(&b.0));
    assert!(!out.is_empty(), "no {needle} captures in {}", dir.display());
    out
}

fn string_at(value: &Value, index: usize) -> String {
    value
        .as_array()
        .unwrap_or_else(|| panic!("expected an array, got {value}"))
        .iter()
        .map(|v| v.as_str().expect("string element").to_string())
        .nth(index)
        .unwrap()
}

fn field_array(value: &Value, field: &str) -> Vec<String> {
    value[field]
        .as_array()
        .unwrap_or_else(|| panic!("field {field} is not an array in {value}"))
        .iter()
        .map(|v| v.as_str().expect("string element").to_string())
        .collect()
}

// --------------------------------------------------------------------------
// ground truth measured from the running Scala 2.13.18 build
//
// `ProbeOrd` (a scratch Scala project) printed these straight out of the real
// collections; they are copied here unchanged.
// --------------------------------------------------------------------------

/// `Tag` case-object `hashCode`s, keyed by the Scala object name.
const TAG_HASHES: &[(&str, i32)] = &[
    ("Christmas", 1235317602),
    ("Pudding", 1430896733),
    ("Lunch", 73782026),
    ("Baking", 1982397558),
    ("NonMeal", -507339760),
    ("Soup", 2583063),
    ("Vegan", 82533797),
    ("VeganIsh", 2043126937),
    ("Vegetarian", 93893310),
    ("VegetarianIsh", 1151962336),
    ("Pescatarian", -2046910099),
    ("GlutenFree", 1912777241),
    ("StephaniUnhealthy", -1432841372),
    ("StephaniIsh", 306036478),
    ("Stephani", 1493755232),
    ("ColdWeather", -1330905008),
    ("HotWeather", -1132390649),
    ("Stodge", -1808213132),
    ("Spicy", 80092930),
    ("Slow", 2580001),
    ("Quick", 78394829),
    ("Scales", -1824322423),
    ("HighEffort", 1231439182),
    ("LowEffort", 562348352),
    ("Freezes", 1060876444),
    ("BetterNextDay", 1113342841),
    ("AI", 2088),
    ("NeverEaten", 1954739957),
    ("Popular", 1270713017),
    ("Infrequent", 56608531),
    ("New", 78208),
];

/// Synthetic sets of objects with chosen `hashCode`s: the hashes inserted in
/// this order, then the order the Scala set iterates them in (input positions).
const SYNTHETIC_SETS: &[(&[i32], &[usize])] = &[
    (&[0, 1, 2], &[0, 1, 2]),
    (&[0, 1, 2, 3], &[0, 1, 2, 3]),
    (&[0, 1, 2, 3, 4], &[0, 1, 2, 3, 4]),
    (&[0, 1, 2, 3, 4, 5], &[0, 5, 1, 2, 3, 4]),
    (&[0, 1, 2, 3, 4, 5, 6], &[0, 5, 1, 6, 2, 3, 4]),
    (
        &[0, 1, 2, 3, 4, 5, 6, 7],
        &[0, 5, 1, 6, 2, 7, 3, 4],
    ),
    (
        &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
        &[0, 5, 10, 14, 1, 6, 9, 13, 2, 12, 7, 3, 11, 8, 4, 15],
    ),
    (
        &[
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19,
        ],
        &[
            0, 5, 10, 14, 1, 6, 9, 13, 2, 17, 12, 7, 3, 18, 16, 11, 8, 19, 4, 15,
        ],
    ),
    (
        &[
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
            24, 25, 26, 27, 28, 29, 30, 31,
        ],
        &[
            0, 5, 10, 14, 1, 6, 9, 13, 2, 17, 12, 7, 3, 18, 11, 8, 4, 15, 24, 25, 20, 29, 28, 21,
            22, 27, 16, 31, 26, 23, 30, 19,
        ],
    ),
    (
        &[
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
            24, 25, 26, 27, 28, 29, 30, 31, 32,
        ],
        &[
            0, 5, 10, 14, 1, 6, 9, 13, 2, 12, 7, 3, 18, 11, 8, 4, 15, 24, 25, 20, 29, 28, 21, 32,
            17, 22, 27, 16, 31, 26, 23, 30, 19,
        ],
    ),
    // a payload above the sub-node's mask: payloads are emitted first
    (&[69, 3, 87, 65, 252], &[0, 1, 2, 4, 3]),
    (&[65, 252, 69, 3, 87], &[2, 3, 4, 1, 0]),
    // two distinct objects with the *same* hashCode: one collision leaf
    (&[3, 7, 11, 13, 13], &[1, 0, 2, 3, 4]),
    // three elements colliding at the root mask, two of them two levels deep
    (&[252, 1032, 625, 69, 3], &[3, 4, 2, 1, 0]),
    (&[224, 285, 1106, 69, 3], &[3, 4, 1, 0, 2]),
    (&[1411, 2094, 83], &[0, 1, 2]),
    (&[1411, 2094, 5365, 83, 90], &[1, 0, 2, 4, 3]),
    (&[83, 2094, 1411], &[0, 1, 2]),
    (&[1, 1, 2, 3, 4], &[2, 3, 4, 0, 1]),
    (&[1024, 1469, 2612, 4094, 1411, 2094, 3944, 5365], &[5, 6, 4, 7, 2, 3, 0, 1]),
];

/// `hashCode` of a few real Scala values, measured in the same probe run.
const MEASURED: &[(&str, i32)] = &[
    ("List().hashCode", 473519988),
    ("List(1).hashCode", 1945410391),
    ("List(1,2).hashCode", 959905896),
    ("List(1,2,3).hashCode", 1836368899),
    ("List(1,2,3,4).hashCode", -2132200068),
    ("List(1,2,3,4,5).hashCode", -595554320),
    ("List(1,3,5,7,9).hashCode", 1529849308),
    ("Set(1,2,3).hashCode", 1510543636),
    ("Set(\"a\",\"b\").hashCode", 672535746),
    ("Map(\"a\"->1,\"b\"->2).hashCode", 2006323191),
];

/// LocalDate, DatedNote, Source and MealStubWithUsageData hashes measured in
/// the probe run, plus one real `/meals/` element for each source variant
/// (from the `capture-scala-csv` data).
const MEASURED_MEALS: &[(&str, i32)] = &[
    ("Goan King Prawn Balchão", -1045326462),
    ("Cod in tomato sauce", 1242844065),
    ("Melty Mushroom Wellingtons", -660974071),
    ("Chicken Curry (WIP)", -1315209783),
    ("Banana Curry", -1547850637),
    ("Mediterranean Fish Stew", 1319629),
    ("Tuna & rice peppers", -249867227),
];

const MEASURED_DATES: &[(&str, i32)] = &[
    ("2020-04-24", 4137240),
    ("2022-01-06", 4141126),
    ("1999-12-31", 4094751),
    ("1970-01-01", 4034625),
    ("2024-02-29", 4145309),
    ("1973-02-21", 4040853),
    ("2026-10-01", 4149889),
    ("2000-01-01", 4096065),
    ("1980-07-04", 4055492),
    ("2021-03-17", 4139217),
];

/// An element with a chosen `hashCode`; equality is by identity, so two of
/// these with the same hash are two elements (a collision leaf in Scala).
#[derive(Debug, Clone, PartialEq, Eq)]
struct Hashed {
    id: usize,
    hash: i32,
}

impl ScalaHash for Hashed {
    fn scala_hash(&self) -> i32 {
        self.hash
    }
}

fn hashed(hashes: &[i32]) -> Vec<Hashed> {
    hashes
        .iter()
        .enumerate()
        .map(|(id, hash)| Hashed { id, hash: *hash })
        .collect()
}

/// The synthetic elements are distinct objects that may share a `hashCode`, so
/// they go through the trie without deduplication.
fn order_ids(items: &[Hashed]) -> Vec<usize> {
    let slots: Vec<Slot> = items
        .iter()
        .enumerate()
        .map(|(idx, item)| Slot { idx, hash: item.hash })
        .collect();
    set_order_distinct(&slots)
        .iter()
        .map(|&i| items[i].id)
        .collect()
}

// --------------------------------------------------------------------------
// the Scala recipe sources
// --------------------------------------------------------------------------

/// The string literal that `key = "..."` assigns, e.g. `val name`.
fn string_literal_after(src: &str, key: &str) -> Option<String> {
    let at = src.find(key)? + key.len();
    let quote = src[at..].find('"')? + at + 1;
    let mut out = String::new();
    let mut escaped = false;
    for c in src[quote..].chars() {
        if escaped {
            out.push(c);
            escaped = false;
        } else if c == '\\' {
            escaped = true;
        } else if c == '"' {
            return Some(out);
        } else {
            out.push(c);
        }
    }
    None
}

/// The `val tags = Set(Tag.X, ...)` list of a recipe source, in source order.
fn scala_tags_of(src: &str) -> Option<Vec<Tag>> {
    let at = src.find("val tags")?;
    let open = src[at..].find("Set(")? + at + 4;
    let mut depth = 1usize;
    let mut end = None;
    for (offset, c) in src[open..].char_indices() {
        match c {
            '(' => depth += 1,
            ')' => {
                depth -= 1;
                if depth == 0 {
                    end = Some(open + offset);
                    break;
                }
            }
            _ => {}
        }
    }
    let body = &src[open..end?];
    let tags: Vec<Tag> = body
        .split(',')
        .map(|part| part.trim().trim_start_matches("Tag."))
        .filter(|part| !part.is_empty())
        .map(|part| Tag::from_object_name(part).unwrap_or_else(|| panic!("unknown tag {part}")))
        .collect();
    Some(tags)
}

fn scala_recipes() -> BTreeMap<String, Vec<Tag>> {
    let dir = scala_recipes_dir();
    let mut out = BTreeMap::new();
    for entry in std::fs::read_dir(&dir).unwrap_or_else(|e| panic!("{}: {e}", dir.display())) {
        let path = entry.expect("dir entry").path();
        if path.extension().map(|e| e != "scala").unwrap_or(true) {
            continue;
        }
        let src = std::fs::read_to_string(&path).expect("read recipe source");
        let name = string_literal_after(&src, "val name").expect("val name");
        let tags = scala_tags_of(&src).expect("val tags");
        out.insert(name, tags);
    }
    assert_eq!(out.len(), 95, "expected 95 recipe sources");
    out
}

/// `Recipe.inheritedTags` = `tags.flatMap(_.allParentTags)`.
///
/// Scala's `allParentTags` is `parent.allParentTags + parent`, so the ancestors
/// come out with the **outermost first**; `Tag::all_parent_tags` (tag.rs, owned
/// by W1 and frozen) walks the chain upwards and so returns them innermost
/// first. The reversal below restores the Scala order. When tag.rs is fixed
/// (`all_parent_tags` returning the outermost ancestor first) this helper must
/// be replaced by a direct `tag.all_parent_tags()` call; nothing else changes.
///
/// Evidence for the order: `/recipes/baked-rigatoni-aubergine` has tags
/// {Vegetarian, Slow, Scales} (a Set3, insertion order) and the capture's
/// `inherited_tags` is `["Pescatarian","Vegetarian-ish"]`, i.e. the outermost
/// ancestor first. It is *not* a sorted or hash order: in recipes whose tags
/// set is a HashSet (5+ tags) the Scala `flatMap` builds a HashSet and the
/// result is in trie order instead, e.g. `/recipes/broccoli-stilton-soup`
/// `inherited_tags` is `["Vegetarian-ish","Pescatarian"]`.
fn inherited_tags(tags: &[Tag]) -> Vec<Tag> {
    scala_set_flat_map(tags, |tag| {
        let mut parents = tag.all_parent_tags();
        parents.reverse();
        parents
    })
}

// --------------------------------------------------------------------------
// the tests
// --------------------------------------------------------------------------

#[test]
fn improve_is_the_scala_hash_mix() {
    // `Hashing.improve` at work: the mask of the improved hash decides the trie
    // slot, and these are the improved values the Scala probe's dumps imply.
    assert_eq!(java_string_hash("New"), 78208);
    assert_eq!(java_string_hash("Vegetarian"), 93893310);
    assert_eq!(case_object_hash("se.reciba.api.model.Tag$Vegan$"), 82533797);
    assert_eq!(case_object_hash("Vegan"), 82533797);
    assert_eq!(java_string_hash("Set"), 83010);
}

#[test]
fn tag_hashes_match_the_scala_probe() {
    for (name, hash) in TAG_HASHES {
        let tag = Tag::from_object_name(name).unwrap_or_else(|| panic!("unknown tag {name}"));
        assert_eq!(tag.scala_hash(), *hash, "Tag::{name}.hashCode");
    }
    // all 31 tag hashes are distinct, so a trie never holds two tags with the
    // same improved hash and the set order is a pure function of them
    let unique: std::collections::BTreeSet<i32> =
        TAG_HASHES.iter().map(|(_, h)| *h).collect();
    assert_eq!(unique.len(), TAG_HASHES.len());
}

#[test]
fn synthetic_sets_match_the_scala_trie_order() {
    for (hashes, expected) in SYNTHETIC_SETS {
        let items = hashed(hashes);
        assert_eq!(
            &order_ids(&items),
            expected,
            "Set({hashes:?}) iteration order"
        );
    }
}

#[test]
fn composite_hashes_match_the_scala_probe() {
    let list = |values: &[i32]| seq_hash(&values.iter().map(|v| v.scala_hash()).collect::<Vec<_>>());
    let set = |values: &[i32]| set_hash(&values.iter().map(|v| v.scala_hash()).collect::<Vec<_>>());
    assert_eq!(list(&[]), 473519988);
    assert_eq!(list(&[1]), 1945410391);
    assert_eq!(list(&[1, 2]), 959905896);
    assert_eq!(list(&[1, 2, 3]), 1836368899);
    assert_eq!(list(&[1, 2, 3, 4]), -2132200068);
    assert_eq!(list(&[1, 2, 3, 4, 5]), -595554320);
    assert_eq!(list(&[1, 3, 5, 7, 9]), 1529849308);
    assert_eq!(set(&[1, 2, 3]), 1510543636);
    assert_eq!(
        set(&[java_string_hash("a"), java_string_hash("b")]),
        672535746
    );
    // a case object, a case class with one String field, a case class of two
    assert_eq!(case_object_hash("None"), 2433880);
    assert_eq!(product_hash("Recibase", &[java_string_hash("baked-rigatoni-aubergine")]), -1277414717);
    // DatedNote(LocalDate.of(2020, 1, 1), "note")
    assert_eq!(
        product_hash("DatedNote", &[4137025, java_string_hash("note")]),
        -315739312
    );

    use chrono::NaiveDate;
    for (date, hash) in MEASURED_DATES {
        let parsed = NaiveDate::parse_from_str(date, "%Y-%m-%d").unwrap();
        assert_eq!(parsed.scala_hash(), *hash, "LocalDate {date}.hashCode");
    }
    let _ = MEASURED;
}

#[test]
fn recipes_match_both_capture_dirs() {
    let recipes = scala_recipes();
    let mut checked = 0;
    for dir in capture_dirs() {
        for (file, value) in read_capture(&dir, "_recipe__") {
            let name = value["name"].as_str().expect("recipe name").to_string();
            let tags = recipes
                .get(&name)
                .unwrap_or_else(|| panic!("{file}: no Scala source for recipe {name}"));

            let expected_tags: Vec<&str> =
                recibase_core::scala_hash::scala_set(tags).iter().map(|t| t.entry_name()).collect();
            assert_eq!(
                expected_tags,
                field_array(&value, "tags"),
                "{file}: tags in Scala Set order"
            );

            let expected_inherited: Vec<&str> = inherited_tags(tags)
                .iter()
                .map(|t| t.entry_name())
                .collect();
            assert_eq!(
                expected_inherited,
                field_array(&value, "inherited_tags"),
                "{file}: inherited_tags in Scala Set order"
            );
            checked += 1;
        }
    }
    assert_eq!(checked, 190, "95 recipes in each of the two capture dirs");
}

#[test]
fn docs_map_matches_the_root_capture() {
    for dir in capture_dirs() {
        let captures = read_capture(&dir, "_root.raw");
        let (file, value) = &captures[0];
        let docs = recibase_core::misc::docs_json();
        let expected: Vec<&str> = docs
            .as_object()
            .expect("docs object")
            .keys()
            .map(|k| k.as_str())
            .collect();
        let actual: Vec<&str> = value
            .as_object()
            .expect("captured docs object")
            .keys()
            .map(|k| k.as_str())
            .collect();
        assert_eq!(expected, actual, "{file}: docs Map iteration order");
        assert_eq!(value, &docs, "{file}: docs body");
    }
}

#[test]
fn meals_order_matches_both_capture_dirs() {
    for dir in capture_dirs() {
        let captures = read_capture(&dir, "_meals.raw");
        let (file, _) = &captures[0];
        let meals = rebuild_meals(&captures[0].1);
        let captured: Vec<String> = captures[0]
            .1
            .as_array()
            .expect("meals array")
            .iter()
            .map(|m| m["name"].as_str().unwrap().to_string())
            .collect();
        assert_eq!(meals.len(), captured.len());

        let hashes: Vec<i32> = meals.iter().map(|m| m.scala_hash()).collect();
        let unique: std::collections::BTreeSet<i32> = hashes.iter().copied().collect();
        assert_eq!(
            unique.len(),
            hashes.len(),
            "{file}: no two meals share a hashCode, so the trie order is canonical"
        );

        let order = set_order(&meals);
        assert_eq!(order.len(), meals.len());
        let actual: Vec<&str> = order.iter().map(|&i| meals[i].name.as_str()).collect();
        assert_eq!(actual, captured, "{file}: /meals/ array order");
    }
}

#[test]
fn measured_meal_hashes_match_the_scala_probe() {
    let dirs = capture_dirs();
    // the CSV capture carries the usage data the probe ran with
    let csv = dirs.last().unwrap();
    let captures = read_capture(csv, "_meals.raw");
    let meals = rebuild_meals(&captures[0].1);
    for (name, hash) in MEASURED_MEALS {
        let meal = meals
            .iter()
            .find(|m| m.name == *name)
            .unwrap_or_else(|| panic!("no meal named {name}"));
        assert_eq!(meal.scala_hash(), *hash, "MealStubWithUsageData({name}).hashCode");
    }
}

fn rebuild_meals(json: &Value) -> Vec<MealStubWithUsageData> {
    use chrono::NaiveDate;
    let date = |v: &Value| -> Option<NaiveDate> {
        v.as_str()
            .map(|s| NaiveDate::parse_from_str(s, "%Y-%m-%d").unwrap())
    };
    json.as_array()
        .expect("meals array")
        .iter()
        .map(|entry| {
            let tags: Vec<Tag> = field_array(entry, "tags")
                .iter()
                .map(|name| Tag::from_entry_name(name).unwrap_or_else(|| panic!("tag {name}")))
                .collect();
            let source = match &entry["source"] {
                Value::Null => None,
                source => {
                    let text = |field: &str| source[field].as_str().unwrap().to_string();
                    Some(match source["type"].as_str().unwrap() {
                        "online" => Source::Online(text("url")),
                        "recibase" => Source::Recibase(text("permalink")),
                        "google_drive" => Source::GoogleDrive(text("id")),
                        other => panic!("unknown source type {other}"),
                    })
                }
            };
            let dated_notes: Vec<DatedNote> = entry["dated_notes"]
                .as_array()
                .expect("dated_notes")
                .iter()
                .map(|note| DatedNote {
                    date: date(&note["date"]).expect("note date"),
                    note: note["note"].as_str().unwrap().to_string(),
                })
                .collect();
            MealStubWithUsageData {
                name: entry["name"].as_str().unwrap().to_string(),
                tags,
                source,
                dated_notes,
                last_eaten: date(&entry["last_eaten"]),
                times_eaten: entry["times_eaten"].as_i64().unwrap(),
                featured: date(&entry["featured"]),
            }
        })
        .collect()
}
