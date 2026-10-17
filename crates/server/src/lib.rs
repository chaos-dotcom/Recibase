//! The Recibase HTTP API: byte-for-byte compatible with the Scala server.

pub mod admin;
pub mod auth;
pub mod config;
pub mod controllers;
pub mod db;
pub mod http;
pub mod meal_log;
pub mod oidc;
pub mod routes;
pub mod submission;
pub mod tokens;

pub use controllers::{Usage, meal_names, meals_json, recipes_json};
pub use routes::{Context, Env, route};
