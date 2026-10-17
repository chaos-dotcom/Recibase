// The meal tracker's client: it signs the owner in through the API and records
// what they have eaten. Everything talks to the API origin (the URL the server
// renders into `data-api-url`), with cookies, since that is where the session
// lives.
(function () {
  var root = document.getElementById("admin-app");
  if (!root) {
    return;
  }

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
  var status = document.getElementById("admin-status");
  var login = document.getElementById("admin-login");
  var loginLink = document.getElementById("admin-login-link");
  var tracker = document.getElementById("admin-tracker");
  var logout = document.getElementById("admin-logout");
  var entries = document.getElementById("admin-entries");
  var errorEl = document.getElementById("admin-error");
  var form = document.getElementById("admin-add");
  var dateInput = document.getElementById("admin-date");
  var mealsList = document.getElementById("admin-meals");

  function upgrade() {
    if (window.componentHandler) {
      window.componentHandler.upgradeDom();
    }
  }

  function today() {
    return new Date().toISOString().slice(0, 10);
  }

  function showError(message) {
    errorEl.textContent = message;
    errorEl.hidden = false;
  }

  function renderEntries(list) {
    entries.innerHTML = "";
    if (!list.length) {
      var empty = document.createElement("li");
      empty.textContent = "No meals logged yet.";
      entries.appendChild(empty);
      return;
    }
    list.forEach(function (entry) {
      var item = document.createElement("li");
      var line = entry.eaten_on + " — " + entry.meal_name;
      if (entry.note) {
        line += " (" + entry.note + ")";
      }
      if (entry.featured) {
        line += " ★";
      }
      var text = document.createElement("span");
      text.textContent = line;

      var remove = document.createElement("button");
      remove.type = "button";
      remove.className = "mdl-button mdl-js-button";
      remove.textContent = "Delete";
      remove.addEventListener("click", function () {
        removeEntry(entry.id);
      });

      item.appendChild(text);
      item.appendChild(remove);
      entries.appendChild(item);
    });
    upgrade();
  }

  function loadEntries() {
    return call("admin/api/entries")
      .then(function (response) {
        return response.json();
      })
      .then(renderEntries);
  }

  function loadMeals() {
    // The public meal list drives the input's suggestions; no cookie needed.
    return fetch(api("meals/"))
      .then(function (response) {
        return response.json();
      })
      .then(function (meals) {
        meals.forEach(function (meal) {
          var option = document.createElement("option");
          option.value = meal.name;
          mealsList.appendChild(option);
        });
      })
      .catch(function () {});
  }

  function addEntry(event) {
    event.preventDefault();
    errorEl.hidden = true;
    var payload = {
      meal_name: form.meal_name.value,
      eaten_on: form.eaten_on.value || today(),
      note: form.note.value,
      featured: form.featured.checked,
    };
    call("admin/api/entries", {
      method: "POST",
      headers: { "Content-Type": "application/json", "X-CSRF-Token": csrf },
      body: JSON.stringify(payload),
    })
      .then(function (response) {
        if (!response.ok) {
          throw new Error("Could not add that meal (" + response.status + ")");
        }
        form.reset();
        dateInput.value = today();
        return loadEntries();
      })
      .catch(function (error) {
        showError(error.message);
      });
  }

  function removeEntry(id) {
    call("admin/api/entries/delete", {
      method: "POST",
      headers: { "Content-Type": "application/json", "X-CSRF-Token": csrf },
      body: JSON.stringify({ id: id }),
    })
      .then(function (response) {
        if (!response.ok) {
          throw new Error("Could not delete that meal (" + response.status + ")");
        }
        return loadEntries();
      })
      .catch(function (error) {
        showError(error.message);
      });
  }

  function signOut() {
    call("auth/logout", { method: "POST" }).then(function () {
      window.location.reload();
    });
  }

  dateInput.value = today();
  call("admin/api/session")
    .then(function (response) {
      return response.json();
    })
    .then(function (session) {
      status.hidden = true;
      if (!session.authenticated) {
        loginLink.href = api("auth/login");
        login.hidden = false;
        upgrade();
        return;
      }
      csrf = session.csrf;
      tracker.hidden = false;
      logout.hidden = false;
      form.addEventListener("submit", addEntry);
      logout.addEventListener("click", signOut);
      loadMeals();
      loadEntries();
      upgrade();
    })
    .catch(function () {
      status.textContent = "Could not reach the Recibase API.";
    });
})();
