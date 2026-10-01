#!/usr/bin/env python3
"""High-pressure throughput with ApacheBench while sampling the server's RSS/CPU.

  python3 ab_run.py --kind scala|rust --binary <path> --port N --url PATH \
      [--requests N] [--concurrency N] [--env K=V]... --label NAME --out results.json
"""
import argparse, json, os, re, socket, subprocess, sys, time

def wait_ready(port, timeout=90.0):
    start = time.time()
    while time.time() - start < timeout:
        try:
            with socket.create_connection(("127.0.0.1", port), timeout=0.5) as sock:
                sock.sendall(b"GET /health HTTP/1.1\r\nHost: 127.0.0.1\r\nConnection: close\r\n\r\n")
                if b"200" in sock.recv(200):
                    return time.time() - start
        except OSError:
            time.sleep(0.02)
    return None

def cpu_seconds(pid):
    out = subprocess.run(["ps", "-o", "time=", "-p", str(pid)], capture_output=True, text=True).stdout.strip()
    if not out:
        return None
    seconds = 0.0
    for part in out.split(":"):
        seconds = seconds * 60 + float(part)
    return seconds

def rss_kb(pid):
    out = subprocess.run(["ps", "-o", "rss=", "-p", str(pid)], capture_output=True, text=True).stdout.strip()
    return int(out) if out else None

def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--kind", required=True)
    ap.add_argument("--binary", required=True)
    ap.add_argument("--port", type=int, default=19081)
    ap.add_argument("--url", default="/recipes/")
    ap.add_argument("--requests", type=int, default=30000)
    ap.add_argument("--concurrency", type=int, default=32)
    ap.add_argument("--env", action="append", default=[])
    ap.add_argument("--label", required=True)
    ap.add_argument("--out", default=None)
    ap.add_argument("--ab", default="/usr/sbin/ab")
    args = ap.parse_args()

    env = dict(os.environ)
    env["PORT"] = str(args.port)
    for item in args.env:
        key, _, value = item.partition("=")
        env[key] = value

    process = subprocess.Popen([args.binary], env=env,
                               stdout=open("/tmp/ab-server.log", "ab"), stderr=subprocess.STDOUT)
    result = {"kind": args.kind, "label": args.label, "url": args.url,
              "requests": args.requests, "concurrency": args.concurrency}
    try:
        if wait_ready(args.port) is None:
            result["error"] = "server never became ready"
            return 1
        time.sleep(3)
        cpu_before, rss_before = cpu_seconds(process.pid), rss_kb(process.pid)
        start = time.time()
        completed = subprocess.run(
            [args.ab, "-k", "-c", str(args.concurrency), "-n", str(args.requests),
             f"http://127.0.0.1:{args.port}{args.url}"],
            capture_output=True, text=True)
        wall = time.time() - start
        cpu_after, rss_after = cpu_seconds(process.pid), rss_kb(process.pid)
        report = completed.stdout
        def grab(pattern, cast=float):
            found = re.search(pattern, report)
            return cast(found.group(1)) if found else None
        result.update({
            "ab_wall_seconds": wall,
            "server_cpu_seconds": (cpu_after - cpu_before) if None not in (cpu_after, cpu_before) else None,
            "server_rss_before_kb": rss_before,
            "server_rss_after_kb": rss_after,
            "requests_per_second": grab(r"Requests per second:\s+([0-9.]+)"),
            "time_per_request_ms": grab(r"Time per request:\s+([0-9.]+)\s+\[ms\] \(mean\)"),
            "failed_requests": grab(r"Failed requests:\s+(\d+)", int),
            "non_2xx": grab(r"Non-2xx responses:\s+(\d+)", int),
            "transfer_rate_kbps": grab(r"Transfer rate:\s+([0-9.]+)"),
            "ab_p50_ms": grab(r"50%\s+(\d+)", int),
            "ab_p95_ms": grab(r"95%\s+(\d+)", int),
            "ab_p99_ms": grab(r"99%\s+(\d+)", int),
        })
        cpu_line = re.search(r"CPU time:\s+user ([0-9.]+)", report)
        result["ab_client_cpu_user_seconds"] = float(cpu_line.group(1)) if cpu_line else None
        if result["server_cpu_seconds"] and result["requests_per_second"]:
            handled = grab(r"Complete requests:\s+(\d+)", int) or 0
            if handled:
                result["server_cpu_ms_per_request"] = result["server_cpu_seconds"] * 1000 / handled
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
