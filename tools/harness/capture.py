#!/usr/bin/env python3
"""Byte-exact HTTP capture harness for the Recibase API.

Sends one request per fresh TCP connection with `Connection: close` and stores
the *raw* response bytes. Also stores a normalised copy with the two things that
belong to the deployment rather than to the application - the volatile `Date`
header and the manifest's deploy `version` (and the `Content-Length` counting
it) - replaced, so two captures taken from different builds or at different
times can be diffed.
"""
import json, os, socket, sys, re, argparse

def raw_request(host, port, path, method="GET", headers=None, body=None, timeout=30, version="HTTP/1.1"):
    """Send one request on a fresh connection and return the response bytes.

    The default is `Connection: close` so the server closes and the read ends at
    EOF. When the caller supplies its own `Connection` header (e.g. keep-alive)
    the read stops after the header block plus `Content-Length` bytes instead,
    so a kept-alive connection does not wait for a timeout.
    """
    hdrs = [f"{method} {path} {version}", f"Host: {host}:{port}",
            "User-Agent: recibase-capture", "Accept: */*"]
    has_connection = bool(headers) and any(k.lower() == "connection" for k in headers)
    if not has_connection:
        hdrs.append("Connection: close")
    if headers:
        hdrs += [f"{k}: {v}" for k, v in headers.items()]
    if body is not None:
        hdrs.append(f"Content-Length: {len(body)}")
    req = ("\r\n".join(hdrs) + "\r\n\r\n").encode() + (body or b"")
    s = socket.create_connection((host, port), timeout=timeout)
    s.settimeout(timeout)
    s.sendall(req)
    chunks = []
    buffered = b""
    content_length = None
    while True:
        if content_length is not None:
            head_end = buffered.find(b"\r\n\r\n")
            if head_end >= 0 and len(buffered) - head_end - 4 >= content_length:
                break
        try:
            chunk = s.recv(65536)
        except socket.timeout:
            break
        if not chunk:
            break
        chunks.append(chunk)
        buffered = b"".join(chunks)
        if content_length is None:
            head_end = buffered.find(b"\r\n\r\n")
            if head_end >= 0:
                for line in buffered[:head_end].split(b"\r\n"):
                    if line.lower().startswith(b"content-length:"):
                        try:
                            content_length = int(line.split(b":", 1)[1].strip())
                        except ValueError:
                            content_length = None
    s.close()
    return b"".join(chunks)

DATE_RE = re.compile(rb"^[Dd]ate:\s*[^\r\n]*\r\n", re.M)
# The manifest's `version` is the commit the server was deployed from, so it
# belongs to the deployment rather than to the application - like `Date`, it is
# normalised so captures taken from different builds can be diffed. Its length
# varies with the commit, so the `Content-Length` that counts it goes too.
VERSION_RE = re.compile(rb'"version":"[^"]*"')
MANIFEST_LENGTH_RE = re.compile(rb"Content-Length:[ \t]*\d+")

def normalise(raw):
    raw = DATE_RE.sub(b"Date: <normalised>\r\n", raw)
    if b'"base_commit_url"' in raw:
        raw = VERSION_RE.sub(b'"version":"<normalised>"', raw)
        raw = MANIFEST_LENGTH_RE.sub(b"Content-Length: <normalised>", raw)
    return raw

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
                          r.get("headers"), (r.get("body") or "").encode() or None,
                          version=r.get("version", "HTTP/1.1"))
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
