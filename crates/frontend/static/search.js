const searchElement = document.getElementById("search");
const searchForm = document.getElementById("searchForm");
const menuItems = Array.from(document.getElementsByClassName("mdl-navigation__link"));
let searchTimer;
let apiUrl;

class OutdatedSearchResult extends Error {
  constructor(searchTerm, options) {
    super(`Outdated search result for '${searchTerm}' returned by API`, options);
  }
}


searchElement.addEventListener("input", resetSearchTimer);
searchForm.addEventListener("submit", onSearchSubmit);

fetch('/manifest.json')
  .then(resp => resp.json())
  .then(json => {
    apiUrl = json['apiUrl'];

    searchElement.disabled = false;
  });

function resetSearchTimer() {
  clearTimeout(searchTimer);

  searchTimer = setTimeout(searchRecipes, 40);
}

function searchRecipes() {
  const searchTerm = searchElement.value.trim();

  const dispatchEventId = searchTimer;
  const params = new URLSearchParams({
    "hasIngredient": searchTerm
  });

  return fetch(`${apiUrl}recipes/?${params.toString()}`)
    .then(resp => resp.json())
    .then(json => {
      if (dispatchEventId != searchTimer) {
        throw new OutdatedSearchResult(searchTerm);
      }

      return new Set(json.map(recipe => recipe['permalink']));
    })
    .then(
      urls => filterRecipes(searchTerm, urls),
      err  => {
        if (err instanceof OutdatedSearchResult) {
          console.log(err);
        } else {
          throw err;
        }
      }
    );
}

function filterRecipes(searchTerm, urls) {
  menuItems.forEach(menuItem => {
    if (menuItem.classList.contains("add-recipe")) {
      return;
    }
    const match = menuItem.text.toLowerCase().includes(searchTerm.toLowerCase())
                    || urls.has(menuItem.getAttribute('href'));

    if (match) {
      menuItem.hidden = false;
    } else {
      menuItem.hidden = true;
    }
  });
}

function onSearchSubmit(e) {
  e.preventDefault();
  clearTimeout(searchTimer);

  searchRecipes().finally(() => {
    visibleMenuItems = document.querySelectorAll(
      ".mdl-navigation__link:not(.add-recipe):not([hidden])"
    );

    if (visibleMenuItems.length == 1) {
      window.location.href = visibleMenuItems[0].getAttribute("href");
    }
  });
}
