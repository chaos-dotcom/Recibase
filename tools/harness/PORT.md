# Recibase, ported to Rust

A port of [The-Silverwood-Institute/Recibase](https://github.com/The-Silverwood-Institute/Recibase)
(Scala 2.13, http4s, circe, specs2) to Rust, **byte-for-byte compatible on the
HTTP API**.

```
recibase-rs/
  core/     model (Recipe, Tag, Meal, Permalink), the 95-recipe corpus, meal
            definitions, usage data, and the Scala collection-order emulation
  submit/   recipe submission: validation, Scala source generation, GitHub
            client, Turnstile, config
  server/   the controllers, the routes and the HTTP/1.1 server
  tools/    gen_recipes.py (registry generator)
```

## Verification

Two byte-exact captures of the *running Scala server* are the ground truth:

| capture | requests | what it covers |
|---|---:|---|
| `capture-scala/` | 114 | no meal log: `/`, `/health`, `/manifest`, `/recipes/` (+ ingredient filters), all 95 recipe permalinks, `/meals/`, `/meals/raw`, the unconfigured submission route, unknown paths, method and trailing-slash variants |
| `capture-scala-csv/` | 114 | the same, with `MEAL_LOG_CSV_URL` set to `meal-log.csv` (48 rows parsed): dated notes, `last_eaten`, `times_eaten`, `featured` and the frequency tags |
| `capture-scala-submit/` | 13 | submission configured from the environment: the oversized-body 400, the invalid-JSON 400, the Turnstile 403s (empty, oversized and rejected tokens, with and without a passcode) |
| `capture-scala-cors/` | 7 | requests carrying `Origin`: the CORS preflight and the `Access-Control-Allow-Origin` header |
| `capture-scala-cors2/` | 20 | the preflight matrix: per-path, per-method, requested-header echoing, `OPTIONS` without an origin, `HEAD`/`PUT`, the CORS header on 404 and 503 |
| `capture-scala-cors3/` | 6 | header-list normalisation and non-preflight requests that carry the preflight headers |
| `capture-scala-keepalive/` | 13 | kept-alive connections: the `Connection` header, HTTP/1.0 vs 1.1, and a preflight on a kept-alive connection |

All 287 responses are byte-identical; only the volatile `Date` header differs.

```
B=recibase-rs/target/release/recibase-server
python3 verify_port.py --reference capture-scala        --out capture-rust        --binary $B --env RECIBASE_TODAY=2026-10-01
python3 verify_port.py --reference capture-scala-csv    --out capture-rust-csv    --binary $B --env RECIBASE_TODAY=2026-10-01 \
        --env MEAL_LOG_CSV_URL=file:///Users/chaos/recibase-work/meal-log.csv
python3 verify_port.py --reference capture-scala-submit --out capture-rust-submit --binary $B --requests requests-submit.json \
        --env RECIPE_SUBMIT_PASSCODE=s3cret --env GITHUB_TOKEN=token --env TURNSTILE_SECRET=turnstile-secret \
        --env TURNSTILE_HOSTNAMES=recipes.example --env RECIBASE_TODAY=2026-10-01
python3 verify_port.py --reference capture-scala-cors   --out capture-rust-cors   --binary $B --requests requests-cors.json  --env RECIBASE_TODAY=2026-10-01
python3 verify_port.py --reference capture-scala-cors2  --out capture-rust-cors2  --binary $B --requests requests-cors2.json --env RECIBASE_TODAY=2026-10-01
python3 verify_port.py --reference capture-scala-cors3  --out capture-rust-cors3  --binary $B --requests requests-cors3.json --env RECIBASE_TODAY=2026-10-01
```

Each run reports every response as byte-identical after normalising the `Date`
header.

`capture.py` sends one request per fresh connection with `Connection: close` and
stores the raw bytes; `diff_captures.py` compares two capture directories.

## What "byte-for-byte" needed

1. **JSON printing.** circe prints compact JSON and keeps the encoder's field
   order, so every object is built in declaration order (`core/src/json.rs`,
   `serde_json` with `preserve_order`) and `None` is `null`, not an absent key.
2. **Scala collection order.** `Set`s and `Map`s serialise in Scala 2.13's
   iteration order, which is *not* source order and *not* sorted order. It is the
   CHAMP trie order of `Hashing.improve(hashCode)`; a set of four or fewer
   elements is a `Set1..Set4` and keeps insertion order. 13 of the 95 recipes,
   every `inherited_tags` list, the `/meals/` array (185 entries) and the `/`
   docs map all depend on it. `core/src/scala_hash.rs` reproduces it, including
   `case object`/`case class` hashing, `MurmurHash3`, `Some`/`None`,
   `LocalDate`, `Set`/`List` hashing, and the rule that a `Set`'s `map`/`flatMap`
   result is a trie again while a small set's is inserted in order.
3. **HTTP and CORS.** http4s' Ember backend writes
   `HTTP/1.1 <status>`, `Date`, `Connection: close`, `Content-Type`,
   `Content-Length`, then the body. `server/src/http.rs` writes exactly that,
   including `Content-Type: text/plain; charset=UTF-8` for plain text and
   http4s' `Not found` body for unmatched routes. The Scala wraps the routes in
   `CORS.policy.withAllowOriginAll`, so an `OPTIONS` request carrying both
   `Origin` and `Access-Control-Request-Method` is answered by the middleware
   with an empty preflight 200 (allow-origin, allow-methods, allow-headers with
   the requested list echoed trimmed and joined with `", "`, `Vary`, and no
   content type); any other request that matches a route gets
   `Access-Control-Allow-Origin: *` written after `Content-Length`; a request
   that matches no route is answered by `orNotFound` outside the middleware and
   gets nothing.
4. **Connection handling.** Ember answers `Connection: keep-alive` when the
   client asks for it (any HTTP version) or sends an HTTP/1.1 request with no
   `Connection` header, and `Connection: close` for an explicit `close` or an
   HTTP/1.0 request without `keep-alive`. The captures only used
   `Connection: close` at first, which hid this; ApacheBench exposed it, and the
   rule is now captured in `capture-scala-keepalive/`.
5. **Dates.** circe encodes `java.time.LocalDate` as `YYYY-MM-DD`.
5. **Text.** `StringUtils.stripAccents` is NFD + remove U+0300..U+036F;
   `Permalink.fromRawString`, `unpluralise`, the temperature formatting
   (`Math.round`, gas marks) and the generated Scala source text are copied
   operation for operation.

## Deliberate differences

* `RECIBASE_TODAY` (server only): the `New` tag depends on the day the server
  runs (`LocalDate.now()`), so the harness can pin it. Unset, it uses the local
  date exactly like the Scala.
* `ureq` is built without TLS features, so the live GitHub/Turnstile calls need a
  TLS feature before deployment (the Scala uses the JDK client). Everything is
  exercised against fake servers instead.
* Two `commons-csv` divergences, both outside the fixture (see
  `core/tests/usage_data.rs`): a leading BOM is stripped here but breaks the
  Scala's header lookup, and blank lines are skipped here while the Scala throws.
* `set_order` de-duplicates by `hashCode` (no `PartialEq` bound on the model
  types), so two *unequal* elements with an equal hash would collapse. The 31
  tag hashes and all 185 meal hashes are distinct; the test asserts it.

## Tests

```
cd recibase-rs && cargo test --workspace     # 132 tests
cd Recibase   && sbt test                    # the Scala suite: 69 examples
```

The Rust suite covers every Scala spec (Permalink, Manifest, Recipes,
RecipeSubmissionRoutes, RecipeSource, ScalaLiteral, Turnstile, GithubClient,
UsageData, TemperatureUtils) plus the capture comparisons: all 95 recipes,
both `/meals/` orders, the docs map and 20 synthetic sets against orders
measured from the Scala itself.
