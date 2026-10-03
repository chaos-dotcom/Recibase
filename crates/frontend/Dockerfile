# syntax=docker/dockerfile:1
#
# The Recibase frontend, in Rust.
#
#   docker build -f crates/frontend/Dockerfile -t recibase-frontend .
#   docker run --rm -p 8080:8080 -e BACKEND_URL=https://api.reciba.se/ recibase-frontend
#
# The context is the repository root, because the crate is a member of the
# workspace whose Cargo.toml lives there (the same as the API's Dockerfile).
#
# The templates are compiled into the binary; only `static/` is read from disk,
# so the image carries the binary and that one directory.

# ---------------------------------------------------------------- build stage
# Pinned to bookworm (Debian 12) for the same reason as the API's Dockerfile: the
# glibc the binary links against must not outrun the runtime image's.
FROM rust:1-slim-bookworm AS build
WORKDIR /src
COPY Cargo.toml Cargo.lock ./
COPY crates/ crates/
RUN cargo build --release --locked -p recibase-frontend \
    && strip target/release/recibase-frontend

# ---------------------------------------------------------- runtime: slim
FROM debian:bookworm-slim AS runtime
RUN useradd --system --uid 10001 --create-home recibase
WORKDIR /app
COPY --from=build /src/target/release/recibase-frontend /usr/local/bin/recibase-frontend
COPY --from=build /src/crates/frontend/static /app/static
USER recibase
ENV PORT=8080 \
    STATIC_DIR=/app/static
EXPOSE 8080
HEALTHCHECK --interval=10s --timeout=5s --start-period=2s --retries=5 \
  CMD bash -c "exec 3<>/dev/tcp/127.0.0.1/8080 && printf 'GET / HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n' >&3 && grep -q '200' <&3"
ENTRYPOINT ["/usr/local/bin/recibase-frontend"]

# ------------------------------ runtime: smallest glibc image, no shell
FROM gcr.io/distroless/cc-debian12 AS distroless
WORKDIR /app
COPY --from=build /src/target/release/recibase-frontend /recibase-frontend
COPY --from=build /src/crates/frontend/static /app/static
ENV PORT=8080 \
    STATIC_DIR=/app/static
EXPOSE 8080
ENTRYPOINT ["/recibase-frontend"]
