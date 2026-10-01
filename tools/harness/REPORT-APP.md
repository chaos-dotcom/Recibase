# Application footprint: Recibase on the JVM vs the Rust port

Everything below was measured on the same machine (Apple Silicon, 16 cores,
128 GiB RAM, macOS 26.7), for the same application, the same endpoints and the
same workload. Scala = the staged Recibase distribution under Java 25
(`stage/bin/recibase`); Rust = `target/release/recibase-server`.

Configuration: no meal log (`MEAL_LOG_CSV_URL` unset), so both servers answer
from the same data. Rust ran with `RECIBASE_TODAY=2026-10-01` to pin the one
date-dependent tag; the Scala reads the same date from the clock.

## 1. Size on disk

| | Scala | Rust | ratio |
|---|---:|---:|---:|
| Distribution | **41.5 MiB** (54 files: 52 jars + 2 scripts) | **2.28 MiB** (1 file) | 18× smaller |
| Same, stripped | — | 1.89 MiB | 22× smaller |
| Application code alone | 1.0 MB (app jar) | included | — |
| Runtime required on the host | **371 MiB** (openjdk@25) | none (links only libSystem + CoreFoundation) | — |
| Total to deploy | **≈ 412 MiB** | **2.28 MiB** | 181× smaller |

The Scala distribution is mostly third-party jars (http4s, cats-effect, circe,
fs2, logback, commons-csv, reflections). The Rust binary is statically linked
apart from two macOS system libraries, so a container image would need no
runtime layer at all.

## 2. Memory

| | Scala | Rust | ratio |
|---|---:|---:|---:|
| RSS at idle (settled 24 s after start) | **307.8 MB** | **6.0 MB** | 51× smaller |
| Peak RSS under load | **985 MB – 1.36 GB** | **12.8 – 14.9 MB** | ~70–100× smaller |
| RSS 12 s after the load ends | **985 MB – 1.42 GB** | 12.6 – 14.8 MB | JVM does not return it |
| Threads, idle | 51 | 1 | 51× fewer |
| Threads under load | 61 | one per open connection | — |

The JVM reserves a 32 GiB max heap by default (a quarter of this machine) and
grows into it under load; RSS climbs from 308 MB to 1–1.4 GB during a 4,000
request burst and stays there afterwards. The Rust server peaks at ~15 MB,
which is the same order as the response bodies it is streaming.

## 3. CPU

Mixed workload: 4,000 requests over four endpoints (`/health`, `/recipes/`,
`/recipes/vegetable-primavera`, `/meals/`), 16 concurrent clients. Server CPU is
read from `ps` before and after the load.

| | Scala | Rust | ratio |
|---|---:|---:|---:|
| CPU while idle | 0.00 – 0.08 % of one core | 0.00 % | — |
| CPU per request, new connection per request | **1.46 ms** | **0.31 ms** | 4.7× less |
| CPU per request, keep-alive | **0.98 ms** | **0.21 ms** | 4.7× less |
| Requests served per CPU-second (keep-alive) | ~1,020 | ~4,760 | 4.7× more work per core |
| CPU-seconds for the 4,000-request burst | 3.92 s keep-alive / 5.82 s new-connection | 0.84 s / 1.25 s | 4.7× less |
| Cores actually busy during the burst | ~10 | ~3–4 | — |

Single-endpoint throughput with ApacheBench (`ab -k -c 32`), which is not the
bottleneck at these numbers:

| endpoint | Scala | Rust | ratio |
|---|---:|---:|---:|
| `/health` | 31,195 req/s (0.27 CPU-ms each) | **155,530 req/s** (0.013 CPU-ms each) | 5.0× more, 20× less CPU |
| `/recipes/vegetable-primavera` | 25,972 req/s (0.39 CPU-ms) | **146,498 req/s** (0.031 CPU-ms) | 5.6× more, 12× less CPU |
| `/meals/` (48 KB) | 6,475 req/s (1.62 CPU-ms) | **17,922 req/s** (0.69 CPU-ms) | 2.8× more, 2.4× less CPU |

(`/meals/` is dominated by serialising 185 meals, so it is the endpoint where
the two implementations are closest.)

## 4. Latency under the mixed workload

| | Scala | Rust |
|---|---:|---:|
| p50 | 0.95 – 1.86 ms | 0.77 – 1.58 ms |
| p95, new connection per request | 4.98 ms | 3.14 ms |
| p95, keep-alive | 4.03 ms | 1.95 ms |
| p99 (`ab`, `/recipes/{permalink}`) | 7 ms | 1 ms |

## 5. Start-up

| | Scala | Rust | ratio |
|---|---:|---:|---:|
| Process start → first `200` on `/health` | **1.23 – 1.26 s** | **0.023 – 0.024 s** | ~52× faster |

The Scala figure is the JVM boot, class loading for ~50 jars and the
`org.reflections` classpath scan that builds the recipe registry. The Rust
figure is process spawn plus building the same registry from a static list.

## 6. How this was measured

```
# RAM / CPU / latency under a mixed load, with RSS and CPU sampled every 50 ms
python3 bench_app.py --kind scala --binary …/stage/bin/recibase --mode keepalive --label scala-keepalive
python3 bench_app.py --kind rust  --binary …/target/release/recibase-server --mode keepalive --label rust-keepalive

# throughput and latency at high pressure (ab is not the bottleneck here)
python3 ab_run.py --kind rust --binary … --url /meals/ --requests 10000 --concurrency 32 --label rust-ab-meals

# size
du -sh …/stage            # 41.5 MiB
ls -l  …/recibase-server  # 2,388,096 bytes; strip → 1,985,736
```

Protocol notes, stated plainly:

* Both servers get the same warm-up (1,000 requests) before the measured burst,
  so the Scala side is measured after JIT compilation, not during it.
* The Python client tops out near 11k–18k req/s, which bounds the Rust numbers
  in the mixed-workload table; the ApacheBench figures are the ones to read for
  throughput.
* CPU is the server process's own CPU time (`ps -o time`), not the client's.
* Peak RSS is sampled every 50 ms; `getrusage` max-RSS at exit agrees with it to
  within a few hundred KB.
* `ps -o vsz` is not reported: on this macOS it returns ~435 GB even for a
  `sleep`, so it says nothing about the application.
* The Scala server was run with its default JVM flags — no `-Xmx`, no tuning, as
  the README's `sbt run` and the Dockerfile do.
