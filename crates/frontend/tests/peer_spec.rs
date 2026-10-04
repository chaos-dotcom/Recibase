//! `peer.rs` - merging another deployment's recipes into the drawer.

mod support;

use recibase_frontend::peer::{
    PeerList, merge_recipe_lists, parse_peer_backends, parse_server_identity,
};
use serde_json::json;

/// `PEER_BACKENDS` is `label|api|site`, several separated by `;`.
#[test]
fn parse_peer_backends_reads_pipe_separated_peers() {
    let peers = parse_peer_backends("Kit & Alex|https://api.reciba.se/|https://reciba.se");
    assert_eq!(peers.len(), 1);
    assert_eq!(peers[0].label, "Kit & Alex");
    assert_eq!(peers[0].recipe_url("pasta"), "https://reciba.se/pasta");
}

/// A malformed entry costs one peer, not the frontend; a trailing slash on the
/// site does not produce a double slash in the link.
#[test]
fn parse_peer_backends_skips_malformed_entries() {
    assert!(parse_peer_backends("").is_empty());
    assert!(parse_peer_backends("Kit & Alex|https://api.reciba.se/").is_empty());
    assert!(parse_peer_backends("|https://api.reciba.se/|https://reciba.se").is_empty());

    let peers = parse_peer_backends("A|http://a/|http://a/; ;B|http://b/|http://b");
    assert_eq!(peers.len(), 2);
    assert_eq!(peers[0].recipe_url("x"), "http://a/x");
    assert_eq!(peers[1].recipe_url("x"), "http://b/x");
}

/// The name is the key: ours wins a collision and gains an `also` link; a
/// peer-only recipe is added with its `source` and a local `href`, so this
/// frontend serves it rather than linking out.
#[test]
fn merge_keeps_ours_on_a_name_collision_and_links_the_peer() {
    let own = vec![json!({"name": "Pasta", "permalink": "pasta", "ours": true})];
    let peers = vec![PeerList {
        label: "Kit & Alex".to_string(),
        site_url: "https://reciba.se".to_string(),
        recipes: vec![
            json!({"name": "Pasta", "permalink": "pasta", "ours": true}),
            json!({"name": "Curry", "permalink": "curry"}),
        ],
    }];

    let merged = merge_recipe_lists(&own, &peers);

    assert_eq!(merged.len(), 2);
    // Our own recipe comes first; a peer's is added after ours.
    let pasta = &merged[0];
    assert_eq!(pasta["name"], "Pasta");
    assert_eq!(pasta["href"], "pasta");
    assert_eq!(pasta["ours"], true);
    assert_eq!(pasta["also"][0]["label"], "Kit & Alex");
    assert_eq!(pasta["also"][0]["url"], "https://reciba.se/pasta");

    let curry = &merged[1];
    assert_eq!(curry["name"], "Curry");
    assert_eq!(curry["source"], "Kit & Alex");
    assert_eq!(curry["href"], "curry");
    // Peer-only, so the drawer keeps it back for the reci-verse toggle.
    assert_eq!(curry["peer"], true);
    // `ours` is relative to the server that sent the list, not ours.
    assert!(curry.get("ours").is_none());
}

/// A recipe that is not ours (no chaos tag) came from the other server first,
/// so its peer is always credited; one of ours is credited only when the digest
/// differs.
#[test]
fn merge_hints_by_originality_then_digest() {
    let peer = |revision: &str| {
        vec![PeerList {
            label: "Kit & Alex".to_string(),
            site_url: "https://reciba.se".to_string(),
            recipes: vec![json!({"name": "Pasta", "permalink": "pasta", "revision": revision})],
        }]
    };

    // Not ours: the peer is credited even when the digest matches.
    let ported = vec![json!({"name": "Pasta", "permalink": "pasta", "revision": "aaaa"})];
    let same = merge_recipe_lists(&ported, &peer("aaaa"));
    assert_eq!(same[0]["also"][0]["url"], "https://reciba.se/pasta");

    // Ours: the peer is credited only when the digest differs.
    let ours =
        vec![json!({"name": "Pasta", "permalink": "pasta", "revision": "aaaa", "ours": true})];
    let identical = merge_recipe_lists(&ours, &peer("aaaa"));
    assert_eq!(identical.len(), 1);
    assert!(identical[0].get("also").is_none());
    // The digest is the sender's, not ours: it is stripped from the drawer.
    assert!(identical[0].get("revision").is_none());

    let differing = merge_recipe_lists(&ours, &peer("bbbb"));
    assert_eq!(differing[0]["also"][0]["url"], "https://reciba.se/pasta");
}

/// `SERVER_IDENTITY` is `label|site`, and a trailing slash is trimmed so the
/// caption's link has no double slash.
#[test]
fn parse_server_identity_reads_label_and_site() {
    let identity = parse_server_identity("Chaos' Recibase Server|https://recibase.shed.gay/")
        .expect("the identity is well formed");
    assert_eq!(identity.label, "Chaos' Recibase Server");
    assert_eq!(identity.site_url, "https://recibase.shed.gay");
}

/// A malformed value leaves the generic caption rather than naming a broken
/// server: both halves are required.
#[test]
fn parse_server_identity_rejects_malformed_values() {
    assert!(parse_server_identity("").is_none());
    assert!(parse_server_identity("Chaos' Recibase Server").is_none());
    assert!(parse_server_identity("|https://recibase.shed.gay").is_none());
    assert!(parse_server_identity("Chaos' Recibase Server|").is_none());
}
