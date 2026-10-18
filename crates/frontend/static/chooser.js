// The meal Chooser, ported into this frontend from
// https://github.com/The-Silverwood-Institute/Chooser (MIT licensed).
//
// It reads the meal list from this deployment's API (`/meals/`), so the tags,
// inherited tags, last-eaten dates and usage counts are live. The query syntax,
// the weighted "Best" sort and the weight sliders are as in the original; the
// one addition is a "Log" button on each meal, shown once the admin tracker is
// signed in, which records today's dinner.
(function () {
  var root = document.getElementById("chooser");
  if (!root) {
    return;
  }

  // The frontend aggregates our meals and the peers' into one list, with the
  // usage scored from this deployment's own log.
  var mealsUrl = "/chooser/meals.json";
  var tagsSearchEl = document.getElementById("tags-search");
  var sortBoxEl = document.getElementById("sort-box");
  var sortNameEl = document.getElementById("sort-name");
  var statusEl = document.getElementById("chooser-status");
  var tagListToggleEl = document.getElementById("tags-list-toggle");
  var tagsListWrapperEl = document.getElementById("tags-list-wrapper");
  var tagsListIconEl = document.getElementById("tags-list-icon");
  var weightsListToggleEl = document.getElementById("weights-list-toggle");
  var weightsListWrapperEl = document.getElementById("weights-list-wrapper");
  var weightsListIconEl = document.getElementById("weights-list-icon");
  var weightsListEl = document.getElementById("weights-list");
  var defaultSearchQuery = tagsSearchEl.value;
  var defaultDate = new Date(0);
  var BEST_SORT_INDEX = 6;
  var defaultSortingMethodIndex = BEST_SORT_INDEX;
  var MS_PER_MONTH = 30.44 * 24 * 60 * 60 * 1000;
  var bestScoreWeightConfig = [
    { key: "timesEaten", label: "Times eaten", min: 0, max: 15, default: 5 },
    { key: "recentPenalty", label: "Recent penalty", min: 0, max: 20, default: 8 },
    { key: "featured", label: "Featured", min: 0, max: 15, default: 6 },
    { key: "newTag", label: "New tag", min: 0, max: 10, default: 4 },
    { key: "neverEaten", label: "Never tried", min: 0, max: 10, default: 3 },
    { key: "neverEatenTag", label: "Never Eaten tag", min: 0, max: 10, default: 3 },
    { key: "infrequent", label: "Infrequent tag", min: 0, max: 10, default: 2 },
    { key: "stale", label: "Staleness bonus", min: 0, max: 15, default: 4 },
    { key: "popularRediscovery", label: "Popular rediscovery", min: 0, max: 10, default: 2 },
  ];
  var bestScoreWeights = Object.fromEntries(
    bestScoreWeightConfig.map(function (entry) {
      return [entry.key, entry.default];
    })
  );
  var hashStringRegex = /^(\d+)\-(.+)$/;
  var monthFilterRegex = /^([<>])([0-9]+)m$/;
  var inactivityTimer;
  var currentSortingMethodIndex = defaultSortingMethodIndex;
  var bestSortReferenceDate = new Date();

  function showStatus(message) {
    if (!statusEl) {
      return;
    }
    statusEl.textContent = message;
    statusEl.hidden = false;
  }

  fetch(mealsUrl)
    .then(function (resp) {
      return resp.json();
    })
    .then(function (rawMeals) {
      var meals = rawMeals.map(function (meal) {
        meal.last_eaten = meal.last_eaten !== null ? new Date(meal.last_eaten) : null;
        meal.featured = meal.featured !== null ? new Date(meal.featured) : null;
        return meal;
      });
      var tags = new Set(meals.flatMap(function (meal) {
        return meal.tags;
      }));
      var normalisedTags = new Map(
        Array.from(tags).map(function (tag) {
          return [tag.toLowerCase(), tag];
        })
      );

      document.getElementById("tags-list").textContent = Array.from(tags).join(", ");

      tagsSearchEl.addEventListener("input", function (ev) {
        var searchQuery = ev.target.value;
        var searchTerms = searchQuery
          .toLowerCase()
          .split(",")
          .map(function (term) {
            return term.trim();
          })
          .filter(function (term) {
            return term;
          });
        var includeTags = searchTerms
          .map(function (term) {
            return normalisedTags.get(term);
          })
          .filter(function (res) {
            return res != undefined;
          });
        var excludeTags = searchTerms
          .filter(function (term) {
            return term.startsWith("-") || term.startsWith("!");
          })
          .map(function (term) {
            return term.slice(1);
          })
          .map(function (term) {
            return normalisedTags.get(term);
          })
          .filter(function (res) {
            return res != undefined;
          });
        var includeTagsWithInherited = searchTerms
          .filter(function (term) {
            return term.startsWith("<") && !term.startsWith("<!");
          })
          .map(function (term) {
            return term.slice(1);
          })
          .map(function (term) {
            return normalisedTags.get(term);
          })
          .filter(function (res) {
            return res != undefined;
          });
        var monthFilters = searchTerms
          .map(function (term) {
            return monthFilterRegex.exec(term);
          })
          .filter(function (m) {
            return m;
          })
          .map(function (match) {
            var d = new Date();
            d.setMonth(d.getMonth() - match[2]);
            return [match[1], d];
          });
        var beforeDates = monthFilters
          .filter(function (match) {
            return match[0] === "<";
          })
          .map(function (m) {
            return m[1];
          });
        var afterDates = monthFilters
          .filter(function (match) {
            return match[0] === ">";
          })
          .map(function (m) {
            return m[1];
          });
        var excludeTagsWithInherited = searchTerms
          .filter(function (term) {
            return term.startsWith("<!");
          })
          .map(function (term) {
            return term.slice(2);
          })
          .map(function (term) {
            return normalisedTags.get(term);
          })
          .filter(function (res) {
            return res != undefined;
          });

        renderFilteredMeals(
          includeTags,
          includeTagsWithInherited,
          excludeTags,
          excludeTagsWithInherited,
          beforeDates,
          afterDates
        );
        delayedSaveSearchQueryToHash(searchQuery);
      });

      var tagsMatchQuery = (
        tags,
        inheritedTags,
        includeTags,
        includeTagsWithInherited,
        excludeTags,
        excludeTagsWithInherited
      ) =>
        includeTags.every(function (tag) {
          return tags.includes(tag);
        }) &&
        !tags.some(function (tag) {
          return excludeTags.includes(tag);
        }) &&
        includeTagsWithInherited.every(function (tag) {
          return tags.includes(tag) || inheritedTags.includes(tag);
        }) &&
        !(
          tags.some(function (tag) {
            return excludeTagsWithInherited.includes(tag);
          }) ||
          inheritedTags.some(function (tag) {
            return excludeTagsWithInherited.includes(tag);
          })
        );

      var lastEatenMatchesQuery = (lastEaten, beforeDates, afterDates) =>
        lastEaten === null ||
        (beforeDates.every(function (date) {
          return lastEaten < date;
        }) &&
          afterDates.every(function (date) {
            return lastEaten > date;
          }));

      var renderFilteredMeals = (
        includeTags,
        includeTagsWithInherited,
        excludeTags,
        excludeTagsWithInherited,
        beforeDates,
        afterDates
      ) => {
        var listEl = document.createElement("ul");
        listEl.id = "meal-list";
        listEl.classList.add("chooser-list");

        var filterMeals = includeTags.length > 0 || excludeTags.length > 0;
        document.getElementById("requires-box").hidden =
          includeTags.length == 0 && includeTagsWithInherited.length == 0;
        document.getElementById("requires").textContent = includeTags.concat(
          includeTagsWithInherited.map(function (tag) {
            return "<" + tag;
          })
        );
        document.getElementById("excludes-box").hidden =
          excludeTags.length == 0 && excludeTagsWithInherited.length == 0;
        document.getElementById("excludes").textContent = excludeTags.concat(
          excludeTagsWithInherited.map(function (tag) {
            return "<!" + tag;
          })
        );

        var visibleMeals = !filterMeals
          ? meals
          : meals.filter(function (meal) {
              return (
                tagsMatchQuery(
                  meal.tags,
                  meal.inherited_tags,
                  includeTags,
                  includeTagsWithInherited,
                  excludeTags,
                  excludeTagsWithInherited
                ) && lastEatenMatchesQuery(meal.last_eaten, beforeDates, afterDates)
              );
            });

        document.getElementById("count").textContent = visibleMeals.length;

        bestSortReferenceDate = new Date();
        var showBestScore = currentSortingMethodIndex === BEST_SORT_INDEX;

        visibleMeals.sort(sortingMethods[currentSortingMethodIndex].sortFunction).forEach(function (meal) {
          var titleEl = document.createElement("span");
          titleEl.classList.add("chooser-meal-name");
          titleEl.textContent = meal.name;

          var tagsEl = document.createElement("span");
          tagsEl.classList.add("chooser-meal-tags");
          tagsEl.textContent = meal.tags.join(", ");

          var wrapperEl = document.createElement("span");
          wrapperEl.classList.add("chooser-meal-text");

          var iconEls = getMealIcons(meal);
          iconEls.forEach(function (iconEl) {
            titleEl.appendChild(iconEl);
          });

          var mealSourceUrl = meal.source ? getMealUrl(meal.source) : null;

          if (mealSourceUrl) {
            var linkEl = document.createElement("a");
            linkEl.href = mealSourceUrl;
            if (meal.source.type == "online") {
              linkEl.rel = "nofollow";
            }
            linkEl.appendChild(titleEl);
            wrapperEl.appendChild(linkEl);
          } else {
            wrapperEl.appendChild(titleEl);
          }

          wrapperEl.appendChild(tagsEl);

          if (meal.peer) {
            var peerEl = document.createElement("span");
            peerEl.classList.add("chooser-meal-source");
            peerEl.textContent = "from " + meal.peer;
            wrapperEl.appendChild(peerEl);
          }

          if (showBestScore) {
            var scoreEl = document.createElement("span");
            scoreEl.classList.add("chooser-meal-score");
            scoreEl.textContent = "Score: " + bestScore(meal, bestSortReferenceDate).toFixed(1);
            wrapperEl.appendChild(scoreEl);
          }

          if (meal.last_eaten !== null) {
            var lastEatenEl = document.createElement("span");
            lastEatenEl.classList.add("chooser-meal-score");
            lastEatenEl.textContent = "Last eaten: " + meal.last_eaten.toLocaleDateString();
            wrapperEl.appendChild(lastEatenEl);
          }

          if (window.recibaseTracker && window.recibaseTracker.authenticated) {
            var logEl = document.createElement("button");
            logEl.type = "button";
            logEl.classList.add("chooser-log");
            logEl.textContent = "Log";
            logEl.addEventListener("click", function () {
              window.recibaseTracker
                .log(meal.name)
                .then(function () {
                  showStatus("Logged " + meal.name);
                })
                .catch(function (error) {
                  showStatus(error.message);
                });
            });
            wrapperEl.appendChild(logEl);
          }

          var listItemEl = document.createElement("li");
          listItemEl.classList.add("chooser-meal");
          listItemEl.appendChild(wrapperEl);
          listEl.appendChild(listItemEl);
        });

        document.getElementById("meal-list").replaceWith(listEl);
      };

      if (location.hash.length > 0) {
        loadSearchQueryFromHash();
      }

      reloadPage();
      document.addEventListener("recibase:session", reloadPage);
    });

  const getMealUrl = (source) => {
    if (source.type == "recibase") {
      return "https://reciba.se/" + source.permalink;
    } else if (source.type == "google_drive") {
      return "https://drive.google.com/file/d/" + source.id + "/view";
    } else if (source.type == "online") {
      return source.url;
    } else {
      console.warn('Unknown meal source type "' + source.type + '"');
      return null;
    }
  };

  const getMealIcons = (meal) => {
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

    if (meal.dated_notes.length > 0) {
      icons.push("fa-comments");
    }

    return icons.map((iconName) => {
      var i = document.createElement("img");
      i.src = "/static/" + iconName + ".svg";
      i.classList.add("icon");
      i.setAttribute("icon-name", iconName);
      if (iconName == "fa-comments") {
        i.addEventListener("click", (event) => displayMealComments(event, meal.dated_notes));
      }
      return i;
    });
  };

  const displayMealComments = (event, datedNotes) => {
    if (!event.target.classList.contains("icon")) {
      return;
    }

    event.preventDefault();

    var parentEl = event.target.closest(".chooser-meal-text");
    if (!parentEl.classList.contains("comments-expanded")) {
      datedNotes.forEach((datedNote) => {
        var noteEl = document.createElement("span");
        noteEl.classList.add("chooser-meal-note");
        noteEl.textContent = datedNote.date + ": " + datedNote.note;
        parentEl.appendChild(noteEl);
      });
      parentEl.classList.add("comments-expanded");
    }
  };

  const displayNoticeIfStillLoading = () => {
    var stillLoadingIndicatorEl = document.getElementById("still-loading");

    if (stillLoadingIndicatorEl) {
      stillLoadingIndicatorEl.hidden = false;
    }
  };

  const delayedSaveSearchQueryToHash = (searchQuery) => {
    clearTimeout(inactivityTimer);

    inactivityTimer = setTimeout(() => saveSearchQueryToHash(searchQuery), 500);
  };

  const saveSearchQueryToHash = (searchQuery) => {
    if (searchQuery != defaultSearchQuery || currentSortingMethodIndex != defaultSortingMethodIndex) {
      var hashString = currentSortingMethodIndex + "-" + searchQuery;
      location.hash = encodeURIComponent(hashString);
    }
  };

  const loadSearchQueryFromHash = () => {
    var hashString = decodeURIComponent(location.hash.slice(1));
    var match = hashString.match(hashStringRegex);

    if (match !== null) {
      currentSortingMethodIndex = parseInt(match[1], 10);
      tagsSearchEl.value = match[2];
    } else {
      tagsSearchEl.value = hashString;
    }
  };

  const reloadPage = () => {
    tagsSearchEl.dispatchEvent(
      new Event("input", {
        bubbles: true,
        cancelable: true,
      })
    );
    sortNameEl.textContent = sortingMethods[currentSortingMethodIndex].name;
  };

  const monthsBetween = (from, to) => (to - from) / MS_PER_MONTH;

  const linearDecay = (elapsed, windowMonths) =>
    elapsed >= windowMonths ? 0 : 1 - elapsed / windowMonths;

  const bestScore = (meal, now = new Date()) => {
    if (meal.tags.includes("Not a Meal")) {
      return -Infinity;
    }

    var score = Math.log1p(meal.times_eaten) * bestScoreWeights.timesEaten;

    if (meal.featured !== null) {
      var monthsFeatured = monthsBetween(meal.featured, now);
      score += bestScoreWeights.featured * linearDecay(monthsFeatured, 6);
    }

    if (meal.last_eaten !== null) {
      var monthsSince = monthsBetween(meal.last_eaten, now);
      score -= bestScoreWeights.recentPenalty * linearDecay(monthsSince, 4);
      if (monthsSince > 6) {
        score += bestScoreWeights.stale * Math.min((monthsSince - 6) / 6, 1);
      }
      if (meal.tags.includes("Popular") && monthsSince > 3) {
        score += bestScoreWeights.popularRediscovery;
      }
    } else {
      score += bestScoreWeights.neverEaten;
    }

    if (meal.tags.includes("New")) {
      score += bestScoreWeights.newTag;
    }
    if (meal.tags.includes("Never Eaten")) {
      score += bestScoreWeights.neverEatenTag;
    }
    if (meal.tags.includes("Infrequent")) {
      score += bestScoreWeights.infrequent;
    }

    return score;
  };

  const sortByBest = (left, right) => {
    var scoreDiff = bestScore(right, bestSortReferenceDate) - bestScore(left, bestSortReferenceDate);
    if (scoreDiff !== 0) {
      return scoreDiff;
    }

    var leftDate = left.last_eaten !== null ? left.last_eaten : defaultDate;
    var rightDate = right.last_eaten !== null ? right.last_eaten : defaultDate;
    var dateDiff = leftDate - rightDate;
    if (dateDiff !== 0) {
      return dateDiff;
    }

    return left.name.localeCompare(right.name);
  };

  const sortByLastEaten = (isAscending) => (left, right) => {
    var leftDate = left.last_eaten !== null ? left.last_eaten : defaultDate;
    var rightDate = right.last_eaten !== null ? right.last_eaten : defaultDate;

    if (isAscending) {
      return leftDate - rightDate;
    } else {
      return rightDate - leftDate;
    }
  };

  const sortingMethods = [
    { name: "Last Eaten ▲", sortFunction: sortByLastEaten(true) },
    { name: "Last Eaten ▼", sortFunction: sortByLastEaten(false) },
    {
      name: "Times Eaten ▲",
      sortFunction: (left, right) => left.times_eaten - right.times_eaten,
    },
    {
      name: "Times Eaten ▼",
      sortFunction: (left, right) => right.times_eaten - left.times_eaten,
    },
    { name: "Name ▼", sortFunction: (left, right) => left.name.localeCompare(right.name) },
    { name: "Name ▲", sortFunction: (left, right) => right.name.localeCompare(left.name) },
    { name: "Best", sortFunction: sortByBest },
  ];

  const changeSortingMethod = () => {
    currentSortingMethodIndex = (currentSortingMethodIndex + 1) % sortingMethods.length;
    reloadPage();
    delayedSaveSearchQueryToHash(tagsSearchEl.value);
  };

  sortBoxEl.addEventListener("click", changeSortingMethod);

  setTimeout(displayNoticeIfStillLoading, 2000);

  tagListToggleEl.addEventListener("click", () => {
    tagsListWrapperEl.hidden = !tagsListWrapperEl.hidden;
    tagsListIconEl.textContent = tagsListWrapperEl.hidden ? "►" : "▼";
  });

  const initWeightsPanel = () => {
    bestScoreWeightConfig.forEach((entry) => {
      var rowEl = document.createElement("div");
      rowEl.classList.add("weight-row");

      var labelEl = document.createElement("label");
      labelEl.htmlFor = "weight-" + entry.key;
      labelEl.textContent = entry.label;

      var sliderEl = document.createElement("input");
      sliderEl.type = "range";
      sliderEl.id = "weight-" + entry.key;
      sliderEl.min = entry.min;
      sliderEl.max = entry.max;
      sliderEl.step = 1;
      sliderEl.value = bestScoreWeights[entry.key];

      var valueEl = document.createElement("span");
      valueEl.classList.add("weight-value");
      valueEl.textContent = bestScoreWeights[entry.key];

      sliderEl.addEventListener("input", () => {
        bestScoreWeights[entry.key] = parseInt(sliderEl.value, 10);
        valueEl.textContent = bestScoreWeights[entry.key];
        reloadPage();
      });

      rowEl.appendChild(labelEl);
      rowEl.appendChild(sliderEl);
      rowEl.appendChild(valueEl);
      weightsListEl.appendChild(rowEl);
    });
  };

  weightsListToggleEl.addEventListener("click", () => {
    weightsListWrapperEl.hidden = !weightsListWrapperEl.hidden;
    weightsListIconEl.textContent = weightsListWrapperEl.hidden ? "►" : "▼";
  });

  initWeightsPanel();
})();
