//! The Recibase web frontend, ported from the Flask application in
//! `The-Silverwood-Institute/Frontend`.
//!
//! The modules mirror the Python ones:
//!
//! | Python | Rust |
//! |---|---|
//! | `app.py` | `app.rs`, with `http.rs` for the server and `pages.rs` for the pages Werkzeug produces |
//! | `scaler.py` | `scaler.rs` |
//! | `contribute.py` | `contribute.rs` |
//! | `cached_backend.py` | `cached_backend.rs` |
//! | `app.py`'s `resolve_deployed_version` | `version.rs` |
//! | `templates/` | `templates.rs`, rendering the same sources with MiniJinja |

pub mod app;
pub mod backend;
pub mod cached_backend;
pub mod contribute;
pub mod form;
pub mod http;
pub mod markupsafe;
pub mod pages;
pub mod peer;
pub mod scaler;
pub mod statics;
pub mod templates;
pub mod version;
