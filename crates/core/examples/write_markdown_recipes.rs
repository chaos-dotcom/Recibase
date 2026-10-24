//! One-off generator: emits a markdown recipe file for every `RecipeDef`.
//!
//! Run with `cargo run -p recibase-core --example write_markdown_recipes`.
//! It is how the corpus was converted from Rust modules to `recipes/*.md`, and
//! is kept as the reference for the file format the parser reads.

use std::fmt::Write as _;
use std::fs;
use std::path::PathBuf;

use recibase_core::recipe::RecipeDef;

fn main() {
    let out_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("recipes");
    fs::create_dir_all(&out_dir).expect("create the recipes directory");

    let mut written = 0;
    for recipe in recibase_core::recipes::recipes() {
        let path = out_dir.join(format!(
            "{}.md",
            recibase_core::recipe::snake_case(&recipe.object_name)
        ));
        fs::write(&path, to_markdown(recipe))
            .unwrap_or_else(|e| panic!("write {}: {e}", path.display()));
        written += 1;
    }
    println!("wrote {written} recipe files to {}", out_dir.display());
}

/// Encodes a value for a table cell or frontmatter value.
///
/// Two characters cannot survive as-is: `|` would end a cell, and leading or
/// trailing spaces would be trimmed. Both are encoded so the value round-trips
/// exactly - which matters, because the upstream data really does contain
/// trailing spaces and the API has to reproduce them.
fn cell(value: &str) -> String {
    let escaped = value.replace('|', "\\|");
    let trimmed = escaped.trim_matches(' ');
    if trimmed.is_empty() {
        // A value of nothing but spaces keeps every one of them.
        return escaped.replace(' ', "&#32;");
    }
    let leading = escaped.len() - escaped.trim_start_matches(' ').len();
    let trailing = escaped.len() - escaped.trim_end_matches(' ').len();
    let mut out = String::new();
    out.push_str(&"&#32;".repeat(leading));
    out.push_str(trimmed);
    out.push_str(&"&#32;".repeat(trailing));
    out
}

fn to_markdown(recipe: &RecipeDef) -> String {
    let mut out = String::new();

    // -- frontmatter -----------------------------------------------------
    out.push_str("---\n");
    writeln!(out, "object_name: {}", recipe.object_name).unwrap();
    writeln!(out, "created_at: {}", recipe.created_at.format("%Y-%m-%d")).unwrap();
    if let Some(permalink) = &recipe.permalink_override {
        writeln!(out, "permalink: {permalink}").unwrap();
    }
    if let Some(source) = &recipe.source {
        writeln!(out, "source: {}", cell(source)).unwrap();
    }
    if let Some(description) = &recipe.description {
        writeln!(out, "description: {}", cell(description)).unwrap();
    }
    if let Some(tagline) = &recipe.tagline {
        writeln!(out, "tagline: {}", cell(tagline)).unwrap();
    }
    if let Some(image) = &recipe.image {
        writeln!(out, "image: {}", cell(&image.src)).unwrap();
        if image.width != 1080 || image.height != 1080 {
            writeln!(out, "image_width: {}", image.width).unwrap();
            writeln!(out, "image_height: {}", image.height).unwrap();
        }
    }
    let tags: Vec<&str> = recipe.tags.iter().map(|tag| tag.entry_name()).collect();
    writeln!(out, "tags: {}", tags.join(", ")).unwrap();
    out.push_str("---\n\n");

    // -- title -----------------------------------------------------------
    writeln!(out, "# {}\n", cell(&recipe.name)).unwrap();

    // -- ingredients -----------------------------------------------------
    let named = recipe
        .ingredients_blocks
        .iter()
        .any(|block| block.name.is_some());
    if !named {
        // A single unnamed block needs no heading of its own.
        out.push_str("## Ingredients\n\n");
        write_table(&mut out, &recipe.ingredients_blocks[0].ingredients);
    } else {
        out.push_str("## Ingredients\n\n");
        for block in &recipe.ingredients_blocks {
            if let Some(name) = &block.name {
                writeln!(out, "### {name}\n").unwrap();
            }
            write_table(&mut out, &block.ingredients);
        }
    }

    // -- method ----------------------------------------------------------
    out.push_str("## Method\n\n");
    for (index, step) in recipe.method.iter().enumerate() {
        // A step's own newlines would break the numbered-list item, so they are
        // written as the line continuation the parser reads back.
        let mut lines = step.split('\n');
        if let Some(first) = lines.next() {
            writeln!(out, "{}. {first}", index + 1).unwrap();
        }
        for line in lines {
            writeln!(out, "   {line}").unwrap();
        }
    }
    out.push('\n');

    // -- notes -----------------------------------------------------------
    if !recipe.notes.is_empty() {
        out.push_str("## Notes\n\n");
        for note in &recipe.notes {
            let mut lines = note.split('\n');
            if let Some(first) = lines.next() {
                writeln!(out, "- {first}").unwrap();
            }
            for line in lines {
                writeln!(out, "  {line}").unwrap();
            }
        }
        out.push('\n');
    }

    out
}

fn write_table(out: &mut String, ingredients: &[recibase_core::recipe::Ingredient]) {
    out.push_str("| Ingredient | Quantity | Prep | Notes |\n");
    out.push_str("| --- | --- | --- | --- |\n");
    for ingredient in ingredients {
        // An absent field is an empty cell; trailing empty cells are dropped
        // because a table row does not need them.
        let mut cells = vec![
            cell(&ingredient.name),
            ingredient.quantity.as_deref().map(cell).unwrap_or_default(),
            ingredient.prep.as_deref().map(cell).unwrap_or_default(),
            ingredient.notes.as_deref().map(cell).unwrap_or_default(),
        ];
        while cells.len() > 1 && cells.last().is_some_and(String::is_empty) {
            cells.pop();
        }
        writeln!(out, "| {} |", cells.join(" | ")).unwrap();
    }
    out.push('\n');
}
