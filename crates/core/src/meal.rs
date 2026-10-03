//! `se.reciba.api.model.Meal`, `MealStub`, `Source` and dated notes.

use crate::json::{arr, obj, opt_date};
use crate::recipe::RecipeDef;
use crate::tag::Tag;
use chrono::NaiveDate;
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Source {
    Online(String),
    Recibase(String),
    GoogleDrive(String),
}

impl Source {
    pub fn to_json(&self) -> Value {
        match self {
            Source::Online(url) => obj(vec![
                ("url", Value::String(url.clone())),
                ("type", Value::String("online".to_string())),
            ]),
            Source::Recibase(permalink) => obj(vec![
                ("permalink", Value::String(permalink.clone())),
                ("type", Value::String("recibase".to_string())),
            ]),
            Source::GoogleDrive(id) => obj(vec![
                ("id", Value::String(id.clone())),
                ("type", Value::String("google_drive".to_string())),
            ]),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DatedNote {
    pub date: NaiveDate,
    pub note: String,
}

impl DatedNote {
    pub fn to_json(&self) -> Value {
        obj(vec![
            ("date", Value::String(self.date.format("%Y-%m-%d").to_string())),
            ("note", Value::String(self.note.clone())),
        ])
    }
}

#[derive(Debug, Clone)]
pub struct MealStub {
    pub name: String,
    /// Insertion order as written in Scala; `Set` order is applied on output.
    pub tags: Vec<Tag>,
    pub source: Option<Source>,
    pub created_at: Option<NaiveDate>,
}

impl MealStub {
    pub fn new(name: &str, tags: Vec<Tag>) -> Self {
        MealStub { name: name.to_string(), tags, source: None, created_at: None }
    }

    pub fn with_source(name: &str, tags: Vec<Tag>, source: Source) -> Self {
        MealStub { name: name.to_string(), tags, source: Some(source), created_at: None }
    }

    /// `MealStub(recipe)`: a stub for a full recipe in the corpus.
    pub fn from_recipe(recipe: &RecipeDef) -> Self {
        MealStub {
            name: recipe.name.clone(),
            tags: recipe.tags.clone(),
            source: Some(Source::Recibase(recipe.permalink())),
            created_at: Some(recipe.created_at),
        }
    }

    pub fn is_dinner(&self) -> bool {
        let non_dinner = Tag::non_dinner_tags();
        !self.tags.iter().any(|t| non_dinner.contains(t))
    }
}

#[derive(Debug, Clone)]
pub struct MealStubWithUsageData {
    pub name: String,
    pub tags: Vec<Tag>,
    pub source: Option<Source>,
    pub dated_notes: Vec<DatedNote>,
    pub last_eaten: Option<NaiveDate>,
    pub times_eaten: i64,
    pub featured: Option<NaiveDate>,
}

impl MealStubWithUsageData {
    pub fn inherited_tags(&self) -> Vec<Tag> {
        crate::scala_hash::scala_set_flat_map(&self.tags, |t| t.all_parent_tags())
    }

    pub fn tags_in_set_order(&self) -> Vec<Tag> {
        crate::scala_hash::scala_set(&self.tags)
    }

    pub fn to_json(&self) -> Value {
        obj(vec![
            ("name", Value::String(self.name.clone())),
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
            ("source", self.source.as_ref().map(|s| s.to_json()).unwrap_or(Value::Null)),
            ("dated_notes", arr(self.dated_notes.iter().map(|n| n.to_json()).collect())),
            ("last_eaten", opt_date(&self.last_eaten)),
            ("times_eaten", Value::from(self.times_eaten)),
            ("featured", opt_date(&self.featured)),
        ])
    }
}
