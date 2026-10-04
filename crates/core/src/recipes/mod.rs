//! The recipe corpus.
//!
//! One module per Scala `case object ... extends Recipe`. The module list, the
//! registry and `chaos_recipes()` are derived from this directory at compile
//! time by `crates/core/build.rs`, so a recipe is added by adding a file here:
//! there is no generator to run and no generated file to commit.

include!(concat!(env!("OUT_DIR"), "/recipes_generated.rs"));
