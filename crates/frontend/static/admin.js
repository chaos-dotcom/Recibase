// The meal tracker's client: it signs the owner in through the API, shows the
// history of what they have eaten, and records meals (the chooser's Log button
// calls `window.recibaseTracker.log`). Everything talks to the API origin, with
// cookies, since that is where the session lives.
(function () {
  var root = document.getElementById("admin-app");
  if (!root) {
    return;
  }

  // The chooser reads this to know whether to offer its Log buttons, and to
  // record a meal. Set synchronously so it exists before that script runs.
  window.recibaseTracker = { authenticated: false };

  var apiBase = (root.getAttribute("data-api-url") || "/").replace(/\/+$/, "");
  function api(path) {
    return apiBase + "/" + path;
  }

  // Cookie-bearing calls to the API (session + JSON actions).
  function call(path, options) {
    options = options || {};
    options.credentials = "include";
    return fetch(api(path), options);
  }

  var csrf = null;
  var statusEl = document.getElementById("admin-status");
  var loginEl = document.getElementById("admin-login");
  var loginLinkEl = document.getElementById("admin-login-link");
  var trackerEl = document.getElementById("admin-tracker");
  var logoutEl = document.getElementById("admin-logout");
  var historyEl = document.getElementById("admin-history");

  function today() {
    return new Date().toISOString().slice(0, 10);
  }

  function formatDay(iso) {
    var parts = iso.split("-").map(Number);
    var date = new Date(parts[0], parts[1] - 1, parts[2]);
    return date.toLocaleDateString(undefined, {
      weekday: "short",
      day: "numeric",
      month: "short",
      year: "numeric",
    });
  }

  // The API's error bodies are plain-text messages; show them, or a fallback.
  function messageFrom(response, fallback) {
    return response.text().then(function (text) {
      var message = (text || "").trim();
      return message || fallback + " (" + response.status + ")";
    });
  }

  // ---- history -----------------------------------------------------------

  function loadHistory() {
    return call("admin/api/entries").then(function (response) {
      if (!response.ok) {
        return messageFrom(response, "Could not load your history").then(function (message) {
          throw new Error(message);
        });
      }
      return response.json().then(renderHistory);
    });
  }

  function renderHistory(entries) {
    historyEl.innerHTML = "";
    if (!entries.length) {
      var empty = document.createElement("p");
      empty.className = "history-empty";
      empty.textContent = "Nothing logged yet. Pick a meal from the chooser and tap Log.";
      historyEl.appendChild(empty);
      return;
    }

    var groups = new Map();
    entries.forEach(function (entry) {
      if (!groups.has(entry.eaten_on)) {
        groups.set(entry.eaten_on, []);
      }
      groups.get(entry.eaten_on).push(entry);
    });

    Array.from(groups.keys())
      .sort()
      .reverse()
      .forEach(function (day) {
        var section = document.createElement("section");
        section.className = "history-day";
        var heading = document.createElement("h4");
        heading.textContent = formatDay(day);
        section.appendChild(heading);

        var list = document.createElement("ul");
        list.className = "history-list";
        groups.get(day).forEach(function (entry) {
          list.appendChild(historyRow(entry));
        });
        section.appendChild(list);
        historyEl.appendChild(section);
      });
  }

  function historyRow(entry) {
    var item = document.createElement("li");
    item.className = "history-item";
    showHistorySummary(item, entry);
    return item;
  }

  function showHistorySummary(item, entry) {
    item.innerHTML = "";

    var text = document.createElement("span");
    text.className = "history-text";
    text.textContent = entry.meal_name;
    if (entry.note) {
      var note = document.createElement("span");
      note.className = "history-note";
      note.textContent = " — " + entry.note;
      text.appendChild(note);
    }
    if (entry.featured) {
      var featured = document.createElement("span");
      featured.className = "history-featured";
      featured.title = "Featured";
      var icon = document.createElement("span");
      icon.className = "material-icons";
      icon.textContent = "star";
      featured.appendChild(icon);
      text.appendChild(featured);
    }
    item.appendChild(text);

    var actions = document.createElement("span");
    actions.className = "history-actions";

    var edit = document.createElement("button");
    edit.type = "button";
    edit.className = "chooser-link";
    edit.textContent = "Edit";
    edit.addEventListener("click", function () {
      showHistoryEditor(item, entry);
    });

    var remove = document.createElement("button");
    remove.type = "button";
    remove.className = "chooser-link";
    remove.textContent = "Delete";
    remove.addEventListener("click", function () {
      remove.disabled = true;
      call("admin/api/entries/delete", {
        method: "POST",
        headers: { "Content-Type": "application/json", "X-CSRF-Token": csrf },
        body: JSON.stringify({ id: entry.id }),
      })
        .then(function (response) {
          if (!response.ok) {
            return messageFrom(response, "Could not delete that entry").then(function (message) {
              throw new Error(message);
            });
          }
          return loadHistory();
        })
        .catch(function (error) {
          remove.disabled = false;
          window.alert(error.message);
        });
    });

    actions.appendChild(edit);
    actions.appendChild(remove);
    item.appendChild(actions);
  }

  function showHistoryEditor(item, entry) {
    item.innerHTML = "";
    var form = document.createElement("form");
    form.className = "history-edit";

    var date = document.createElement("input");
    date.type = "date";
    date.value = entry.eaten_on;
    date.setAttribute("aria-label", "Date");

    var note = document.createElement("input");
    note.type = "text";
    note.value = entry.note || "";
    note.placeholder = "Note";
    note.setAttribute("aria-label", "Note");

    var featuredLabel = document.createElement("label");
    featuredLabel.className = "history-featured-toggle";
    var featured = document.createElement("input");
    featured.type = "checkbox";
    featured.checked = entry.featured;
    featuredLabel.appendChild(featured);
    featuredLabel.appendChild(document.createTextNode(" Featured"));

    var save = document.createElement("button");
    save.type = "submit";
    save.className = "chooser-link";
    save.textContent = "Save";

    var cancel = document.createElement("button");
    cancel.type = "button";
    cancel.className = "chooser-link";
    cancel.textContent = "Cancel";
    cancel.addEventListener("click", function () {
      showHistorySummary(item, entry);
    });

    var error = document.createElement("span");
    error.className = "history-error";

    form.appendChild(date);
    form.appendChild(note);
    form.appendChild(featuredLabel);
    form.appendChild(save);
    form.appendChild(cancel);
    form.appendChild(error);
    item.appendChild(form);

    form.addEventListener("submit", function (event) {
      event.preventDefault();
      save.disabled = true;
      error.textContent = "";
      call("admin/api/entries/update", {
        method: "POST",
        headers: { "Content-Type": "application/json", "X-CSRF-Token": csrf },
        body: JSON.stringify({
          id: entry.id,
          eaten_on: date.value || entry.eaten_on,
          note: note.value,
          featured: featured.checked,
        }),
      })
        .then(function (response) {
          if (!response.ok) {
            return messageFrom(response, "Could not save that entry").then(function (message) {
              throw new Error(message);
            });
          }
          return loadHistory();
        })
        .catch(function (problem) {
          save.disabled = false;
          error.textContent = problem.message;
        });
    });
  }

  // ---- logging (called by the chooser) -----------------------------------

  function quickLog(mealName, options) {
    options = options || {};
    return call("admin/api/entries", {
      method: "POST",
      headers: { "Content-Type": "application/json", "X-CSRF-Token": csrf },
      body: JSON.stringify({
        meal_name: mealName,
        eaten_on: options.eaten_on || today(),
        note: options.note || "",
        featured: !!options.featured,
      }),
    }).then(function (response) {
      if (!response.ok) {
        return messageFrom(response, "Could not log that meal").then(function (message) {
          throw new Error(message);
        });
      }
      return loadHistory();
    });
  }

  function signOut() {
    call("auth/logout", { method: "POST" }).then(function () {
      window.location.reload();
    });
  }

  // ---- start -------------------------------------------------------------

  call("admin/api/session")
    .then(function (response) {
      return response.json();
    })
    .then(function (session) {
      statusEl.hidden = true;
      if (!session.authenticated) {
        loginLinkEl.href = api("auth/login");
        loginEl.hidden = false;
        document.dispatchEvent(new CustomEvent("recibase:session"));
        return;
      }
      csrf = session.csrf;
      trackerEl.hidden = false;
      logoutEl.addEventListener("click", signOut);
      window.recibaseTracker = { authenticated: true, log: quickLog, refresh: loadHistory };
      document.dispatchEvent(new CustomEvent("recibase:session"));
      loadHistory();
    })
    .catch(function () {
      statusEl.textContent = "Could not reach the Recibase API.";
    });
})();
