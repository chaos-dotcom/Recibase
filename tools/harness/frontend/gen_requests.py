
#!/usr/bin/env python3
"""Builds the request list used to capture the frontend.

Usage: gen_requests.py <backend-url> <out.json> [--permalinks-from url]

The list is deliberately wide: every recipe at five scale factors, the
homepage, the sitemap, the contributor form, the redirect and error pages, the
static assets with their conditional and range variants, and the method and
trailing-slash variants of every route.
"""
import argparse, json, urllib.parse, urllib.request


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--backend", default="http://127.0.0.1:8081/")
    ap.add_argument("--out", required=True)
    ap.add_argument("--frontend", default="http://127.0.0.1:8080",
                    help="where to read the static ETag/Last-Modified from")
    ap.add_argument("--include-random", action="store_true",
                    help="also capture /random, which is not deterministic")
    args = ap.parse_args()

    recipes = json.load(urllib.request.urlopen(args.backend + "recipes/"))
    permalinks = [r["permalink"] for r in recipes]

    reqs = []

    def add(name, path, method="GET", headers=None, body=None):
        entry = {"name": name, "path": path, "method": method}
        if headers:
            entry["headers"] = headers
        if body is not None:
            entry["body"] = body
        reqs.append(entry)

    # --- the pages ---------------------------------------------------------
    add("home", "/")
    add("home-query", "/?x=1")
    add("manifest", "/manifest.json")
    add("manifest-query", "/manifest.json?v=2")
    add("sitemap", "/sitemap.xml")
    add("contribute", "/contribute")
    add("contribute-query", "/contribute?x=1")
    add("notfound", "/does-not-exist")
    add("notfound-deep", "/a/b/c")
    add("notfound-recipes", "/recipes")
    add("notfound-upper", "/Does-Not-Exist")
    add("lower-301", "/Vegetable-Primavera")
    add("upper-301", "/CHICKEN-CURRY")
    add("upper-manifest-301", "/MANIFEST.JSON")
    add("mixed-sitemap-301", "/Sitemap.XML")

    # --- scaling -----------------------------------------------------------
    for raw in ["0", "1", "1.0", "0.0", "-1", "-1.0", "51", "50", "49.9", "abc",
                "", "2.5", ".5", "2", "3", "0.5", "1e2", " 2", "2 ", "+2",
                "nan", "inf", "1_0", "2.0", "+1", "00"]:
        add("scale-%s" % (raw.strip() or "empty"), "/chicken-curry?scale=" + urllib.parse.quote(raw))
    add("scale-repeated", "/chicken-curry?scale=2&scale=3")
    add("scale-with-other", "/chicken-curry?scale=2&other=1")

    # --- every recipe, five scale factors ---------------------------------
    for permalink in permalinks:
        add("recipe-" + permalink, "/" + permalink)
    for factor, prefix in [(2, "r2-"), (0.5, "rhalf-"), (3, "r3-"), (1.5, "r1.5-")]:
        for permalink in permalinks:
            add(prefix + permalink, "/%s?scale=%s" % (permalink, factor))

    # --- static assets -----------------------------------------------------
    assets = ["styles.css", "copy.js", "search.js", "wakelock.js",
              "material.min.js", "contribute.js",
              "material.indigo-deep_purple.min.css"]
    for asset in assets:
        add("static-" + asset, "/static/" + asset)
    add("static-versioned", "/static/styles.css?v=abc123")
    add("static-missing", "/static/nope.js")
    add("static-dir", "/static/")
    add("static-root", "/static")
    add("static-traversal", "/static/../app.py")
    add("static-nested", "/static/sub/dir/file.js")
    add("head-static", "/static/styles.css", method="HEAD")

    # --- methods and trailing slashes -------------------------------------
    add("head-home", "/", method="HEAD")
    add("head-recipe", "/chicken-curry", method="HEAD")
    add("head-missing", "/does-not-exist", method="HEAD")
    add("head-manifest", "/manifest.json", method="HEAD")
    form = "passcode=p&name=Soup&method=Simmer."
    add("post-contribute", "/contribute", method="POST",
        headers={"Content-Type": "application/x-www-form-urlencoded"}, body=form)
    add("post-empty-contribute", "/contribute", method="POST",
        headers={"Content-Type": "application/x-www-form-urlencoded"}, body="")
    add("post-contribute-empty-name", "/contribute", method="POST",
        headers={"Content-Type": "application/x-www-form-urlencoded"},
        body="passcode=p&name=&method=")
    add("put-contribute", "/contribute", method="PUT")
    add("delete-contribute", "/contribute", method="DELETE")
    add("patch-home", "/", method="PATCH")
    add("options-home", "/", method="OPTIONS")
    add("trailing-contribute", "/contribute/")
    add("trailing-recipe", "/chicken-curry/")
    add("trailing-manifest", "/manifest.json/")
    add("double-slash", "//chicken-curry")
    add("percent-encoded", "/chicken%2Dcurry")
    add("dot-segment", "/./chicken-curry")

    # --- static caching and ranges ----------------------------------------
    if args.frontend:
        probe = urllib.request.urlopen(args.frontend.rstrip("/") + "/static/styles.css")
        etag = probe.headers.get("ETag")
        last_modified = probe.headers.get("Last-Modified")
        if etag:
            add("static-if-none-match", "/static/styles.css",
                headers={"If-None-Match": etag})
        add("static-if-none-match-stale", "/static/styles.css",
            headers={"If-None-Match": '"deadbeef"'})
        if last_modified:
            add("static-if-modified-since", "/static/styles.css",
                headers={"If-Modified-Since": last_modified})
        add("static-if-modified-since-old", "/static/styles.css",
            headers={"If-Modified-Since": "Thu, 01 Jan 1970 00:00:00 GMT"})
        add("static-range-0-9", "/static/styles.css", headers={"Range": "bytes=0-9"})
        add("static-range-open", "/static/styles.css", headers={"Range": "bytes=10-"})
        add("static-range-suffix", "/static/styles.css", headers={"Range": "bytes=-5"})
        add("static-range-invalid", "/static/styles.css", headers={"Range": "bytes=999999-1000000"})
        add("static-range-multi", "/static/styles.css", headers={"Range": "bytes=0-1,4-5"})
        if etag:
            add("static-if-range", "/static/styles.css",
                headers={"Range": "bytes=0-9", "If-Range": etag})
        # If-Match: a match is a plain 200, anything else is Werkzeug's 412, which
        # keeps the whole response and changes only the status.
        if etag:
            add("static-if-match", "/static/styles.css", headers={"If-Match": etag})
            add("static-if-match-unquoted", "/static/styles.css",
                headers={"If-Match": etag.strip('"')})
        add("static-if-match-stale", "/static/styles.css", headers={"If-Match": '"deadbeef"'})
        add("static-if-match-weak", "/static/styles.css", headers={"If-Match": 'W/"deadbeef"'})
        add("static-if-match-star", "/static/styles.css", headers={"If-Match": "*"})
        add("static-if-match-garbage", "/static/styles.css", headers={"If-Match": "garbage!!"})
        add("static-if-none-match-star", "/static/styles.css", headers={"If-None-Match": "*"})
        if etag:
            add("static-if-none-match-wins-over-if-match", "/static/styles.css",
                headers={"If-None-Match": etag, "If-Match": '"deadbeef"'})
        add("static-if-unmodified-since-old", "/static/styles.css",
            headers={"If-Unmodified-Since": "Thu, 01 Jan 1970 00:00:00 GMT"})
        add("static-if-unmodified-since-new", "/static/styles.css",
            headers={"If-Unmodified-Since": "Thu, 01 Jan 2100 00:00:00 GMT"})

    # --- request headers ---------------------------------------------------
    add("origin-home", "/", headers={"Origin": "http://example.com"})
    add("origin-contribute", "/contribute", headers={"Origin": "http://example.com"})
    add("accept-json", "/recipes/", headers={"Accept": "application/json"})
    add("unknown-header", "/chicken-curry", headers={"X-Nonsense": "1"})

    if args.include_random:
        for i in range(5):
            add("random-%d" % i, "/random")

    json.dump(reqs, open(args.out, "w"), indent=1)
    print("%d requests -> %s" % (len(reqs), args.out))


if __name__ == "__main__":
    main()
