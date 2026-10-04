
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

Or in containers, from the repository root - `compose.yaml` carries no comments,
so this is where its settings are explained:

```
docker compose -f compose.yaml -f compose.local.yaml up --build   # on a laptop
docker compose up --build                                        # behind Traefik
```

`compose.yaml` defines the website on port 8080 and the API on 8081, with Traefik
labels for the network `apps-internal` and no published ports - Traefik forwards
to the ports in the labels over that network. `compose.local.yaml` is an override
that adds both ports to the host and points the website at the local API, for a
machine with no Traefik. The Traefik labels stay in place and do nothing when
nothing is watching that network.

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
| `SERVER_IDENTITY` | - | this deployment's own server, `label\|site`, for the recipe page's "reci-verse" caption (e.g. `Chaos' Recibase Server\|https://recibase.shed.gay`) |
| `STATIC_DIR` | discovered: `static` beside the executable, else `crates/frontend/static` above the executable or the working directory | the directory `/static/` is served from |
| `SOURCE_COMMIT`, `GIT_COMMIT`, `GITHUB_SHA` | - | the deployed commit, shown in the footer |
| `GIT_COMMIT` (file) | - | read beside the executable, then in the working directory, when none of the variables holds a commit |

`PEER_BACKENDS` is a port addition, not in the Python: other Recibase
deployments whose recipes the drawer lists alongside ours. Entries are
`label|api|site`, separated by `;`, e.g.
`Kit & Alex|https://api.reciba.se/|https://reciba.se`. A peer's recipe is
labelled with their name and **served by this frontend**: its drawer link is the
local permalink and `/<permalink>` renders it from the peer's API, so a reader
never leaves for the peer's site. A name we already hold is not listed again.

Originality decides the caption. A recipe without `chaos-tag:` came from the
other server first, so the caption credits the peer that lists it - `Found on
<peer> across the reci-verse.` - rather than the deployment hosting the page;
one of ours is credited to our own server (below). Our own recipe gains a `Found
on <peer> too.` hint that *does* go to their site only when a peer holds a
*different* version, which the frontend decides from each backend's
`recipes/?withRevision=true` content digest (opt-in, so the default API response
is unchanged): an equal digest means the same recipe, a missing one means we
cannot tell, so we hint. An unreachable peer is skipped rather than failing the
page, and its recipes simply do not appear.

The drawer lists our own recipes first, then the peers'. It keeps to the recipes
we host by default; a peer-only recipe (one we do not have at all) appears only
when the reader ticks **Show recipes from across the reci-verse**, an
unticked-by-default box in the drawer (its state is remembered in
`localStorage`).

`SERVER_IDENTITY` is the same idea for this deployment itself: `label|site`, so
a recipe that is ours can say `Found on <label> across the reci-verse.` and link
the label to that site, instead of the generic `this server`. Because it lives on
the frontend, the same image can be pointed at another deployment's API and name
whichever server it is attached to. Unset, the caption is the generic one.

When none of these is set the footer still names a commit: `crates/frontend/build.rs`
bakes the checkout's `HEAD` - or a `SOURCE_COMMIT` / `GIT_COMMIT` / `GITHUB_SHA` build
argument - into the binary, so the `frontend` service in `compose.yaml`, which runs
without the commit environment, shows `v<hash>`. The order is environment, then file,
then the baked commit, then `latest`.

The templates are compiled into the binary; only `static/` is read from disk, so
a deployment is the binary plus that directory.

## Behind a reverse proxy

`compose.yaml` carries Traefik labels for the network `apps-internal` and routes
by host name:

```
https://recibase.shed.gay/       -> the website  (port 8080)
https://api.recibase.shed.gay/   -> the API      (port 8081)
```

**Two host names, not one**, and that is not decoration. The frontend renders the
pages, but `search.js` reads the API's URL out of `/manifest.json` and calls it
from the browser:

```js
fetch('/manifest.json').then(resp => resp.json()).then(json => {
  apiUrl = json['apiUrl'];
  ...
  return fetch(`${apiUrl}recipes/?${params.toString()}`)
```

So the API has to be reachable from the browser under a name that resolves
publicly - which is also why it answers CORS preflights - while the frontend's
own server-side calls go through the compose network by service name. Put one
host name in front of everything and the search box silently stops working.

That is the same shape the Python deployment has (`reciba.se` and
`api.reciba.se`), so this is a property of the application, not of the port.

```
# .env
RECIBASE_HOST=recibase.shed.gay
RECIBASE_API_HOST=api.recibase.shed.gay
TRAEFIK_CERTRESOLVER=myresolver      # if yours is not called myresolver
BACKEND_URL=https://api.recibase.shed.gay/
```

`BACKEND_URL` does double duty: the frontend proxies the API through it, and the
browser is handed it in `/manifest.json`.

Nothing is published to the host - Traefik reaches both containers over
`apps-internal` and forwards to the port in the labels - so the deployment needs
no host ports at all, and the same file works on any machine that can reach that
network. To run the same images on a laptop, with no Traefik, add the ports back
with the override:

```
docker compose -f compose.yaml -f compose.local.yaml up --build
open http://localhost:8080/
```

## How the port was verified

`tools/harness/frontend/capture-flask.tar.gz` is the recorded Python: 578 requests
and their raw response bytes, taken with

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

## The capture is pinned to a commit

`capture-flask` belongs to **`a8808f9`**, and `capture-flask.origin.json` says so.
At that commit the repository answered all 573 comparable responses byte for
byte; the harness checks that before it compares anything, and refuses when the
paths that shape a response have changed since:

```
$ python3 tools/harness/frontend/verify_frontend.py --reference tools/harness/frontend/capture-flask ...
capture-flask belongs to commit a8808f9, and this tree has moved on since:
  changed: crates/core/src/misc.rs
  changed: crates/core/src/recipe.rs
  ...
```

The check is `git diff --quiet a8808f9 -- :/crates :/tools/harness/frontend/requests.json`.
Two flags relax it:

* `--allow-diverged` - compare anyway, knowing that it compares two different
  applications. Off the pinned commit the answer is a list of what the product
  changed, not a pass or a fail.
* `--repo <path>` - check a different checkout, for when the binary was built
  somewhere else.

Why pin it at all: the capture was taken from the Python while the Rust answered
the same bytes, and the Rust has since moved on - a new site description, more
recipes, an only-ours filter. Comparing today's tree against it reports 42
identical and 531 differing, every difference being one of those three changes
(the recipe pages themselves are untouched: strip the drawer, the only-ours block
and the description line from a recipe page and the two bodies differ by 18
lines, all of them leftovers of the checkbox block).

Reproducing the original result:

```
git worktree add /tmp/recibase-pinned a8808f9
cd /tmp/recibase-pinned && cargo build -p recibase-server -p recibase-frontend
PORT=8081 MEAL_LOG_CSV_URL=file:///path/to/meal-log.csv ./target/debug/recibase-server &
python3 /path/to/recibase-rs/tools/harness/frontend/verify_frontend.py \
    --reference /path/to/recibase-rs/tools/harness/frontend/capture-flask \
    --binary target/debug/recibase-frontend --repo /tmp/recibase-pinned \
    --requests /path/to/recibase-rs/tools/harness/frontend/requests.json \
    --port 8080 --env BACKEND_URL=http://localhost:8081/ \
    --env STATIC_DIR=/path/to/Frontend/static
```

What guards the frontend now is `cargo test --workspace`: 275 tests, including a
ported pytest case for every Python one, 2,902 fixture cases generated from the
Python's own output, and the golden rendering checks. The capture is the record of
the port, not a live check.

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
