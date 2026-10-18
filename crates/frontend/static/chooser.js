// The meal Chooser, ported into this frontend from
// https://github.com/The-Silverwood-Institute/Chooser (MIT licensed) and given a
// friendlier interface: tags are clickable chips, sorting and dates are menus,
// and the active filters are shown as removable chips. The filtering, scoring
// and sorting engine is unchanged.
(function () {
  var root = document.getElementById("chooser");
  if (!root) {
    return;
  }

  var mealsUrl = "/chooser/meals.json";

  var searchEl = document.getElementById("tags-search");
  var sortEl = document.getElementById("chooser-sort");
  var dateEl = document.getElementById("chooser-date");
  var countEl = document.getElementById("chooser-count");
  var clearEl = document.getElementById("chooser-clear");
  var activeWrapEl = document.getElementById("chooser-active");
  var activeChipsEl = document.getElementById("chooser-active-chips");
  var tagsToggleEl = document.getElementById("tags-list-toggle");
  var tagsWrapEl = document.getElementById("tags-list-wrapper");
  var tagsListEl = document.getElementById("tags-list");
  var weightsToggleEl = document.getElementById("weights-list-toggle");
  var weightsWrapEl = document.getElementById("weights-list-wrapper");
  var weightsListEl = document.getElementById("weights-list");
  var statusEl = document.getElementById("chooser-status");
  var listHost = document.getElementById("meal-list");
  var helpToggleEl = document.getElementById("chooser-help-toggle");
  var helpEl = document.getElementById("chooser-help");

  var DEFAULT_QUERY = "!Not a Meal, !Pudding, !Lunch, <Pescatarian";
  var defaultDate = new Date(0);
  var BEST_SORT_INDEX = 6;
  var MS_PER_MONTH = 30.44 * 24 * 60 * 60 * 1000;
  var weightConfig = [
    { key: "timesEaten", label: "Times eaten", min: 0, max: 15, value: 5 },
    { key: "recentPenalty", label: "Recent penalty", min: 0, max: 20, value: 8 },
    { key: "featured", label: "Featured", min: 0, max: 15, value: 6 },
    { key: "newTag", label: "New tag", min: 0, max: 10, value: 4 },
    { key: "neverEaten", label: "Never tried", min: 0, max: 10, value: 3 },
    { key: "neverEatenTag", label: "Never Eaten tag", min: 0, max: 10, value: 3 },
    { key: "infrequent", label: "Infrequent tag", min: 0, max: 10, value: 2 },
    { key: "stale", label: "Staleness bonus", min: 0, max: 15, value: 4 },
    { key: "popularRediscovery", label: "Popular rediscovery", min: 0, max: 10, value: 2 },
  ];
  var weights = {};
  weightConfig.forEach(function (entry) {
    weights[entry.key] = entry.value;
  });
  var hashStringRegex = /^(\d+)\-(.+)$/;
  var monthFilterRegex = /^([<>])([0-9]+)m$/;

  var query = DEFAULT_QUERY;
  searchEl.value = query;
  var meals = [];
  var knownTags = [];
  var normalisedTags = new Map();
  var tagButtons = new Map();
  var currentSortIndex = BEST_SORT_INDEX;
  var bestSortReferenceDate = new Date();
  var hashTimer;
  var statusTimer;
  var loading = true;

  function showStatus(message) {
    statusEl.textContent = message;
    statusEl.hidden = false;
    clearTimeout(statusTimer);
    statusTimer = setTimeout(function () {
      statusEl.hidden = true;
    }, 4000);
  }

  // ---- the query string is the single source of truth --------------------

  function terms() {
    return query
      .split(",")
      .map(function (term) {
        return term.trim();
      })
      .filter(Boolean);
  }

  function setQuery(next) {
    query = next;
    searchEl.value = query;
    render();
    scheduleHash();
  }

  function hasTerm(term) {
    var wanted = term.toLowerCase();
    return terms().some(function (existing) {
      return existing.toLowerCase() === wanted;
    });
  }

  function removeTerm(term) {
    var unwanted = term.toLowerCase();
    setQuery(
      terms()
        .filter(function (existing) {
          return existing.toLowerCase() !== unwanted;
        })
        .join(", ")
    );
  }

  function isExcludeTerm(term) {
    return term.charAt(0) === "!" || term.charAt(0) === "-";
  }

  function parseFilters() {
    var lowered = terms().map(function (term) {
      return term.toLowerCase();
    });
    var include = [];
    var includeInherited = [];
    var exclude = [];
    var excludeInherited = [];
    var beforeDates = [];
    var afterDates = [];

    lowered.forEach(function (term) {
      var tag;
      if (term.startsWith("<!")) {
        tag = normalisedTags.get(term.slice(2));
        if (tag) excludeInherited.push(tag);
      } else if (term.startsWith("<")) {
        tag = normalisedTags.get(term.slice(1));
        if (tag) includeInherited.push(tag);
      } else if (term.startsWith("!") || term.startsWith("-")) {
        tag = normalisedTags.get(term.slice(1));
        if (tag) exclude.push(tag);
      } else {
        var month = monthFilterRegex.exec(term);
        if (month) {
          var date = new Date();
          date.setMonth(date.getMonth() - Number(month[2]));
          (month[1] === "<" ? beforeDates : afterDates).push(date);
        }
      }
    });

    lowered.forEach(function (term) {
      var tag = normalisedTags.get(term);
      if (tag) include.push(tag);
    });

    return {
      include: include,
      includeInherited: includeInherited,
      exclude: exclude,
      excludeInherited: excludeInherited,
      beforeDates: beforeDates,
      afterDates: afterDates,
    };
  }

  // ---- filtering ---------------------------------------------------------

  function tagsMatchQuery(meal, f) {
    var mealTags = meal.tags || [];
    var inherited = meal.inherited_tags || [];
    return (
      f.include.every(function (tag) {
        return mealTags.includes(tag);
      }) &&
      !mealTags.some(function (tag) {
        return f.exclude.includes(tag);
      }) &&
      f.includeInherited.every(function (tag) {
        return mealTags.includes(tag) || inherited.includes(tag);
      }) &&
      !(
        mealTags.some(function (tag) {
          return f.excludeInherited.includes(tag);
        }) ||
        inherited.some(function (tag) {
          return f.excludeInherited.includes(tag);
        })
      )
    );
  }

  function lastEatenMatchesQuery(lastEaten, f) {
    return (
      lastEaten === null ||
      (f.beforeDates.every(function (date) {
        return lastEaten < date;
      }) &&
        f.afterDates.every(function (date) {
          return lastEaten > date;
        }))
    );
  }

  function filteredMeals() {
    var f = parseFilters();
    var anyFilter =
      f.include.length > 0 ||
      f.includeInherited.length > 0 ||
      f.exclude.length > 0 ||
      f.excludeInherited.length > 0 ||
      f.beforeDates.length > 0 ||
      f.afterDates.length > 0;
    if (!anyFilter) {
      return meals.slice();
    }
    return meals.filter(function (meal) {
      return tagsMatchQuery(meal, f) && lastEatenMatchesQuery(meal.last_eaten, f);
    });
  }

  // ---- scoring and sorting (ported, unchanged) ---------------------------

  function monthsBetween(from, to) {
    return (to - from) / MS_PER_MONTH;
  }

  function linearDecay(elapsed, windowMonths) {
    return elapsed >= windowMonths ? 0 : 1 - elapsed / windowMonths;
  }

  function bestScore(meal, now) {
    if (meal.tags.includes("Not a Meal")) {
      return -Infinity;
    }

    var score = Math.log1p(meal.times_eaten) * weights.timesEaten;

    if (meal.featured !== null) {
      score += weights.featured * linearDecay(monthsBetween(meal.featured, now), 6);
    }

    if (meal.last_eaten !== null) {
      var monthsSince = monthsBetween(meal.last_eaten, now);
      score -= weights.recentPenalty * linearDecay(monthsSince, 4);
      if (monthsSince > 6) {
        score += weights.stale * Math.min((monthsSince - 6) / 6, 1);
      }
      if (meal.tags.includes("Popular") && monthsSince > 3) {
        score += weights.popularRediscovery;
      }
    } else {
      score += weights.neverEaten;
    }

    if (meal.tags.includes("New")) {
      score += weights.newTag;
    }
    if (meal.tags.includes("Never Eaten")) {
      score += weights.neverEatenTag;
    }
    if (meal.tags.includes("Infrequent")) {
      score += weights.infrequent;
    }

    return score;
  }

  function sortByBest(left, right) {
    var diff = bestScore(right, bestSortReferenceDate) - bestScore(left, bestSortReferenceDate);
    if (diff !== 0) {
      return diff;
    }
    var leftDate = left.last_eaten !== null ? left.last_eaten : defaultDate;
    var rightDate = right.last_eaten !== null ? right.last_eaten : defaultDate;
    if (leftDate - rightDate !== 0) {
      return leftDate - rightDate;
    }
    return left.name.localeCompare(right.name);
  }

  function sortByLastEaten(ascending) {
    return function (left, right) {
      var leftDate = left.last_eaten !== null ? left.last_eaten : defaultDate;
      var rightDate = right.last_eaten !== null ? right.last_eaten : defaultDate;
      return ascending ? leftDate - rightDate : rightDate - leftDate;
    };
  }

  var sortingMethods = [
    { name: "Last Eaten ▲", sort: sortByLastEaten(true) },
    { name: "Last Eaten ▼", sort: sortByLastEaten(false) },
    {
      name: "Times Eaten ▲",
      sort: function (left, right) {
        return left.times_eaten - right.times_eaten;
      },
    },
    {
      name: "Times Eaten ▼",
      sort: function (left, right) {
        return right.times_eaten - left.times_eaten;
      },
    },
    {
      name: "Name ▼",
      sort: function (left, right) {
        return left.name.localeCompare(right.name);
      },
    },
    {
      name: "Name ▲",
      sort: function (left, right) {
        return right.name.localeCompare(left.name);
      },
    },
    { name: "Best", sort: sortByBest },
  ];

  // ---- rendering ---------------------------------------------------------

  function buildSortOptions() {
    sortingMethods.forEach(function (method, index) {
      var option = document.createElement("option");
      option.value = String(index);
      option.textContent = "Sort: " + method.name;
      sortEl.appendChild(option);
    });
  }

  function renderActiveChips() {
    var current = terms();
    activeWrapEl.hidden = current.length === 0;
    activeChipsEl.innerHTML = "";
    current.forEach(function (term) {
      var chip = document.createElement("button");
      chip.type = "button";
      chip.className = "chip" + (isExcludeTerm(term) ? " chip-exclude" : "");
      chip.setAttribute("aria-label", "Remove filter " + term);
      chip.appendChild(document.createTextNode(term));

      var close = document.createElement("span");
      close.className = "material-icons";
      close.textContent = "close";
      chip.appendChild(close);

      chip.addEventListener("click", function () {
        removeTerm(term);
      });
      activeChipsEl.appendChild(chip);
    });
  }

  function buildTagPalette() {
    tagsListEl.innerHTML = "";
    tagButtons.clear();
    knownTags.forEach(function (tag) {
      var chip = document.createElement("button");
      chip.type = "button";
      chip.className = "chip";
      chip.textContent = tag;
      chip.addEventListener("click", function () {
        if (hasTerm(tag)) {
          removeTerm(tag);
        } else {
          var next = terms();
          next.push(tag);
          setQuery(next.join(", "));
        }
      });
      tagButtons.set(tag, chip);
      tagsListEl.appendChild(chip);
    });
  }

  function syncTagPalette(f) {
    tagButtons.forEach(function (chip, tag) {
      chip.setAttribute("aria-pressed", f.include.includes(tag) ? "true" : "false");
    });
  }

  function syncDateSelect() {
    var monthTerm = terms().find(function (term) {
      return monthFilterRegex.test(term.toLowerCase());
    });
    dateEl.value = monthTerm ? monthTerm.toLowerCase() : "";
  }

  function getMealUrl(source) {
    if (source.type === "recibase") {
      return "https://reciba.se/" + source.permalink;
    } else if (source.type === "google_drive") {
      return "https://drive.google.com/file/d/" + source.id + "/view";
    } else if (source.type === "online") {
      return source.url;
    }
    console.warn('Unknown meal source type "' + source.type + '"');
    return null;
  }

  function getMealIcons(meal) {
    var icons = [];
    if (meal.source !== null) {
      if (meal.source.type === "recibase") {
        icons.push("fa-registered");
      } else if (meal.source.type === "online") {
        icons.push("fa-globe");
      } else if (meal.source.type === "google_drive") {
        icons.push("fa-google-drive");
      }
    }
    if ((meal.dated_notes || []).length > 0) {
      icons.push("fa-comments");
    }
    return icons.map(function (iconName) {
      var img = document.createElement("img");
      img.src = "/static/" + iconName + ".svg";
      img.classList.add("icon");
      img.setAttribute("icon-name", iconName);
      img.alt = "";
      if (iconName === "fa-comments") {
        img.title = "Notes";
        img.addEventListener("click", function (event) {
          event.preventDefault();
          toggleNotes(event.target, meal.dated_notes);
        });
      }
      return img;
    });
  }

  function toggleNotes(icon, datedNotes) {
    var parent = icon.closest(".chooser-meal-main");
    if (parent.classList.contains("notes-expanded")) {
      return;
    }
    datedNotes.forEach(function (datedNote) {
      var noteEl = document.createElement("span");
      noteEl.classList.add("chooser-meal-note");
      noteEl.textContent = datedNote.date + ": " + datedNote.note;
      parent.appendChild(noteEl);
    });
    parent.classList.add("notes-expanded");
  }

  function mealRow(meal, showBestScore) {
    var row = document.createElement("li");
    row.className = "chooser-meal";

    var main = document.createElement("div");
    main.className = "chooser-meal-main";

    var nameEl;
    var url = meal.source ? getMealUrl(meal.source) : null;
    if (url) {
      nameEl = document.createElement("a");
      nameEl.className = "chooser-meal-name";
      nameEl.href = url;
      if (meal.source.type === "online") {
        nameEl.rel = "nofollow";
      }
    } else {
      nameEl = document.createElement("span");
      nameEl.className = "chooser-meal-name";
    }
    nameEl.appendChild(document.createTextNode(meal.name));
    getMealIcons(meal).forEach(function (icon) {
      nameEl.appendChild(icon);
    });
    main.appendChild(nameEl);

    if (meal.peer) {
      var peerEl = document.createElement("span");
      peerEl.className = "chooser-meal-source";
      peerEl.textContent = "from " + meal.peer;
      main.appendChild(peerEl);
    }

    var tagsEl = document.createElement("span");
    tagsEl.className = "chooser-meal-tags";
    tagsEl.textContent = meal.tags.join(", ");
    main.appendChild(tagsEl);

    var metaEl = document.createElement("span");
    metaEl.className = "chooser-meal-meta";
    var meta = [];
    meta.push(meal.last_eaten === null ? "Never eaten" : "Last eaten " + meal.last_eaten.toLocaleDateString());
    if (meal.times_eaten > 0) {
      meta.push(meal.times_eaten + "×");
    }
    metaEl.textContent = meta.join(" · ");
    main.appendChild(metaEl);

    if (showBestScore) {
      var scoreEl = document.createElement("span");
      scoreEl.className = "chooser-meal-score";
      scoreEl.textContent = "Score " + bestScore(meal, bestSortReferenceDate).toFixed(1);
      main.appendChild(scoreEl);
    }

    row.appendChild(main);

    if (window.recibaseTracker && window.recibaseTracker.authenticated) {
      var logEl = document.createElement("button");
      logEl.type = "button";
      logEl.className = "chooser-log";
      logEl.textContent = "Log";
      logEl.setAttribute("aria-label", "Log " + meal.name + " for today");
      logEl.addEventListener("click", function () {
        logEl.disabled = true;
        window.recibaseTracker
          .log(meal.name)
          .then(function () {
            showStatus("Logged " + meal.name);
          })
          .catch(function (error) {
            showStatus(error.message);
          })
          .finally(function () {
            logEl.disabled = false;
          });
      });
      row.appendChild(logEl);
    }

    return row;
  }

  function renderEmpty() {
    var empty = document.createElement("div");
    empty.className = "chooser-empty";
    var message = document.createElement("p");
    message.textContent = "No meals match these filters.";
    var clear = document.createElement("button");
    clear.type = "button";
    clear.className = "chooser-link";
    clear.textContent = "Clear filters";
    clear.addEventListener("click", function () {
      setQuery("");
    });
    empty.appendChild(message);
    empty.appendChild(clear);
    return empty;
  }

  function renderSkeleton() {
    var skeleton = document.createElement("div");
    skeleton.className = "chooser-skeleton";
    for (var i = 0; i < 6; i++) {
      skeleton.appendChild(document.createElement("span"));
    }
    return skeleton;
  }

  function render() {
    var visible = filteredMeals();
    countEl.textContent = loading
      ? "Loading meals…"
      : visible.length + (visible.length === 1 ? " meal" : " meals");

    var f = parseFilters();
    renderActiveChips();
    syncTagPalette(f);
    syncDateSelect();

    listHost.innerHTML = "";
    if (loading) {
      listHost.appendChild(renderSkeleton());
      return;
    }
    if (visible.length === 0) {
      listHost.appendChild(renderEmpty());
      return;
    }

    bestSortReferenceDate = new Date();
    var showBestScore = currentSortIndex === BEST_SORT_INDEX;
    var list = document.createElement("ul");
    list.className = "chooser-results";
    visible.sort(sortingMethods[currentSortIndex].sort).forEach(function (meal) {
      list.appendChild(mealRow(meal, showBestScore));
    });
    listHost.appendChild(list);
  }

  // ---- hash persistence --------------------------------------------------

  function scheduleHash() {
    clearTimeout(hashTimer);
    hashTimer = setTimeout(saveHash, 500);
  }

  function saveHash() {
    if (query !== DEFAULT_QUERY || currentSortIndex !== BEST_SORT_INDEX) {
      location.hash = encodeURIComponent(currentSortIndex + "-" + query);
    }
  }

  function loadHash() {
    var hash = decodeURIComponent(location.hash.slice(1));
    if (!hash) {
      return;
    }
    var match = hash.match(hashStringRegex);
    if (match !== null) {
      currentSortIndex = parseInt(match[1], 10);
      query = match[2];
    } else {
      query = hash;
    }
    if (!sortingMethods[currentSortIndex]) {
      currentSortIndex = BEST_SORT_INDEX;
    }
    searchEl.value = query;
  }

  // ---- controls ----------------------------------------------------------

  searchEl.addEventListener("input", function (event) {
    query = event.target.value;
    render();
    scheduleHash();
  });

  sortEl.addEventListener("change", function () {
    currentSortIndex = parseInt(sortEl.value, 10);
    render();
    scheduleHash();
  });

  dateEl.addEventListener("change", function () {
    var kept = terms().filter(function (term) {
      return !monthFilterRegex.test(term.toLowerCase());
    });
    if (dateEl.value) {
      kept.push(dateEl.value);
    }
    setQuery(kept.join(", "));
  });

  clearEl.addEventListener("click", function () {
    setQuery("");
  });

  tagsToggleEl.addEventListener("click", function () {
    tagsWrapEl.hidden = !tagsWrapEl.hidden;
    tagsToggleEl.setAttribute("aria-expanded", String(!tagsWrapEl.hidden));
  });

  weightsToggleEl.addEventListener("click", function () {
    weightsWrapEl.hidden = !weightsWrapEl.hidden;
    weightsToggleEl.setAttribute("aria-expanded", String(!weightsWrapEl.hidden));
  });

  helpToggleEl.addEventListener("click", function () {
    helpEl.hidden = !helpEl.hidden;
    helpToggleEl.setAttribute("aria-expanded", String(!helpEl.hidden));
  });

  function buildWeightsPanel() {
    weightConfig.forEach(function (entry) {
      var row = document.createElement("div");
      row.className = "weight-row";

      var label = document.createElement("label");
      label.htmlFor = "weight-" + entry.key;
      label.textContent = entry.label;

      var slider = document.createElement("input");
      slider.type = "range";
      slider.id = "weight-" + entry.key;
      slider.min = entry.min;
      slider.max = entry.max;
      slider.step = 1;
      slider.value = weights[entry.key];

      var value = document.createElement("span");
      value.className = "weight-value";
      value.textContent = weights[entry.key];

      slider.addEventListener("input", function () {
        weights[entry.key] = parseInt(slider.value, 10);
        value.textContent = weights[entry.key];
        render();
      });

      row.appendChild(label);
      row.appendChild(slider);
      row.appendChild(value);
      weightsListEl.appendChild(row);
    });
  }

  // ---- start -------------------------------------------------------------

  buildSortOptions();
  buildWeightsPanel();
  sortEl.value = String(currentSortIndex);
  loadHash();
  render();

  fetch(mealsUrl)
    .then(function (response) {
      return response.json();
    })
    .then(function (rawMeals) {
      meals = rawMeals.map(function (meal) {
        meal.last_eaten = meal.last_eaten !== null ? new Date(meal.last_eaten) : null;
        meal.featured = meal.featured !== null ? new Date(meal.featured) : null;
        return meal;
      });
      knownTags = Array.from(
        new Set(
          meals.flatMap(function (meal) {
            return meal.tags;
          })
        )
      ).sort(function (a, b) {
        return a.localeCompare(b);
      });
      knownTags.forEach(function (tag) {
        normalisedTags.set(tag.toLowerCase(), tag);
      });
      buildTagPalette();
      loading = false;
      render();
    })
    .catch(function () {
      loading = false;
      countEl.textContent = "";
      listHost.innerHTML = "";
      var failed = document.createElement("div");
      failed.className = "chooser-empty";
      failed.textContent = "Could not load the meals. Try again in a moment.";
      listHost.appendChild(failed);
    });

  document.addEventListener("recibase:session", render);
})();
