#!/usr/bin/env python3
"""Benchmark a Recibase container: start-up, memory, CPU and throughput.

  python3 bench_container.py --image <img> --name <c> --port 9101 \
      [--requests N] [--concurrency N] [--url PATH] [--env K=V]... [--out f.json]

Reports the image size, the time from `docker run -d` to the first 200 on
/health, idle and peak container memory, throughput and latency from
ApacheBench, and container CPU sampled from `docker stats` during the load.
"""
import argparse, json, os, re, socket, subprocess, sys, threading, time

AB = "/usr/sbin/ab"

def sh(cmd, **kw):
    return subprocess.run(cmd, capture_output=True, text=True, **kw).stdout.strip()

def image_size(image):
    out = sh(["docker", "images", "--format", "{{.Size}}", image])
    return out or None

def wait_ready(port, container, timeout=120.0):
    start = time.time()
    while time.time() - start < timeout:
        if sh(["docker", "inspect", "-f", "{{.State.Running}}", container]) != "true":
            return None
        try:
            with socket.create_connection(("127.0.0.1", port), timeout=0.5) as sock:
                sock.sendall(b"GET /health HTTP/1.1\r\nHost: 127.0.0.1\r\nConnection: close\r\n\r\n")
                if b"200" in sock.recv(200):
                    return time.time() - start
        except OSError:
            time.sleep(0.05)
    return None

def parse_mem(text):
    found = re.match(r"([0-9.]+)\s*([KMGiB]*)\s*/", text.strip())
    if not found:
        return None
    value, unit = float(found.group(1)), found.group(2)
    # return megabytes
    factor = {"B": 1/1024/1024, "KiB": 1/1024, "MiB": 1.0, "GiB": 1024.0, "kB": 1/1024/1024}.get(unit, 1.0)
    return value * factor

class Stats(threading.Thread):
    def __init__(self, container, interval=0.2):
        super().__init__(daemon=True)
        self.container, self.interval = container, interval
        self.samples = []
        self.stop_flag = threading.Event()

    def run(self):
        while not self.stop_flag.is_set():
            out = sh(["docker", "stats", "--no-stream",
                      "--format", "{{.CPUPerc}}|{{.MemUsage}}", self.container])
            if out and "|" in out:
                cpu, mem = out.split("|", 1)
                self.samples.append({"cpu_percent": float(cpu.strip().rstrip("%")),
                                     "mem_mb": parse_mem(mem)})
            if self.stop_flag.is_set():
                break

    def mean_cpu(self):
        values = [s["cpu_percent"] for s in self.samples]
        return sum(values) / len(values) if values else None

    def peak_mem(self):
        values = [s["mem_mb"] for s in self.samples if s["mem_mb"]]
        return max(values) if values else None

def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--image", required=True)
    ap.add_argument("--name", required=True)
    ap.add_argument("--port", type=int, default=9101)
    ap.add_argument("--url", default="/recipes/vegetable-primavera")
    ap.add_argument("--requests", type=int, default=60000)
    ap.add_argument("--concurrency", type=int, default=32)
    ap.add_argument("--idle-seconds", type=float, default=10.0)
    ap.add_argument("--env", action="append", default=[])
    ap.add_argument("--out", default=None)
    args = ap.parse_args()

    subprocess.run(["docker", "rm", "-f", args.name], capture_output=True, text=True)
    result = {"image": args.image, "container": args.name, "port": args.port,
              "url": args.url, "requests": args.requests, "concurrency": args.concurrency,
              "image_size": image_size(args.image)}
    run_cmd = ["docker", "run", "-d", "--name", args.name, "-p", f"{args.port}:8081",
               "-e", "PORT=8081"]
    for item in args.env:
        run_cmd += ["-e", item]
    run_cmd.append(args.image)
    container_id = sh(run_cmd)
    result["container_id"] = container_id[:12]
    try:
        ready = wait_ready(args.port, args.name)
        result["startup_seconds"] = ready
        if ready is None:
            result["error"] = "container never became ready"
            result["log"] = sh(["docker", "logs", "--tail", "20", args.name])
            return 1

        result["manifest"] = sh(["docker", "exec", args.name, "true"]) and None
        time.sleep(args.idle_seconds)

        idle = Stats(args.name)
        idle.start()
        time.sleep(4)
        idle.stop_flag.set()
        idle.join(timeout=15)
        result["idle"] = {"mem_mb": idle.peak_mem(), "cpu_percent": idle.mean_cpu(),
                          "samples": len(idle.samples)}

        stats = Stats(args.name)
        stats.start()
        start = time.time()
        report = subprocess.run([AB, "-k", "-c", str(args.concurrency), "-n", str(args.requests),
                                 f"http://127.0.0.1:{args.port}{args.url}"],
                                capture_output=True, text=True).stdout
        wall = time.time() - start
        stats.stop_flag.set()
        stats.join(timeout=20)

        def grab(pattern, cast=float):
            found = re.search(pattern, report)
            return cast(found.group(1)) if found else None

        rps = grab(r"Requests per second:\s+([0-9.]+)")
        result["load"] = {
            "wall_seconds": wall,
            "requests_per_second": rps,
            "mean_ms": grab(r"Time per request:\s+([0-9.]+)\s+\[ms\] \(mean\)"),
            "p50_ms": grab(r"50%\s+(\d+)", int),
            "p99_ms": grab(r"99%\s+(\d+)", int),
            "failed": grab(r"Failed requests:\s+(\d+)", int),
            "cpu_percent": stats.mean_cpu(),
            "cpu_seconds": (stats.mean_cpu() / 100.0) * wall if stats.mean_cpu() else None,
            "peak_mem_mb": stats.peak_mem(),
            "stats_samples": len(stats.samples),
        }
        if rps:
            result["load"]["cpu_ms_per_request"] = (
                (stats.mean_cpu() / 100.0) * wall * 1000 / rps / wall) if stats.mean_cpu() else None
        time.sleep(args.idle_seconds)
        after = Stats(args.name)
        after.start()
        time.sleep(4)
        after.stop_flag.set()
        after.join(timeout=15)
        result["after_load"] = {"mem_mb": after.peak_mem(), "cpu_percent": after.mean_cpu()}
    finally:
        subprocess.run(["docker", "stop", "-t", "5", args.name], capture_output=True, text=True)
        subprocess.run(["docker", "rm", "-f", args.name], capture_output=True, text=True)
    print(json.dumps(result, indent=1))
    if args.out:
        with open(args.out, "a") as fh:
            fh.write(json.dumps(result) + "\n")
    return 0

if __name__ == "__main__":
    sys.exit(main())
