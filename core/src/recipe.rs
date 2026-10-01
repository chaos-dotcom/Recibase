//! `se.reciba.api.model.Recipe` and its ingredient types.

use crate::json::{arr, obj, opt_str};
use crate::permalink::from_raw_string;
use crate::tag::Tag;
use crate::utils::string_utils::unpluralise;
use chrono::NaiveDate;
use serde_json::Value;

pub const RECIPE_DIR: &str = "https://github.com/The-Silverwood-Institute/Recibase/tree/master/src/main/scala/se/reciba/api/recibase/recipes";

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Ingredient {
    pub name: String,
    pub quantity: Option<String>,
    pub prep: Option<String>,
    pub notes: Option<String>,
}

impl Ingredient {
    /// `Ingredient(name)`.
    pub fn new(name: &str) -> Self {
        Ingredient { name: name.to_string(), ..Default::default() }
    }

    /// `Ingredient(name, quantity)`.
    pub fn q(name: &str, quantity: &str) -> Self {
        Ingredient { name: name.to_string(), quantity: Some(quantity.to_string()), ..Default::default() }
    }

    /// `Ingredient(name, quantity, prep)`.
    pub fn qp(name: &str, quantity: &str, prep: &str) -> Self {
        Ingredient { name: name.to_string(), quantity: Some(quantity.to_string()), prep: Some(prep.to_string()), notes: None }
    }

    /// `Ingredient(name, quantity, prep, notes)`.
    pub fn qpn(name: &str, quantity: &str, prep: &str, notes: &str) -> Self {
        Ingredient {
            name: name.to_string(),
            quantity: Some(quantity.to_string()),
            prep: Some(prep.to_string()),
            notes: Some(notes.to_string()),
        }
    }

    /// `Ingredient(name, quantity.some, prep, notes)` - the four-argument form
    /// with explicit `Option`s, as used by the recipe files.
    pub fn opt(
        name: &str,
        quantity: Option<&str>,
        prep: Option<&str>,
        notes: Option<&str>,
    ) -> Self {
        Ingredient {
            name: name.to_string(),
            quantity: quantity.map(|s| s.to_string()),
            prep: prep.map(|s| s.to_string()),
            notes: notes.map(|s| s.to_string()),
        }
    }

    pub fn to_json(&self) -> Value {
        obj(vec![
            ("name", Value::String(self.name.clone())),
            ("quantity", opt_str(&self.quantity)),
            ("prep", opt_str(&self.prep)),
            ("notes", opt_str(&self.notes)),
        ])
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct IngredientsBlock {
    pub name: Option<String>,
    pub ingredients: Vec<Ingredient>,
}

impl IngredientsBlock {
    pub fn new(name: Option<&str>, ingredients: Vec<Ingredient>) -> Self {
        IngredientsBlock { name: name.map(|s| s.to_string()), ingredients }
    }

    /// `IngredientsBlock.simple(...)`.
    pub fn simple(ingredients: Vec<Ingredient>) -> Vec<IngredientsBlock> {
        vec![IngredientsBlock { name: None, ingredients }]
    }

    /// `prefixIngredients(additionalIngredients: _*)`.
    pub fn prefix_ingredients(&self, additional: Vec<Ingredient>) -> IngredientsBlock {
        let mut ingredients = additional;
        ingredients.extend(self.ingredients.iter().cloned());
        IngredientsBlock { name: self.name.clone(), ingredients }
    }

    pub fn to_json(&self) -> Value {
        obj(vec![
            ("name", opt_str(&self.name)),
            (
                "ingredients",
                Value::Array(self.ingredients.iter().map(|i| i.to_json()).collect()),
            ),
        ])
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Image {
    pub src: String,
    pub width: i64,
    pub height: i64,
}

impl Image {
    /// `Image(src)` - 1080x1080 by default.
    pub fn new(src: &str) -> Self {
        Image { src: src.to_string(), width: 1080, height: 1080 }
    }

    pub fn sized(src: &str, width: i64, height: i64) -> Self {
        Image { src: src.to_string(), width, height }
    }

    pub fn to_json(&self) -> Value {
        obj(vec![
            ("src", Value::String(self.src.clone())),
            ("width", Value::from(self.width)),
            ("height", Value::from(self.height)),
        ])
    }
}

/// A `Recipe` case object: name, metadata, tags and the content itself.
///
/// `tags` keeps the insertion order used in the Scala source; the Scala `Set`
/// iteration order used on the wire is applied when encoding.
#[derive(Debug, Clone, Default)]
pub struct RecipeDef {
    pub object_name: String,
    pub name: String,
    pub created_at: NaiveDate,
    pub permalink_override: Option<String>,
    pub source: Option<String>,
    pub description: Option<String>,
    pub tagline: Option<String>,
    pub notes: Vec<String>,
    pub tags: Vec<Tag>,
    pub image: Option<Image>,
    pub ingredients_blocks: Vec<IngredientsBlock>,
    pub method: Vec<String>,
}

impl RecipeDef {
    pub fn permalink(&self) -> String {
        match &self.permalink_override {
            Some(p) => p.clone(),
            None => from_raw_string(&self.name),
        }
    }

    pub fn edit(&self) -> String {
        format!("{}/{}.scala", RECIPE_DIR, self.object_name)
    }

    /// `hasIngredient`, including the `!` negation prefix.
    pub fn has_ingredient(&self, ingredient: &str) -> bool {
        let normalised = unpluralise(&ingredient.to_lowercase());
        let contains = |needle: &str| {
            self.ingredients_blocks
                .iter()
                .flat_map(|b| b.ingredients.iter())
                .any(|i| i.name.to_lowercase().contains(needle))
        };
        match normalised.strip_prefix('!') {
            Some(filtered) => !contains(filtered),
            None => contains(&normalised),
        }
    }

    /// `isDinner`: no tag intersects `Tag.nonDinnerTags`.
    pub fn is_dinner(&self) -> bool {
        let non_dinner = Tag::non_dinner_tags();
        !self.tags.iter().any(|t| non_dinner.contains(t))
    }

    /// `tags.flatMap(_.allParentTags)` as a Scala `Set`, in iteration order.
    pub fn inherited_tags(&self) -> Vec<Tag> {
        crate::scala_hash::scala_set_flat_map(&self.tags, |t| t.all_parent_tags())
    }

    /// `tags` as a Scala `Set`, in iteration order.
    pub fn tags_in_set_order(&self) -> Vec<Tag> {
        crate::scala_hash::scala_set(&self.tags)
    }

    pub fn to_json_with_usage(&self, dated_notes: &[crate::meal::DatedNote]) -> Value {
        obj(vec![
            ("name", Value::String(self.name.clone())),
            ("permalink", Value::String(self.permalink())),
            ("edit", Value::String(self.edit())),
            ("source", opt_str(&self.source)),
            ("description", opt_str(&self.description)),
            ("tagline", opt_str(&self.tagline)),
            ("notes", arr(self.notes.iter().map(|n| Value::String(n.clone())).collect())),
            (
                "dated_notes",
                arr(dated_notes.iter().map(|n| n.to_json()).collect()),
            ),
            (
                "tags",
                arr(self
                    .tags_in_set_order()
                    .iter()
                    .map(|t| Value::String(t.entry_name().to_string()))
                    .collect()),
            ),
            (
                "inherited_tags",
                arr(self
                    .inherited_tags()
                    .iter()
                    .map(|t| Value::String(t.entry_name().to_string()))
                    .collect()),
            ),
            ("image", self.image.as_ref().map(|i| i.to_json()).unwrap_or(Value::Null)),
            (
                "ingredients_blocks",
                arr(self.ingredients_blocks.iter().map(|b| b.to_json()).collect()),
            ),
            ("method", arr(self.method.iter().map(|m| Value::String(m.clone())).collect())),
        ])
    }

}
