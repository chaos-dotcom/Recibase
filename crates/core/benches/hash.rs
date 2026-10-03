//! Native micro-benchmarks for the hashing: the recipe content digest
//! (`RecipeDef::revision`) and the Scala hash engine (`scala_hash`).
//!
//! `harness = false`, so this is a plain `main` - stable Rust, no dependencies:
//!
//!     cargo bench -p recibase-core
//!
//! Each figure is the best of several rounds (the least disturbed by the OS),
//! in nanoseconds per operation and operations per second.

use std::hint::black_box;
use std::time::Instant;

use recibase_core::recipe::RecipeDef;
use recibase_core::tag::Tag;
use recibase_core::{json, recipes, scala_hash};

/// Best-of-`rounds` nanoseconds per call of `op`, which runs `iterations` times
/// per round.
fn time(rounds: usize, iterations: u64, mut op: impl FnMut()) -> f64 {
    let mut best = f64::MAX;
    for _ in 0..rounds {
        let started = Instant::now();
        for _ in 0..iterations {
            op();
        }
        best = best.min(started.elapsed().as_nanos() as f64 / iterations as f64);
    }
    best
}

fn report(label: &str, ns: f64, unit: &str) {
    println!(
        "{label:<32} {ns:>9.1} ns/{unit}  {:>13} {unit}/s",
        (1e9 / ns) as u64
    );
}

fn main() {
    let corpus: &[RecipeDef] = recipes::recipes();
    let tags: Vec<Tag> = corpus.iter().flat_map(|r| r.tags.iter().copied()).collect();
    println!("corpus: {} recipes, {} tags\n", corpus.len(), tags.len());

    // The whole digest: build the JSON, drop `edit`, serialise, FNV-1a it.
    let mut index = 0usize;
    report(
        "RecipeDef::revision",
        time(7, 100 * corpus.len() as u64, || {
            index = (index + 1) % corpus.len();
            black_box(corpus[index].revision());
        }),
        "digest",
    );

    // The serialisation alone, to attribute the cost of the digest.
    let mut index = 0usize;
    report(
        "  to_bytes(to_json_with_usage)",
        time(7, 100 * corpus.len() as u64, || {
            index = (index + 1) % corpus.len();
            black_box(json::to_bytes(&corpus[index].to_json_with_usage(&[])));
        }),
        "recipe",
    );

    // The Scala hash engine: `improve` is the CHAMP trie's per-node step, and
    // `set_order` reproduces a Scala `Set`'s iteration order over the tags.
    report(
        "scala_hash::improve",
        time(7, 5_000_000, || {
            black_box(scala_hash::improve(black_box(0x1234_5678)));
        }),
        "call",
    );
    report(
        "scala_hash::set_order(tags)",
        time(7, 50_000, || {
            black_box(scala_hash::set_order(black_box(tags.as_slice())));
        }),
        "sort",
    );
}
