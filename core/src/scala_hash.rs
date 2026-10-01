//! Scala 2.13 collection hashing and iteration order.
//!
//! The API is byte-for-byte compatible with the Scala implementation, so the
//! order in which `Set`s and `Map`s are serialised must match too. This module
//! provides the Scala `hashCode` values and the iteration order of Scala's
//! `immutable.HashSet` / `immutable.HashMap`.
//!
//! OWNER: workstream W2. `case_object_hash` and `set_order`/`map_order` are
//! implemented there against `capture-scala/` ground truth.

use crate::meal::{DatedNote, MealStubWithUsageData, Source};
use crate::tag::Tag;

pub trait ScalaHash {
    fn scala_hash(&self) -> i32;
}

/// `hashCode` of a Scala `case object` with the given fully qualified class
/// name (e.g. `se.reciba.api.model.Tag$Vegan$`).
pub fn case_object_hash(_class_name: &str) -> i32 {
    unimplemented!("scala_hash::case_object_hash (workstream W2)")
}

/// Iteration order of `scala.collection.immutable.HashSet(items)` where the
/// values are inserted in the order given. Returns indices into `items`.
pub fn set_order<T: ScalaHash>(_items: &[T]) -> Vec<usize> {
    unimplemented!("scala_hash::set_order (workstream W2)")
}

/// Iteration order of `scala.collection.immutable.HashMap(pairs)` (or of
/// `Map(...)` built from `pairs` in order). Returns indices into `pairs`.
pub fn map_order<K: ScalaHash, V>(_pairs: &[(K, V)]) -> Vec<usize> {
    unimplemented!("scala_hash::map_order (workstream W2)")
}

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

/// `java.lang.String.hashCode`, which is also Scala's `String##`.
pub fn java_string_hash(s: &str) -> i32 {
    let mut h: i32 = 0;
    for unit in s.encode_utf16() {
        h = h.wrapping_mul(31).wrapping_add(unit as i32);
    }
    h
}

impl<T: ScalaHash> ScalaHash for Option<T> {
    fn scala_hash(&self) -> i32 {
        match self {
            None => 0,
            Some(value) => value.scala_hash() + 1,
        }
    }
}

/// `scala.util.hashing.MurmurHash3.productHash`, used by every case class and
/// case object `hashCode` (`prefix` is the class's simple name).
pub fn product_hash(_prefix: &str, _element_hashes: &[i32]) -> i32 {
    unimplemented!("scala_hash::product_hash (workstream W2)")
}

/// `MurmurHash3.unorderedHash` - the hash of a `Set` of these element hashes.
pub fn set_hash(_element_hashes: &[i32]) -> i32 {
    unimplemented!("scala_hash::set_hash (workstream W2)")
}

/// `MurmurHash3.orderedHash` - the hash of a `List`/`Seq` of these hashes.
pub fn seq_hash(_element_hashes: &[i32]) -> i32 {
    unimplemented!("scala_hash::seq_hash (workstream W2)")
}

/// The tags of a `Set` built by inserting `items` in order, in the iteration
/// order Scala uses when it walks that set.
pub fn scala_set<T: ScalaHash + Clone>(items: &[T]) -> Vec<T> {
    let order = set_order(items);
    order.iter().map(|&i| items[i].clone()).collect()
}

/// `items.flatMap(f)` where `items` is a Scala `Set`: the union of the mapped
/// results, in the iteration order of the resulting set.
pub fn scala_set_flat_map<T: ScalaHash + Clone, U: ScalaHash + Clone>(
    items: &[T],
    f: impl Fn(&T) -> Vec<U>,
) -> Vec<U> {
    // Scala builds the result by folding over the source set in iteration
    // order; with distinct hashes the final trie is independent of that order.
    let mut mapped: Vec<U> = Vec::new();
    for item in scala_set(items).iter() {
        mapped.extend(f(item));
    }
    scala_set(&mapped)
}

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

impl ScalaHash for chrono::NaiveDate {
    fn scala_hash(&self) -> i32 {
        unimplemented!("scala_hash: NaiveDate (workstream W2)")
    }
}

impl ScalaHash for u8 {
    fn scala_hash(&self) -> i32 {
        *self as i32
    }
}

impl<T: ScalaHash> ScalaHash for Box<T> {
    fn scala_hash(&self) -> i32 {
        (**self).scala_hash()
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
                crate::scala_hash::set_hash(
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
            Source::Online(url) => {
                crate::scala_hash::product_hash("Online", &[url.scala_hash()])
            }
            Source::Recibase(permalink) => {
                crate::scala_hash::product_hash("Recibase", &[permalink.scala_hash()])
            }
            Source::GoogleDrive(id) => {
                crate::scala_hash::product_hash("GoogleDrive", &[id.scala_hash()])
            }
        }
    }
}


impl ScalaHash for Tag {
    fn scala_hash(&self) -> i32 {
        // Scala `case object`s hash as their own class; measured against the
        // running Scala implementation (see tools/ in this crate).
        case_object_hash(&format!(
            "se.reciba.api.model.Tag${}$",
            self.object_name()
        ))
    }
}
