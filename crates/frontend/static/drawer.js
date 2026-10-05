// Remembers the drawer's scroll position across navigations, so clicking a
// recipe keeps the list where it was instead of snapping back to the top.
(function () {
  var KEY = 'recibase-drawer-scroll';
  var drawer = document.querySelector('.mdl-layout__drawer');
  if (!drawer) return;

  // MDL rewrites the layout into a fresh container on `load`, which resets the
  // drawer's scroll. The restore therefore waits for that to finish, and saving
  // stays quiet until then, so the transient reset cannot overwrite the value.
  var ready = false;

  function read() {
    try {
      return window.sessionStorage.getItem(KEY);
    } catch (error) {
      return null;
    }
  }

  function write() {
    if (!ready) return;
    try {
      window.sessionStorage.setItem(KEY, String(drawer.scrollTop));
    } catch (error) {
      // Private mode or a blocked storage: the position just does not persist.
    }
  }

  // Nudge the current recipe into view, so a direct load (a shared link, or a
  // first visit) does not leave it somewhere off-screen.
  function revealCurrent() {
    var current = drawer.querySelector('.mdl-navigation__link.is-current');
    if (!current) return;
    var top = current.offsetTop;
    var bottom = top + current.offsetHeight;
    if (top < drawer.scrollTop) {
      drawer.scrollTop = top;
    } else if (bottom > drawer.scrollTop + drawer.clientHeight) {
      drawer.scrollTop = bottom - drawer.clientHeight;
    }
  }

  function restore() {
    var saved = read();
    var top = saved === null ? NaN : parseInt(saved, 10);
    if (!isNaN(top)) drawer.scrollTop = top;
    revealCurrent();
    ready = true;
  }

  // Deferred scripts run before `load`, which is when MDL moves the layout, so
  // restoring has to wait for that event. An already-loaded document (a cached
  // script) restores immediately.
  if (document.readyState === 'complete') restore();
  else window.addEventListener('load', restore);

  var queued = false;
  drawer.addEventListener('scroll', function () {
    if (queued) return;
    queued = true;
    window.requestAnimationFrame(function () {
      queued = false;
      write();
    });
  });
  window.addEventListener('pagehide', write);
})();
