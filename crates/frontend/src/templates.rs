//! The Jinja2 templates, rendered with MiniJinja.
//!
//! `templates/` is a byte-identical copy of the Flask application's
//! directory; the sources are compiled into the binary so that a deployment
//! is one file. `layout.html` and `sitemap.xml` call `fetchRecipeList()` and
//! `fetchApiVersion()`, the two callables `app.py` injects into every
//! template through its context processor, so they are installed as
//! MiniJinja globals backed by the same 15-minute cache.

use std::sync::Arc;

use minijinja::value::Value;
use minijinja::{AutoEscape, Environment};

use crate::cached_backend::{BackendUnavailable, CachedBackendCall};
use crate::peer::{Peer, PeerList, merge_recipe_lists};

/// The eight templates, compiled in.
pub const TEMPLATES: &[(&str, &str)] = &[
    ("layout.html", include_str!("../templates/layout.html")),
    ("home.html", include_str!("../templates/home.html")),
    ("recipe.html", include_str!("../templates/recipe.html")),
    (
        "contribute.html",
        include_str!("../templates/contribute.html"),
    ),
    ("notfound.html", include_str!("../templates/notfound.html")),
    (
        "internalerror.html",
        include_str!("../templates/internalerror.html"),
    ),
    (
        "backendunavailable.html",
        include_str!("../templates/backendunavailable.html"),
    ),
    ("sitemap.xml", include_str!("../templates/sitemap.xml")),
];

/// The message of the error a failed `fetchRecipeList()` raises, which the
/// request handler turns into the 503 page.
const BACKEND_UNAVAILABLE: &str = "recibase-frontend: the recipe API is unavailable";

pub fn is_backend_unavailable(error: &minijinja::Error) -> bool {
    error.to_string().contains(BACKEND_UNAVAILABLE)
}

fn backend_unavailable() -> minijinja::Error {
    minijinja::Error::new(minijinja::ErrorKind::InvalidOperation, BACKEND_UNAVAILABLE)
}

/// MiniJinja implements no `Mapping.get`, and `recipe.html` calls it once. This
/// is the only change made to the template sources, and it is applied when they
/// are loaded rather than to the files, which stay identical to the Flask
/// application's. MiniJinja's undefined behaviour is lenient, so an absent
/// `scaled` key is falsy - which is what `dict.get` returning `None` does in the
/// Python.
const TEMPLATE_SUBSTITUTIONS: &[(&str, &str)] = &[
    ("ingredient.get('scaled')", "ingredient['scaled']"),
    (
        "recipe['source'].startswith('http')",
        "recipe['source'][:4] == 'http'",
    ),
];

fn prepare_source(source: &str) -> String {
    let mut prepared = source.to_string();
    for (from, to) in TEMPLATE_SUBSTITUTIONS {
        prepared = prepared.replace(from, to);
    }
    prepared
}

pub struct Templates {
    env: Environment<'static>,
}

impl Templates {
    pub fn new(
        recipe_list: Arc<CachedBackendCall<serde_json::Value>>,
        api_version: Arc<CachedBackendCall<String>>,
        frontend_version: &str,
    ) -> Templates {
        Self::with_peers(recipe_list, api_version, frontend_version, Vec::new())
    }

    /// As [`Templates::new`], plus peer deployments to list alongside ours.
    pub fn with_peers(
        recipe_list: Arc<CachedBackendCall<serde_json::Value>>,
        api_version: Arc<CachedBackendCall<String>>,
        frontend_version: &str,
        peers: Vec<Peer>,
    ) -> Templates {
        let mut env = Environment::new();
        for (name, source) in TEMPLATES {
            env.add_template_owned(name.to_string(), prepare_source(source))
                .expect("the embedded templates compile");
        }
        // Flask's default `select_autoescape` escapes every template.
        env.set_auto_escape_callback(|_name| AutoEscape::Html);
        env.add_global(
            "config",
            Value::from_serialize(serde_json::json!({
                "frontendVersion": frontend_version,
            })),
        );
        env.add_global(
            "fetchRecipeList",
            Value::from_function(move || -> Result<Value, minijinja::Error> {
                let own = recipe_list
                    .fetch_data()
                    .map_err(|_: BackendUnavailable| backend_unavailable())?;
                let own_entries = own.as_array().cloned().unwrap_or_default();
                // A peer that cannot be reached is skipped, not fatal: the
                // drawer still shows our recipes (and any peer that answered).
                let lists: Vec<PeerList> = peers
                    .iter()
                    .filter_map(|peer| {
                        let value = peer.recipes.fetch_data().ok()?;
                        Some(PeerList {
                            label: peer.label.clone(),
                            site_url: peer.site_url.clone(),
                            recipes: value.as_array().cloned()?,
                        })
                    })
                    .collect();
                Ok(Value::from_serialize(merge_recipe_lists(
                    &own_entries,
                    &lists,
                )))
            }),
        );
        env.add_global(
            "fetchApiVersion",
            Value::from_function(move || -> Result<Value, minijinja::Error> {
                Ok(Value::from(
                    api_version
                        .fetch_data()
                        .map_err(|_: BackendUnavailable| backend_unavailable())?,
                ))
            }),
        );
        Templates { env }
    }

    /// `render_template`. The result is put through
    /// [`crate::markupsafe::markupsafe_compat`] so that the escaping matches
    /// Jinja2's.
    pub fn render(&self, name: &str, context: Value) -> Result<String, minijinja::Error> {
        let template = self.env.get_template(name)?;
        let rendered = template.render(context)?;
        Ok(crate::markupsafe::markupsafe_compat(&rendered))
    }
}
