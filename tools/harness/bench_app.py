#!/usr/bin/env python3
"""Before/after application measurements: RAM, CPU, size, start-up, throughput.

  python3 bench_app.py --kind scala|rust --binary <path> [--port N] [--requests N]
                       [--concurrency N] [--mode close|keepalive]
                       [--env K=V]... [--label NAME] [--out results.json]

Starts the server exactly as an operator would, waits for /health, settles,
samples RSS/CPU/threads while idle, drives a mixed HTTP load, samples again, then
stops the process and reads its resource usage (ru_maxrss, ru_utime, ru_stime).
"""
import argparse, http.client, json, os, socket, statistics, subprocess, sys, threading, time

PATHS = ["/health", "/recipes/", "/recipes/vegetable-primavera", "/meals/"]

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

def ps_fields(pid):
    out = subprocess.run(
        ["ps", "-o", "rss=,vsz=,time=", "-p", str(pid)],
        capture_output=True, text=True).stdout.split()
    if len(out) < 3:
        return None
    rss_kb, vsz_kb, cpu = int(out[0]), int(out[1]), out[2]
    return {"rss_kb": rss_kb, "vsz_kb": vsz_kb, "cpu_seconds": parse_cpu(cpu)}

def parse_cpu(text):
    parts = text.split(":")
    parts = [float(p) for p in parts]
    seconds = 0.0
    for part in parts:
        seconds = seconds * 60 + part
    return seconds

def thread_count(pid):
    out = subprocess.run(["ps", "-M", str(pid)], capture_output=True, text=True).stdout
    return max(0, len(out.strip().splitlines()) - 1)

class Sampler(threading.Thread):
    def __init__(self, pid, interval=0.05):
        super().__init__(daemon=True)
        self.pid, self.interval = pid, interval
        self.samples = []
        self.stop_flag = threading.Event()

    def run(self):
        while not self.stop_flag.is_set():
            fields = ps_fields(self.pid)
            if fields:
                self.samples.append(fields)
            time.sleep(self.interval)

    def peak_rss_kb(self):
        return max((s["rss_kb"] for s in self.samples), default=0)

    def last(self):
        return self.samples[-1] if self.samples else None

def one_request(conn, path):
    conn.request("GET", path, headers={"Host": "127.0.0.1"})
    response = conn.getresponse()
    body = response.read()
    return response.status, len(body)

def load(port, total, concurrency, mode):
    latencies = []
    statuses = {}
    lock = threading.Lock()
    per_thread = total // concurrency

    def worker():
        local = []
        conn = http.client.HTTPConnection("127.0.0.1", port, timeout=30)
        for i in range(per_thread):
            path = PATHS[i % len(PATHS)]
            start = time.perf_counter()
            try:
                if mode == "close":
                    conn.close()
                    conn = http.client.HTTPConnection("127.0.0.1", port, timeout=30)
                status, _ = one_request(conn, path)
            except Exception as error:  # noqa: BLE001
                status = f"error:{type(error).__name__}"
            elapsed = time.perf_counter() - start
            local.append(elapsed)
            with lock:
                statuses[status] = statuses.get(status, 0) + 1
        conn.close()
        with lock:
            latencies.extend(local)

    threads = [threading.Thread(target=worker) for _ in range(concurrency)]
    start = time.perf_counter()
    for t in threads: t.start()
    for t in threads: t.join()
    wall = time.perf_counter() - start
    latencies.sort()
    def pct(p):
        if not latencies: return None
        return latencies[min(len(latencies) - 1, int(len(latencies) * p))]
    return {
        "requests": len(latencies), "wall_seconds": wall,
        "requests_per_second": len(latencies) / wall if wall else None,
        "latency_p50_ms": pct(0.50) * 1000 if latencies else None,
        "latency_p95_ms": pct(0.95) * 1000 if latencies else None,
        "latency_max_ms": latencies[-1] * 1000 if latencies else None,
        "statuses": {str(k): v for k, v in statuses.items()},
    }

def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--kind", required=True)
    ap.add_argument("--binary", required=True)
    ap.add_argument("--port", type=int, default=19081)
    ap.add_argument("--requests", type=int, default=4000)
    ap.add_argument("--concurrency", type=int, default=16)
    ap.add_argument("--mode", default="close", choices=["close", "keepalive"])
    ap.add_argument("--idle-seconds", type=float, default=12.0)
    ap.add_argument("--warmup", type=int, default=1000)
    ap.add_argument("--settle-seconds", type=float, default=4.0)
    ap.add_argument("--env", action="append", default=[])
    ap.add_argument("--label", default=None)
    ap.add_argument("--out", default=None)
    args = ap.parse_args()

    env = dict(os.environ)
    env["PORT"] = str(args.port)
    for item in args.env:
        key, _, value = item.partition("=")
        env[key] = value

    started = time.time()
    process = subprocess.Popen([args.binary], env=env,
                               stdout=open("/tmp/bench-server.log", "ab"),
                               stderr=subprocess.STDOUT)
    result = {"kind": args.kind, "label": args.label or args.kind, "binary": args.binary,
              "port": args.port, "mode": args.mode, "requests": args.requests,
              "concurrency": args.concurrency}
    try:
        ready = wait_ready(args.port)
        result["startup_seconds"] = ready
        if ready is None:
            result["error"] = "server never became ready"
            return 1
        result["spawn_to_ready_seconds"] = time.time() - started

        time.sleep(args.idle_seconds)
        idle_before = ps_fields(process.pid)
        idle_start = time.time()
        time.sleep(args.idle_seconds)
        idle_after = ps_fields(process.pid)
        idle_elapsed = time.time() - idle_start
        result["idle"] = {
            **idle_after,
            "threads": thread_count(process.pid),
            "settle_seconds": args.idle_seconds,
            "cpu_seconds_in_window": idle_after["cpu_seconds"] - idle_before["cpu_seconds"],
            "cpu_percent_of_one_core": (idle_after["cpu_seconds"] - idle_before["cpu_seconds"])
            / idle_elapsed * 100.0,
        }

        if args.warmup:
            result["warmup"] = load(args.port, args.warmup, args.concurrency, args.mode)
            time.sleep(args.settle_seconds)
            warm = ps_fields(process.pid)
            result["after_warmup"] = {**warm, "threads": thread_count(process.pid)}

        sampler = Sampler(process.pid)
        sampler.start()
        before = ps_fields(process.pid)
        result["load"] = load(args.port, args.requests, args.concurrency, args.mode)
        after = ps_fields(process.pid)
        sampler.stop_flag.set()
        sampler.join(timeout=2)
        cpu_seconds = after["cpu_seconds"] - before["cpu_seconds"]
        result["load"]["server_cpu_seconds"] = cpu_seconds
        result["load"]["server_cpu_ms_per_request"] = (
            cpu_seconds * 1000 / result["load"]["requests"] if result["load"]["requests"] else None)
        result["load"]["peak_rss_kb"] = sampler.peak_rss_kb()
        result["load"]["threads"] = thread_count(process.pid)

        time.sleep(args.idle_seconds)
        post = ps_fields(process.pid)
        result["after_load"] = {**post, "threads": thread_count(process.pid),
                                "settle_seconds": args.idle_seconds}
    finally:
        process.terminate()
        try:
            pid, status, rusage = os.wait4(process.pid, 0)
            result["rusage"] = {
                "max_rss_kb": rusage.ru_maxrss // 1024 if rusage.ru_maxrss > 10**7 else rusage.ru_maxrss,
                "user_seconds": rusage.ru_utime,
                "system_seconds": rusage.ru_stime,
                "exit_status": os.waitstatus_to_exitcode(status),
            }
        except ChildProcessError:
            pass

    print(json.dumps(result, indent=1))
    if args.out:
        with open(args.out, "a") as fh:
            fh.write(json.dumps(result) + "\n")
    return 0

if __name__ == "__main__":
    sys.exit(main())
