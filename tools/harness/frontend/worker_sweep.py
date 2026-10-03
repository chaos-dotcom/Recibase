#!/usr/bin/env python3
"""How many gunicorn workers would it take to match the Rust server, and what
would they cost?

  python3 worker_sweep.py --workers 1,2,4,6,8,10,12,16,20 --url / \
      --command "/path/to/.venv/bin/gunicorn app:app --bind 0.0.0.0:19080" \
      --cwd /path/to/Frontend --port 19080 --requests 30000 --concurrency 32 \
      --out worker-sweep.json

For each worker count it starts gunicorn, lets it settle, records the process
tree (RSS, processes, CPU), runs ApacheBench against one URL, and records the
tree again. The Rust server is measured the same way with `--workers 0`, which
runs `--command` unchanged, so the two columns of the table come from one
protocol.
"""
import argparse, json, os, shlex, shutil, subprocess, sys, tempfile, time

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from bench_frontend import tree_stats, wait_ready


def run_one(args, workers):
    command = args.command
    if workers:
        command = command + " --workers %d" % workers
    env = dict(os.environ)
    env["PORT"] = str(args.port)
    process = subprocess.Popen(shlex.split(command), cwd=args.cwd, env=env,
                               stdout=open("/tmp/worker-sweep.log", "ab"),
                               stderr=subprocess.STDOUT)
    result = {"workers": workers, "command": command, "url": args.url,
              "requests": args.requests, "concurrency": args.concurrency}
    try:
        if wait_ready(args.port) is None:
            result["error"] = "never became ready"
            return result
        time.sleep(args.settle)
        idle = tree_stats(process.pid)
        result["idle_rss_kb"] = idle["rss_kb"] if idle else None
        result["processes"] = idle["processes"] if idle else None
        before = tree_stats(process.pid)
        flags = ["-k"] if args.keep_alive else []
        completed = subprocess.run(
            [args.ab] + flags + ["-c", str(args.concurrency), "-n", str(args.requests),
             "http://127.0.0.1:%d%s" % (args.port, args.url)],
            capture_output=True, text=True)
        if completed.returncode != 0 and not completed.stdout.endswith("\n"):
            result["ab_failed"] = completed.stderr.strip().splitlines()[-1:] or ["ab failed"]
        after = tree_stats(process.pid)
        import re
        def grab(pattern, cast=float):
            found = re.search(pattern, completed.stdout)
            return cast(found.group(1)) if found else None
        handled = grab(r"Complete requests:\s+(\d+)", int) or 0
        cpu = (after["cpu_seconds"] - before["cpu_seconds"]) if before and after else None
        result.update({
            "requests_per_second": grab(r"Requests per second:\s+([0-9.]+)"),
            "server_cpu_seconds": cpu,
            "server_cpu_ms_per_request": cpu * 1000 / handled if cpu and handled else None,
            "cpu_cores_busy": cpu / (handled / (grab(r"Requests per second:\s+([0-9.]+)") or 1))
                              if cpu and handled else None,
            "peak_rss_kb": after["rss_kb"] if after else None,
            "ab_p50_ms": grab(r"50%\s+(\d+)", int),
            "ab_p95_ms": grab(r"95%\s+(\d+)", int),
            "failed_requests": grab(r"Failed requests:\s+(\d+)", int),
        })
        print(json.dumps(result), flush=True)
        return result
    finally:
        process.terminate()
        try:
            process.wait(timeout=20)
        except subprocess.TimeoutExpired:
            process.kill()


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--workers", default="1,2,4,6,8,10,12,16,20",
                    help="comma-separated worker counts; use 0 for the command as given")
    ap.add_argument("--command", required=True)
    ap.add_argument("--cwd", default=None)
    ap.add_argument("--port", type=int, required=True)
    ap.add_argument("--url", default="/")
    ap.add_argument("--requests", type=int, default=30000)
    ap.add_argument("--concurrency", type=int, default=32)
    ap.add_argument("--settle", type=float, default=3.0)
    ap.add_argument("--ab", default="ab")
    ap.add_argument("--keep-alive", dest="keep_alive", action="store_true", default=True,
                    help="pass -k to ab (the default)")
    ap.add_argument("--no-keep-alive", dest="keep_alive", action="store_false",
                    help="one TCP connection per request, which is all the Python stack allows")
    ap.add_argument("--out", default=None)
    ap.add_argument("--label", default=None)
    args = ap.parse_args()

    results = []
    for text in args.workers.split(","):
        workers = int(text)
        results.append(run_one(args, workers))
        time.sleep(1.0)
    if args.out:
        for result in results:
            result["label"] = args.label
            with open(args.out, "a") as fh:
                fh.write(json.dumps(result) + "\n")
    print("%d configurations" % len(results))
    return 0


if __name__ == "__main__":
    sys.exit(main())
