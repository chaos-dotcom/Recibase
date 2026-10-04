# Recibase, ported to Rust

A port of [The-Silverwood-Institute/Recibase](https://github.com/The-Silverwood-Institute/Recibase)
(Scala 2.13, http4s, circe, specs2) to Rust, **byte-for-byte compatible on the
HTTP API**.

```
recibase-rs/
  crates/
    core/   model (Recipe, Tag, Meal, Permalink), the 95-recipe corpus, meal
            definitions, usage data, and the Scala collection-order emulation.
            Its build.rs derives the recipe registry from src/recipes/
    submit/ recipe submission: validation, Scala source generation, GitHub
            client, Turnstile, config
    server/ the controllers, the routes and the HTTP/1.1 server
  tools/    the capture and verification harness
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

All 287 responses are byte-identical; only the volatile `Date` header and the
manifest's deploy `version` differ (both normalised, see below).

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
header and the manifest's deploy `version`.

`capture.py` sends one request per fresh connection with `Connection: close` and
stores the raw bytes; `diff_captures.py` compares two capture directories.

The Scala side can be regenerated with `capture_scala.py`, which sends the same
requests but logs the Scala's per-request output to a file (a full stdout pipe
would stall the JVM mid-capture):

```
sbt -batch stage
python3 capture_scala.py \
    --binary <Recibase>/target/out/jvm/scala-2.13.18/recibase/universal/stage/bin/recibase \
    --requests requests.json --out capture-scala
```

The captures here were regenerated from `c02106f`; every response is
byte-identical after normalising `Date` and the manifest `version`.

## What "byte-for-byte" needed

1. **JSON printing.** circe prints compact JSON and keeps the encoder's field
   order, so every object is built in declaration order (`crates/core/src/json.rs`,
   `serde_json` with `preserve_order`) and `None` is `null`, not an absent key.
2. **Scala collection order.** `Set`s and `Map`s serialise in Scala 2.13's
   iteration order, which is *not* source order and *not* sorted order. It is the
   CHAMP trie order of `Hashing.improve(hashCode)`; a set of four or fewer
   elements is a `Set1..Set4` and keeps insertion order. 13 of the 95 recipes,
   every `inherited_tags` list, the `/meals/` array (185 entries) and the `/`
   docs map all depend on it. `crates/core/src/scala_hash.rs` reproduces it, including
   `case object`/`case class` hashing, `MurmurHash3`, `Some`/`None`,
   `LocalDate`, `Set`/`List` hashing, and the rule that a `Set`'s `map`/`flatMap`
   result is a trie again while a small set's is inserted in order.
3. **HTTP and CORS.** http4s' Ember backend writes
   `HTTP/1.1 <status>`, `Date`, `Connection: close`, `Content-Type`,
   `Content-Length`, then the body. `crates/server/src/http.rs` writes exactly that,
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
6. **The manifest.** `/manifest` reports the deployed commit plus the three
   fixed multi-tenancy fields added upstream in
   [`c02106f`](https://github.com/The-Silverwood-Institute/Recibase/commit/c02106f66a77ec67679fae51d916249bbd7536ad):
   `version`, `name` (`Recibase`), `source_url` and `base_commit_url`, in that
   order (circe's `Encoder.forProduct4`). `version` is the first of
   `GIT_COMMIT` / `SOURCE_COMMIT` / `GITHUB_SHA` whose trimmed value matches
   `[0-9a-fA-F]{7,40}`, otherwise `latest` (`crates/core/src/misc.rs`). The API
   image has none of those set, so `crates/server/build.rs` bakes the build's
   commit in as a last resort before `latest`; the harness normalises the field
   (and the `Content-Length` counting it) like `Date`, since it names the deploy
   and not the application.

## Deliberate differences

* `RECIBASE_TODAY` (server only): the `New` tag depends on the day the server
  runs (`LocalDate.now()`), so the harness can pin it. Unset, it uses the local
  date exactly like the Scala.
* `ureq` is built without TLS features, so the live GitHub/Turnstile calls need a
  TLS feature before deployment (the Scala uses the JDK client). Everything is
  exercised against fake servers instead.
* Two `commons-csv` divergences, both outside the fixture (see
  `crates/core/tests/usage_data.rs`): a leading BOM is stripped here but breaks the
  Scala's header lookup, and blank lines are skipped here while the Scala throws.
* `set_order` de-duplicates by `hashCode` (no `PartialEq` bound on the model
  types), so two *unequal* elements with an equal hash would collapse. The 31
  tag hashes and all 185 meal hashes are distinct; the test asserts it.
* `?withRevision=true` on `/recipes/` adds each entry's `revision`: a 16-hex
  FNV-1a digest of the recipe JSON (`RecipeDef::revision`) with the
  deployment-specific `edit` link removed. It is opt-in, so the default
  response stays byte-identical to the Scala's. Two deployments holding the
  same recipe agree on the digest, which is how the frontend tells a copied
  recipe from a same-named *different* one (see `FRONTEND-PORT.md`).

## The contract, and what is only content

The port began as a byte-for-byte reimplementation, and the Scala captures are
its record. A deployment is now a product of its own - it hosts a subset of the
upstream recipes plus its own - so byte equality cannot hold for anything that
carries the corpus. The line drawn is:

* **The declaration** - the routes, the JSON keys and their order, the status
  codes, the content types, the docs map, the manifest's field list - is the
  contract, and stays identical to Kit's and Alex's API.
* **The manifest of that declaration** - the recipes, the tags, the meals and
  the manifest's *values* - belongs to the deployment and may differ.

Two specs hold that line, alongside the port's captured record:

* `crates/server/tests/contract.rs` pins the declaration against this
  deployment's own data (it takes a recipe from the registry rather than naming
  one), so it is deterministic offline.
* `crates/server/tests/live_contract.rs` compares that declaration with another
  live deployment's, shape for shape. It runs only when `RECIBASE_LIVE_API`
  names one (e.g. `https://api.reciba.se/`), so it is never the only gate.

The corpus spec (`crates/core/tests/recipe_corpus.rs`) still compares every
upstream recipe we host byte for byte, and records the ones this deployment
deliberately does not host in `crates/core/tests/fixtures/omitted_recipes.txt`,
so that dropping a recipe is a deliberate edit rather than a silent loss.

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
