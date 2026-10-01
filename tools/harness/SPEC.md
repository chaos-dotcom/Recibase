# Recibase Rust port - interface contract

Goal: a Rust port of https://github.com/The-Silverwood-Institute/Recibase that is
**byte-for-byte compatible on the HTTP API** with the Scala implementation.

* Scala source (read-only reference): `/Users/chaos/recibase-work/Recibase`
* Byte-exact captures from the running Scala server: `/Users/chaos/recibase-work/capture-scala`
  (`NNN_<name>.raw` = raw response bytes, `.norm` = same with the `Date` header
  normalised, `index.json` = request list with statuses).
* Capture harness: `/Users/chaos/recibase-work/capture.py`
* Request list: `/Users/chaos/recibase-work/requests.json`
* Rust workspace: `/Users/chaos/recibase-work/recibase-rs` (crates `core`,
  `submit`, `server`).

## Layout

```
recibase-rs/
  core/src/json.rs          JSON helpers (owned by W1)
  core/src/scala_hash.rs    Scala hashCode + Set/Map iteration order (W2)
  core/src/tag.rs           Tag enum (W1)
  core/src/recipe.rs        Ingredient(sBlock), Image, RecipeDef (W1)
  core/src/meal.rs          Source, MealStub(WithUsageData), DatedNote (W1)
  core/src/misc.rs          Manifest, MenuEntry, docs map (W1)
  core/src/permalink.rs     Permalink.fromRawString (W1)
  core/src/stop_words.rs    generated from Scala StopWords.scala (W1)
  core/src/utils.rs         StringUtils, IntUtils.TemperatureUtils (W1)
  core/src/usage.rs         UsageData + meal log CSV (W6)
  core/src/ice_cream.rs     the IceCream mixin (W1)
  core/src/recipes/<Object>.rs + recipes/mod.rs + recipes/all.rs (W4)
  core/src/meal_definitions.rs (W5)
  submit/src/*.rs           submission, Scala literal, RecipeSource, GitHub,
                            Turnstile, config (W3)
  server/src/*.rs           controllers, routes, HTTP server (W1)
```

Rules: only edit files you own. `core/src/{lib.rs,json.rs,tag.rs,recipe.rs,meal.rs,misc.rs}`
are the frozen interface - ask W1 before changing a signature.

## Compatibility rules

1. **JSON field order** is the Scala encoder's field order. Always build objects
   with `core::json::obj(vec![("field", value), ...])` in that order.
   `serde_json` is compiled with `preserve_order`.
2. **`None` is JSON `null`** (circe does not drop absent fields).
3. **Collections**: a Scala `Set`/`Map` is serialised in *Scala's* iteration
   order, not source order and not sorted order. Call
   `scala_hash::scala_set(...)` / `scala_hash::map_order(...)` (W2) for anything
   that is a `Set`/`Map` in the Scala source. For 95+4 collections the order is
   the Java/String-hash-based CHAMP trie order; 13 recipes already differ from
   source order, and `/meals/` and `/` differ too.
4. **Strings**: Java/Scala `String.toLowerCase` is locale-independent here;
   `StringUtils.stripAccents` is NFD + remove U+0300..U+036F.
5. **HTTP**: response shape is
   `HTTP/1.1 <status>\r\nDate: ...\r\nConnection: close\r\nContent-Type: <ct>\r\nContent-Length: <n>\r\n\r\n<body>`
   with `Content-Type: application/json` for JSON, `text/plain; charset=UTF-8`
   for plain text. `Date` is the only volatile header and is normalised in `.norm`.
6. **No invented behaviour**: if the Scala does something surprising, copy it.

## Ground truth

`capture-scala/index.json` lists 114 requests: `/`, `/health`, `/manifest`,
`/recipes/` (and filters), every recipe permalink, `/meals/`, `/meals/raw`, the
unconfigured submission route, unknown paths, and method/slash variants.

Compare Rust output with:

```
python3 tools/capture_diff.py <rust-port>        # starts nothing; diffs a capture dir
```

## Build and test

```
cd /Users/chaos/recibase-work/recibase-rs
cargo check -p recibase-core
cargo test  -p recibase-core        # unit + capture-comparison tests
cargo run   -p recibase-server      # serves on PORT (default 8081)
```
