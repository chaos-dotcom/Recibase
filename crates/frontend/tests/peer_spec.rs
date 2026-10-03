//! `peer.rs` - merging another deployment's recipes into the drawer.

mod support;

use recibase_frontend::peer::{PeerList, merge_recipe_lists, parse_peer_backends};
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
/// peer-only recipe is added with its `source` and an absolute `href`.
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
    // Sorted by name, so Curry (theirs, added) comes before Pasta (ours).
    let curry = &merged[0];
    assert_eq!(curry["name"], "Curry");
    assert_eq!(curry["source"], "Kit & Alex");
    assert_eq!(curry["href"], "https://reciba.se/curry");
    // `ours` is relative to the server that sent the list, not ours.
    assert!(curry.get("ours").is_none());

    let pasta = &merged[1];
    assert_eq!(pasta["name"], "Pasta");
    assert_eq!(pasta["href"], "pasta");
    assert_eq!(pasta["ours"], true);
    assert_eq!(pasta["also"][0]["label"], "Kit & Alex");
    assert_eq!(pasta["also"][0]["url"], "https://reciba.se/pasta");
}
