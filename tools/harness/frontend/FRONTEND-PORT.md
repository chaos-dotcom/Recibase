
# The Recibase frontend, in Rust

A port of [The-Silverwood-Institute/Frontend](https://github.com/The-Silverwood-Institute/Frontend)
- the Flask application that serves reciba.se - to Rust. It lives in
`crates/frontend` beside the API in `crates/server`; `PORT.md` is the API's port
and `REPORT.md`/`REPORT-APP.md` are its measurements, and
`REPORT-FRONTEND.md` here is this one's.

The port renders **the same bytes** as the Flask application. Every page, every
redirect, every error page and every static file response was captured from the
running Flask application and compared with the Rust server's answer; 573 of the
578 captured requests are byte-identical, and the five that are not compared are
the five `/random` requests, whose recipe is chosen at random.

## What it does

It is the same application, on the same routes, talking to the same API:

| route | what |
|---|---|
| `/` | the homepage, with the recipe drawer filled from the API |
| `/<name>` | one recipe, optionally scaled (`?scale=2`) |
| `/random` | 302 to a recipe chosen at random |
| `/manifest.json` | the frontend version and the API URL the browser should use |
| `/sitemap.xml` | every recipe |
| `/contribute` | the contribution form, and the `POST` that opens the pull request |
| `/static/<file>` | the stylesheets and scripts, conditionally |

## Running it

```
cargo build --release -p recibase-frontend
BACKEND_URL=http://localhost:8081/ PORT=8080 ./target/release/recibase-frontend
```

Then open <http://localhost:8080/>. `tools/harness/frontend/serve-stack.sh` runs
this frontend together with the Rust API and starts both for you.

The stylesheets and scripts are served from disk. `STATIC_DIR` overrides where
they are looked for; without it the binary tries `static/` beside itself (the
shape of a deployment that copies the assets next to the binary, as the
`Dockerfile` does) and then `crates/frontend/static` above the executable or the
working directory, which is where a checkout keeps them. It prints the directory
it settled on at start-up, and warns when there is none.

Environment variables, all of which the Python reads with the same names and the
same defaults:

| variable | default | meaning |
|---|---|---|
| `PORT` | `8080` | the port to listen on |
| `BACKEND_URL` | `http://localhost:8081/` | the recipe API, **with** its trailing slash |
| `STATIC_DIR` | discovered: `static` beside the executable, else `crates/frontend/static` above the executable or the working directory | the directory `/static/` is served from |
| `SOURCE_COMMIT`, `GIT_COMMIT`, `GITHUB_SHA` | - | the deployed commit, shown in the footer |
| `GIT_COMMIT` (file) | - | read beside the executable, then in the working directory, when none of the variables holds a commit |

The templates are compiled into the binary; only `static/` is read from disk, so
a deployment is the binary plus that directory.

## How the port was verified

`tools/harness/frontend/capture-flask.tar.gz` is the recorded Python: 578 requests and
their raw response bytes, taken with

```
# 1. record what the Flask application answers (needs it running on :8080)
python3 tools/harness/frontend/gen_requests.py --out tools/harness/frontend/requests.json
python3 tools/harness/capture.py --port 8080 --out capture-flask --requests tools/harness/frontend/requests.json
```

To compare the Rust server against it, unpack the archive and run

```
# 2. start the Rust server, replay the same requests, compare
python3 tools/harness/frontend/verify_frontend.py --reference capture-flask --out capture-rust \
        --binary target/release/recibase-frontend --requests tools/harness/frontend/requests.json \
        --port 8080 --env BACKEND_URL=http://localhost:8081/ --env STATIC_DIR=static
```

`--reference` unpacks `capture-flask.tar.gz` itself if the directory is not
there. Point `STATIC_DIR` at the very directory the Flask application served - it
is `static/` beside `app.py` - because a static file's `ETag` is
`"<mtime>-<size>-<adler32 of the path>"`, and a copy of the same file at another
path has another `ETag`.

`capture.py` sends one request per fresh TCP connection and stores the raw
response bytes. `verify_frontend.py` compares the status line, the reason
phrase, the order and value of every header and the body, after removing the two
headers that belong to the server rather than to the application: `Server` (which
is `Werkzeug/...` for the development server and `gunicorn` in production) and
`Date`. The `Allow` header of a 405 or an OPTIONS answer is a Python `set`, so
its token order differs between Flask processes; the harness sorts it.

The request list covers all 95 recipes at five scale factors, the homepage, the
sitemap, the contribution form (GET and POST), the redirect and error pages, the
static files with their conditional and range variants, the method and
trailing-slash variants of every route, and a handful of odd paths
(`//chicken-curry`, `/./chicken-curry`, `/static/../app.py`).

`tools/harness/frontend/results.json` holds the before/after measurements; `REPORT.md`
reads them.

## What is different from the Python

Stated plainly.

* **MiniJinja's escaping is rewritten.** Jinja2 escapes `'` and `"` as `&#39;`
  and `&#34;` and leaves `/` alone; MiniJinja writes `&#x27;`, `&quot;` and
  `&#x2f;`. The rendered page is passed through a three-entry rewrite so the
  bytes match. See `src/markupsafe.rs`.
* **The static ETag and Last-Modified depend on the file.** They are derived the
  way Werkzeug derives them - the ETag is `"<mtime>-<size>-<adler32 of the
  path>"` - so they match when both servers serve the same directory, but a
  fresh checkout has different file times and therefore different ETags. That is
  a property of `send_file`, not of this port.
* **`Allow` header order.** Werkzeug builds it from a `set`, so the order is
  random per Python process. This port writes `HEAD, GET, OPTIONS` (and `POST`
  after `HEAD`) and the harness sorts the tokens.
* **The JSON sent to the API is printed compactly.** `requests` prints with
  `json.dumps` defaults (`", "` and `": "` separators) and this port prints
  `{"a":1}`. The API parses JSON, so both are read the same way.
* **Only `application/x-www-form-urlencoded` bodies are parsed.** Werkzeug also
  parses `multipart/form-data`. The contribution form has no file inputs, so a
  browser never sends multipart; a multipart POST would be seen as an empty form.
* **`url_root` is always `http://`.** Flask uses `wsgi.url_scheme`, which is
  `http` unless the WSGI server says otherwise, and the Python application does
  not install `ProxyFix` either, so a request through a TLS terminator produces
  the same `http://` sitemap URLs in both.
* **A missing `version` in the API's manifest** is a 503 here and a 500 in the
  Python, where it raises an uncaught `KeyError`. The API always sends one.
* **The `POST /contribute` path is not covered by the capture.** The API's
  submission route needs a passcode, a Turnstile secret and a GitHub token, so
  the captured case is the unconfigured 503. The rest of the path is covered by
  the ported tests in `crates/frontend/tests/`.

## Tests

```
cargo test --workspace
```

`crates/frontend/tests/` holds the port of the pytest suite (63 cases in
`test_routes.py`, `test_scaler.py` and `test_cached_backend.py`) plus tests
driven by golden files that were generated from the Python itself:
2,544 scaling cases, 212 quantity classifications, 31 scale-factor parses, the
copy text of all 95 recipes, 14 contribution forms, 16 API failure responses and
14 pull-request URL payloads.
