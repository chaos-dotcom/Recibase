# syntax=docker/dockerfile:1
#
# The Recibase API, in Rust.
#
#   docker build -t recibase-rust:distroless --target distroless .   # ~51 MB, glibc
#   docker build -t recibase-rust:slim       --target runtime .      # shell + healthcheck
#
# The Scala image (ghcr.io/the-silverwood-institute/recibase) uses
# eclipse-temurin:25 and a bash /dev/tcp HEALTHCHECK; the `runtime` target below
# keeps the same healthcheck so the two can be compared like for like.

# ---------------------------------------------------------------- build stage
# glibc on purpose: musl's allocator takes a process-wide lock, and this server
# allocates a fresh JSON tree per request, so a musl build serialises under
# concurrency (measured: throughput falls as connections rise).
FROM rust:1-slim AS build
WORKDIR /src
COPY Cargo.toml Cargo.lock ./
COPY crates/ crates/
RUN cargo build --release --locked -p recibase-server \
    && strip target/release/recibase-server

# ------------------------------------------------- runtime: shell + healthcheck
FROM debian:bookworm-slim AS runtime
RUN useradd --system --uid 10001 --create-home recibase
COPY --from=build /src/target/release/recibase-server /usr/local/bin/recibase-server
USER recibase
ENV PORT=8081
EXPOSE 8081
HEALTHCHECK --interval=10s --timeout=5s --start-period=5s --retries=5 \
  CMD bash -c "exec 3<>/dev/tcp/127.0.0.1/8081 && printf 'GET /health HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n' >&3 && grep -q '200' <&3"
ENTRYPOINT ["/usr/local/bin/recibase-server"]

# ------------------------------ runtime: smallest glibc image, no shell
FROM gcr.io/distroless/cc-debian12 AS distroless
COPY --from=build /src/target/release/recibase-server /recibase-server
ENV PORT=8081
EXPOSE 8081
ENTRYPOINT ["/recibase-server"]
