//! Generates the recipe registry from `recipes/*.md`, for `src/recipes/mod.rs`.
//!
//! Rust has no reflection, so the corpus needs a hand-maintained list. Rather
//! than commit a generated file and keep it in sync with `recipes/` by hand,
//! this build script derives the list from `recipes/*.md` at compile time:
//! adding, removing or renaming a recipe file is enough.
//!
//! Each file is `include_str!`d, so the recipes are compiled into the binary and
//! still parsed without reading the filesystem at runtime.
//!
//! `mod.rs` `include!`s the emitted file, so `recipes::recipes()` and
//! `recipes::chaos_recipes()` keep their paths.
//!
//! Recipe order does not affect byte compatibility: every consumer treats the
//! corpus as a Set or sorts it, and the Scala side's reflection order is itself
//! unstable. The list is sorted by filename to keep a build reproducible.

use std::env;
use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};

/// A recipe file: its identifier (the filename stem) and its absolute path.
struct RecipeFile {
    ident: String,
    file: PathBuf,
}

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR"));
    let recipes_dir = manifest_dir.join("recipes");
    // A directory is fingerprinted recursively, so adding or removing a recipe
    // reruns this script just as editing one does.
    println!("cargo:rerun-if-changed={}", recipes_dir.display());

    let files = recipe_files(&recipes_dir);
    assert!(
        !files.is_empty(),
        "{} contains no recipe files",
        recipes_dir.display()
    );

    // The chaos marker is read here, at build time, so the generated list cannot
    // drift from the files.
    let chaos = chaos_object_names(&files);

    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR"));
    let out = out_dir.join("recipes_generated.rs");
    fs::write(&out, generate(&files, &chaos))
        .unwrap_or_else(|e| panic!("write {}: {e}", out.display()));
}

/// The `object_name`s of the files marked `chaos: true`, sorted.
fn chaos_object_names(files: &[RecipeFile]) -> Vec<String> {
    let mut names: Vec<String> = files
        .iter()
        .filter_map(|file| {
            let source = fs::read_to_string(&file.file)
                .unwrap_or_else(|e| panic!("read {}: {e}", file.file.display()));
            // A build script cannot depend on the crate it builds, so the marker
            // is read here rather than through `recibase_core::md`.
            has_chaos_marker(&source).then(|| frontmatter_value(&source, "object_name"))
        })
        .collect();
    names.sort();
    names
}

/// Whether the document's frontmatter marks it as ours.
fn has_chaos_marker(source: &str) -> bool {
    let Some(front) = frontmatter(source) else {
        return false;
    };
    front.lines().any(|line| {
        let Some((key, value)) = line.split_once(':') else {
            return false;
        };
        key.trim() == "chaos" && !matches!(value.trim(), "" | "false" | "no")
    })
}

/// The frontmatter block, without its `---` fences.
fn frontmatter(source: &str) -> Option<&str> {
    let rest = source.strip_prefix("---")?;
    let rest = rest
        .strip_prefix('\n')
        .or_else(|| rest.strip_prefix("\r\n"))?;
    let mut offset = 0;
    for line in rest.split_inclusive('\n') {
        let end = offset + line.len();
        if line.trim_end() == "---" {
            return Some(&rest[..offset]);
        }
        offset = end;
    }
    None
}

fn frontmatter_value(source: &str, key: &str) -> String {
    frontmatter(source)
        .and_then(|front| {
            front.lines().find_map(|line| {
                let (k, v) = line.split_once(':')?;
                (k.trim() == key).then(|| v.trim().to_string())
            })
        })
        .unwrap_or_else(|| panic!("a chaos recipe has no `{key}`"))
}

/// Every `*.md` in `dir`, sorted by filename.
///
/// The identifier is the stem, which is a valid Rust identifier only because the
/// filenames are `snake_case`; it names the `const` holding the file's text.
fn recipe_files(dir: &Path) -> Vec<RecipeFile> {
    let mut paths: Vec<PathBuf> = fs::read_dir(dir)
        .unwrap_or_else(|e| panic!("read {}: {e}", dir.display()))
        .map(|entry| entry.expect("directory entry").path())
        .filter(|path| path.extension().is_some_and(|ext| ext == "md"))
        .collect();
    paths.sort();

    paths
        .into_iter()
        .map(|file| {
            let stem = file
                .file_stem()
                .expect("recipe file has a stem")
                .to_string_lossy()
                .into_owned();
            assert!(
                is_ident(&stem),
                "{}: the filename must be a snake_case Rust identifier",
                file.display()
            );
            RecipeFile { ident: stem, file }
        })
        .collect()
}

/// ASCII identifier: the filenames are ours, so this is a guard rather than a
/// full Unicode check.
fn is_ident(name: &str) -> bool {
    let mut chars = name.chars();
    matches!(chars.next(), Some(c) if c.is_ascii_alphabetic() || c == '_')
        && chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

/// The generated file, `include!`d by `src/recipes/mod.rs`.
fn generate(files: &[RecipeFile], chaos: &[String]) -> String {
    let mut out = String::new();

    // A plain comment, not `//!`: this text is `include!`d into `mod.rs`, which
    // already carries the module's inner doc comment, so an inner one here is
    // rejected (`E0753`).
    out.push_str(
        "// GENERATED by crates/core/build.rs - do not edit by hand.\n\n\
         use crate::recipe::RecipeDef;\nuse std::sync::LazyLock;\n\n\
         /// The markdown source of every recipe, in filename order.\n\
         const SOURCES: &[(&str, &str)] = &[\n",
    );
    for file in files {
        writeln!(
            out,
            "    ({:?}, include_str!({:?})),",
            file.ident,
            file.file.display().to_string()
        )
        .unwrap();
    }
    out.push_str("];\n\n");

    out.push_str(
        "/// The corpus, parsed once on first use.\n\
         pub fn recipes() -> &'static [RecipeDef] {\n\
         \x20   static RECIPES: LazyLock<Vec<RecipeDef>> = LazyLock::new(|| {\n\
         \x20       SOURCES\n\
         \x20           .iter()\n\
         \x20           .map(|(file, source)| crate::md::parse(source, file))\n\
         \x20           .collect()\n\
         \x20   });\n\
         \x20   &RECIPES\n}\n\n",
    );

    out.push_str(
        "/// The `object_name`s of recipes that are ours rather than part of the\n\
         /// upstream corpus. `recipe_corpus` skips them, and the API marks their\n\
         /// list entries so the frontend can tell them from the upstream corpus.\n\
         pub fn chaos_recipes() -> &'static [&'static str] {\n    &[\n",
    );
    for object_name in chaos {
        writeln!(out, "        {object_name:?},").unwrap();
    }
    out.push_str("    ]\n}\n");

    out
}
