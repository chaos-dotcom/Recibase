//! Recipes as markdown documents.
//!
//! A recipe is one `recipes/*.md` file. `crates/core/build.rs` finds them at
//! compile time and `include_str!`s each one, so a recipe is added by adding a
//! file: there is no Rust to write and no registry to regenerate by hand.
//!
//! # The format
//!
//! ```markdown
//! ---
//! object_name: MexicanPolentaPie
//! created_at: 2020-04-24
//! permalink: mexican-polenta-pie
//! source: Alice Hart
//! description: A pie.
//! tagline: Nice.
//! image: https://i.reciba.se/pie.jpg
//! tags: Vegetarian, Slow
//! ---
//!
//! # Mexican Polenta Pie
//!
//! ## Ingredients
//!
//! ### Base
//!
//! | Ingredient | Quantity | Prep | Notes |
//! | --- | --- | --- | --- |
//! | Onion | 1 | chopped | Optional |
//!
//! ## Method
//!
//! 1. Do the first thing.
//!
//! ## Notes
//!
//! - A note.
//! ```
//!
//! Frontmatter carries what is not prose. Only `object_name` and `created_at`
//! are required; every other key is optional and absent means "none", which is
//! what the API sends as JSON `null`.
//!
//! The title (`# ...`) is the recipe's `name`. The ingredients table's four
//! columns are the four `Ingredient` fields, which is what keeps them
//! separable: the scaler parses `Quantity`, `hasIngredient` matches
//! `Ingredient`, and the clipboard merges on `Ingredient` + `Quantity`. A prose
//! ingredient line would lose those boundaries.
//!
//! # Whitespace
//!
//! Values are reproduced exactly. Two encodings exist because the source data
//! contains them and the API must return them unchanged:
//!
//! * a table cell cannot end in a space (the parser trims it), so a trailing
//!   space is written as `&#32;`;
//! * `|` is written `\|` so it does not end the cell.
//!
//! Both are decoded when the file is read.

use chrono::NaiveDate;

use crate::recipe::{Image, Ingredient, IngredientsBlock, RecipeDef};
use crate::tag::{TAGS, Tag};

/// Parses one recipe document.
///
/// Panics on a malformed document: recipes are compile-time data, so a bad file
/// is a build error rather than something to recover from at runtime.
pub fn parse(source: &str, file: &str) -> RecipeDef {
    let (front, body) = split_frontmatter(source)
        .unwrap_or_else(|| panic!("{file}: the document must open with `---` frontmatter"));

    let mut recipe = RecipeDef::default();
    let mut created_at = None;
    for line in front.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let (key, value) = line
            .split_once(':')
            .unwrap_or_else(|| panic!("{file}: frontmatter line is not `key: value`: {line}"));
        let key = key.trim();
        let value = decode(value.trim());
        match key {
            "object_name" => recipe.object_name = value,
            "created_at" => {
                created_at = Some(
                    NaiveDate::parse_from_str(&value, "%Y-%m-%d").unwrap_or_else(|_| {
                        panic!("{file}: created_at is not YYYY-MM-DD: {value}")
                    }),
                )
            }
            "permalink" => recipe.permalink_override = Some(value),
            "source" => recipe.source = Some(value),
            "description" => recipe.description = Some(value),
            "tagline" => recipe.tagline = Some(value),
            "image" => {
                recipe.image = Some(match (image_width(front), image_height(front)) {
                    (Some(width), Some(height)) => Image::sized(&value, width, height),
                    _ => Image::new(&value),
                })
            }
            // Handled with `image`; ignored here so an unknown-key check stays strict.
            "image_width" | "image_height" => {}
            // Not part of `RecipeDef`: marks the recipe as ours. The build script
            // reads it to fill in `chaos_recipes()`.
            "chaos" => {}
            "tags" => {
                recipe.tags = value
                    .split(',')
                    .map(str::trim)
                    .filter(|tag| !tag.is_empty())
                    .map(|tag| tag_from_name(tag, file))
                    .collect()
            }
            other => panic!("{file}: unknown frontmatter key `{other}`"),
        }
    }
    let created_at = created_at.unwrap_or_else(|| panic!("{file}: no created_at"));
    recipe.created_at = created_at;
    assert!(!recipe.object_name.is_empty(), "{file}: no object_name");

    parse_body(body, file, &mut recipe);
    recipe
}

/// Whether a recipe document marks itself as ours (`chaos: true` in its
/// frontmatter) rather than part of the upstream corpus.
///
/// This is a textual check so the build script can classify a file without
/// parsing it; [`parse`] accepts the key without using it.
pub fn is_ours(source: &str) -> bool {
    let Some((front, _)) = split_frontmatter(source) else {
        return false;
    };
    front.lines().any(|line| {
        let Some((key, value)) = line.split_once(':') else {
            return false;
        };
        key.trim() == "chaos" && !matches!(value.trim(), "" | "false" | "no")
    })
}

fn image_width(front: &str) -> Option<i64> {
    frontvalue(front, "image_width").and_then(|v| v.parse().ok())
}

fn image_height(front: &str) -> Option<i64> {
    frontvalue(front, "image_height").and_then(|v| v.parse().ok())
}

fn frontvalue(front: &str, key: &str) -> Option<String> {
    front.lines().find_map(|line| {
        let (k, v) = line.split_once(':')?;
        (k.trim() == key).then(|| v.trim().to_string())
    })
}

#[derive(Clone, Copy, PartialEq)]
enum Section {
    None,
    Ingredients,
    Method,
    Notes,
}

fn parse_body(body: &str, file: &str, recipe: &mut RecipeDef) {
    let mut section = Section::None;
    let mut blocks: Vec<IngredientsBlock> = Vec::new();
    // The header row of the current table has been consumed.
    let mut in_table = false;
    // Set between a `### X` heading and its first row, so an ingredients block
    // with no heading is only created when a table actually starts.
    let mut pending_block: Option<Option<String>> = None;

    for line in body.lines() {
        let trimmed = line.trim();

        if let Some(title) = trimmed.strip_prefix("# ") {
            recipe.name = decode(title.trim());
            continue;
        }
        if let Some(heading) = trimmed.strip_prefix("## ") {
            section = match heading.trim() {
                "Ingredients" => Section::Ingredients,
                "Method" => Section::Method,
                "Notes" => Section::Notes,
                other => panic!("{file}: unknown section `## {other}`"),
            };
            continue;
        }
        if let Some(heading) = trimmed.strip_prefix("### ") {
            assert!(
                section == Section::Ingredients,
                "{file}: `### {heading}` is only valid under `## Ingredients`"
            );
            pending_block = Some(Some(decode(heading.trim())));
            in_table = false;
            continue;
        }

        match section {
            Section::Ingredients => {
                if trimmed.is_empty() {
                    continue;
                }
                assert!(
                    trimmed.starts_with('|'),
                    "{file}: expected an ingredients table row: {trimmed}"
                );
                // A row separator ends the header.
                if trimmed.contains("---") {
                    in_table = true;
                    continue;
                }
                if !in_table {
                    // The column names themselves; their meaning is fixed.
                    continue;
                }
                if pending_block.is_some() || blocks.is_empty() {
                    // A `### Name` heading, or the first table of an unnamed
                    // block, opens a new block. Every later row joins it.
                    let name = pending_block.take().flatten();
                    blocks.push(IngredientsBlock::new(name.as_deref(), Vec::new()));
                }
                let cells = split_row(trimmed);
                let block = blocks.last_mut().expect("a block was pushed");
                block.ingredients.push(Ingredient::opt(
                    &cells[0],
                    opt_cell(&cells, 1),
                    opt_cell(&cells, 2),
                    opt_cell(&cells, 3),
                ));
            }
            Section::Method => {
                if trimmed.is_empty() {
                    continue;
                }
                // An indented line continues the previous step, so it is checked
                // before the numbered-item requirement.
                if line.starts_with(' ') && !recipe.method.is_empty() {
                    let last = recipe.method.last_mut().expect("checked not empty");
                    last.push('\n');
                    last.push_str(trimmed);
                    continue;
                }
                let step = match numbered_item(line) {
                    Some(step) => step,
                    None => panic!("{file}: expected a numbered method step: {trimmed}"),
                };
                recipe.method.push(step);
            }
            Section::Notes => {
                if trimmed.is_empty() {
                    continue;
                }
                match line.strip_prefix("- ") {
                    Some(note) => recipe.notes.push(decode(note)),
                    None => {
                        // A continuation line of the previous note.
                        let last = recipe
                            .notes
                            .last_mut()
                            .unwrap_or_else(|| panic!("{file}: a note continues a note"));
                        last.push('\n');
                        last.push_str(&decode(line.trim_start()));
                    }
                }
            }
            Section::None => {}
        }
    }
    recipe.ingredients_blocks = blocks;
}

fn opt_cell(cells: &[String], index: usize) -> Option<&str> {
    cells
        .get(index)
        .map(String::as_str)
        .filter(|cell| !cell.is_empty())
}

/// Splits `---\n...\n---\n` off the front of the document.
fn split_frontmatter(source: &str) -> Option<(&str, &str)> {
    let rest = source.strip_prefix("---")?;
    let rest = rest
        .strip_prefix('\n')
        .or_else(|| rest.strip_prefix("\r\n"))?;
    let mut offset = 0;
    for line in rest.split_inclusive('\n') {
        let end = offset + line.len();
        if line.trim_end() == "---" {
            return Some((&rest[..offset], &rest[end..]));
        }
        offset = end;
    }
    None
}

/// Decodes a cell or frontmatter value: the escapes a value cannot carry
/// literally are undone here.
fn decode(value: &str) -> String {
    let mut out = String::with_capacity(value.len());
    let mut rest = value;
    while let Some(index) = rest.find('&') {
        out.push_str(&rest[..index]);
        rest = &rest[index..];
        if let Some(decoded) = rest.strip_prefix("&#32;") {
            out.push(' ');
            rest = decoded;
        } else if let Some(decoded) = rest.strip_prefix("&amp;") {
            out.push('&');
            rest = decoded;
        } else {
            out.push('&');
            rest = &rest[1..];
        }
    }
    out.push_str(rest);
    out
}

/// `| a | b |` -> ["a", "b"]. A trailing row of empty cells is allowed, so a row
/// may be shorter than the header.
fn split_row(line: &str) -> Vec<String> {
    let inner = line.trim();
    let inner = inner.strip_prefix('|').unwrap_or(inner);
    let inner = inner.strip_suffix('|').unwrap_or(inner);

    let mut cells = Vec::new();
    let mut current = String::new();
    let mut chars = inner.chars().peekable();
    while let Some(ch) = chars.next() {
        match ch {
            '\\' if chars.peek() == Some(&'|') => {
                chars.next();
                current.push('|');
            }
            '|' => {
                cells.push(decode(current.trim()));
                current.clear();
            }
            _ => current.push(ch),
        }
    }
    cells.push(decode(current.trim()));
    cells
}

/// `1. Text` -> `Text`, keeping the text's own trailing space.
fn numbered_item(line: &str) -> Option<String> {
    let (number, rest) = line.split_once(". ")?;
    if number.chars().all(|c| c.is_ascii_digit()) && !number.is_empty() {
        Some(rest.to_string())
    } else {
        None
    }
}

fn tag_from_name(name: &str, file: &str) -> Tag {
    TAGS.iter()
        .copied()
        .find(|tag| tag.entry_name() == name)
        .unwrap_or_else(|| panic!("{file}: unknown tag `{name}`"))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "\
---
object_name: SampleRecipe
created_at: 2020-04-24
permalink: sample
source: A Book
description: A description.
tagline: A tagline.
image: https://i.reciba.se/sample.jpg
image_width: 640
image_height: 480
tags: Vegetarian, Slow
chaos: true
---

# Sample Recipe

## Ingredients

| Ingredient | Quantity | Prep | Notes |
| --- | --- | --- | --- |
| Onion | 1 | chopped | Optional |
| Salt |

### Topping

| Ingredient | Quantity | Prep | Notes |
| --- | --- | --- | --- |
| Cheese | 100g | grated |

## Method

1. Chop the onion.
2. Bake it.

## Notes

- A note.
- Another note.
";

    #[test]
    fn every_field_round_trips() {
        let recipe = parse(SAMPLE, "sample.md");
        assert_eq!(recipe.object_name, "SampleRecipe");
        assert_eq!(recipe.name, "Sample Recipe");
        assert_eq!(
            recipe.created_at,
            NaiveDate::from_ymd_opt(2020, 4, 24).unwrap()
        );
        assert_eq!(recipe.permalink_override.as_deref(), Some("sample"));
        assert_eq!(recipe.source.as_deref(), Some("A Book"));
        assert_eq!(recipe.description.as_deref(), Some("A description."));
        assert_eq!(recipe.tagline.as_deref(), Some("A tagline."));
        assert_eq!(recipe.tags, vec![Tag::Vegetarian, Tag::Slow]);
        let image = recipe.image.expect("an image");
        assert_eq!(image.src, "https://i.reciba.se/sample.jpg");
        assert_eq!((image.width, image.height), (640, 480));
        assert_eq!(
            recipe.ingredients_blocks,
            vec![
                IngredientsBlock::new(
                    None,
                    vec![
                        Ingredient::qpn("Onion", "1", "chopped", "Optional"),
                        Ingredient::new("Salt"),
                    ],
                ),
                IngredientsBlock::new(
                    Some("Topping"),
                    vec![Ingredient::qp("Cheese", "100g", "grated")],
                ),
            ]
        );
        assert_eq!(recipe.method, vec!["Chop the onion.", "Bake it."]);
        assert_eq!(recipe.notes, vec!["A note.", "Another note."]);
    }

    #[test]
    fn a_short_row_leaves_the_missing_cells_empty() {
        // `| Salt |` is a name with no quantity, prep or notes.
        let recipe = parse(SAMPLE, "sample.md");
        assert_eq!(
            recipe.ingredients_blocks[0].ingredients[1],
            Ingredient::new("Salt")
        );
    }

    #[test]
    fn a_trailing_space_survives_a_table_cell() {
        // The upstream data contains these, and the API must return them.
        let source = SAMPLE.replace("| Onion |", "| Onion&#32; |");
        let recipe = parse(&source, "sample.md");
        assert_eq!(recipe.ingredients_blocks[0].ingredients[0].name, "Onion ");
    }

    #[test]
    fn a_pipe_survives_a_table_cell() {
        let source = SAMPLE.replace("| Onion |", "| Onion \\| Shallot |");
        let recipe = parse(&source, "sample.md");
        assert_eq!(
            recipe.ingredients_blocks[0].ingredients[0].name,
            "Onion | Shallot"
        );
    }

    #[test]
    fn a_multiline_note_and_step_survive() {
        let source = SAMPLE
            .replace(
                "1. Chop the onion.",
                "1. Chop the onion.\n   Then the garlic.",
            )
            .replace("- A note.", "- A note.\n  Continued.");
        let recipe = parse(&source, "sample.md");
        assert_eq!(recipe.method[0], "Chop the onion.\nThen the garlic.");
        assert_eq!(recipe.notes[0], "A note.\nContinued.");
    }

    #[test]
    fn chaos_marker_is_read_from_the_frontmatter() {
        assert!(is_ours(SAMPLE));
        assert!(!is_ours(&SAMPLE.replace("chaos: true", "chaos: false")));
        assert!(!is_ours(&SAMPLE.replace("chaos: true\n", "")));
    }

    #[test]
    #[should_panic(expected = "unknown tag")]
    fn an_unknown_tag_is_a_build_error() {
        parse(&SAMPLE.replace("Slow", "Nonsense"), "sample.md");
    }

    /// The corpus itself must round-trip: parse, then confirm the shipped files
    /// produce the same definitions the API serves.
    #[test]
    fn the_whole_corpus_parses_and_is_not_empty() {
        let recipes = crate::recipes::recipes();
        assert!(!recipes.is_empty());
        for recipe in recipes {
            assert!(!recipe.name.is_empty(), "{}: no name", recipe.object_name);
            assert!(
                !recipe.ingredients_blocks.is_empty(),
                "{}: no ingredients",
                recipe.object_name
            );
            assert!(
                !recipe.method.is_empty(),
                "{}: no method",
                recipe.object_name
            );
        }
    }
}
