# Recibase → Rust: port, byte-for-byte API compatibility, and test performance

Recibase is a Scala 2.13 service (http4s, circe, enumeratum, specs2; 124 main
sources / 7,963 lines plus 10 spec files / 1,346 lines). It has been ported to
Rust in `recibase-rs/` and the port is **byte-for-byte compatible on the HTTP
API**, verified against captures of the running Scala server.

## 1. What was built

| crate | contents | lines |
|---|---|---:|
| `core` | `Recipe`/`Ingredient`/`Tag`/`Meal`/`Permalink` model, the 95-recipe corpus, `MealDefinitions` (90 hand-written stubs), usage data + meal-log CSV, and the Scala collection-order emulation | 7,529 |
| `submit` | submission decoding, validation, generated-Scala-source rendering, GitHub client, Turnstile, config | 1,633 |
| `server` | recipe/meal/meta controllers, routes, submission route, CORS, and a hand-written HTTP/1.1 server | 668 |
| tests | 139 tests in 13 files (every Scala spec is ported, plus capture comparisons) | 3,456 |

`cargo run -p recibase-server` serves the same API on `PORT` (default 8081) and
honours the same environment variables (`MEAL_LOG_CSV_URL`, `PORT`, the
submission variables, `GIT_COMMIT`/`SOURCE_COMMIT`/`GITHUB_SHA`).

## 2. Byte-for-byte compatibility: the evidence

`capture.py` sends one request per fresh TCP connection with `Connection: close`
and stores the **raw response bytes**. Six captures were taken from the Scala
server; the Rust server is then captured with the same request list and diffed:

| capture | requests | scenario | result |
|---|---:|---|---|
| `capture-scala/` | 114 | no meal log | 114/114 identical |
| `capture-scala-csv/` | 114 | meal log loaded (notes, `last_eaten`, `times_eaten`, `featured`, frequency tags) | 114/114 identical |
| `capture-scala-submit/` | 13 | submission configured from the environment | 13/13 identical |
| `capture-scala-cors/` | 7 | requests carrying `Origin` | 7/7 identical |
| `capture-scala-cors2/` | 20 | the full preflight matrix | 20/20 identical |
| `capture-scala-cors3/` | 6 | requested-header normalisation | 6/6 identical |

**274/274 responses are byte-identical.** Every raw response differs from its
Scala counterpart in the `Date` header only — that header is a timestamp and
cannot match by construction; `diff_captures.py` normalises exactly that one
line and compares everything else, headers and body, byte for byte.

### What that required

1. **circe's JSON.** Compact printing, encoder field order, `None` as `null`,
   `LocalDate` as `YYYY-MM-DD`.
2. **Scala 2.13 collection order.** `Set`s and `Map`s are serialised in CHAMP
   trie order — not source order, not sorted order. `core/src/scala_hash.rs`
   reproduces `Hashing.improve`, the trie walk (payloads before sub-nodes), the
   `Set1..Set4` insertion-order rule for four or fewer elements, and Scala's
   hashing of case objects, case classes (`MurmurHash3.caseClassHash`), `Option`,
   `LocalDate`, `Set` and `List`. This was necessary for 13 of the 95 recipes,
   every `inherited_tags`, the 185-entry `/meals/` array and the `/` docs map.
   The rule was read out of the Scala 2.13.18 sources and confirmed against 200+
   orders measured from the running Scala server.
3. **http4s/Ember response bytes.** Status line, `Date`, `Connection: close`,
   `Content-Type`, `Content-Length`, body; no content type on a preflight;
   `Not found` for unmatched routes.
4. **CORS.** The Scala wraps its routes in `CORS.policy.withAllowOriginAll`:
   `OPTIONS` with `Origin` and `Access-Control-Request-Method` is answered by the
   middleware, matched routes get `Access-Control-Allow-Origin: *` after
   `Content-Length`, and unmatched requests (handled by `orNotFound`, outside the
   middleware) get nothing.
5. **Text handling.** NFD accent stripping, `unpluralise`, gas-mark temperature
   formatting with `Math.round`, and the exact Scala source text that the
   submission feature generates.

### What is *not* covered by a byte capture

The submission paths behind the Turnstile gate — the passcode 401, the
validation 400/409 messages and the GitHub results (200/502/409). Reaching them
over HTTP needs a live Cloudflare `siteverify` answer with `action: contribute`
(the documented test keys return `success` but no `action`, so even those do not
pass the gate), and the GitHub half needs a real token. They are covered instead
by the ported spec tests, and the GitHub client is exercised against a fake
GitHub server on a real socket.

## 3. Test performance, before and after

Machine: Apple Silicon, 16 cores, 128 GiB RAM, macOS 26.7; Java 25 (Homebrew
Temurin-equivalent), sbt 2.0.9, Rust 1.98.1. Median of 3 runs, machine idle.

Both sides start from the same kind of state. *Cold* = no build outputs and no
compiled-artifact cache (`target/`, plus sbt 2's action/CAS cache for the Scala
side — without clearing it a Scala `clean` is free). *Warm* = run again with
nothing changed. Dependency jars and the cargo registry cache are kept on both
sides, exactly as a developer's machine would have them.

| what | Scala | Rust (debug) | Rust (release) | Rust speed-up |
|---|---:|---:|---:|---:|
| **Cold: build + run the suite** | **11.70 s** (5.1/min) | **8.05 s** (7.5/min) | **6.65 s** (9.0/min) | 1.5× / 1.8× |
| Cold: compile/build only | 9.26 s | 3.49 s | — | 2.7× |
| **Warm: run the suite** | **2.90 s** (20.7/min) | **0.31 s** (194/min) | **0.27 s** (223/min) | 9.4× / 10.7× |
| Suite-reported test execution | 1.00 s | 0.09 s | 0.09 s | ~11× |
| Tests | 69 examples (10 specs) | 139 tests (13 binaries) | | |

Read as rates: from a cold tree the Rust suite completes **7.5 times a minute
against the Scala's 5.1**; re-running it after a change is **194/minute against
20.7** — a change-edit-test cycle of about a third of a second instead of three.

The gap is wider on the warm path than on the cold one because the Scala build
spends most of its cold time in the same place (Zinc compiling 124 + 10 sources),
while the warm path still pays JVM and sbt-server startup on every run, which
Rust does not have at all.

## 4. Reproducing all of this

```
# the Rust suite
cd recibase-rs && cargo test --workspace            # 139 tests

# the Scala suite (sbt 2's `test` skips unchanged suites; testOnly * runs them)
cd Recibase && JAVA_HOME=/opt/homebrew/opt/openjdk@25 sbt --batch --server "testOnly *"

# byte-for-byte against the Scala captures (see PORT.md for the exact commands)
python3 verify_port.py --reference capture-scala --out capture-rust \
        --binary recibase-rs/target/release/recibase-server --env RECIBASE_TODAY=2026-10-01

# the timing protocol (clears caches between rounds, median of 3)
python3 measure2.py --only both --repeat 3
```

## 5. Caveats, stated plainly

* The comparison is a test-suite comparison, not a runtime benchmark of the
  service. No throughput or latency of the running servers was measured.
* The Scala timing includes JVM and sbt startup on every run; that is a real
  property of the workflow, not a handicap invented for the comparison.
* `RECIBASE_TODAY` is a port addition (documented in `PORT.md`) so that the `New`
  tag, which depends on the current date, can be pinned for capture comparison.
* `ureq` is built without TLS features, so a real GitHub or Turnstile call needs
  a TLS feature enabled before deployment.
* Two `commons-csv` divergences and one hashCode-collision limitation are
  documented in `PORT.md`; none of them is reachable from the captured API.
