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

/// The name is the key: ours wins a collision, so a peer's same-named recipe is
/// skipped entirely; a peer-only recipe is added with its `source` and a local
/// `href`, so this frontend serves it rather than linking out.
#[test]
fn merge_keeps_ours_and_adds_only_the_recipes_we_lack() {
    let own = vec![json!({"name": "Pasta", "permalink": "pasta", "ours": true})];
    let peers = vec![PeerList {
        label: "Kit & Alex".to_string(),
        site_url: "https://reciba.se".to_string(),
        recipes: vec![
            json!({"name": "Pasta", "permalink": "pasta", "revision": "bbbb"}),
            json!({"name": "Curry", "permalink": "curry"}),
        ],
    }];

    let merged = merge_recipe_lists(&own, &peers);

    // Only the peer-only recipe is added, and it comes after our own.
    assert_eq!(merged.len(), 2);
    let pasta = &merged[0];
    assert_eq!(pasta["name"], "Pasta");
    assert_eq!(pasta["href"], "pasta");
    assert_eq!(pasta["ours"], true);
    // A name we already hold gets no cross-reference, whatever the digest.
    assert!(pasta.get("also").is_none());
    assert!(pasta.get("revision").is_none());

    let curry = &merged[1];
    assert_eq!(curry["name"], "Curry");
    assert_eq!(curry["source"], "Kit & Alex");
    assert_eq!(curry["href"], "curry");
    // `ours` is relative to the server that sent the list, not ours.
    assert!(curry.get("ours").is_none());
}

/// A peer recipe whose name we already hold is dropped whether or not its
/// digest matches ours: the digest is not used to decide anything.
#[test]
fn merge_skips_a_peer_recipe_we_already_have() {
    let own = vec![json!({"name": "Pasta", "permalink": "pasta", "revision": "aaaa"})];
    for revision in ["aaaa", "bbbb"] {
        let peers = vec![PeerList {
            label: "Kit & Alex".to_string(),
            site_url: "https://reciba.se".to_string(),
            recipes: vec![json!({"name": "Pasta", "permalink": "pasta", "revision": revision})],
        }];
        let merged = merge_recipe_lists(&own, &peers);
        assert_eq!(merged.len(), 1, "revision {revision}");
        assert_eq!(merged[0]["name"], "Pasta");
        assert!(merged[0].get("also").is_none());
    }
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
