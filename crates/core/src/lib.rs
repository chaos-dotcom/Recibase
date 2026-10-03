//! A port of the Recibase recipe API, byte-for-byte compatible with the Scala
//! implementation in `Se/Recibase`.

pub mod ice_cream;
pub mod json;
pub mod meal;
pub mod meal_definitions;
pub mod misc;
pub mod permalink;
pub mod recipe;
pub mod recipes;
pub mod scala_hash;
pub mod stop_words;
pub mod tag;
pub mod usage;
pub mod utils;

pub use meal::{DatedNote, MealStub, MealStubWithUsageData, Source};
pub use misc::{docs_json, Manifest, MenuEntry};
pub use permalink::Permalink;
pub use recipe::{Image, Ingredient, IngredientsBlock, RecipeDef};
pub use tag::Tag;
