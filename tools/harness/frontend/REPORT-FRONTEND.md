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

The headline, read against the Rust binary rather than against a configuration:
one Rust process of 5.6 MiB serves **43,525 requests a second** on the busiest
endpoint, and matching it would take **about 22 gunicorn workers if Python held
its single-worker efficiency, or about 45 at the efficiency it really shows once
ten are running** - 1 to 2 GiB of memory, 23 to 46 processes, and more cores than
this machine has. Section 7 has the sweep, and the reason the Python cannot be
pushed there at all: it is the stack that answers `Connection: close` on every
response, so `ab -k` buys it nothing.

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
| RSS at rest, once the first request has settled | 55.0 MiB | 161.0 MiB | **5.6 MiB** | **9.8x smaller** |
| Peak RSS during a 4,000-request burst | 62.6 MiB | 194.0 MiB | **11.8 - 13.7 MiB** | 4.6x smaller |
| RSS 45 s after the load ends | 62.6 MiB | 73.5 MiB | **5.6 MiB** | 11x smaller |
| Processes at rest | 2 | 5 | **1** | |

The Python figures are the whole process tree, which is what the service costs
the machine: gunicorn's master plus its worker or workers. The Rust server keeps
one thread per open connection while a request is in flight, so its peak follows
the largest body it holds at once - a 17 KB recipe page. The clients of this
workload keep their connections alive, but only the Rust server accepts that:
see the note in section 6 and section 7.

The Rust "at rest" figure moves with *when* it is sampled: macOS returns the
pages of the first full-page render lazily, so a reading taken seconds after `/`
is first served is about 8.3 MiB and the resting 5.6 MiB appears between 15 and
60 seconds later. Section 7 has the samples. Every "at rest" number here is a
settled one; the earlier 13.7 MiB "after the load" figure in this table was the
same effect on the way down, and 45 seconds is enough for it to reach 5.6 MiB.

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

Note when reading the throughput columns: `ab -k` is inert against the Python
stack. Werkzeug answers `Connection: close` on every response, so gunicorn opens
one connection per request whatever the client asks for, while the Rust server
reuses them - `ab` reports 0 keep-alive requests out of 2,000 for the first and
2,000 out of 2,000 for the second. Section 7 repeats this comparison with a
connection per request on both sides, where the Rust server loses that advantage
and still wins.

## 7. How many Python workers would it take to match the Rust binary?

The target is the Rust frontend in this repository, `crates/frontend`, served by
`target/release/recibase-frontend` as one process. On `/`, under `ab -k -c 32 -n
20000`, three runs, it serves a median of **43,525 requests a second** - and that
is the number to hold in mind, because the question is what gunicorn would have
to be given to reach it.

Before the table, one fact that has to be stated or the numbers look invented:
**the Python stack does not use HTTP keep-alive, and the Rust server does.**

```
$ curl -s -o /dev/null -D - http://127.0.0.1:19080/   # gunicorn, 1 worker
HTTP/1.1 200 OK
Server: gunicorn
Connection: close

$ curl -s -o /dev/null -D - http://127.0.0.1:19081/   # the Rust server
HTTP/1.1 200 OK
Connection: keep-alive
```

Werkzeug answers `Connection: close` on every response - the Flask development
server does the same, and it does it whether or not the client asked to keep the
connection, and whether or not gunicorn was configured to - so `ab -k` buys the
Python nothing: it opens one TCP connection per request, and one gunicorn worker
per connection at a time. `ab` confirms it, and confirms the opposite for Rust:

```
gunicorn:  Keep-Alive requests: 0        # out of 2,000
Rust:      Keep-Alive requests: 2,000
```

### The worker sweep

Every row is the same measurement - start the server, let it settle, run
`ab -k -c 32 -n 30000` against `/`, read the server's own process tree before and
after. Only gunicorn appears in it; the Rust binary is the target the rest of
this section is measured against, at 43,525 req/s, 4.9 cores, 5.6 MiB at rest and
13.7 MiB at peak from one process.

The Rust row of the "at rest" column needs one caveat, because it is easy to
measure it wrong and I did: **the same binary reads 5.6 MiB or 8.3 MiB
depending on when you look.** macOS returns the pages of the first full-page
render to the system lazily, so a sample taken seconds after `/` is first served
reads about 8.3 MiB, and the resting figure of 5.6 MiB appears between 15 and 60
seconds later - at a moment that varies from run to run:

| sample after the first `/` | +3 s | +15 s | +30 s | +60 s |
|---|---:|---:|---:|---:|
| run 1 | 8,288 KiB | 8,288 KiB | 8,288 KiB | **5,664 KiB** |
| run 2 | 8,272 KiB | 7,824 KiB | **5,664 KiB** | 5,664 KiB |
| run 3 | 8,352 KiB | **5,728 KiB** | 5,728 KiB | 5,728 KiB |

Every reading in this report that says "at rest" comes from a settle long enough
to reach the resting figure; the ones taken from the worker sweep, which settles
for three seconds, read high and have been corrected here.

| gunicorn workers | req/s | CPU per request | cores busy | RSS, 3 s settle | processes | p95 |
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

Python stops climbing at about six workers: from there to 24 workers it serves
between 7,083 and 7,608 requests a second while the memory it holds grows from
274 MiB to 1,000 MiB and the process count from 7 to 25.

The RSS column is sampled 3 s after the readiness probe, which overstates it -
the Python's allocator returns freed pages too, just not immediately. With a
45 s settle the same two configurations read **57.4 MiB** for one worker (against
74.7 here) and **346.0 MiB** for ten (against 434.5 here), 20 to 30 % lower. The
answer table below uses the settled values. **Twenty-four gunicorn
workers, a gigabyte of memory and 25 processes serve one sixth of what one Rust
process serves.**

### The answer

Workers are not the useful unit here, because a worker's throughput falls as
workers are added. Cores are:

| | cores needed | workers it would take | RSS | processes |
|---|---:|---:|---:|---:|
| at a single worker's cost per request (0.47-0.50 ms) | **22** | 22 | ~0.75 GiB | 23 |
| at the cost measured at ten workers (1.02 ms) | **45** | 45 | ~1.5 GiB | 46 |
| the Rust binary | **4.9** | 1 process | 5.6 MiB at rest, 13.7 MiB peak | 1 |

So: **about 22 gunicorn workers to match one Rust process, if Python could hold
its single-worker efficiency; about 45 at the efficiency it actually shows once
ten of them are running.** Either way it is 23 to 46 processes and 0.75 to 1.5 GiB
of resident memory against 1 process and 5.6 MiB.

And the second number is the real one, because on this machine the Python cannot
be pushed to 43,525 req/s at all: it plateaus near 7,500 req/s and the box has 10
cores. The 22-and-45 figures are what the *cost per request* implies, not a
configuration anyone can run here.

### The comparison that is most favourable to Python

If the client opens one TCP connection per request, the Rust server loses its
keep-alive advantage - it spawns a thread per connection - and drops from 43,525
to 24,820 requests a second. That is the fairest ground to compare on, and
Python still loses:

| `ab -c 32 -n 20000`, one connection per request | req/s | CPU per request | cores busy |
|---|---:|---:|---:|
| gunicorn, 1 worker | 1,757 | 0.543 ms | 0.95 |
| gunicorn, 4 workers | 4,356 | 0.808 ms | 3.52 |
| gunicorn, 10 workers | 6,634 | 0.991 ms | 6.57 |
| **the Rust binary** | **24,820** | **0.161 ms** | **3.98** |

To match that 24,820: **14 cores** at the cost of one Python worker (0.543 ms),
or **25 cores** at the cost it shows at ten workers (0.991 ms) - still more than
this machine has, and still 14 to 25 processes.

### At equal resources

* **Equal CPU** - the Rust binary's 4.9 cores buy 43,525 req/s; ten gunicorn
  workers, which also take about 6 cores, serve 7,565 req/s. The Rust is **5.8x**
  the throughput for the same CPU.
* **Equal memory** - the Rust binary rests at 5.6 MiB. Python cannot run in that:
  gunicorn's smallest configuration is a master and one worker, 2 processes and
  55 MiB at rest.
* **Equal processes** - one Rust process against gunicorn's master plus 24
  workers, which together serve 15 % of what that one process serves.

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

# the same sweep with one connection per request, and the Rust binary with and
# without keep-alive (the Python stack only ever does the former)
python3 tools/harness/frontend/worker_sweep.py --workers 1,4,10 \
    --command "/path/to/Frontend/.venv/bin/gunicorn app:app --bind 0.0.0.0:19080" \
    --cwd /path/to/Frontend --port 19080 --url / --requests 20000 --concurrency 32 \
    --no-keep-alive --out tools/harness/frontend/results.json
python3 tools/harness/frontend/worker_sweep.py --workers 0 \
    --command "target/release/recibase-frontend" --port 19081 --url / \
    --requests 20000 --concurrency 32 --no-keep-alive --out tools/harness/frontend/results.json

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
