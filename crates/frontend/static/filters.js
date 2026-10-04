// The drawer's filters, in one place so they cannot fight over a row's
// visibility: the "show the reci-verse" toggle, the ingredient search, and the
// tag chips (which a recipe page links to as `/?tag=<tag>`). Replaces search.js
// and onlyours.js. Several tags can be active at once: a row shows if it carries
// any of them.
(function () {
  var STORAGE_KEY = 'recibase-reci-verse';
  var box = document.getElementById('reciVerse');
  var searchElement = document.getElementById('search');
  var searchForm = document.getElementById('searchForm');
  var filtersBar = document.getElementById('tagFilters');
  var statusBar = document.getElementById('filterStatus');
  var randomLink = document.querySelector('.random-recipe a');
  var rows = Array.prototype.slice.call(
    document.querySelectorAll('.mdl-navigation__link[data-ours]')
  );

  // The facets the bar offers, grouped so the chips read as categories with a
  // dashed rule between them. Only groups that have present tags are shown, so
  // the bar stays short.
  var FACETS = [
    ['Vegetarian', 'Vegan', 'Pescatarian', 'Vegetarian-ish', 'Vegan-ish', 'Gluten-Free'],
    ['Pudding', 'Lunch', 'Soup', 'Baking', 'Christmas'],
    ['Quick', 'Low Effort', 'Slow', 'High Effort', 'Scales'],
    ['Cold Weather', 'Hot Weather', 'Spicy'],
    ['Freezes', 'Better Next Day']
  ];

  var activeTags = new URLSearchParams(window.location.search).getAll('tag').filter(Boolean);
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
    // Own recipes always show; a peer's only when the reci-verse box is ticked.
    if (box && !box.checked && row.getAttribute('data-ours') !== 'true') return false;
    if (activeTags.length) {
      var tags = tagsOf(row);
      var carriesOne = activeTags.some(function (tag) {
        return tags.indexOf(tag) !== -1;
      });
      if (!carriesOne) return false;
    }
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
      // Own-only when the reci-verse box is unticked, matching the drawer.
      randomLink.href = box && !box.checked ? '/random?onlyOurs=true' : '/random';
    }
    renderChips();
    renderStatus(visible);
  }

  function chipFor(tag) {
    var active = activeTags.indexOf(tag) !== -1;
    var chip = document.createElement('button');
    chip.type = 'button';
    chip.className = 'tag-chip' + (active ? ' is-active' : '');
    chip.textContent = tag;
    chip.setAttribute('aria-pressed', active ? 'true' : 'false');
    chip.addEventListener('click', function () {
      toggleTag(tag);
    });
    return chip;
  }

  function renderChips() {
    if (!filtersBar) return;
    var present = presentTags();
    var groups = FACETS.map(function (facet) {
      return facet.filter(function (tag) {
        return present.indexOf(tag) !== -1;
      });
    }).filter(function (tags) {
      return tags.length > 0;
    });
    // An active tag always shows, even when it is not a facet or is absent.
    activeTags.forEach(function (tag) {
      var shown = groups.some(function (tags) {
        return tags.indexOf(tag) !== -1;
      });
      if (!shown) groups.push([tag]);
    });
    if (!groups.length) {
      filtersBar.hidden = true;
      filtersBar.textContent = '';
      return;
    }
    filtersBar.hidden = false;
    filtersBar.textContent = '';
    groups.forEach(function (tags, index) {
      if (index > 0) {
        var divider = document.createElement('span');
        divider.className = 'tag-divider';
        divider.setAttribute('aria-hidden', 'true');
        filtersBar.appendChild(divider);
      }
      var group = document.createElement('span');
      group.className = 'tag-group';
      tags.forEach(function (tag) {
        group.appendChild(chipFor(tag));
      });
      filtersBar.appendChild(group);
    });
  }

  function renderStatus(visible) {
    if (!statusBar) return;
    if (!activeTags.length && !searchTerm) {
      statusBar.hidden = true;
      statusBar.textContent = '';
      return;
    }
    statusBar.hidden = false;
    statusBar.textContent = '';
    var count = document.createElement('span');
    count.textContent = visible + (visible === 1 ? ' recipe' : ' recipes');
    statusBar.appendChild(count);
    if (activeTags.length) {
      var clear = document.createElement('button');
      clear.type = 'button';
      clear.className = 'filter-clear';
      clear.textContent = 'Clear';
      clear.addEventListener('click', clearTags);
      statusBar.appendChild(clear);
    }
  }

  function syncUrl() {
    var url = new URL(window.location.href);
    url.searchParams.delete('tag');
    activeTags.forEach(function (tag) {
      url.searchParams.append('tag', tag);
    });
    window.history.replaceState({}, '', url);
  }

  function toggleTag(tag) {
    var at = activeTags.indexOf(tag);
    if (at === -1) activeTags.push(tag);
    else activeTags.splice(at, 1);
    syncUrl();
    apply();
  }

  function clearTags() {
    activeTags = [];
    syncUrl();
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
