
#!/usr/bin/env bash
# Run the Recibase stack locally: the Rust API plus the Rust frontend.
#
#   ./tools/harness/frontend/serve-stack.sh                   # api :8081, frontend :8080
#   BACKEND_PORT=9091 FRONTEND_PORT=9090 ./tools/harness/frontend/serve-stack.sh
#
# Ctrl-C stops both.
set -euo pipefail

WS="$(cd "$(dirname "$0")/../.." && pwd)"
BACKEND_PORT="${BACKEND_PORT:-8081}"
FRONTEND_PORT="${FRONTEND_PORT:-8080}"
export BACKEND_URL="${BACKEND_URL:-http://localhost:${BACKEND_PORT}/}"

BACKEND_BIN="${BACKEND_BIN:-$WS/target/release/recibase-server}"
FRONTEND_BIN="${FRONTEND_BIN:-$WS/target/release/recibase-frontend}"
export STATIC_DIR="${STATIC_DIR:-$WS/crates/frontend/static}"

for path in "$BACKEND_BIN" "$FRONTEND_BIN"; do
  if [ ! -x "$path" ]; then
    echo "missing $path" >&2
    echo "build it:  cargo build --release" >&2
    exit 1
  fi
done

PORT="$BACKEND_PORT" "$BACKEND_BIN" &
BACKEND_PID=$!
trap 'kill "$BACKEND_PID" 2>/dev/null || true' EXIT INT TERM

for _ in $(seq 1 100); do
  if curl -fsS -o /dev/null "http://127.0.0.1:${BACKEND_PORT}/health" 2>/dev/null; then break; fi
  sleep 0.1
done

echo
echo "  api      (Rust) : http://127.0.0.1:${BACKEND_PORT}/   pid ${BACKEND_PID}"
echo "  frontend (Rust) : http://localhost:${FRONTEND_PORT}/   <-- open this"
echo

PORT="$FRONTEND_PORT" "$FRONTEND_BIN"
