#!/usr/bin/env python3
"""Final container-to-container comparison: Scala vs Rust, same client, same path.

Load runs inside the Docker VM (ab from httpd:2-alpine on the same bridge), so
Docker Desktop's host port forwarder is not in the path. Container CPU comes from
the container's own cgroup read through `docker exec` (exact), memory from
`docker stats` sampled during the load.
"""
import json, re, subprocess, sys, threading, time

CLIENT = "rb-ab-client"
NETWORK = "rb-bench"
IMAGES = [
    ("scala-published-amd64", "ghcr.io/the-silverwood-institute/recibase:latest", True, "emulated amd64"),
    ("scala-local-arm64", "recibase:latest", True, "native arm64"),
    ("rust-glibc-slim", "recibase-rust-gnu:slim", True, "native arm64"),
    ("rust-musl-slim", "recibase-rust:slim", True, "native arm64"),
    ("rust-musl-scratch", "recibase-rust-musl:minimal", False, "native arm64, no shell"),
]
LOADS = [("/recipes/vegetable-primavera", 20000), ("/meals/", 10000)]

def sh(cmd):
    return subprocess.run(cmd, capture_output=True, text=True).stdout.strip()

def mem_mb(container):
    out = sh(["docker", "stats", "--no-stream", "--format", "{{.MemUsage}}", container])
    found = re.match(r"([0-9.]+)\s*([KMGiB]*)", out.strip())
    if not found:
        return None
    value, unit = float(found.group(1)), found.group(2)
    return value * {"MiB": 1.0, "GiB": 1024.0, "KiB": 1 / 1024}.get(unit, 1.0)

def cpu_micros(container, has_shell):
    if not has_shell:
        return None
    out = sh(["docker", "exec", container, "cat", "/sys/fs/cgroup/cpu.stat"])
    for line in out.splitlines():
        parts = line.split()
        if len(parts) == 2 and parts[0] == "usage_usec":
            return int(parts[1])
    return None

class Mem(threading.Thread):
    def __init__(self, container):
        super().__init__(daemon=True)
        self.container, self.peak, self.stop_flag = container, None, threading.Event()

    def run(self):
        while not self.stop_flag.is_set():
            value = mem_mb(self.container)
            if value and (self.peak is None or value > self.peak):
                self.peak = value

def wait_ready(timeout=180.0):
    start = time.time()
    while time.time() - start < timeout:
        if "ok" in sh(["docker", "exec", CLIENT, "wget", "-qO-", "--timeout=2",
                       "http://srv:8081/health"]):
            return time.time() - start
        time.sleep(0.05)
    return None

def main():
    results = []
    for label, image, has_shell, note in IMAGES:
        sh(["docker", "rm", "-f", "srv"])
        sh(["docker", "run", "-d", "--name", "srv", "--network", NETWORK,
            "--network-alias", "srv", "-e", "PORT=8081", image])
        entry = {"label": label, "image": image, "note": note,
                 "image_size": sh(["docker", "images", "--format", "{{.Size}}", image])}
        try:
            ready = wait_ready()
            entry["startup_seconds"] = ready
            if ready is None:
                entry["error"] = "never became ready"
                print(json.dumps(entry))
                results.append(entry)
                continue
            time.sleep(8)
            entry["idle_mem_mb"] = mem_mb("srv")
            entry["loads"] = {}
            for url, count in LOADS:
                cpu_before = cpu_micros("srv", has_shell)
                sampler = Mem("srv")
                sampler.start()
                start = time.time()
                report = sh(["docker", "exec", CLIENT, "/usr/local/apache2/bin/ab", "-k",
                             "-c", "32", "-n", str(count), f"http://srv:8081{url}"])
                wall = time.time() - start
                sampler.stop_flag.set()
                sampler.join(timeout=5)
                cpu_after = cpu_micros("srv", has_shell)
                def grab(pattern, cast=float):
                    found = re.search(pattern, report)
                    return cast(found.group(1)) if found else None
                cpu_seconds = ((cpu_after - cpu_before) / 1e6
                               if cpu_before is not None and cpu_after is not None else None)
                entry["loads"][url] = {
                    "requests": count,
                    "rps": grab(r"Requests per second:\s+([0-9.]+)"),
                    "p50_ms": grab(r"50%\s+(\d+)", int),
                    "p99_ms": grab(r"99%\s+(\d+)", int),
                    "failed": grab(r"Failed requests:\s+(\d+)", int),
                    "cpu_seconds": cpu_seconds,
                    "cores_busy": (cpu_seconds / wall) if cpu_seconds else None,
                    "cpu_ms_per_request": (cpu_seconds * 1000 / count) if cpu_seconds else None,
                    "peak_mem_mb": sampler.peak,
                }
            time.sleep(5)
            entry["after_load_mem_mb"] = mem_mb("srv")
        finally:
            sh(["docker", "stop", "-t", "5", "srv"])
            sh(["docker", "rm", "-f", "srv"])
        print(json.dumps(entry))
        results.append(entry)
    with open("/Users/chaos/recibase-work/container-final.json", "w") as fh:
        json.dump(results, fh, indent=1)

if __name__ == "__main__":
    main()
