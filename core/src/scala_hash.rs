//! Scala 2.13 collection hashing and iteration order.
//!
//! The API is byte-for-byte compatible with the Scala implementation, so the
//! order in which `Set`s and `Map`s are serialised must match too. This module
//! provides the Scala `hashCode` values and the iteration order of Scala's
//! `immutable.HashSet` / `immutable.HashMap`.
//!
//! # The fitted rule (Scala 2.13.18)
//!
//! `scala.collection.immutable.HashSet`/`HashMap` are CHAMP hash tries
//! (`scala.collection.immutable.HashSet`, `HashMap`, and `ChampCommon`). The
//! whole rule below was fitted against the real implementation and then
//! verified: 20 synthetic sets (dumped through the Scala probe), 95 recipe tag
//! sets + 95 inherited-tag sets, the `/` docs map and 185 `/meals/` entries
//! (hashes compared element by element with the running Scala build).
//!
//! 1. Every element's `hashCode` is "improved" first, exactly as
//!    `scala.collection.Hashing.improve` does:
//!
//!    ```text
//!    h = hcode + ~(hcode << 9)
//!    h = h ^ (h >>> 14)
//!    h = h + (h << 4)
//!    h = h ^ (h >>> 10)
//!    ```
//!
//!    (All steps are 32-bit, `>>>` is an unsigned shift.) This is *not*
//!    `byteswap32`, `fmix32`, or a plain multiply - which is why every
//!    "chunked golden-ratio" fit fails.
//!
//! 2. A node splits its elements by one 5-bit slice of the improved hash:
//!    `mask = (improved >>> shift) & 31`, with `shift = 0` at the root, `+5`
//!    per level, and `shift >= 32` meaning "identical improved hash - a
//!    `HashCollisionSetNode` holding the elements in **insertion** order".
//!
//! 3. `BitmapIndexedSetNode` stores one payload per mask that exactly one
//!    element maps to (payloads are kept sorted by ascending mask) and one
//!    sub-node per mask that several elements share.
//!
//! 4. Iteration (`ChampBaseIterator`) is a depth-first pre-order walk that
//!    yields **all of a node's payloads first, in ascending mask order**, and
//!    only then its sub-nodes, also in ascending mask order. So an element
//!    that owns its root mask is emitted before elements buried in a root
//!    sub-node with a *lower* mask.
//!
//! 5. Sets of size <= 4 are not tries at all: `Set1`..`Set4` store their
//!    elements in construction order and iterate in that order, and
//!    `Map1`..`Map4` likewise. The trie only takes over at size 5
//!    (`SetBuilderImpl.addOne` switches to `HashSetBuilder`).
//!
//! 6. Which shape a *result* collection has depends on the source:
//!    `HashSet` overrides `iterableFactory` with itself, so `map`/`flatMap`
//!    and friends on a `HashSet` build another `HashSet` (trie order, even for
//!    a 2-element result), while the same operations on `Set1`..`Set4` build
//!    `Set1`..`Set4` again (insertion order) and only switch to a `HashSet`
//!    when a 5th distinct element arrives. `scala_set_flat_map` reproduces
//!    that.
//!
//! 7. `case object`s hash as `productPrefix.hashCode`
//!    (`MurmurHash3.productHash` inlines that for arity 0), and a case class
//!    hashes as `MurmurHash3.caseClassHash(x, productSeed, simpleName)`:
//!    `h = mix(0xcafebabe, name.hashCode)`, then `mix` each element, then
//!    `finalizeHash(h, arity)`.
//!
//! 8. `Set.hashCode` = `unorderedHash(elements, "Set".hashCode)`,
//!    `List`/`Seq.hashCode` = `listHash(elements, "Seq".hashCode)` (with the
//!    range-recognition shortcut, which agrees with `orderedHash`),
//!    `Some(x)` is a case class ("Some") and `None` a case object ("None"),
//!    and `java.time.LocalDate.hashCode` is
//!    `(year & 0xFFFFF800) ^ ((year << 11) + (month << 6) + day)`.

use crate::meal::{DatedNote, MealStubWithUsageData, Source};
use crate::tag::Tag;

pub trait ScalaHash {
    fn scala_hash(&self) -> i32;
}

// ---------------------------------------------------------------------------
// java.lang.String / scala Any##
// ---------------------------------------------------------------------------

/// `java.lang.String.hashCode`, which is also Scala's `String##`.
pub fn java_string_hash(s: &str) -> i32 {
    let mut h: i32 = 0;
    for unit in s.encode_utf16() {
        h = h.wrapping_mul(31).wrapping_add(unit as i32);
    }
    h
}

/// The simple name a case object/class reports as its `productPrefix`:
/// `se.reciba.api.model.Tag$Vegan$` -> `Vegan`.
fn simple_name(class_name: &str) -> &str {
    let last = class_name.rsplit('.').next().unwrap_or(class_name);
    let trimmed = last.strip_suffix('$').unwrap_or(last);
    trimmed.rsplit('$').next().unwrap_or(trimmed)
}

/// `hashCode` of a Scala `case object` with the given fully qualified class
/// name (e.g. `se.reciba.api.model.Tag$Vegan$`).
///
/// A `case object` has arity 0, so `MurmurHash3.productHash` returns
/// `productPrefix.hashCode` and `SyntheticMethods` inlines exactly that.
pub fn case_object_hash(class_name: &str) -> i32 {
    java_string_hash(simple_name(class_name))
}

// ---------------------------------------------------------------------------
// MurmurHash3 (scala.util.hashing.MurmurHash3)
// ---------------------------------------------------------------------------

/// `MurmurHash3.productSeed`.
const PRODUCT_SEED: i32 = 0xcafebabeu32 as i32;

fn mix(hash: i32, data: i32) -> i32 {
    let h = mix_last(hash, data);
    h.rotate_left(13).wrapping_mul(5).wrapping_add(0xe6546b64u32 as i32)
}

fn mix_last(hash: i32, data: i32) -> i32 {
    let mut k = data.wrapping_mul(0xcc9e2d51u32 as i32);
    k = k.rotate_left(15);
    k = k.wrapping_mul(0x1b873593u32 as i32);
    hash ^ k
}

fn avalanche(hash: i32) -> i32 {
    let mut h = hash;
    h ^= ((h as u32) >> 16) as i32;
    h = h.wrapping_mul(0x85ebca6bu32 as i32);
    h ^= ((h as u32) >> 13) as i32;
    h = h.wrapping_mul(0xc2b2ae35u32 as i32);
    h ^ ((h as u32) >> 16) as i32
}

fn finalize_hash(hash: i32, length: i32) -> i32 {
    avalanche(hash ^ length)
}

fn range_hash(start: i32, step: i32, last: i32, seed: i32) -> i32 {
    avalanche(mix(mix(mix(seed, start), step), last))
}

/// `scala.util.hashing.MurmurHash3.productHash` (or `caseClassHash(x, seed,
/// name)` for a 2.13.17+ case class), used by every case class and case object
/// `hashCode` (`prefix` is the class's simple name).
pub fn product_hash(prefix: &str, element_hashes: &[i32]) -> i32 {
    if element_hashes.is_empty() {
        // Case objects have their `productPrefix.hashCode` inlined.
        return java_string_hash(prefix);
    }
    let mut h = mix(PRODUCT_SEED, java_string_hash(prefix));
    for element in element_hashes {
        h = mix(h, *element);
    }
    finalize_hash(h, element_hashes.len() as i32)
}

/// `MurmurHash3.unorderedHash(xs, seed)` - the hash of a `Set` of these
/// element hashes.
pub fn unordered_hash(element_hashes: &[i32], seed: i32) -> i32 {
    let mut a: i32 = 0;
    let mut b: i32 = 0;
    let mut c: i32 = 1;
    let mut n: i32 = 0;
    for h in element_hashes {
        a = a.wrapping_add(*h);
        b ^= *h;
        c = c.wrapping_mul(*h | 1);
        n += 1;
    }
    let mut h = seed;
    h = mix(h, a);
    h = mix(h, b);
    h = mix_last(h, c);
    finalize_hash(h, n)
}

/// `MurmurHash3.orderedHash(xs, seed)` - order-dependent.
pub fn ordered_hash(element_hashes: &[i32], seed: i32) -> i32 {
    let mut h = seed;
    if element_hashes.is_empty() {
        return finalize_hash(h, 0);
    }
    if element_hashes.len() == 1 {
        return finalize_hash(mix(h, element_hashes[0]), 1);
    }
    let initial = element_hashes[0];
    h = mix(h, initial);
    let h0 = h;
    let mut prev = element_hashes[1];
    let range_diff = prev.wrapping_sub(initial);
    let mut i = 2;
    while i < element_hashes.len() {
        h = mix(h, prev);
        let hash = element_hashes[i];
        if range_diff != hash.wrapping_sub(prev) || range_diff == 0 {
            h = mix(h, hash);
            i += 1;
            while i < element_hashes.len() {
                h = mix(h, element_hashes[i]);
                i += 1;
            }
            return finalize_hash(h, i as i32);
        }
        prev = hash;
        i += 1;
    }
    avalanche(mix(mix(h0, range_diff), prev))
}

/// `MurmurHash3.listHash(xs, seed)` - what `List`/`Seq.hashCode` calls. It
/// recognises arithmetic progressions and folds them with `rangeHash`, which
/// `orderedHash` does too, so this is also `orderedHash` for a general `Seq`.
pub fn list_hash(element_hashes: &[i32], seed: i32) -> i32 {
    let mut n = 0i32;
    let mut h = seed;
    let mut range_state = 0;
    let mut range_diff = 0i32;
    let mut prev = 0i32;
    let mut initial = 0i32;
    for hash in element_hashes {
        let hash = *hash;
        h = mix(h, hash);
        match range_state {
            0 => {
                initial = hash;
                range_state = 1;
            }
            1 => {
                range_diff = hash.wrapping_sub(prev);
                range_state = 2;
            }
            2 => {
                if range_diff != hash.wrapping_sub(prev) || range_diff == 0 {
                    range_state = 3;
                }
            }
            _ => {}
        }
        prev = hash;
        n += 1;
    }
    if range_state == 2 {
        range_hash(initial, range_diff, prev, seed)
    } else {
        finalize_hash(h, n)
    }
}

/// `MurmurHash3.setSeed` = `"Set".hashCode`.
pub fn set_seed() -> i32 {
    java_string_hash("Set")
}

/// `MurmurHash3.seqSeed` = `"Seq".hashCode`.
pub fn seq_seed() -> i32 {
    java_string_hash("Seq")
}

/// `MurmurHash3.unorderedHash` - the hash of a `Set` of these element hashes.
pub fn set_hash(element_hashes: &[i32]) -> i32 {
    unordered_hash(element_hashes, set_seed())
}

/// The hash of a Scala `List`/`Seq` of these hashes (`MurmurHash3.seqHash`).
pub fn seq_hash(element_hashes: &[i32]) -> i32 {
    list_hash(element_hashes, seq_seed())
}

// ---------------------------------------------------------------------------
// The CHAMP trie order
// ---------------------------------------------------------------------------

/// `scala.collection.Hashing.improve`, the hash the trie actually indexes on.
pub fn improve(hcode: i32) -> i32 {
    let mut h = hcode;
    h = h.wrapping_add(!(h << 9));
    h ^= ((h as u32) >> 14) as i32;
    h = h.wrapping_add(h << 4);
    h ^ ((h as u32) >> 10) as i32
}

fn mask_from(hash: i32, shift: u32) -> usize {
    (((hash as u32) >> shift) & 31) as usize
}

/// One element of the trie: its position in the caller's slice and its
/// (unimproved) Scala `hashCode`.
#[derive(Clone, Copy, Debug)]
pub struct Slot {
    /// Position of the element in the caller's slice.
    pub idx: usize,
    /// Its unimproved Scala `hashCode` (`T::scala_hash`).
    pub hash: i32,
}

/// Depth-first pre-order walk of the CHAMP trie: a node's payloads (ascending
/// mask) come before its sub-nodes (ascending mask); a hash-collision leaf
/// keeps insertion order.
fn trie_order(items: &[Slot], out: &mut Vec<usize>) {
    if items.len() == 1 {
        out.push(items[0].idx);
        return;
    }
    visit(items, 0, out);
}

fn visit(items: &[Slot], shift: u32, out: &mut Vec<usize>) {
    if items.len() == 1 {
        out.push(items[0].idx);
        return;
    }
    if shift >= 32 {
        // `HashCollisionSetNode`: identical improved hashes, insertion order.
        out.extend(items.iter().map(|slot| slot.idx));
        return;
    }
    let mut buckets: Vec<Vec<Slot>> = (0..32).map(|_| Vec::new()).collect();
    for slot in items {
        buckets[mask_from(improve(slot.hash), shift)].push(*slot);
    }
    for bucket in buckets.iter() {
        if bucket.len() == 1 {
            out.push(bucket[0].idx);
        }
    }
    for bucket in buckets.iter() {
        if bucket.len() > 1 {
            visit(bucket, shift + 5, out);
        }
    }
}

/// Elements in insertion order, duplicates dropped, each carrying its hash.
///
/// Deduplication is by `hashCode` (`T` is only required to be `ScalaHash` so
/// that callers do not need `PartialEq` on the model types). `equals` implies
/// `hashCode`, so every true duplicate is dropped; two *unequal* elements that
/// happen to share a `hashCode` would be dropped too, which Scala would keep in
/// a collision leaf. No `Set` in this API contains such a pair: all 31 `Tag`
/// hashes are distinct, and the 185 `/meals/` elements have 185 distinct
/// hashes in both captures (asserted by `core/tests/scala_set_order.rs`).
fn distinct<T: ScalaHash>(items: &[T]) -> Vec<Slot> {
    let mut out: Vec<Slot> = Vec::with_capacity(items.len());
    for (idx, item) in items.iter().enumerate() {
        let hash = item.scala_hash();
        if out.iter().any(|slot| slot.hash == hash) {
            continue;
        }
        out.push(Slot { idx, hash });
    }
    out
}

/// Iteration order of `scala.collection.immutable.HashSet(items)` where the
/// values are inserted in the order given. Returns indices into `items`.
///
/// Sizes 1..4 are `Set1`..`Set4`, which keep insertion order; 5 or more is a
/// CHAMP trie and the order is a pure function of the element hashes.
pub fn set_order<T: ScalaHash>(items: &[T]) -> Vec<usize> {
    set_order_distinct(&distinct(items))
}

/// The same order for `items` that are already known to be distinct elements
/// (nothing is deduplicated, so two hash-equal items become one hash-collision
/// leaf, exactly as `HashCollisionSetNode` does).
pub fn set_order_distinct(slots: &[Slot]) -> Vec<usize> {
    if slots.len() <= 4 {
        return slots.iter().map(|slot| slot.idx).collect();
    }
    let mut out = Vec::with_capacity(slots.len());
    trie_order(slots, &mut out);
    out
}

/// Iteration order of `scala.collection.immutable.HashMap(pairs)` (or of
/// `Map(...)` built from `pairs` in order). Returns indices into `pairs`.
///
/// Only the keys take part in the trie.
pub fn map_order<K: ScalaHash, V>(pairs: &[(K, V)]) -> Vec<usize> {
    let mut slots: Vec<Slot> = Vec::with_capacity(pairs.len());
    for (idx, (key, _)) in pairs.iter().enumerate() {
        let hash = key.scala_hash();
        if slots.iter().any(|slot| slot.hash == hash) {
            continue;
        }
        slots.push(Slot { idx, hash });
    }
    set_order_distinct(&slots)
}

/// The tags of a `Set` built by inserting `items` in order, in the iteration
/// order Scala uses when it walks that set.
pub fn scala_set<T: ScalaHash + Clone>(items: &[T]) -> Vec<T> {
    let order = set_order(items);
    order.iter().map(|&i| items[i].clone()).collect()
}

/// `items.flatMap(f)` where `items` is a Scala `Set`.
///
/// Scala iterates the source set, then the `Set` that `f` returns, and appends
/// everything to a builder. The *result's* shape follows the source's factory:
/// a source `HashSet` (5 or more elements here) always builds another
/// `HashSet`, so the result iterates in trie order even when it holds 2
/// elements; a source `Set1`..`Set4` builds `Set1`..`Set4` (insertion order)
/// unless a 5th distinct element arrives.
///
/// `f` must return the mapped elements in the order the Scala `Set` it models
/// iterates. Note that Scala's `Tag.allParentTags` is `parent.allParentTags +
/// parent`, i.e. **outermost ancestor first**, while `Tag::all_parent_tags`
/// (tag.rs) walks the chain upwards and returns them innermost first: callers
/// must reverse it, or `tag.rs` must return the Scala order.
pub fn scala_set_flat_map<T, U>(
    items: &[T],
    f: impl Fn(&T) -> Vec<U>,
) -> Vec<U>
where
    T: ScalaHash + Clone,
    U: ScalaHash + Clone,
{
    let source = distinct(items);
    let source_is_hash_set = source.len() >= 5;
    let source_order = if source_is_hash_set {
        let mut out = Vec::with_capacity(source.len());
        trie_order(&source, &mut out);
        out
    } else {
        source.iter().map(|slot| slot.idx).collect()
    };

    let mut mapped: Vec<U> = Vec::new();
    for idx in source_order {
        mapped.extend(f(&items[idx]));
    }

    let unique: Vec<Slot> = distinct(&mapped);
    if source_is_hash_set || unique.len() >= 5 {
        let mut out = Vec::with_capacity(unique.len());
        trie_order(&unique, &mut out);
        out.iter().map(|&i| mapped[i].clone()).collect()
    } else {
        unique.iter().map(|slot| mapped[slot.idx].clone()).collect()
    }
}

// ---------------------------------------------------------------------------
// ScalaHash for the core types
// ---------------------------------------------------------------------------

macro_rules! scala_hash_prim {
    ($($t:ty => $f:expr),* $(,)?) => {$(
        impl ScalaHash for $t {
            fn scala_hash(&self) -> i32 { let f: fn(&$t) -> i32 = $f; f(self) }
        }
    )*};
}

scala_hash_prim! {
    i32 => |v: &i32| *v,
    i64 => |v: &i64| (*v as i32) ^ ((*v >> 32) as i32),
    u32 => |v: &u32| *v as i32,
    bool => |v: &bool| if *v { 1231 } else { 1237 },
    String => |v: &String| java_string_hash(v),
}

impl ScalaHash for &str {
    fn scala_hash(&self) -> i32 {
        java_string_hash(self)
    }
}

impl<T: ScalaHash> ScalaHash for Box<T> {
    fn scala_hash(&self) -> i32 {
        (**self).scala_hash()
    }
}

impl ScalaHash for u8 {
    fn scala_hash(&self) -> i32 {
        *self as i32
    }
}

/// `Some` is a case class and `None` a case object, so this is *not*
/// `value.hashCode + 1` (that was Scala 2.12).
impl<T: ScalaHash> ScalaHash for Option<T> {
    fn scala_hash(&self) -> i32 {
        match self {
            None => case_object_hash("None"),
            Some(value) => product_hash("Some", &[value.scala_hash()]),
        }
    }
}

/// A Scala `List`/`Seq` of these values (`MurmurHash3.seqHash`).
impl<T: ScalaHash + Clone> ScalaHash for Vec<T> {
    fn scala_hash(&self) -> i32 {
        seq_hash(&self.iter().map(|v| v.scala_hash()).collect::<Vec<_>>())
    }
}

impl<T: ScalaHash + Clone> ScalaHash for &[T] {
    fn scala_hash(&self) -> i32 {
        seq_hash(&self.iter().map(|v| v.scala_hash()).collect::<Vec<_>>())
    }
}

/// `java.time.LocalDate.hashCode`, which is what `LocalDate.##` returns and so
/// what a `DatedNote`'s `date` contributes to its `hashCode`.
impl ScalaHash for chrono::NaiveDate {
    fn scala_hash(&self) -> i32 {
        use chrono::Datelike;
        let year = self.year();
        let month = self.month() as i32;
        let day = self.day() as i32;
        (year & 0xFFFFF800u32 as i32) ^ ((year << 11).wrapping_add((month << 6).wrapping_add(day)))
    }
}

impl ScalaHash for Tag {
    fn scala_hash(&self) -> i32 {
        // Scala `case object`s hash as their own simple name.
        case_object_hash(self.object_name())
    }
}

impl ScalaHash for MealStubWithUsageData {
    fn scala_hash(&self) -> i32 {
        crate::scala_hash::product_hash(
            "MealStubWithUsageData",
            &[
                self.name.scala_hash(),
                crate::scala_hash::set_hash(
                    &self
                        .tags
                        .iter()
                        .map(|t| t.scala_hash())
                        .collect::<Vec<_>>(),
                ),
                self.source.scala_hash(),
                crate::scala_hash::seq_hash(
                    &self
                        .dated_notes
                        .iter()
                        .map(|n| n.scala_hash())
                        .collect::<Vec<_>>(),
                ),
                self.last_eaten.scala_hash(),
                (self.times_eaten as i32).scala_hash(),
                self.featured.scala_hash(),
            ],
        )
    }
}

impl ScalaHash for DatedNote {
    fn scala_hash(&self) -> i32 {
        crate::scala_hash::product_hash(
            "DatedNote",
            &[self.date.scala_hash(), self.note.scala_hash()],
        )
    }
}

impl ScalaHash for Source {
    fn scala_hash(&self) -> i32 {
        match self {
            Source::Online(url) => crate::scala_hash::product_hash("Online", &[url.scala_hash()]),
            Source::Recibase(permalink) => {
                crate::scala_hash::product_hash("Recibase", &[permalink.scala_hash()])
            }
            Source::GoogleDrive(id) => {
                crate::scala_hash::product_hash("GoogleDrive", &[id.scala_hash()])
            }
        }
    }
}
