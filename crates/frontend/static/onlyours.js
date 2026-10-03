// Temporary: hide Kit's and Alex's recipes so only ours (the entries the API
// marks `ours`) show. Ships with the "multiple backends" work; remove when it
// is pulled away.
(function () {
  var STORAGE_KEY = 'recibase-only-ours';
  var box = document.getElementById('onlyOurs');
  if (!box) {
    return;
  }
  var randomLink = document.querySelector('.random-recipe a');

  function apply() {
    var onlyOurs = box.checked;
    var links = document.querySelectorAll('.mdl-navigation__link[data-ours]');
    for (var i = 0; i < links.length; i++) {
      var ours = links[i].getAttribute('data-ours') === 'true';
      links[i].style.display = (onlyOurs && !ours) ? 'none' : '';
    }
    if (randomLink) {
      // Let /random draw from the same set the drawer is showing.
      randomLink.href = onlyOurs ? '/random?onlyOurs=true' : '/random';
    }
  }

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

  apply();
})();
