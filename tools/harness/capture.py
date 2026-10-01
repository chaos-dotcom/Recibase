#!/usr/bin/env python3
"""Byte-exact HTTP capture harness for the Recibase API.

Sends one request per fresh TCP connection with `Connection: close` and stores
the *raw* response bytes. Also stores a normalised copy with the volatile
Date header replaced, so two captures taken at different times can be diffed.
"""
import json, os, socket, sys, re, argparse

def raw_request(host, port, path, method="GET", headers=None, body=None, timeout=30):
    hdrs = [f"{method} {path} HTTP/1.1", f"Host: {host}:{port}",
            "User-Agent: recibase-capture", "Accept: */*", "Connection: close"]
    if headers:
        hdrs += [f"{k}: {v}" for k, v in headers.items()]
    if body is not None:
        hdrs.append(f"Content-Length: {len(body)}")
    req = ("\r\n".join(hdrs) + "\r\n\r\n").encode() + (body or b"")
    s = socket.create_connection((host, port), timeout=timeout)
    s.settimeout(timeout)
    s.sendall(req)
    chunks = []
    while True:
        try:
            chunk = s.recv(65536)
        except socket.timeout:
            break
        if not chunk:
            break
        chunks.append(chunk)
    s.close()
    return b"".join(chunks)

DATE_RE = re.compile(rb"^[Dd]ate:\s*[^\r\n]*\r\n", re.M)

def normalise(raw):
    return DATE_RE.sub(b"Date: <normalised>\r\n", raw)

def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--port", type=int, required=True)
    ap.add_argument("--host", default="127.0.0.1")
    ap.add_argument("--out", required=True)
    ap.add_argument("--requests", required=True, help="json file: list of {name,path,method,headers,body}")
    args = ap.parse_args()

    os.makedirs(args.out, exist_ok=True)
    reqs = json.load(open(args.requests))
    index = []
    for i, r in enumerate(reqs):
        name = r["name"]
        raw = raw_request(args.host, args.port, r["path"], r.get("method", "GET"),
                          r.get("headers"), (r.get("body") or "").encode() or None)
        with open(os.path.join(args.out, f"{i:03d}_{name}.raw"), "wb") as fh:
            fh.write(raw)
        with open(os.path.join(args.out, f"{i:03d}_{name}.norm"), "wb") as fh:
            fh.write(normalise(raw))
        head = raw.split(b"\r\n\r\n", 1)[0].decode("latin1")
        status = head.split("\r\n")[0]
        index.append({"i": i, "name": name, "path": r["path"], "status": status,
                      "bytes": len(raw)})
        print(f"{i:03d} {status:24s} {len(raw):7d}  {name}")
    json.dump(index, open(os.path.join(args.out, "index.json"), "w"), indent=1)

if __name__ == "__main__":
    main()
