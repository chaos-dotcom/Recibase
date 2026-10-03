//! Helpers for building circe-compatible JSON.
//!
//! circe prints compact JSON and preserves the order in which fields were
//! declared, so the port always builds objects in the field order the Scala
//! encoder uses. `serde_json` is built with the `preserve_order` feature.

use serde_json::{Map, Value};

pub fn obj(fields: Vec<(&str, Value)>) -> Value {
    let mut map = Map::new();
    for (key, value) in fields {
        map.insert(key.to_string(), value);
    }
    Value::Object(map)
}

pub fn arr(items: Vec<Value>) -> Value {
    Value::Array(items)
}

/// circe encodes `None` as JSON `null`.
pub fn opt_str(value: &Option<String>) -> Value {
    match value {
        Some(v) => Value::String(v.clone()),
        None => Value::Null,
    }
}

pub fn opt_date(value: &Option<chrono::NaiveDate>) -> Value {
    match value {
        Some(v) => Value::String(v.format("%Y-%m-%d").to_string()),
        None => Value::Null,
    }
}

pub fn str_value(value: &str) -> Value {
    Value::String(value.to_string())
}

/// Compact printing, exactly as circe's default printer does.
pub fn to_bytes(value: &Value) -> Vec<u8> {
    serde_json::to_vec(value).expect("JSON serialisation cannot fail")
}

pub fn to_string(value: &Value) -> String {
    serde_json::to_string(value).expect("JSON serialisation cannot fail")
}
