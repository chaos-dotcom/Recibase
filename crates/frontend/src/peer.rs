//! Peers: other Recibase deployments whose recipes we list alongside our own.
//!
//! Ours wins a name collision, so a copy we hold never shadows the peer's
//! entry; a peer-only recipe is added, labelled with their name and linked to
//! their site; and our entry gains an `also` link when a peer lists the same
//! name, which is the hint that they have this recipe too.

use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use serde_json::{Value, json};

use crate::backend::BackendClient;
use crate::cached_backend::CachedBackendCall;

/// One extra backend, e.g. Kit & Alex's `api.reciba.se`.
#[derive(Clone)]
pub struct Peer {
    /// The owners' name, shown next to their recipes ("Kit & Alex").
    pub label: String,
    /// The peer's website, with no trailing slash, so their recipes can link
    /// out as `<site_url>/<permalink>`.
    pub site_url: String,
    /// The peer's `/recipes/` list, cached on the same 15-minute TTL as ours.
    pub recipes: Arc<CachedBackendCall<Value>>,
}

impl Peer {
    /// `api_base_url` is the peer's API, **with** its trailing slash.
    pub fn new(label: String, api_base_url: String, site_url: String) -> Peer {
        let backend = Arc::new(BackendClient::new(api_base_url));
        let recipes = Arc::new(CachedBackendCall::new({
            let backend = Arc::clone(&backend);
            move || backend.get_json("recipes/")
        }));
        Peer {
            label,
            site_url: site_url.trim_end_matches('/').to_string(),
            recipes,
        }
    }

    /// The peer's page for one of their recipes.
    pub fn recipe_url(&self, permalink: &str) -> String {
        format!("{}/{}", self.site_url, permalink)
    }
}

/// `PEER_BACKENDS`: peers separated by `;`, fields by `|` -
/// `Kit & Alex|https://api.reciba.se/|https://reciba.se`.
///
/// A malformed entry is skipped, so a typo costs one peer rather than the whole
/// frontend.
pub fn parse_peer_backends(value: &str) -> Vec<Peer> {
    value
        .split(';')
        .filter_map(|entry| {
            let mut fields = entry.split('|').map(str::trim);
            let label = fields.next().filter(|field| !field.is_empty())?;
            let api = fields.next().filter(|field| !field.is_empty())?;
            let site = fields.next().filter(|field| !field.is_empty())?;
            Some(Peer::new(
                label.to_string(),
                api.to_string(),
                site.to_string(),
            ))
        })
        .collect()
}

/// One peer's recipe list, already fetched and ready to merge.
pub struct PeerList {
    pub label: String,
    /// The peer's site URL, with no trailing slash.
    pub site_url: String,
    pub recipes: Vec<Value>,
}

impl PeerList {
    fn recipe_url(&self, permalink: &str) -> String {
        format!("{}/{}", self.site_url, permalink)
    }
}

/// Merge our list with the peers' into the drawer list, keyed on the recipe
/// `name`. See the module docs for the rules. Every entry ends up with an
/// `href`; ours is the relative permalink, a peer's the absolute URL.
pub fn merge_recipe_lists(own: &[Value], peers: &[PeerList]) -> Vec<Value> {
    let mut seen: HashSet<&str> = HashSet::new();
    for entry in own {
        if let Some(name) = entry.get("name").and_then(Value::as_str) {
            seen.insert(name);
        }
    }

    let mut also: HashMap<&str, Vec<Value>> = HashMap::new();
    let mut extras: Vec<Value> = Vec::new();
    for peer in peers {
        for entry in &peer.recipes {
            let Some(name) = entry.get("name").and_then(Value::as_str) else {
                continue;
            };
            let permalink = entry
                .get("permalink")
                .and_then(Value::as_str)
                .unwrap_or_default();
            let link = json!({ "label": peer.label, "url": peer.recipe_url(permalink) });
            if seen.contains(name) {
                also.entry(name).or_default().push(link);
            } else {
                seen.insert(name);
                let mut copy = entry.clone();
                if let Value::Object(map) = &mut copy {
                    // `ours` is relative to the server that sent the list, so a
                    // peer's copy of one of ours is not "ours" in our drawer.
                    map.remove("ours");
                    map.insert(
                        "href".to_string(),
                        Value::String(peer.recipe_url(permalink)),
                    );
                    map.insert("source".to_string(), Value::String(peer.label.clone()));
                }
                extras.push(copy);
            }
        }
    }

    let mut merged: Vec<Value> = own
        .iter()
        .map(|entry| {
            let mut copy = entry.clone();
            let permalink = entry
                .get("permalink")
                .and_then(Value::as_str)
                .unwrap_or_default();
            if let Value::Object(map) = &mut copy {
                map.insert("href".to_string(), Value::String(permalink.to_string()));
                if let Some(name) = entry.get("name").and_then(Value::as_str)
                    && let Some(links) = also.get(name)
                {
                    map.insert("also".to_string(), Value::Array(links.clone()));
                }
            }
            copy
        })
        .collect();
    merged.extend(extras);
    merged.sort_by(|a, b| name_of(a).cmp(name_of(b)));
    merged
}

fn name_of(entry: &Value) -> &str {
    entry
        .get("name")
        .and_then(Value::as_str)
        .unwrap_or_default()
}
