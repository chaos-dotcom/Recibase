//! Peers: other Recibase deployments whose recipes we list alongside our own.
//!
//! Ours wins a name collision, so a copy we hold never shadows the peer's
//! entry; a peer-only recipe is added, labelled with their name, and served by
//! *this* frontend - its drawer link is the local permalink and the recipe page
//! is rendered from the peer's API, so a reader never leaves for the peer's
//! site. Our own entry gains a peer hint when a peer lists the same name with a
//! different content digest - `Found on <peer> too.` - a link that does go to
//! their site. Our own recipes are listed first, ahead of the peers'.

use std::collections::HashMap;
use std::sync::Arc;

use serde_json::{Value, json};

use crate::backend::{BackendClient, BackendResponse};
use crate::cached_backend::{BackendUnavailable, CachedBackendCall};

/// One extra backend, e.g. Kit & Alex's `api.reciba.se`.
#[derive(Clone)]
pub struct Peer {
    /// The owners' name, shown next to their recipes ("Kit & Alex").
    pub label: String,
    /// The peer's website, with no trailing slash, so the hint can point at
    /// their copy as `<site_url>/<permalink>`.
    pub site_url: String,
    /// The peer's `/recipes/` list, cached on the same 15-minute TTL as ours.
    pub recipes: Arc<CachedBackendCall<Value>>,
    /// The peer's API, for fetching one of their recipes to render here.
    backend: Arc<BackendClient>,
}

impl Peer {
    /// `api_base_url` is the peer's API, **with** its trailing slash.
    pub fn new(label: String, api_base_url: String, site_url: String) -> Peer {
        let backend = Arc::new(BackendClient::new(api_base_url));
        let recipes = Arc::new(CachedBackendCall::new({
            let backend = Arc::clone(&backend);
            move || backend.get_json("recipes/?withRevision=true&withTags=true")
        }));
        Peer {
            label,
            site_url: site_url.trim_end_matches('/').to_string(),
            recipes,
            backend,
        }
    }

    /// One of the peer's recipes, as their API returns it, so this frontend can
    /// render it instead of sending the reader to their site.
    pub fn recipe(&self, permalink: &str) -> Result<BackendResponse, BackendUnavailable> {
        self.backend.get(&format!("recipes/{permalink}"))
    }

    /// The peer's page for one of their recipes, for the hint that does leave
    /// for their site.
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

/// The frontend's own server, for the recipe page's "reci-verse" caption:
/// the name shown and the site it links to.
///
/// `SERVER_IDENTITY` supplies it - `label|site`, e.g.
/// `Chaos' Recibase Server|https://recibase.shed.gay`. Unset, the caption
/// keeps its generic "this server", so the same frontend can be pointed at any
/// deployment and name whichever one it is attached to.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ServerIdentity {
    /// The name shown in the caption ("Chaos' Recibase Server").
    pub label: String,
    /// The site linked to, with no trailing slash.
    pub site_url: String,
}

/// Parses `SERVER_IDENTITY`. A malformed value is `None`, so a typo leaves the
/// generic caption rather than failing the page.
pub fn parse_server_identity(value: &str) -> Option<ServerIdentity> {
    let mut fields = value.split('|').map(str::trim);
    let label = fields.next().filter(|field| !field.is_empty())?;
    let site = fields.next().filter(|field| !field.is_empty())?;
    Some(ServerIdentity {
        label: label.to_string(),
        site_url: site.trim_end_matches('/').to_string(),
    })
}

/// One peer's recipe list, already fetched and ready to merge.
pub struct PeerList {
    pub label: String,
    /// The peer's site URL, with no trailing slash.
    pub site_url: String,
    pub recipes: Vec<Value>,
}

/// Merge our list with the peers' into the drawer list, keyed on the recipe
/// `name`. See the module docs for the rules. Every entry ends up with an
/// `href`; both ours and a peer's are the relative permalink, because this
/// frontend serves a peer's recipe from the peer's API rather than linking out.
pub fn merge_recipe_lists(own: &[Value], peers: &[PeerList]) -> Vec<Value> {
    // The names we have, and our content digest for each when we have one.
    let mut known: HashMap<&str, Option<&str>> = HashMap::new();
    for entry in own {
        if let Some(name) = entry.get("name").and_then(Value::as_str) {
            known.insert(name, entry.get("revision").and_then(Value::as_str));
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
            let link =
                json!({ "label": peer.label, "url": format!("{}/{}", peer.site_url, permalink) });
            match known.get(name).copied() {
                // Same name. An equal digest is the same recipe, so there is
                // nothing to point at; anything else is a difference to hint.
                Some(our_revision) => {
                    if !same_revision(our_revision, entry.get("revision").and_then(Value::as_str)) {
                        also.entry(name).or_default().push(link);
                    }
                }
                None => {
                    let mut copy = entry.clone();
                    if let Value::Object(map) = &mut copy {
                        // `ours` and the digest are the sending deployment's,
                        // not ours, and neither belongs in the drawer.
                        map.remove("ours");
                        map.remove("revision");
                        // The local permalink, so this frontend renders their
                        // recipe from their API instead of linking out.
                        map.insert("href".to_string(), Value::String(permalink.to_string()));
                        map.insert("source".to_string(), Value::String(peer.label.clone()));
                    }
                    known.insert(name, None);
                    extras.push(copy);
                }
            }
        }
    }

    // Our own recipes first, then the peers': our server is prioritised, and
    // each group is sorted by name.
    let mut ours: Vec<Value> = own
        .iter()
        .map(|entry| {
            let mut copy = entry.clone();
            let permalink = entry
                .get("permalink")
                .and_then(Value::as_str)
                .unwrap_or_default();
            if let Value::Object(map) = &mut copy {
                map.remove("revision");
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
    ours.sort_by(|a, b| name_of(a).cmp(name_of(b)));
    extras.sort_by(|a, b| name_of(a).cmp(name_of(b)));
    ours.extend(extras);
    ours
}

/// Two digests agree only when both are present: a missing digest means the
/// sender did not report one, which is not the same as "identical".
pub fn same_revision(left: Option<&str>, right: Option<&str>) -> bool {
    matches!((left, right), (Some(a), Some(b)) if a == b)
}

fn name_of(entry: &Value) -> &str {
    entry
        .get("name")
        .and_then(Value::as_str)
        .unwrap_or_default()
}
