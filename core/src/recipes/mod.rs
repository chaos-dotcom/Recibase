//! The recipe corpus.
//!
//! One module per Scala `case object ... extends Recipe`. `mod.rs` is
//! generated from the directory listing; `recipes()` returns the corpus in the
//! order the Scala classpath scan (`org.reflections`) produced.

pub mod all;

pub use all::recipes;
