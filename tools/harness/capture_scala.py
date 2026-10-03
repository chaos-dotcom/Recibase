#!/usr/bin/env python3
"""Regenerate a `capture-scala*` ground-truth directory from the real Scala server.

`verify_port.py` starts the *Rust* binary, which logs little, so it can keep the
child's stdout in a pipe. The Scala server instead logs every request
(`Logger.httpApp(true, true)`) and never exits on its own; a pipe nobody drains
fills up and blocks the JVM mid-capture. This driver sends the server's output to
a file, waits for the port, runs `capture.py`, then stops it.

Build the server first (JDK + sbt 2.x, per `project/build.properties`):

    sbt -batch stage

then, from this directory:

    python3 capture_scala.py \
        --binary <Recibase>/target/out/jvm/scala-2.13.18/recibase/universal/stage/bin/recibase \
        --requests requests.json --out capture-scala

The manifest's `version` field comes from the *deploy* environment
(`GIT_COMMIT` / `SOURCE_COMMIT` / `GITHUB_SHA`); an unset environment yields
`{"version":"latest",...}`. The harness normalises that field like `Date`
(see `capture.py`), so it need not be pinned to match the committed captures.
Add `--env MEAL_LOG_CSV_URL=file:///.../meal-log.csv` for the csv set, or the
`RECIPE_SUBMIT_PASSCODE` / `GITHUB_TOKEN` / `TURNSTILE_SECRET` /
`TURNSTILE_HOSTNAMES` quartet for the submit set.

Compare the result with what is committed:

    python3 diff_captures.py capture-scala <out>
"""
import argparse
import os
import socket
import subprocess
import sys
import time

HERE = os.path.dirname(os.path.abspath(__file__))


def free_port():
    s = socket.socket()
    s.bind(("127.0.0.1", 0))
    port = s.getsockname()[1]
    s.close()
    return port


def wait_for(port, timeout=90):
    deadline = time.time() + timeout
    while time.time() < deadline:
        try:
            socket.create_connection(("127.0.0.1", port), timeout=0.5).close()
            return True
        except OSError:
            time.sleep(0.2)
    return False


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--binary", required=True, help="the staged Scala launcher")
    ap.add_argument("--out", required=True, help="capture directory to write")
    ap.add_argument("--requests", default=os.path.join(HERE, "requests.json"))
    ap.add_argument("--env", action="append", default=[])
    args = ap.parse_args()

    port = free_port()
    env = dict(os.environ)
    env["PORT"] = str(port)
    for item in args.env:
        key, _, value = item.partition("=")
        env[key] = value

    os.makedirs(args.out, exist_ok=True)
    log_path = os.path.join(args.out, "server.log")
    with open(log_path, "wb") as log:
        server = subprocess.Popen(
            [args.binary], env=env, stdout=log, stderr=subprocess.STDOUT
        )
        try:
            if not wait_for(port):
                sys.stderr.write("server did not start; tail of log:\n")
                sys.stderr.write(open(log_path, errors="replace").read()[-4000:])
                return 2
            subprocess.run(
                [
                    sys.executable,
                    os.path.join(HERE, "capture.py"),
                    "--port", str(port),
                    "--out", args.out,
                    "--requests", args.requests,
                ],
                check=True,
            )
        finally:
            server.terminate()
            try:
                server.wait(timeout=15)
            except subprocess.TimeoutExpired:
                server.kill()
    return 0


if __name__ == "__main__":
    sys.exit(main())
