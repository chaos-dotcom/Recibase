// Remembers the drawer's scroll position across navigations, so clicking a
// recipe keeps the list where it was instead of snapping back to the top.
(function () {
  var KEY = 'recibase-drawer-scroll';
  var drawer = document.querySelector('.mdl-layout__drawer');
  if (!drawer) return;

  function read() {
    try {
      return window.sessionStorage.getItem(KEY);
    } catch (error) {
      return null;
    }
  }

  function write() {
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

  var saved = read();
  var top = saved === null ? NaN : parseInt(saved, 10);
  if (!isNaN(top)) drawer.scrollTop = top;
  revealCurrent();

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
