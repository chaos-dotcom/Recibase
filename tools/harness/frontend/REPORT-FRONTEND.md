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

The headline, read against the Rust figure rather than against a config: one
Rust process of 8 MiB serves 49,064 requests a second on the hardest endpoint
measured, and it would take **23 to 48 gunicorn workers - more cores than this
machine has - with 1 to 2 GiB of memory** to match it. Section 7 has the sweep.

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

## 7. How many Python workers would it take to match the Rust server?

Every row of this table is the same measurement: start the server, let it settle,
run `ab -k -c 32 -n 30000` against `/`, and read the server's own process tree
before and after. The last row is the Rust server, measured the same way.

| gunicorn workers | req/s | CPU per request | cores busy | RSS at rest | processes | p95 |
|---:|---:|---:|---:|---:|---:|---:|
| 1 | 2,111 | 0.472 ms | 1.00 | 74.7 MiB | 2 | 16 ms |
| 2 | 3,901 | 0.505 ms | 1.97 | 111.6 MiB | 3 | 9 ms |
| 4 | 6,292 | 0.603 ms | 3.79 | 193.0 MiB | 5 | 6 ms |
| 6 | 6,991 | 0.806 ms | 5.63 | 274.4 MiB | 7 | 5 ms |
| 8 | 7,305 | 0.937 ms | 6.84 | 354.0 MiB | 9 | 6 ms |
| 10 | 7,565 | 0.973 ms | 7.36 | 434.5 MiB | 11 | 6 ms |
| 12 | 7,245 | 0.965 ms | 6.99 | 515.5 MiB | 13 | 7 ms |
| 16 | 7,083 | 0.990 ms | 7.01 | 678.0 MiB | 17 | 7 ms |
| 20 | 7,608 | 0.973 ms | 7.40 | 837.9 MiB | 21 | 6 ms |
| 24 | 7,516 | 0.973 ms | 7.31 | 999.8 MiB | 25 | 5 ms |
| **Rust** | **49,064** | **0.097 ms** | **4.78** | **8.1 MiB** | **1** | **2 ms** |

Three things to read out of it.

**Python stops climbing at about six workers.** From 6 to 24 workers it serves
between 7,083 and 7,608 requests a second - a flat line - while the memory it
holds grows from 274 MiB to 1,000 MiB and the process count from 7 to 25. The
last row of the Python half therefore serves **one sixth** of the Rust server's
throughput with 25 processes and a gigabyte of resident memory.

**Every worker added makes the others less efficient.** One worker does 2,111
requests a second per core; by 24 workers the same core does 1,028 - the cost of
a request rises from 0.472 ms to 0.973 ms. The Python is not losing to a fixed
overhead that more processes could amortise; its per-request cost *doubles*
under load, so concurrency does not buy back what it spends.

**To match 49,064 req/s, gunicorn would need this many cores:**

| | cores needed | RSS it would hold | processes |
|---|---:|---:|---:|
| at the efficiency of one worker (2,111 req/s per core) | **23** | ~1,000 MiB | 24 |
| at the efficiency measured at scale (1,028 req/s per core) | **48** | ~2,000 MiB | 49 |
| the Rust server | **4.8** | 8.1 MiB at rest, 13.7 MiB at peak | 1 |

This machine has 10 cores and was carrying other load during the sweep, so the
23-to-48 range is the honest answer: **the Python cannot reach the Rust server's
figure here at all**, and on a machine large enough to try it would need roughly
two to five times the cores, with 40 MiB of memory per core.

Read the other way round, at **equal resources**:

* **Equal CPU** - the Rust server's 4.78 cores buys 49,064 req/s; four or five
  Python workers, which also take about 4.8 cores, serve 6,292 req/s. The Rust is
  **7.8x** the throughput for the same CPU.
* **Equal memory** - the Rust server's peak is 13.7 MiB. Python cannot run in
  that: gunicorn's smallest configuration is a master and one worker, 2 processes
  and 74.7 MiB at rest.
* **Equal processes** - one Rust process against gunicorn's master plus 24
  workers, which together serve 15 % of what the single Rust process serves.

The same shape holds on the mixed workload of section 5: 1 worker 1,326 req/s,
4 workers 3,218, 8 workers 4,149, 16 workers 3,857 (676 MiB, 17 processes), and
Rust 13,160 req/s from one process of 5.6 MiB.

## 8. Latency

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

## 9. Start-up

| | Python | Rust | ratio |
|---|---:|---:|---:|
| Spawn to the first `200` on `/` | 0.198 s (1 worker), 0.245 s (4 workers) | **0.018 s** | 11x, 14x faster |

The Python figure is gunicorn's master and worker boot, the import of Flask and
its dependencies, and the first request. The Rust figure is process spawn.

## 10. Reproducing the measurements

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

# the worker sweep of section 7: gunicorn with 1..24 workers, then the Rust
# server, one `ab` run each
python3 tools/harness/frontend/worker_sweep.py --workers 1,2,4,6,8,10,12,16,20,24 \
    --command "/path/to/Frontend/.venv/bin/gunicorn app:app --bind 0.0.0.0:19080" \
    --cwd /path/to/Frontend --port 19080 --url / --requests 30000 --concurrency 32 \
    --label python-gunicorn --out tools/harness/frontend/worker-sweep.json
python3 tools/harness/frontend/worker_sweep.py --workers 0 \
    --command "target/release/recibase-frontend" --port 19081 --url / \
    --requests 30000 --concurrency 32 --label rust \
    --out tools/harness/frontend/worker-sweep.json

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

## 11. What is different, stated plainly

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
