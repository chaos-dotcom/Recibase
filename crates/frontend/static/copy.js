document.getElementById("copy").addEventListener("click", e => {
  const copyEl = e.currentTarget;
  const ingredientList = copyEl.getAttribute("data-ingredients");

  navigator.clipboard.writeText(ingredientList).then(() => {
    copyEl.classList.remove("is-copied");
    void copyEl.offsetWidth;
    copyEl.classList.add("is-copied");
  });
});
