
#!/usr/bin/env python3
"""Compares the Rust frontend against a capture of the Flask one.

Starts the Rust binary, replays the same request list against it, and compares
every response with its Flask counterpart byte for byte after normalising the
two headers that belong to the server rather than to the application:

  * `Server:`  - `Werkzeug/...` for the Flask development server, `gunicorn`
                 under the production Procfile; the Rust server sends its own.
  * `Date:`    - a timestamp.

Everything else - status line, reason phrase, header order, header values and
the body - must be identical.

Usage:
  verify_frontend.py --reference capture-flask --out capture-rust \
      --binary target/release/recibase-frontend \
      --env BACKEND_URL=http://127.0.0.1:8081/ --env STATIC_DIR=../static
"""
import argparse, json, os, re, socket, subprocess, sys, tarfile, tempfile, time

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
from capture import raw_request, normalise  # the shared capture harness, one level up

VOLATILE = re.compile(rb"^(?:Server|[Dd]ate):[^\r\n]*\r\n", re.M)
ALLOW = re.compile(rb"^Allow:[ \t]*([^\r\n]*)\r\n", re.M)


def sort_allow(match):
    """Werkzeug builds `Allow` from a Python `set`, so its token order changes
    between Flask processes. The tokens are what must agree, not their order."""
    tokens = sorted(token.strip() for token in match.group(1).decode("latin1").split(","))
    return b"Allow: " + ", ".join(tokens).encode("latin1") + b"\r\n"


def normalise_all(raw):
    return ALLOW.sub(sort_allow, VOLATILE.sub(b"", raw))


def split_headers(raw):
    head, _, body = raw.partition(b"\r\n\r\n")
    lines = head.decode("latin1").split("\r\n")
    return lines[0], [l for l in lines[1:] if l], body


def free_port():
    with socket.socket() as s:
        s.bind(("127.0.0.1", 0))
        return s.getsockname()[1]


def wait_for_port(port, process, timeout=15.0):
    deadline = time.time() + timeout
    while time.time() < deadline:
        if process.poll() is not None:
            raise SystemExit("server exited: %s" % process.returncode)
        try:
            with socket.create_connection(("127.0.0.1", port), timeout=0.25):
                return
        except OSError:
            time.sleep(0.02)
    raise SystemExit("server did not start listening on %d" % port)


# /random picks a recipe at random, so its Location cannot be compared.
LOOSE = ("random-",)
# The ETag and Last-Modified of a static file describe the file on disk: its
# mtime, size and path. They match when both servers serve the same directory,
# which the harness arranges, so they are compared like everything else.
STATIC_VOLATILE = ()


def resolve_reference(path):
    """A capture directory, or a `capture-x.tar.gz` archive of one."""
    if os.path.isdir(path):
        return path
    archive = path.rstrip("/") + ".tar.gz"
    if os.path.exists(archive):
        target = os.path.join(tempfile.mkdtemp(prefix="verify-frontend-"), os.path.basename(path))
        with tarfile.open(archive) as tar:
            tar.extractall(os.path.dirname(target))
        return target
    raise SystemExit("no capture at %s or %s" % (path, archive))


# The paths that decide what a response looks like: the templates, the static
# assets, the frontend itself, and the recipe corpus and API behind it. If any of
# them has changed since the commit the capture belongs to, the comparison is
# describing two different applications and says nothing.
# `:/` so the pathspecs are relative to the repository root whatever the
# working directory is; a pathspec that matches nothing makes `git diff
# --quiet` report success, which would silently disable this check.
SHAPING_PATHS = [":/crates", ":/tools/harness/frontend/requests.json"]


def check_origin(reference, allow_diverged, repo_override=None):
    """Refuse to compare unless the tree still matches the capture's commit.

    Returns True when the comparison may go ahead. The capture was taken from the
    Flask application while this repository answered the same bytes; both have
    moved since, so a run off the recorded commit would report differences that
    are simply the product changing.
    """
    origin = os.path.join(reference + ".origin.json")
    if not os.path.exists(origin):
        return True
    recorded = json.load(open(origin))
    commit = recorded.get("rust_commit")
    if not commit:
        return True
    # <repo>/tools/harness/frontend/verify_frontend.py
    repo = repo_override or os.path.dirname(os.path.dirname(os.path.dirname(
        os.path.dirname(os.path.abspath(__file__)))))

    def git(*args):
        return subprocess.run(["git", "-C", repo] + list(args),
                              capture_output=True, text=True)

    if git("rev-parse", "--git-dir").returncode != 0:
        print("note: %s is not a git checkout, so the capture's commit (%s) cannot "
              "be checked" % (repo, commit))
        return True
    if git("cat-file", "-e", commit + "^{commit}").returncode != 0:
        print("note: %s records commit %s, which this checkout does not have"
              % (origin, commit))
        return True
    if git("diff", "--quiet", commit, "--", *SHAPING_PATHS).returncode == 0:
        print("comparing against %s, taken from the Flask application at %s "
              "(unchanged since)" % (os.path.basename(reference), commit))
        return True

    changed = git("diff", "--name-only", commit, "--", *SHAPING_PATHS).stdout.split()
    print("%s belongs to commit %s, and this tree has moved on since:"
          % (os.path.basename(reference), commit))
    for path in changed[:20]:
        print("  changed: %s" % path)
    if len(changed) > 20:
        print("  ... and %d more" % (len(changed) - 20))
    print("""
The capture records the port: it was taken from the Flask application while this
repository answered the same bytes (573 of 573 comparable responses identical).
The paths above shape what a response looks like, so anything they have changed
makes this a comparison of two different applications rather than a check.

To reproduce the original result, compare against the recorded commit:

  git worktree add /tmp/recibase-pinned %s
  cd /tmp/recibase-pinned && cargo build -p recibase-server -p recibase-frontend
  PORT=8081 MEAL_LOG_CSV_URL=file://$PWD/../recibase-rs/tools/harness/meal-log.csv \
      ./target/debug/recibase-server &
  python3 tools/harness/frontend/verify_frontend.py --reference \
      tools/harness/frontend/capture-flask --binary target/debug/recibase-frontend \
      --requests tools/harness/frontend/requests.json --port 8080 \
      --env BACKEND_URL=http://localhost:8081/ --env STATIC_DIR=/path/to/Frontend/static

Run that harness with --repo /tmp/recibase-pinned, since the binary was built
there. Pass --allow-diverged to run the comparison anyway, knowing what it
means.""" % commit)
    return allow_diverged


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--reference", required=True)
    ap.add_argument("--out", required=True)
    ap.add_argument("--binary", required=True)
    ap.add_argument("--requests", required=True)
    ap.add_argument("--port", type=int, default=0)
    ap.add_argument("--env", action="append", default=[])
    ap.add_argument("--repo", default=None,
                    help="the checkout to test the capture's commit against, when "
                         "the binary was built somewhere else (a worktree of the "
                         "capture's commit, say). Defaults to this repository.")
    ap.add_argument("--allow-diverged", action="store_true",
                    help="compare even when the tree has moved on from the "
                         "capture's commit")
    ap.add_argument("--details", type=int, default=5,
                    help="how many differing responses to describe in full")
    ap.add_argument("--verbose", action="store_true")
    args = ap.parse_args()

    # The origin file sits beside the archive, so check before resolving: that
    # may unpack the capture into a temporary directory.
    given = args.reference.rstrip("/")
    args.reference = resolve_reference(given)
    if not check_origin(given, args.allow_diverged, args.repo):
        return 3
    port = args.port or free_port()
    env = dict(os.environ)
    env["PORT"] = str(port)
    for item in args.env:
        key, _, value = item.partition("=")
        env[key] = value

    os.makedirs(args.out, exist_ok=True)
    process = subprocess.Popen([args.binary], env=env,
                               stdout=subprocess.PIPE, stderr=subprocess.STDOUT)
    try:
        wait_for_port(port, process)
        requests = json.load(open(args.requests))
        index = json.load(open(os.path.join(args.reference, "index.json")))
        by_name = {entry["name"]: entry["i"] for entry in index}

        ok = diff = loose = 0
        failures = []
        for entry in requests:
            name = entry["name"]
            raw = raw_request("127.0.0.1", port, entry["path"],
                              entry.get("method", "GET"), entry.get("headers"),
                              (entry.get("body") or "").encode() or None)
            reference_path = os.path.join(
                args.reference, "%03d_%s.raw" % (by_name[name], name))
            reference = open(reference_path, "rb").read()
            with open(os.path.join(args.out, "%03d_%s.raw" % (by_name[name], name)), "wb") as fh:
                fh.write(raw)
            got = normalise_all(raw)
            want = normalise_all(reference)
            if name.startswith(LOOSE):
                status_got, _, _ = split_headers(got)
                status_want, _, _ = split_headers(want)
                if status_got == status_want:
                    loose += 1
                else:
                    diff += 1
                    failures.append((name, "status %r != %r" % (status_got, status_want)))
                continue
            if got == want:
                ok += 1
            else:
                diff += 1
                failures.append((name, describe(want, got)))

        print("%d identical, %d differing, %d not compared (non-deterministic)"
              % (ok, diff, loose))
        differing = [name for name, _ in failures]
        if differing:
            print("differing: %s%s" % (", ".join(differing[:80]),
                                       " ..." if len(differing) > 80 else ""))
        for name, why in failures[:args.details]:
            print("\n### %s\n%s" % (name, why))
        return 1 if diff else 0
    finally:
        process.terminate()
        try:
            process.wait(timeout=5)
        except subprocess.TimeoutExpired:
            process.kill()


def describe(want, got):
    want_status, want_headers, want_body = split_headers(want)
    got_status, got_headers, got_body = split_headers(got)
    out = []
    if want_status != got_status:
        out.append("  status: want %r got %r" % (want_status, got_status))
    for i in range(max(len(want_headers), len(got_headers))):
        a = want_headers[i] if i < len(want_headers) else "<missing>"
        b = got_headers[i] if i < len(got_headers) else "<missing>"
        if a != b:
            out.append("  header %d: want %r got %r" % (i, a, b))
    if want_body != got_body:
        out.append("  body: want %d bytes got %d bytes" % (len(want_body), len(got_body)))
        for i in range(min(len(want_body), len(got_body))):
            if want_body[i] != got_body[i]:
                out.append("  first difference at byte %d:" % i)
                out.append("    want %r" % want_body[max(0, i - 60):i + 60])
                out.append("    got  %r" % got_body[max(0, i - 60):i + 60])
                break
    return "\n".join(out)


if __name__ == "__main__":
    sys.exit(main())
