//! `se.reciba.api.submit.RecipeSubmission` and `object Passcode`.

use serde_json::Value;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct IngredientSubmission {
    pub name: String,
    pub quantity: Option<String>,
    pub prep: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct RecipeSubmission {
    pub name: String,
    pub source: Option<String>,
    pub description: Option<String>,
    pub notes: Vec<String>,
    pub tags: Vec<String>,
    pub ingredients: Vec<IngredientSubmission>,
    pub method: Vec<String>,
}

/// A circe `DecodingFailure`: the expectation and the cursor path, rendered the
/// way circe's `show` renders them.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubmissionDecodeError {
    pub message: String,
}

impl fmt::Display for SubmissionDecodeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl std::error::Error for SubmissionDecodeError {}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Step {
    Field(String),
    Index(usize),
}

type DecodeResult<T> = Result<T, SubmissionDecodeError>;

fn failure(expected: &str, history: &[Step]) -> SubmissionDecodeError {
    let mut path = String::new();
    for step in history {
        match step {
            Step::Field(name) => {
                path.push('.');
                path.push_str(name);
            }
            Step::Index(index) => path.push_str(&format!("[{}]", index)),
        }
    }
    SubmissionDecodeError {
        message: format!("DecodingFailure at {}: {}", path, expected),
    }
}

/// `HCursor.downField(key)`: a value that is not an object has no fields, which
/// is a failed cursor.
fn field<'a>(value: &'a Value, key: &str) -> Option<&'a Value> {
    value.as_object().and_then(|object| object.get(key))
}

fn push_field(history: &[Step], key: &str) -> Vec<Step> {
    let mut out = history.to_vec();
    out.push(Step::Field(key.to_string()));
    out
}

fn push_index(history: &[Step], index: usize) -> Vec<Step> {
    let mut out = history.to_vec();
    out.push(Step::Index(index));
    out
}

/// `cursor.get[String](key)`: the key must be present and hold a string.
fn required_string(object: &Value, key: &str, history: &[Step]) -> DecodeResult<String> {
    let here = push_field(history, key);
    match field(object, key) {
        None => Err(failure("Attempt to decode value on failed cursor", &here)),
        Some(Value::String(text)) => Ok(text.clone()),
        Some(_) => Err(failure("String", &here)),
    }
}

/// `cursor.get[Option[String]](key)`: a missing key or `null` is `None`.
fn optional_string(object: &Value, key: &str, history: &[Step]) -> DecodeResult<Option<String>> {
    let here = push_field(history, key);
    match field(object, key) {
        None | Some(Value::Null) => Ok(None),
        Some(Value::String(text)) => Ok(Some(text.clone())),
        Some(_) => Err(failure("String", &here)),
    }
}

fn string_items(items: &[Value], history: &[Step]) -> DecodeResult<Vec<String>> {
    let mut out = Vec::with_capacity(items.len());
    for (index, item) in items.iter().enumerate() {
        match item {
            Value::String(text) => out.push(text.clone()),
            _ => return Err(failure("String", &push_index(history, index))),
        }
    }
    Ok(out)
}

/// `optionalList`: a missing key or `null` is an empty list.
fn optional_string_list(object: &Value, key: &str, history: &[Step]) -> DecodeResult<Vec<String>> {
    let here = push_field(history, key);
    match field(object, key) {
        None | Some(Value::Null) => Ok(Vec::new()),
        Some(Value::Array(items)) => string_items(items, &here),
        Some(_) => Err(failure("List", &here)),
    }
}

fn required_string_list(object: &Value, key: &str, history: &[Step]) -> DecodeResult<Vec<String>> {
    let here = push_field(history, key);
    match field(object, key) {
        None => Err(failure("Attempt to decode value on failed cursor", &here)),
        Some(Value::Array(items)) => string_items(items, &here),
        Some(_) => Err(failure("List", &here)),
    }
}

fn required_ingredients(
    object: &Value,
    key: &str,
    history: &[Step],
) -> DecodeResult<Vec<IngredientSubmission>> {
    let here = push_field(history, key);
    match field(object, key) {
        None => Err(failure("Attempt to decode value on failed cursor", &here)),
        Some(Value::Array(items)) => {
            let mut out = Vec::with_capacity(items.len());
            for (index, item) in items.iter().enumerate() {
                out.push(IngredientSubmission::decode_at(
                    item,
                    &push_index(&here, index),
                )?);
            }
            Ok(out)
        }
        Some(_) => Err(failure("List", &here)),
    }
}

impl IngredientSubmission {
    /// The circe `Decoder[IngredientSubmission]`.
    pub fn decode(value: &Value) -> DecodeResult<IngredientSubmission> {
        IngredientSubmission::decode_at(value, &[])
    }

    fn decode_at(value: &Value, history: &[Step]) -> DecodeResult<IngredientSubmission> {
        let name = required_string(value, "name", history)?;
        let quantity = optional_string(value, "quantity", history)?;
        let prep = optional_string(value, "prep", history)?;
        let notes = optional_string(value, "notes", history)?;
        Ok(IngredientSubmission {
            name,
            quantity,
            prep,
            notes,
        })
    }
}

impl RecipeSubmission {
    /// The circe `Decoder[RecipeSubmission]`, including its exact acceptance:
    /// `name` and `ingredients` and `method` are required, `notes` and `tags`
    /// are optional lists, `source`/`description` accept `null`.
    pub fn decode(value: &Value) -> DecodeResult<RecipeSubmission> {
        let name = required_string(value, "name", &[])?;
        let source = optional_string(value, "source", &[])?;
        let description = optional_string(value, "description", &[])?;
        let notes = optional_string_list(value, "notes", &[])?;
        let tags = optional_string_list(value, "tags", &[])?;
        let ingredients = required_ingredients(value, "ingredients", &[])?;
        let method = required_string_list(value, "method", &[])?;
        Ok(RecipeSubmission {
            name,
            source,
            description,
            notes,
            tags,
            ingredients,
            method,
        })
    }

    /// As [`RecipeSubmission::decode`], for callers that only need to know
    /// whether the body decoded.
    pub fn from_json(value: &Value) -> Option<RecipeSubmission> {
        RecipeSubmission::decode(value).ok()
    }
}

pub struct Passcode;

impl Passcode {
    /// `java.security.MessageDigest.isEqual`: a constant-time comparison of the
    /// UTF-8 bytes. A length difference is folded into the same accumulator, so
    /// the comparison never returns early.
    pub fn equal(expected: &str, provided: &str) -> bool {
        let expected = expected.as_bytes();
        let provided = provided.as_bytes();
        if provided.is_empty() {
            return expected.is_empty();
        }
        let mut result: i32 = 0;
        result |= expected.len() as i32 - provided.len() as i32;
        for (index, byte) in expected.iter().enumerate() {
            let other = if index < provided.len() { index } else { 0 };
            result |= (*byte ^ provided[other]) as i32;
        }
        result == 0
    }
}
