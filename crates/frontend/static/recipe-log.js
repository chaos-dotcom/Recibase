// The recipe page's "Log meal" button. The session lives on the API origin, so
// this asks the API (with cookies) whether the visitor is signed in, and only
// then reveals the button. Logging records today's meal; the tracker page can
// add a note or change the date afterwards.
(function () {
  var root = document.getElementById("recipe-log");
  if (!root) {
    return;
  }

  var button = document.getElementById("recipe-log-button");
  var statusEl = document.getElementById("recipe-log-status");
  var mealName = root.getAttribute("data-meal-name") || "";
  var apiBase = (root.getAttribute("data-api-url") || "").replace(/\/+$/, "");
  if (!button || !statusEl || !mealName) {
    return;
  }

  var csrf = null;

  function api(path) {
    return apiBase + "/" + path;
  }

  // Cookie-bearing calls to the API, where the session lives.
  function call(path, options) {
    options = options || {};
    options.credentials = "include";
    return fetch(api(path), options);
  }

  function today() {
    return new Date().toISOString().slice(0, 10);
  }

  // The API's error bodies are plain text, but a few are JSON; show either.
  function messageFrom(response, fallback) {
    return response.text().then(function (text) {
      var message = (text || "").trim();
      try {
        var parsed = JSON.parse(message);
        if (parsed && parsed.error) {
          message = parsed.error;
        }
      } catch (error) {
        // not JSON, keep the plain text
      }
      return message || fallback + " (" + response.status + ")";
    });
  }

  function setStatus(message, isError) {
    statusEl.textContent = message;
    statusEl.className = "recipe-log-status" + (isError ? " is-error" : "");
  }

  // Only a signed-in owner sees the button, so visitors get exactly the recipe
  // page they had before.
  call("admin/api/session")
    .then(function (response) {
      return response.ok ? response.json() : null;
    })
    .then(function (session) {
      if (!session || !session.authenticated || !session.csrf) {
        return;
      }
      csrf = session.csrf;
      root.hidden = false;
    })
    .catch(function () {
      // The API is unreachable: leave the button hidden.
    });

  button.addEventListener("click", function () {
    button.disabled = true;
    setStatus("Logging…", false);
    call("admin/api/entries", {
      method: "POST",
      headers: { "Content-Type": "application/json", "X-CSRF-Token": csrf },
      body: JSON.stringify({
        meal_name: mealName,
        eaten_on: today(),
        note: "",
        featured: false,
      }),
    })
      .then(function (response) {
        if (!response.ok) {
          return messageFrom(response, "Could not log that meal").then(function (message) {
            throw new Error(message);
          });
        }
      })
      .then(function () {
        button.textContent = "Logged";
        setStatus("Logged " + mealName + " for today.", false);
      })
      .catch(function (problem) {
        button.disabled = false;
        setStatus(problem.message, true);
      });
  });
})();
