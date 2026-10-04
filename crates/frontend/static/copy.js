// Copy a recipe's ingredients. The `#copy` control only exists on a recipe
// page, but this script loads on every page, so guard before binding.
const copyEl = document.getElementById("copy");
if (copyEl) {
  copyEl.addEventListener("click", e => {
    const ingredientList = e.currentTarget.getAttribute("data-ingredients");

    navigator.clipboard.writeText(ingredientList).then(() => {
      copyEl.classList.remove("is-copied");
      void copyEl.offsetWidth;
      copyEl.classList.add("is-copied");
    });
  });
}
