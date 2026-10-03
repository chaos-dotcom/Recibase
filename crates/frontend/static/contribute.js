(function () {
  var list = document.getElementById("ingredient-rows");
  var addButton = document.getElementById("add-ingredient");
  var form = document.querySelector(".contribute-form");

  function rows() {
    return list.querySelectorAll(".ingredient-row");
  }

  function addRow(focus) {
    var clone = rows()[rows().length - 1].cloneNode(true);
    clone.querySelectorAll("input").forEach(function (input) {
      input.value = "";
    });
    list.appendChild(clone);
    if (focus) {
      var name = clone.querySelector('input[name="ingredient_name"]');
      if (name) {
        name.focus();
      }
    }
  }

  addButton.addEventListener("click", function () {
    addRow(true);
  });

  list.addEventListener("click", function (event) {
    var button = event.target.closest("[data-remove-ingredient]");
    if (!button) {
      return;
    }
    var row = button.closest(".ingredient-row");
    if (rows().length === 1) {
      row.querySelectorAll("input").forEach(function (input) {
        input.value = "";
      });
      return;
    }
    row.remove();
  });

  form.addEventListener("submit", function () {
    var button = form.querySelector('[type="submit"]');
    button.disabled = true;
    button.textContent = "Submitting…";
  });

  var saveButton = document.getElementById("save-draft");
  var deleteButton = document.getElementById("delete-draft");
  var cookieName = "recipe_draft";
  var maxAge = 60 * 60 * 24 * 7;

  function readDraft() {
    var prefix = cookieName + "=";
    var parts = document.cookie ? document.cookie.split("; ") : [];
    for (var i = 0; i < parts.length; i++) {
      if (parts[i].indexOf(prefix) === 0) {
        try {
          return JSON.parse(decodeURIComponent(parts[i].slice(prefix.length)));
        } catch (error) {
          return null;
        }
      }
    }
    return null;
  }

  function writeDraft(draft) {
    var encoded = encodeURIComponent(JSON.stringify(draft));
    if (encoded.length > 3500) {
      return false;
    }
    document.cookie = cookieName + "=" + encoded
      + "; Max-Age=" + maxAge
      + "; Path=/contribute; SameSite=Lax";
    return readDraft() !== null;
  }

  function clearDraft() {
    document.cookie = cookieName + "=; Max-Age=0; Path=/contribute; SameSite=Lax";
  }

  function fieldValue(name) {
    var field = form.elements[name];
    return field ? field.value : "";
  }

  function selectedTags() {
    var tags = [];
    form.querySelectorAll('input[name="tags"]:checked').forEach(function (input) {
      tags.push(input.value);
    });
    var diet = form.querySelector('select[name="tags"]');
    if (diet && diet.value) {
      tags.push(diet.value);
    }
    return tags;
  }

  function collectDraft() {
    return {
      name: fieldValue("name"),
      source: fieldValue("source"),
      description: fieldValue("description"),
      method: fieldValue("method"),
      notes: fieldValue("notes"),
      tags: selectedTags(),
      ingredients: Array.prototype.map.call(rows(), function (row) {
        function value(name) {
          var input = row.querySelector('input[name="' + name + '"]');
          return input ? input.value : "";
        }
        return {
          name: value("ingredient_name"),
          quantity: value("ingredient_quantity"),
          prep: value("ingredient_prep"),
          notes: value("ingredient_notes"),
        };
      }),
    };
  }

  function formIsBlank() {
    var draft = collectDraft();
    if (draft.name || draft.source || draft.description || draft.method || draft.notes) {
      return false;
    }
    if (draft.tags.length) {
      return false;
    }
    return draft.ingredients.every(function (row) {
      return !row.name && !row.quantity && !row.prep && !row.notes;
    });
  }

  function setField(name, value) {
    var field = form.elements[name];
    if (field) {
      field.value = value || "";
    }
  }

  function restoreDraft(draft) {
    setField("name", draft.name);
    setField("source", draft.source);
    setField("description", draft.description);
    setField("method", draft.method);
    setField("notes", draft.notes);

    var tags = Array.isArray(draft.tags) ? draft.tags : [];
    form.querySelectorAll('input[name="tags"]').forEach(function (input) {
      input.checked = tags.indexOf(input.value) !== -1;
    });
    var diet = form.querySelector('select[name="tags"]');
    if (diet) {
      var match = "";
      Array.prototype.forEach.call(diet.options, function (option) {
        if (option.value && tags.indexOf(option.value) !== -1) {
          match = option.value;
        }
      });
      diet.value = match;
    }

    var ingredients = Array.isArray(draft.ingredients) && draft.ingredients.length
      ? draft.ingredients
      : [{ name: "", quantity: "", prep: "", notes: "" }];
    while (rows().length < ingredients.length) {
      addRow(false);
    }
    while (rows().length > ingredients.length) {
      rows()[rows().length - 1].remove();
    }
    Array.prototype.forEach.call(rows(), function (row, index) {
      var ingredient = ingredients[index] || {};
      [
        ["ingredient_name", ingredient.name],
        ["ingredient_quantity", ingredient.quantity],
        ["ingredient_prep", ingredient.prep],
        ["ingredient_notes", ingredient.notes],
      ].forEach(function (pair) {
        var input = row.querySelector('input[name="' + pair[0] + '"]');
        if (input) {
          input.value = pair[1] || "";
        }
      });
    });
  }

  function showDelete(visible) {
    deleteButton.hidden = !visible;
  }

  var existing = readDraft();
  if (existing) {
    if (formIsBlank()) {
      restoreDraft(existing);
    }
    showDelete(true);
  }

  saveButton.addEventListener("click", function () {
    if (!writeDraft(collectDraft())) {
      window.alert("This draft is too large to save in a cookie.");
      return;
    }
    showDelete(true);
  });

  deleteButton.addEventListener("click", function () {
    clearDraft();
    showDelete(false);
    restoreDraft({});
  });
})();
