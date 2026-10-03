//! The Recibase HTTP API: byte-for-byte compatible with the Scala server.

pub mod controllers;
pub mod http;
pub mod meal_log;
pub mod routes;
pub mod submission;

pub use controllers::{Usage, meal_names, meals_json, recipes_json};
pub use routes::{Context, Env, route};
