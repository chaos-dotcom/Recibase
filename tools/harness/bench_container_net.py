#!/usr/bin/env python3
"""Benchmark a Recibase container over the Docker network (no macOS port proxy).

The load generator runs *inside* the Docker VM (ab from httpd:2-alpine on the
same bridge network), so the measurement is not capped by Docker Desktop's host
port forwarder. Container CPU comes from the VM's own cgroup tree
(/sys/fs/cgroup/docker/<id>/cpu.stat) read by a privileged helper, which works
even for a `scratch` image with no shell.

  python3 bench_container_net.py --image <img> --name <c> [--requests N] ...
"""
import argparse, json, os, re, subprocess, sys, time

NETWORK = "rb-bench"
CLIENT = "rb-ab-client"

def sh(cmd, **kw):
    return subprocess.run(cmd, capture_output=True, text=True, **kw).stdout.strip()

def ensure_network_and_client():
    if not sh(["docker", "network", "ls", "--format", "{{.Name}}", "--filter", f"name=^{NETWORK}$"]):
        sh(["docker", "network", "create", NETWORK])
    if sh(["docker", "inspect", "-f", "{{.State.Running}}", CLIENT]) != "true":
        sh(["docker", "rm", "-f", CLIENT])
        sh(["docker", "run", "-d", "--name", CLIENT, "--network", NETWORK,
            "httpd:2-alpine", "sleep", "3600"])

def from_container(url):
    return sh(["docker", "exec", CLIENT, "wget", "-qO-", "--timeout=2", url])

def wait_ready(timeout=180.0):
    start = time.time()
    while time.time() - start < timeout:
        if "ok" in from_container("http://srv:8081/health"):
            return time.time() - start
        time.sleep(0.05)
    return None

def cgroup_stat(container_id):
    out = sh(["docker", "run", "--rm", "--privileged", "-v", "/sys/fs/cgroup:/host:ro",
              "alpine", "cat", f"/host/docker/{container_id}/cpu.stat"])
    values = {}
    for line in out.splitlines():
        parts = line.split()
        if len(parts) == 2:
            values[parts[0]] = int(parts[1])
    return values or None

def mem_mb(container):
    out = sh(["docker", "stats", "--no-stream", "--format", "{{.MemUsage}}", container])
    found = re.match(r"([0-9.]+)\s*([KMGiB]*)", out.strip())
    if not found:
        return None
    value, unit = float(found.group(1)), found.group(2)
    return value * {"MiB": 1.0, "GiB": 1024.0, "KiB": 1 / 1024}.get(unit, 1.0)

def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--image", required=True)
    ap.add_argument("--name", required=True)
    ap.add_argument("--url", default="/recipes/vegetable-primavera")
    ap.add_argument("--requests", type=int, default=50000)
    ap.add_argument("--concurrency", type=int, default=32)
    ap.add_argument("--idle-seconds", type=float, default=10.0)
    ap.add_argument("--env", action="append", default=[])
    ap.add_argument("--out", default=None)
    args = ap.parse_args()

    ensure_network_and_client()
    sh(["docker", "rm", "-f", args.name])
    result = {"image": args.image, "container": args.name, "url": args.url,
              "requests": args.requests, "concurrency": args.concurrency,
              "image_size": sh(["docker", "images", "--format", "{{.Size}}", args.image]) or None}
    run_cmd = ["docker", "run", "-d", "--name", args.name, "--network", NETWORK,
               "--network-alias", "srv", "-e", "PORT=8081"]
    for item in args.env:
        run_cmd += ["-e", item]
    run_cmd.append(args.image)
    result["container_id"] = sh(run_cmd)[:12]
    full_id = sh(["docker", "inspect", "-f", "{{.Id}}", args.name])
    try:
        ready = wait_ready()
        result["startup_seconds"] = ready
        if ready is None:
            result["error"] = "never became ready"
            result["log"] = sh(["docker", "logs", "--tail", "20", args.name])
            return 1

        time.sleep(args.idle_seconds)
        idle_cpu = cgroup_stat(full_id)
        result["idle"] = {"mem_mb": mem_mb(args.name)}
        time.sleep(args.idle_seconds)
        idle_cpu_2 = cgroup_stat(full_id)
        if idle_cpu and idle_cpu_2:
            delta = (idle_cpu_2["usage_usec"] - idle_cpu["usage_usec"]) / 1e6
            result["idle"]["window_seconds"] = args.idle_seconds
            result["idle"]["cpu_seconds"] = delta
            result["idle"]["cpu_percent_of_one_core"] = delta / args.idle_seconds * 100

        before = cgroup_stat(full_id)
        start = time.time()
        report = sh(["docker", "exec", CLIENT, "/usr/local/apache2/bin/ab", "-k",
                     "-c", str(args.concurrency), "-n", str(args.requests),
                     f"http://srv:8081{args.url}"])
        wall = time.time() - start
        after = cgroup_stat(full_id)

        def grab(pattern, cast=float):
            found = re.search(pattern, report)
            return cast(found.group(1)) if found else None

        cpu_seconds = (after["usage_usec"] - before["usage_usec"]) / 1e6 if before and after else None
        rps = grab(r"Requests per second:\s+([0-9.]+)")
        result["load"] = {
            "wall_seconds": wall,
            "requests_per_second": rps,
            "mean_ms": grab(r"Time per request:\s+([0-9.]+)\s+\[ms\] \(mean\)"),
            "p50_ms": grab(r"50%\s+(\d+)", int),
            "p95_ms": grab(r"95%\s+(\d+)", int),
            "p99_ms": grab(r"99%\s+(\d+)", int),
            "complete": grab(r"Complete requests:\s+(\d+)", int),
            "failed": grab(r"Failed requests:\s+(\d+)", int),
            "user_seconds": (after["user_usec"] - before["user_usec"]) / 1e6 if before and after else None,
            "system_seconds": (after["system_usec"] - before["system_usec"]) / 1e6 if before and after else None,
            "cpu_seconds": cpu_seconds,
            "cores_busy": (cpu_seconds / wall) if cpu_seconds and wall else None,
            "peak_mem_mb": mem_mb(args.name),
        }
        if cpu_seconds and result["load"]["complete"]:
            result["load"]["cpu_ms_per_request"] = cpu_seconds * 1000 / result["load"]["complete"]
        time.sleep(args.idle_seconds)
        result["after_load"] = {"mem_mb": mem_mb(args.name)}
    finally:
        sh(["docker", "stop", "-t", "5", args.name])
        sh(["docker", "rm", "-f", args.name])
    print(json.dumps(result, indent=1))
    if args.out:
        with open(args.out, "a") as fh:
            fh.write(json.dumps(result) + "\n")
    return 0

if __name__ == "__main__":
    sys.exit(main())
