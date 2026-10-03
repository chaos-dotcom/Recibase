
#!/usr/bin/env python3
"""Before/after measurements for the frontend: start-up, RAM, CPU and latency.

  python3 bench_frontend.py --label python --command "gunicorn app:app --bind 0.0.0.0:19080" \
      --cwd /path/to/Frontend --port 19080 --env BACKEND_URL=http://localhost:8081/ \
      --requests 4000 --concurrency 16 --mode keepalive --out results.json

The same protocol is used for both implementations, so the numbers are
comparable: start the process the way an operator would, wait for the first
`200` on a page, let it settle, sample the process tree while idle, drive a
mixed HTTP load, sample during it, then settle again and sample once more.

A gunicorn master forks its worker, so RAM and CPU are summed over the whole
process tree - what the service costs the machine - rather than the pid that
was spawned. For the single-process Rust binary the tree is one process.
"""
import argparse, http.client, json, os, shlex, socket, statistics, subprocess, sys, threading, time

DEFAULT_PATHS = ["/", "/chicken-curry", "/static/styles.css", "/manifest.json", "/does-not-exist"]


def wait_ready(port, path="/", timeout=90.0):
    """Seconds until the first `200` on `path`, which is what a deploy probe sees."""
    start = time.time()
    while time.time() - start < timeout:
        try:
            with socket.create_connection(("127.0.0.1", port), timeout=0.5) as sock:
                request = "GET %s HTTP/1.1\r\nHost: 127.0.0.1\r\nConnection: close\r\n\r\n" % path
                sock.sendall(request.encode())
                if b"200" in sock.recv(200):
                    return time.time() - start
        except OSError:
            time.sleep(0.01)
    return None


def parse_cpu(text):
    seconds = 0.0
    for part in text.split(":"):
        seconds = seconds * 60 + float(part)
    return seconds


def process_table():
    out = subprocess.run(["ps", "-eo", "pid=,ppid=,rss=,time="],
                         capture_output=True, text=True).stdout
    table = {}
    for line in out.strip().splitlines():
        fields = line.split(None, 3)
        if len(fields) < 4:
            continue
        try:
            table[int(fields[0])] = (int(fields[1]), int(fields[2]), parse_cpu(fields[3]))
        except ValueError:
            continue
    return table


def tree_stats(root_pid):
    """RSS (KiB), CPU seconds and process count for `root_pid` and its descendants."""
    table = process_table()
    if root_pid not in table:
        return None
    children = {}
    for pid, (ppid, _, _) in table.items():
        children.setdefault(ppid, []).append(pid)
    seen, stack = set(), [root_pid]
    while stack:
        pid = stack.pop()
        if pid in seen or pid not in table:
            continue
        seen.add(pid)
        stack.extend(children.get(pid, []))
    return {
        "rss_kb": sum(table[pid][1] for pid in seen),
        "cpu_seconds": sum(table[pid][2] for pid in seen),
        "processes": len(seen),
    }


class Sampler(threading.Thread):
    def __init__(self, pid, interval=0.05):
        super().__init__(daemon=True)
        self.pid, self.interval = pid, interval
        self.samples = []
        self.stop_flag = threading.Event()

    def run(self):
        while not self.stop_flag.is_set():
            stats = tree_stats(self.pid)
            if stats:
                self.samples.append(stats)
            time.sleep(self.interval)

    def peak_rss_kb(self):
        return max((s["rss_kb"] for s in self.samples), default=0)


def one_request(conn, path):
    conn.request("GET", path, headers={"Host": "127.0.0.1"})
    response = conn.getresponse()
    body = response.read()
    return response.status, len(body)


def load(port, paths, total, concurrency, mode):
    latencies, statuses = [], {}
    lock = threading.Lock()
    per_thread = max(1, total // concurrency)

    def worker():
        local = []
        conn = http.client.HTTPConnection("127.0.0.1", port, timeout=30)
        for i in range(per_thread):
            path = paths[i % len(paths)]
            start = time.perf_counter()
            try:
                if mode == "close":
                    conn.close()
                    conn = http.client.HTTPConnection("127.0.0.1", port, timeout=30)
                status, _ = one_request(conn, path)
            except Exception as error:  # noqa: BLE001
                status = "error:%s" % type(error).__name__
            local.append(time.perf_counter() - start)
            with lock:
                statuses[status] = statuses.get(status, 0) + 1
        conn.close()
        with lock:
            latencies.extend(local)

    threads = [threading.Thread(target=worker) for _ in range(concurrency)]
    started = time.perf_counter()
    for thread in threads:
        thread.start()
    for thread in threads:
        thread.join()
    wall = time.perf_counter() - started
    latencies.sort()

    def percentile(fraction):
        if not latencies:
            return None
        return latencies[min(len(latencies) - 1, int(len(latencies) * fraction))] * 1000

    return {
        "requests": len(latencies),
        "wall_seconds": wall,
        "requests_per_second": len(latencies) / wall if wall else None,
        "latency_p50_ms": percentile(0.50),
        "latency_p95_ms": percentile(0.95),
        "latency_p99_ms": percentile(0.99),
        "latency_max_ms": latencies[-1] * 1000 if latencies else None,
        "statuses": {str(k): v for k, v in sorted(statuses.items(), key=lambda kv: str(kv[0]))},
    }


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--label", required=True)
    ap.add_argument("--command", required=True,
                    help="the command line to start the server; run without a shell")
    ap.add_argument("--cwd", default=None)
    ap.add_argument("--port", type=int, required=True)
    ap.add_argument("--paths", default=",".join(DEFAULT_PATHS))
    ap.add_argument("--requests", type=int, default=4000)
    ap.add_argument("--concurrency", type=int, default=16)
    ap.add_argument("--mode", default="keepalive", choices=["close", "keepalive"])
    ap.add_argument("--idle-seconds", type=float, default=15.0)
    ap.add_argument("--warmup", type=int, default=1000)
    ap.add_argument("--settle-seconds", type=float, default=4.0)
    ap.add_argument("--env", action="append", default=[])
    ap.add_argument("--out", default=None)
    args = ap.parse_args()

    env = dict(os.environ)
    env["PORT"] = str(args.port)
    for item in args.env:
        key, _, value = item.partition("=")
        env[key] = value
    paths = [path for path in args.paths.split(",") if path]

    started = time.time()
    process = subprocess.Popen(shlex.split(args.command), env=env, cwd=args.cwd,
                               stdout=open("/tmp/bench-frontend.log", "ab"),
                               stderr=subprocess.STDOUT)
    result = {"label": args.label, "command": args.command, "port": args.port,
              "mode": args.mode, "requests": args.requests, "concurrency": args.concurrency,
              "paths": paths}
    try:
        ready = wait_ready(args.port)
        result["startup_seconds"] = ready
        result["spawn_to_ready_seconds"] = time.time() - started
        if ready is None:
            result["error"] = "server never became ready"
            return 1

        time.sleep(args.idle_seconds)
        idle_before = tree_stats(process.pid)
        idle_started = time.time()
        time.sleep(args.idle_seconds)
        idle_after = tree_stats(process.pid)
        idle_elapsed = time.time() - idle_started
        result["idle"] = {
            **idle_after,
            "settle_seconds": args.idle_seconds,
            "cpu_seconds_in_window": idle_after["cpu_seconds"] - idle_before["cpu_seconds"],
            "cpu_percent_of_one_core":
                (idle_after["cpu_seconds"] - idle_before["cpu_seconds"]) / idle_elapsed * 100.0,
        }

        if args.warmup:
            result["warmup"] = load(args.port, paths, args.warmup, args.concurrency, args.mode)
            time.sleep(args.settle_seconds)
            result["after_warmup"] = tree_stats(process.pid)

        sampler = Sampler(process.pid)
        sampler.start()
        before = tree_stats(process.pid)
        result["load"] = load(args.port, paths, args.requests, args.concurrency, args.mode)
        after = tree_stats(process.pid)
        sampler.stop_flag.set()
        sampler.join(timeout=2)
        cpu_seconds = after["cpu_seconds"] - before["cpu_seconds"]
        result["load"]["server_cpu_seconds"] = cpu_seconds
        result["load"]["server_cpu_ms_per_request"] = (
            cpu_seconds * 1000 / result["load"]["requests"] if result["load"]["requests"] else None)
        result["load"]["peak_rss_kb"] = sampler.peak_rss_kb()
        result["load"]["processes_during_load"] = after["processes"]

        time.sleep(args.idle_seconds)
        result["after_load"] = {**tree_stats(process.pid), "settle_seconds": args.idle_seconds}
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
