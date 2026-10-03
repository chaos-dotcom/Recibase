
#!/usr/bin/env python3
"""Throughput and latency under ApacheBench, with the server's RSS and CPU sampled.

  python3 ab_frontend.py --label rust --command "./recibase-frontend" --port 19080 \
      --url /chicken-curry --requests 10000 --concurrency 32 --out ab.json

`ab` reports the client's view; the RSS and CPU columns are the server's own,
read from its process tree before and after the run.
"""
import argparse, json, os, re, shlex, socket, subprocess, sys, time

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from bench_frontend import tree_stats, wait_ready


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--label", required=True)
    ap.add_argument("--command", required=True)
    ap.add_argument("--cwd", default=None)
    ap.add_argument("--port", type=int, required=True)
    ap.add_argument("--url", default="/")
    ap.add_argument("--requests", type=int, default=10000)
    ap.add_argument("--concurrency", type=int, default=32)
    ap.add_argument("--env", action="append", default=[])
    ap.add_argument("--out", default=None)
    ap.add_argument("--ab", default="ab")
    args = ap.parse_args()

    env = dict(os.environ)
    env["PORT"] = str(args.port)
    for item in args.env:
        key, _, value = item.partition("=")
        env[key] = value

    process = subprocess.Popen(shlex.split(args.command), env=env, cwd=args.cwd,
                               stdout=open("/tmp/ab-frontend.log", "ab"),
                               stderr=subprocess.STDOUT)
    result = {"label": args.label, "url": args.url,
              "requests": args.requests, "concurrency": args.concurrency}
    try:
        if wait_ready(args.port) is None:
            result["error"] = "server never became ready"
            return 1
        time.sleep(3)
        before = tree_stats(process.pid)
        completed = subprocess.run(
            [args.ab, "-k", "-c", str(args.concurrency), "-n", str(args.requests),
             "http://127.0.0.1:%d%s" % (args.port, args.url)],
            capture_output=True, text=True)
        after = tree_stats(process.pid)
        report = completed.stdout
        result["ab_stderr"] = completed.stderr.strip() or None

        def grab(pattern, cast=float):
            found = re.search(pattern, report)
            return cast(found.group(1)) if found else None

        result.update({
            "server_rss_before_kb": before["rss_kb"] if before else None,
            "server_rss_after_kb": after["rss_kb"] if after else None,
            "server_cpu_seconds": (after["cpu_seconds"] - before["cpu_seconds"])
                if before and after else None,
            "requests_per_second": grab(r"Requests per second:\s+([0-9.]+)"),
            "time_per_request_ms": grab(r"Time per request:\s+([0-9.]+)\s+\[ms\] \(mean\)"),
            "failed_requests": grab(r"Failed requests:\s+(\d+)", int),
            "non_2xx": grab(r"Non-2xx responses:\s+(\d+)", int),
            "ab_p50_ms": grab(r"50%\s+(\d+)", int),
            "ab_p95_ms": grab(r"95%\s+(\d+)", int),
            "ab_p99_ms": grab(r"99%\s+(\d+)", int),
            "ab_p100_ms": grab(r"100%\s+(\d+)", int),
        })
        handled = grab(r"Complete requests:\s+(\d+)", int) or 0
        if result["server_cpu_seconds"] and handled:
            result["server_cpu_ms_per_request"] = result["server_cpu_seconds"] * 1000 / handled
            result["requests_per_server_cpu_second"] = handled / result["server_cpu_seconds"]
    finally:
        process.terminate()
        try:
            process.wait(timeout=15)
        except subprocess.TimeoutExpired:
            process.kill()
    print(json.dumps(result, indent=1))
    if args.out:
        with open(args.out, "a") as fh:
            fh.write(json.dumps(result) + "\n")
    return 0


if __name__ == "__main__":
    sys.exit(main())
