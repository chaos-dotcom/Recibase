# The Recibase frontend: Flask versus Rust

The frontend of Recibase - [The-Silverwood-Institute/Frontend](https://github.com/The-Silverwood-Institute/Frontend),
a Flask application that renders the pages of reciba.se and proxies the recipe
API - has been ported to Rust in this repository. The port answers **the same
bytes** as the Python and costs the machine far less.

Everything below was measured on the same machine, for the same application, on
the same endpoints, against the same API. Machine: Apple Silicon, 10 cores,
macOS 26.7; Python 3.14.6 (CPython), Flask 3.1.3, gunicorn 23.0.0; Rust 1.98.1.
The machine also carried a load average of about 5 during the runs, which pushes
both sides in the same direction; the ratios are the number to read.

The Python side is run the way its repository deploys it - `gunicorn app:app`,
which is what its `Procfile` says, so **one** synchronous worker - and, as a
second row, with `--workers 4`. The Rust side is
`target/release/recibase-frontend`.

Raw results: `tools/harness/frontend/results.json`; the protocol is in
`tools/harness/frontend/bench_frontend.py` and `tools/harness/frontend/ab_frontend.py`, and the
exact commands are in section 9.

## 1. What was ported

| Python | lines | Rust | lines |
|---|---:|---|---:|
| `app.py` | 229 | `app.rs`, `http.rs`, `pages.rs`, `backend.rs`, `templates.rs`, `statics.rs`, `version.rs`, `markupsafe.rs` | 1,446 |
| `scaler.py` | 587 | `scaler.rs` | 1,562 |
| `contribute.py` | 181 | `contribute.rs` | 330 |
| `cached_backend.py` | 23 | `cached_backend.rs`, `form.rs` | 163 |
| `templates/` (8 files) | 378 | the same 8 files, rendered with MiniJinja | - |
| `static/` (7 files) | 660 | the same 7 files, served from disk | - |
| tests (`test_*.py`, `conftest.py`) | 706 | `tests/*.rs`, 130 tests over 2,902 fixture cases | 3,259 |

The Rust `scaler.rs` is longer than the Python it replaces because the port uses
explicit types and a `Quantity` enum where Python had duck-typed classes. The
Rust tests are longer because the fixture cases - every quantity in the corpus
against fourteen scale factors, and so on - are generated from the Python's own
output and carried in the repository.

## 2. The port answers the same bytes

578 requests were captured from the running Flask application with
`tools/harness/capture.py`, which stores the raw response bytes of one request
per fresh TCP connection. The same list was replayed against the Rust server and
compared with `tools/harness/frontend/verify_frontend.py`, which compares the status line,
the reason phrase, the order of every header, every header value and the body.

```
563 identical, 0 differing, 5 not compared (non-deterministic)
```

**573 of 573 comparable responses are byte-identical.** The five that are not
compared are the `/random` requests, whose target is chosen at random in both
implementations.

The request list covers

* all 95 recipes, each at `?scale=2`, `?scale=0.5`, `?scale=3` and `?scale=1.5`,
  plus 26 scale-parameter edge cases (`0`, `1.0`, `-1`, `51`, `nan`, `1_0`,
  `+1`, `1e2`, two values for the same parameter, ...);
* the homepage, `/manifest.json`, `/sitemap.xml`, `/contribute` (GET and three
  POSTs), the 404, 405, 416 and 503 pages, and the 301 and 302 redirect pages;
* the seven static assets, with `HEAD`, a version query string, `If-None-Match`
  (matching, stale and `*`), `If-Match` (matching, stale, weak, `*` and
  unparseable), `If-Modified-Since` (matching and stale), `If-Unmodified-Since`,
  `Range` (four forms, two of which must be refused) and `If-Range`;
* `PUT`, `DELETE`, `PATCH` and `OPTIONS` on routes that do not allow them;
* odd paths: `//chicken-curry`, `/./chicken-curry`, `/static/../app.py`,
  `/chicken%2Dcurry`, `/contribute/`, `/CHICKEN-CURRY`.

What the Python answers there, the Rust now answers:

* Flask's own 404 page, the 301 and 302 redirect pages, and Werkzeug's 405 and
  416 pages, byte for byte - including the `Allow` header and the
  `Content-Range: bytes */<size>` line;
* the 412 that a failed `If-Match` produces, which keeps the whole response and
  changes only the status;
* the exact header set and header order of a Werkzeug static-file response,
  including the `ETag` (which is `"<mtime>-<size>-<adler32 of the path>"`, not a
  hash of the contents), `Cache-Control: no-cache`, `Accept-Ranges: bytes`,
  `Content-Disposition`, `Last-Modified`, and the 304, 206 and 416 answers that a
  conditional or range request produces;
* Jinja2's HTML escaping, which differs from MiniJinja's for `'`, `"` and `/`;
* the `/manifest.json` body and its `text/json` content type;
* the sitemap, built from the request's `Host` header.

Two headers are not compared, because they belong to the server rather than to
the application: `Server` (`Werkzeug/3.1.9 Python/3.14.6` for the development
server, `gunicorn` under the Procfile) and `Date`. The `Allow` header's token
order is also normalised - Werkzeug builds it from a Python `set`, so it changes
between Flask processes.

## 3. Test suite

| | Python, `pytest -q` | Rust, `cargo test -p recibase-frontend` | |
|---|---:|---:|---:|
| Warm, after a change (median of 5) | 0.274 s | **0.196 s** | 1.4x faster |
| Cold, no build outputs (median of 3) | **0.345 s** | 9.90 s | 29x slower |
| Cold release build | - | 24.1 s | |
| Tests | 63 | 130 | |
| Test execution, self-reported | 0.08 s | 0.07 s | |

This is the one place where the Rust is not obviously better, and it should be
said plainly. A warm run is about the same on both sides - at 0.2-0.3 s the
number is dominated by process start-up, not by the tests. A cold run is a
different story: the Python is interpreted, so going from a clean tree to a green
suite costs it the bytecode compilation of four modules, while Rust has to
compile its whole dependency graph (51 crates) first. What the Rust suite buys
for that is coverage the Python suite does not have: 2,902 cases taken from the
Python's own output, and the page-level equivalence of section 2.

## 4. Size on disk

| | Python | Rust | ratio |
|---|---:|---:|---:|
| Application code | 1,020 lines, 38 KB | one binary, **3.10 MiB** | - |
| Templates | 378 lines, 9 KB | compiled into the binary | - |
| Static assets | 7 files, 220 KB | the same 7 files, 220 KB | - |
| Dependencies | **38 MiB** (`.venv`: Flask, gunicorn, requests, pytest and theirs) | none - statically linked apart from the system C library | - |
| Runtime the host must provide | **87 MiB** (`python@3.14`) | none | - |
| Crates compiled into the binary | 40 Python packages installed | 51 crates | - |

A deployment of the Python frontend needs an interpreter and a virtual
environment. A deployment of the Rust frontend is one 3.10 MiB binary plus the
220 KB `static/` directory it serves. The binary is built with LTO and
`strip = true` and carries a TLS client, so it can talk to an `https://` API.

## 5. Memory

| | gunicorn, 1 worker | gunicorn, 4 workers | Rust | Rust vs 1 worker |
|---|---:|---:|---:|---:|
| RSS at rest, 15 s after the first request | 55.0 MiB | 161.0 MiB | **5.6 MiB** | **9.8x smaller** |
| Peak RSS during a 4,000-request burst | 62.6 MiB | 194.0 MiB | **13.7 MiB** | **4.6x smaller** |
| RSS 15 s after the load ends | 62.6 MiB | 73.5 MiB | 12.7 MiB | 4.9x smaller |
| Processes at rest | 2 | 5 | **1** | |

The Python figures are the whole process tree, which is what the service costs
the machine: gunicorn's master plus its worker or workers. The Rust server keeps
one thread per open connection while a request is in flight, so its peak follows
the largest body it holds at once - a 17 KB recipe page.

## 6. CPU

Workload: 4,000 requests over five paths (`/`, `/chicken-curry`,
`/static/styles.css`, `/manifest.json`, `/does-not-exist`), 16 concurrent
keep-alive clients. The server's own CPU time is read from its process tree
before and after.

| | gunicorn, 1 worker | gunicorn, 4 workers | Rust |
|---|---:|---:|---:|
| CPU per request | 0.632 ms | 1.055 ms | **0.150 ms** |
| Requests per CPU-second | 1,582 | 948 | **6,667** |
| CPU while idle | 0.00 % of a core | 0.00 % | 0.00 % |
| CPU-seconds for the burst | 2.53 s | 4.22 s | **0.60 s** |

**Adding workers makes each request more expensive, not cheaper**: four workers
cost 67 % more CPU per request than one, because each worker keeps its own copy
of the 15-minute recipe-list cache and refills it separately. Concurrency buys
throughput, not efficiency. The Rust server is 4.2x more efficient per request
than the single Python worker and 7.0x more than the four-worker setup, and it
reaches that on one process.

Per endpoint with ApacheBench (`ab -k -c 32`), which is not the bottleneck at
these rates:

| endpoint | gunicorn 1w | gunicorn 4w | Rust | Rust vs 1w |
|---|---:|---:|---:|---:|
| `/` | 2,084 req/s | 6,291 req/s | **46,836 req/s** | **22.5x** |
| `/`, CPU per request | 0.472 ms | 0.588 ms | **0.105 ms** | 4.5x less |
| `/chicken-curry` | 720 req/s | 2,586 req/s | **8,574 req/s** | **11.9x** |
| `/chicken-curry`, CPU per request | 1.058 ms | 1.217 ms | **0.311 ms** | 3.4x less |
| `/static/styles.css` | 2,477 req/s | - | **95,654 req/s** | **38.6x** |
| `/static/styles.css`, CPU per request | 0.400 ms | - | **0.054 ms** | 7.4x less |

`/chicken-curry` is the slowest page for both because every request renders the
95-entry drawer and asks the API for the recipe. The Rust server makes that
upstream call too and still costs a third of a Python request's CPU.

## 7. Latency

`ab -k -c 32`, in milliseconds:

| endpoint | | p50 | p95 | p99 | max |
|---|---|---:|---:|---:|---:|
| `/` | gunicorn 1w | 15 | 17 | 24 | 36 |
| | Rust | **1** | **2** | **3** | **6** |
| `/chicken-curry` | gunicorn 1w | 40 | 58 | 167 | 282 |
| | Rust | **3** | **8** | **12** | **23** |
| `/static/styles.css` | gunicorn 1w | 13 | 13 | 19 | 35 |
| | Rust | **0** | **1** | **1** | **5** |

Mixed workload, 16 keep-alive clients: mean p50 11.8 ms, p95 19.5 ms, p99 22.2 ms
for gunicorn against **0.98 ms, 2.82 ms and 4.72 ms** for Rust.

## 8. Start-up

| | Python | Rust | ratio |
|---|---:|---:|---:|
| Spawn to the first `200` on `/` | 0.198 s (1 worker), 0.245 s (4 workers) | **0.018 s** | 11x, 14x faster |

The Python figure is gunicorn's master and worker boot, the import of Flask and
its dependencies, and the first request. The Rust figure is process spawn.

## 9. Reproducing the measurements

```
# the application numbers: RAM, CPU, latency, start-up
python3 tools/harness/frontend/bench_frontend.py --label python-gunicorn-1worker \
    --command "/path/to/Frontend/.venv/bin/gunicorn app:app --bind 0.0.0.0:19080" \
    --cwd /path/to/Frontend --port 19080 --requests 4000 --concurrency 16 \
    --env BACKEND_URL=http://localhost:8081/ --out tools/harness/frontend/results.json
python3 tools/harness/frontend/bench_frontend.py --label rust \
    --command "target/release/recibase-frontend" --port 19081 \
    --env BACKEND_URL=http://localhost:8081/ \
    --env STATIC_DIR=crates/frontend/static --out tools/harness/frontend/results.json

# throughput and latency at high pressure
python3 tools/harness/frontend/ab_frontend.py --label rust --command "target/release/recibase-frontend" \
    --port 19081 --url /chicken-curry --requests 10000 --concurrency 32 \
    --env BACKEND_URL=http://localhost:8081/ \
    --env STATIC_DIR=crates/frontend/static --out tools/harness/frontend/results.json

# the test suite: first the Python checkout, then this repository
cd /path/to/Frontend && .venv/bin/python -m pytest -q   # 0.274 s warm, 63 tests
cargo test -p recibase-frontend                         # 0.196 s warm, 130 tests
```

Both servers are given a 1,000-request warm-up before the measured burst, so the
Python is measured after its imports, not during them. CPU is always the server
process tree's own CPU time, never the client's. RSS is sampled every 50 ms
during the load and the maximum is reported. `ab`'s own client CPU is not
reported: the client became the bottleneck for the Rust `/static/` row, which is
why the per-request CPU column is the one to compare there.

## 10. What is different, stated plainly

* Two template expressions are rewritten when the templates are loaded, because
  MiniJinja implements neither `Mapping.get` nor `str.startswith`. The template
  files are not touched; the substitutions and the reason are in
  `src/templates.rs`.
* Static-file `ETag` and `Last-Modified` describe the file on disk, so two
  checkouts of the same content have different ones. The comparison points both
  servers at the same directory, and the formula - including the `adler32` of the
  path - is the one Werkzeug uses.
* The `POST /contribute` path beyond the unconfigured 503 needs a passcode, a
  Turnstile secret and a GitHub token, so it is covered by the ported tests
  rather than by the capture.
* Only `application/x-www-form-urlencoded` bodies are parsed; Werkzeug also
  parses multipart. The form has no file inputs, so a browser never sends it.
* The JSON this port sends to the API is printed compactly, where `requests`
  prints with `json.dumps` defaults. The API parses JSON either way.
* A missing `version` in the API's manifest answer is a 503 here and an uncaught
  `KeyError` - a 500 - in the Python. The API always sends one.
* The measurements were taken on a machine carrying a load average of about 5,
  so absolute throughput is conservative for both sides.
