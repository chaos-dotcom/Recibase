# Application footprint: Recibase on the JVM vs the Rust port

Measured on the same machine (Apple Silicon, 16 cores, 128 GiB RAM, macOS 26.7),
for the same application, the same endpoints and the same workload.
Scala = the staged Recibase distribution under Java 25 (`stage/bin/recibase`,
default JVM flags — no `-Xmx`, as the README's `sbt run` does).
Rust = `target/release/recibase-server`.

Configuration: no meal log (`MEAL_LOG_CSV_URL` unset), so both servers answer
from the same data. Rust ran with `RECIBASE_TODAY=2026-10-01` to pin the one
date-dependent tag; the Scala reads the same date from the clock.

Every pair of numbers below was taken back-to-back, same protocol, same
conditions. The machine also had ~6 unrelated cores busy with another job, which
is why absolute throughput is conservative; a repeat on an idle machine gave the
Rust numbers unchanged and the Scala numbers ~5 % better.

## 1. Size on disk

| | Scala | Rust | ratio |
|---|---:|---:|---:|
| Distribution | **41.5 MiB** (54 files: 52 jars + 2 scripts) | **2.28 MiB** (1 file, 2,388,096 bytes) | 18× smaller |
| Same, stripped | — | 1.89 MiB (1,985,736 bytes) | 22× smaller |
| Application code alone | 1.0 MB (app jar) | included | — |
| Runtime the host must provide | **371 MiB** (openjdk@25) | none — links only `libSystem` and `CoreFoundation` | — |
| Total to deploy | **≈ 412 MiB** | **2.28 MiB** | 181× smaller |
| Files to ship | 54 | 1 | 54× fewer |

The Scala distribution is mostly third-party jars (http4s, cats-effect, circe,
fs2, logback, commons-csv, reflections). The Rust binary is statically linked
apart from two macOS system libraries, so a container would need no runtime
layer at all.

## 2. Memory

| | Scala | Rust | ratio |
|---|---:|---:|---:|
| RSS at idle (24 s after start) | **303 – 309 MB** | **6.0 – 6.1 MB** | ~50× smaller |
| Peak RSS during a 4,000-request burst | **987 MB – 1.53 GB** | **12.8 – 15.1 MB** | ~65 – 110× smaller |
| RSS 12 s after the load ends | **987 MB – 1.53 GB** | 12.6 – 14.9 MB | JVM never gives it back |
| RSS after a 30,000-request `ab` run | 909 MB – **2.55 GB** | 7 – **33 MB** | — |
| Threads idle → under load | 51 → 61 | 1 → one per open connection | 51× fewer at rest |

The JVM reserves a 32 GiB max heap (a quarter of this machine by default) and
grows into it: RSS climbs from ~305 MB to 1–1.5 GB during a burst, and after the
heaviest run it sits at 2.55 GB and stays. The Rust server peaks at ~15 MB —
about the size of the largest response it buffers (a 48 KB `/meals/` body).

## 3. CPU

Mixed workload: 4,000 requests over four endpoints (`/health`, `/recipes/`,
`/recipes/vegetable-primavera`, `/meals/`), 16 concurrent clients; server CPU
read from `ps` before and after.

| | Scala | Rust | ratio |
|---|---:|---:|---:|
| CPU while idle | 0.08 % of one core | **0.00 %** | — |
| CPU per request, keep-alive | **1.013 ms** | **0.215 ms** | 4.7× less |
| CPU per request, new connection each | **1.462 ms** | **0.307 ms** | 4.8× less |
| Requests served per CPU-second (keep-alive) | 988 | **4,651** | 4.7× more per core |
| CPU-seconds for the burst (keep-alive) | 4.05 s | **0.86 s** | 4.7× less |
| Cores actually busy during the burst | ~10 | ~3.8 | — |

Per endpoint with ApacheBench (`ab -k -c 32`), which is not the bottleneck at
these rates:

| endpoint | Scala | Rust | throughput | CPU per request |
|---|---:|---:|---:|---:|
| `/health` | 33,496 req/s | **157,113 req/s** | 4.7× more | 0.272 → **0.0137 ms** (20× less) |
| `/recipes/vegetable-primavera` | 27,813 req/s | **145,730 req/s** | 5.2× more | 0.336 → **0.028 ms** (12× less) |
| `/meals/` (48 KB) | 6,191 req/s | **18,729 req/s** | 3.0× more | 1.721 → **0.674 ms** (2.6× less) |

`/meals/` is where the two are closest, because it is dominated by serialising
185 meals rather than by framework and runtime overhead.

## 4. Latency

`ab -k -c 32` mean / p50 / p95 / p99, in milliseconds:

| endpoint | Scala | Rust |
|---|---|---|
| `/health` | 0.955 / 1 / 2 / 9 | **0.204 / 0 / 0 / 1** |
| `/recipes/{permalink}` | 1.151 / 1 / 3 / 6 | **0.220 / 0 / 0 / 0** |
| `/meals/` | 5.169 / 3 / 10 / 76 | **1.709 / 1 / 3 / 5** |

Mixed workload, 16 clients: p95 4.68 ms (keep-alive) / 6.34 ms (new connection)
for the Scala against 1.96 ms / 3.13 ms for Rust.

## 5. Start-up

| | Scala | Rust | ratio |
|---|---:|---:|---:|
| Process start → first `200` on `/health` | **1.25 – 1.48 s** | **0.023 – 0.028 s** | ~50× faster |

The Scala figure is JVM boot, class loading for ~50 jars and the
`org.reflections` classpath scan that builds the recipe registry. The Rust figure
is process spawn plus building the same registry from a static list. A first-ever
run of the Rust binary on a cold page cache measured 0.30 s, still ~4× faster.

## 6. How this was measured

```
# RAM, CPU, start-up and latency under a mixed load (RSS and CPU sampled every 50 ms)
python3 bench_app.py --kind scala --binary …/stage/bin/recibase  --mode keepalive --label scala-keepalive
python3 bench_app.py --kind rust  --binary …/target/release/recibase-server --mode keepalive --label rust-keepalive

# throughput and latency at high pressure
python3 ab_run.py --kind rust --binary … --url /meals/ --requests 10000 --concurrency 32 --label rust-ab-meals

# size
du -sh …/stage                     # 41.5 MiB in 54 files
du -sh /opt/homebrew/Cellar/openjdk@25/25.0.4.1   # 371 MiB
ls -l  …/recibase-server           # 2,388,096 bytes;  strip → 1,985,736
```

Stated plainly:

* Both servers get a 1,000-request warm-up before the measured burst, so the
  Scala side is measured after JIT compilation, not during it.
* The Python client tops out near 18k req/s, which caps the Rust numbers in the
  mixed-workload rows; the ApacheBench rows are the ones to read for throughput.
* CPU is the server process's own CPU time (`ps -o time`), never the client's.
* Peak RSS is sampled every 50 ms and agrees with `getrusage` max-RSS at exit.
* `ps -o vsz` is deliberately not reported: on this macOS it returns ~435 GB even
  for a `sleep`, so it says nothing about the application.
* Raw captures of both measurements: `app-loaded.json`, `ab-loaded.json`.
