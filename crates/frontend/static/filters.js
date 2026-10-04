// The drawer's filters, in one place so they cannot fight over a row's
// visibility: the "only ours" switch, the ingredient search, and the tag chips
// (which a recipe page links to as `/?tag=<tag>`). Replaces search.js and
// onlyours.js.
(function () {
  var STORAGE_KEY = 'recibase-only-ours';
  var box = document.getElementById('onlyOurs');
  var searchElement = document.getElementById('search');
  var searchForm = document.getElementById('searchForm');
  var filtersBar = document.getElementById('tagFilters');
  var statusBar = document.getElementById('filterStatus');
  var randomLink = document.querySelector('.random-recipe a');
  var rows = Array.prototype.slice.call(
    document.querySelectorAll('.mdl-navigation__link[data-ours]')
  );

  // Curated facets, in reading order. Only the ones present in the list are
  // shown, so the bar stays short.
  var PREFERRED = [
    'Vegetarian', 'Vegan', 'Pescatarian', 'Vegetarian-ish', 'Vegan-ish', 'Gluten-Free',
    'Made of Meat',
    'Quick', 'Low Effort', 'Slow', 'High Effort', 'Scales',
    'Pudding', 'Lunch', 'Soup', 'Baking', 'Christmas',
    'Cold Weather', 'Hot Weather', 'Spicy', 'Freezes', 'Better Next Day'
  ];

  var activeTag = new URLSearchParams(window.location.search).get('tag') || null;
  var searchTerm = '';
  var matchedPermalinks = null;
  var apiUrl = null;
  var searchTimer;

  function tagsOf(row) {
    var raw = row.getAttribute('data-tags');
    return raw ? raw.split('|') : [];
  }

  function presentTags() {
    var present = [];
    rows.forEach(function (row) {
      tagsOf(row).forEach(function (tag) {
        if (tag && present.indexOf(tag) === -1) present.push(tag);
      });
    });
    return present;
  }

  function rowVisible(row) {
    if (box && box.checked && row.getAttribute('data-ours') !== 'true') return false;
    if (activeTag && tagsOf(row).indexOf(activeTag) === -1) return false;
    if (searchTerm) {
      var byName = row.textContent.toLowerCase().indexOf(searchTerm.toLowerCase()) !== -1;
      var byIngredient = matchedPermalinks && matchedPermalinks.has(row.getAttribute('href'));
      if (!byName && !byIngredient) return false;
    }
    return true;
  }

  function apply() {
    var visible = 0;
    rows.forEach(function (row) {
      var show = rowVisible(row);
      row.hidden = !show;
      if (show) visible += 1;
    });
    if (randomLink) {
      randomLink.href = box && box.checked ? '/random?onlyOurs=true' : '/random';
    }
    renderChips();
    renderStatus(visible);
  }

  function renderChips() {
    if (!filtersBar) return;
    var present = presentTags();
    var chips = PREFERRED.filter(function (tag) {
      return present.indexOf(tag) !== -1;
    });
    // The active tag always gets a chip, even if it is not a preferred facet.
    if (activeTag && chips.indexOf(activeTag) === -1) chips.push(activeTag);
    if (!chips.length) {
      filtersBar.hidden = true;
      filtersBar.textContent = '';
      return;
    }
    filtersBar.hidden = false;
    filtersBar.textContent = '';
    chips.forEach(function (tag) {
      var chip = document.createElement('button');
      chip.type = 'button';
      chip.className = 'tag-chip' + (tag === activeTag ? ' is-active' : '');
      chip.textContent = tag;
      chip.setAttribute('aria-pressed', tag === activeTag ? 'true' : 'false');
      chip.addEventListener('click', function () {
        setActiveTag(tag === activeTag ? null : tag);
      });
      filtersBar.appendChild(chip);
    });
  }

  function renderStatus(visible) {
    if (!statusBar) return;
    if (!activeTag && !searchTerm) {
      statusBar.hidden = true;
      statusBar.textContent = '';
      return;
    }
    statusBar.hidden = false;
    statusBar.textContent = '';
    var count = document.createElement('span');
    count.textContent = visible + (visible === 1 ? ' recipe' : ' recipes');
    statusBar.appendChild(count);
    if (activeTag) {
      var clear = document.createElement('button');
      clear.type = 'button';
      clear.className = 'filter-clear';
      clear.textContent = 'Clear';
      clear.addEventListener('click', function () {
        setActiveTag(null);
      });
      statusBar.appendChild(clear);
    }
  }

  function setActiveTag(tag) {
    activeTag = tag;
    var url = new URL(window.location.href);
    if (tag) url.searchParams.set('tag', tag);
    else url.searchParams.delete('tag');
    window.history.replaceState({}, '', url);
    apply();
  }

  function runSearch() {
    searchTerm = searchElement ? searchElement.value.trim() : '';
    if (!searchTerm || !apiUrl) {
      matchedPermalinks = null;
      apply();
      return Promise.resolve();
    }
    var dispatchId = searchTimer;
    var params = new URLSearchParams({ hasIngredient: searchTerm });
    return fetch(apiUrl + 'recipes/?' + params.toString())
      .then(function (resp) {
        return resp.json();
      })
      .then(function (json) {
        if (dispatchId !== searchTimer) return; // a newer keystroke won
        matchedPermalinks = new Set(
          json.map(function (recipe) {
            return recipe['permalink'];
          })
        );
        apply();
      })
      .catch(function () {
        // Leave the previous match set in place.
      });
  }

  function onSearchInput() {
    clearTimeout(searchTimer);
    searchTimer = setTimeout(runSearch, 40);
  }

  function onSearchSubmit(event) {
    event.preventDefault();
    clearTimeout(searchTimer);
    runSearch().then(function () {
      var visible = rows.filter(function (row) {
        return !row.hidden;
      });
      if (visible.length === 1) {
        window.location.href = visible[0].getAttribute('href');
      }
    });
  }

  if (searchElement) {
    searchElement.addEventListener('input', onSearchInput);
  }
  if (searchForm) {
    searchForm.addEventListener('submit', onSearchSubmit);
  }

  if (box) {
    try {
      box.checked = window.localStorage.getItem(STORAGE_KEY) === 'true';
    } catch (error) {
      // Private mode or a blocked storage: the filter just does not persist.
    }
    box.addEventListener('change', function () {
      try {
        window.localStorage.setItem(STORAGE_KEY, box.checked ? 'true' : 'false');
      } catch (error) {
        // As above.
      }
      apply();
    });
  }

  fetch('/manifest.json')
    .then(function (resp) {
      return resp.json();
    })
    .then(function (json) {
      apiUrl = json['apiUrl'];
      if (searchElement) searchElement.disabled = false;
    })
    .catch(function () {
      // Search stays disabled if the manifest cannot be read.
    });

  apply();
})();
