# Containers: Scala vs Rust

Same host (Apple Silicon 16 cores, Docker Desktop 4.40, 16 CPUs / 94 GiB in the
VM), same client, same path for every row: `ab -k -c 32` running *inside* the
Docker VM (`httpd:2-alpine` on the same bridge network) against the container,
so Docker Desktop's host port forwarder is not in the path. Container CPU is the
container's own cgroup (`usage_usec`), memory is `docker stats` sampled during
the load.

| container | platform | image size | start-up | idle RSS | peak RSS | `/recipes/{permalink}` (2 KB) | `/meals/` (48 KB) | CPU per request (recipes / meals) |
|---|---|---|---|---|---|---|---|---|
| Scala, published `ghcr.io/the-silverwood-institute/recibase:latest` | linux/amd64, **emulated** | **667 MB** | 4.87 s | 461 MB | 2.87 GB | 2,787 req/s (p99 183 ms) | 2,183 req/s | 0.83 ms / 2.04 ms |
| Scala, built locally from source | linux/arm64, native | 692 MB | 1.21 s | 384 MB | 2.58 GB | 22,349 req/s (p99 8 ms) | 7,826 req/s | 0.46 ms / 0.93 ms |
| **Rust, `recibase-rust:distroless`** (recommended) | linux/arm64 | **50.6 MB** | 0.05 s | 1 MB | 6.0 MB | **117,608 req/s** (p99 1 ms) | **24,822 req/s** | same binary as slim |
| Rust, `recibase-rust:slim` (shell + healthcheck) | linux/arm64 | 139 MB | 0.05 s | 1 MB | 6.1 MB | 117,457 req/s (p99 1 ms) | 23,135 req/s | 0.028 ms / 0.59 ms |
| Rust, `recibase-rust:scratch` (musl, no libc) | linux/arm64 | **3.05 MB** | 0.05 s | 1 MB | 18.5 MB | 5,954 req/s (p99 28 ms) | 107 req/s | see note 2 |
| Rust, musl on `debian-slim` (same binary as scratch) | linux/arm64 | 139 MB | 0.05 s | 1 MB | 18.8 MB | 5,999 req/s | 110 req/s | 2.54 ms / 143 ms |

Native-vs-native (the arm64 Scala build against `recibase-rust:slim`):

| | Scala | Rust | ratio |
|---|---:|---:|---:|
| Image size | 692 MB | 139 MB (50.6 MB distroless) | 5x (14x) smaller |
| Start-up | 1.21 s | 0.05 s | 24x faster |
| Idle RSS | 384 MB | 1 MB | ~380x smaller |
| Peak RSS | 2.58 GB | 6.1 MB | ~420x smaller |
| `/recipes/{permalink}` | 22,349 req/s | 117,457 req/s | 5.3x more |
| `/meals/` | 7,826 req/s | 23,135 req/s | 3.0x more |
| CPU per request | 0.46 / 0.93 ms | 0.028 / 0.59 ms | 16x / 1.6x less |

## Three things worth knowing

1. **The published Scala image is amd64-only.** On this arm64 Mac it runs under
   QEMU: 4.87 s to start (against 1.21 s for the same sources built for arm64)
   and 8x less throughput (2,787 against 22,349 req/s). Any container comparison
   against the published tag on Apple Silicon or Graviton is measuring emulation
   as much as the implementation.

2. **Do not ship the musl images for concurrency.** `scratch`/`alpine` link musl,
   whose allocator takes a process-wide lock, and this server allocates a fresh
   JSON tree per request. Throughput therefore *falls* as connections rise
   (5,024 -> 2,341 -> 451 req/s at 1, 4 and 32 connections) and most of the CPU
   is kernel time (futex). The same binary built against glibc scales the other
   way (6,795 -> 25,659 -> 95,025 req/s) and is 16x faster at c=32 with 90x less
   CPU per request. `recibase-rust:distroless` keeps glibc and is still 50.6 MB.

3. **The 3 MB image is a trap.** `recibase-rust:scratch` is 3.05 MB and looks
   unbeatable until it is measured: 107 req/s on `/meals/` at c=32. Choose
   `distroless` (50.6 MB, glibc, no shell) for production, or `slim` (139 MB) if
   a shell and the `HEALTHCHECK` are wanted.

## Reproducing

```
# rust images (build context = recibase-rs/)
docker build -t recibase-rust:distroless --target distroless .
docker build -t recibase-rust:slim       --target runtime .

# the Scala container, built from this checkout (arm64 native)
cd Recibase && JAVA_HOME=/opt/homebrew/opt/openjdk@25 sbt --batch --server Docker/publishLocal
# or the published one (amd64, emulated here)
docker pull ghcr.io/the-silverwood-institute/recibase:latest

# run
docker run -d -p 8081:8081 -e PORT=8081 recibase-rust:distroless

# the comparison (in-VM client, exact cgroup CPU)
python3 bench_containers_final.py
```

Repeat runs of the two shipped Rust tags vary by a few percent
(`recibase-rust:slim` 119,615 / 25,460 req/s and `recibase-rust:distroless`
116,448 / 25,013 req/s on a second pass); the table quotes the first pass. The
Rust `runtime` image carries the same `HEALTHCHECK` as the Scala image (bash +
`/dev/tcp` on `/health`); the distroless image has no shell, so a
healthcheck there would need an in-binary probe. (The musl/`scratch` target has
since been removed; the glibc `distroless` and `slim` images remain.)

Raw numbers: `container-final.json` (the five-image run), `container-results.json`
(the first pass through published host ports).
