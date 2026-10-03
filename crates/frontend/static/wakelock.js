const wakelockEl = document.getElementById("wakelock");
const iconWakelockActive = 'lock';
const iconWakelockInactive = 'visibility';
const textWakelockActive = 'Remove screen lock prevention';
const textWakelockInactive = 'Remove screen lock prevention';

if ("wakeLock" in navigator & wakelockEl != null) {
    wakelockEl.classList.remove("wakelock-unsupported");
    wakelockEl.innerHTML = iconWakelockInactive;
    wakelockEl.title = textWakelockInactive;

    let wakelock = null;

    const toggleWakeLock = async () => {
        if (wakelock == null) {
            wakelock = await navigator.wakeLock.request("screen");
            wakelockEl.innerHTML = iconWakelockActive;
            wakelockEl.title = textWakelockActive;
            wakelock.addEventListener("release", () => {
                wakelock = null;
                wakelockEl.innerHTML = iconWakelockInactive;
                wakelockEl.title = textWakelockInactive;
            });
        } else {
            await wakelock.release();
        }
    };

    wakelockEl.addEventListener('click', toggleWakeLock);
};
  