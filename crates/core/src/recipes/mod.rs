//! The recipe corpus.
//!
//! One markdown file per recipe in `recipes/`. The registry and
//! `chaos_recipes()` are derived from that directory at compile time by
//! `crates/core/build.rs`, so a recipe is added by adding a file there: there is
//! no generator to run and no generated file to commit. See [`crate::md`] for
//! the file format.

include!(concat!(env!("OUT_DIR"), "/recipes_generated.rs"));
