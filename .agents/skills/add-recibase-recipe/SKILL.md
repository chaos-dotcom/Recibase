---
name: add-recibase-recipe
description: Add or edit Recibase recipes. Use when the user asks to add a recipe, convert recipe text into markdown, type up a MealStub, or work in crates/core/recipes/.
---

# Add a Recibase Recipe

## Overview

Every recipe is one markdown file in `crates/core/recipes/`. Nothing else is
needed to add one: `crates/core/build.rs` finds the files at compile time and
compiles them in, so there is no Rust to write and no registry to regenerate.

`crates/core/src/md.rs` defines the format and parses it. The recipe's JSON is
compared against the Scala capture by `crates/core/tests/recipe_corpus.rs`, so a
recipe ported from the upstream Scala must encode byte for byte as it did.

## Workflow

1. Read 1–2 nearby files for style (e.g. `dahl.md`,
   `mexican_polenta_pie.md`).
2. Copy one and edit it, or write a new `snake_case.md`.
3. Verify: `cargo test -p recibase-core` and `cargo test --workspace`.

The filename must be `snake_case(object_name)` (`BeefStroganoff` →
`beef_stroganoff.md`): it names the generated constant holding the file.

## The file

```markdown
---
object_name: ExampleRecipe
created_at: 2020-04-24
permalink: example-recipe
source: Gousto
description: One sentence for the page.
tagline: A short aside.
image: https://i.reciba.se/example.jpg
tags: Vegetarian, Low Effort
chaos: true
---

# Example Recipe

## Ingredients

| Ingredient | Quantity | Prep | Notes |
| --- | --- | --- | --- |
| Onion | 1 | chopped | Optional |
| Salt |

### Topping

| Ingredient | Quantity | Prep | Notes |
| --- | --- | --- | --- |
| Cheese | 100g | grated |

## Method

1. Chop the onion.
2. Sprinkle the cheese over the top.

## Notes

- A tip.
```

### Blank template

Copy this for a new recipe. Only `object_name` and `created_at` are required;
delete the optional keys and the `### Section` / `## Notes` parts you do not
need.

```markdown
---
object_name: ExampleRecipe
created_at: 2020-04-24
tags:
---

# Example Recipe

## Ingredients

| Ingredient | Quantity | Prep | Notes |
| --- | --- | --- | --- |
|  |  |  |  |

## Method

1. First step.
```

### Frontmatter

| Key | Required | Notes |
|-----|----------|-------|
| `object_name` | yes | The Scala case object name; also the `edit` link's filename |
| `created_at` | yes | `YYYY-MM-DD` |
| `tags` | no | Comma-separated; must be exact `entry_name`s from `crates/core/src/tag.rs` (e.g. `Cold Weather`, not `ColdWeather`) |
| `permalink` | no | Only when the slug from the name is wrong; otherwise omit |
| `source`, `description`, `tagline` | no | Plain text; a URL in `source` is rendered as a link |
| `image` | no | URL; 1080×1080 unless `image_width`/`image_height` say otherwise |
| `chaos` | no | `true` marks a recipe as ours - see below |

Omit a key rather than writing an empty value: absent means JSON `null`.

The `# Title` is the recipe's `name`; it is not repeated in the frontmatter.

### Ingredients

One table per section; a `### Name` heading makes a named section (`Base`,
`Topping`, `Dressing`). A recipe with no named sections has a single table
directly under `## Ingredients`. A recipe can also mix the two: a table before
any heading is the unnamed section (`fish_finger_katsu.md`).

The four columns are the four fields of an ingredient, which is what keeps them
separable - the scaler parses `Quantity`, `hasIngredient` matches `Ingredient`,
and the clipboard merges on `Ingredient` + `Quantity`:

- **Ingredient** - the name; required.
- **Quantity** - `200g`, `1 tbsp`, `1 x 400g tin`, `A decent slosh`.
- **Prep** - `chopped`, `finely diced`.
- **Notes** - `Optional`, `Quantity is approximate`.

Trailing empty cells can be left off: `| Salt |` is a name-only ingredient.
Do not invent a quantity you do not have.

### Method and notes

The method is a numbered list, one step per item. Notes are a bullet list.
Both are **Markdown**, rendered by the frontend (`crates/frontend/src/markdown.rs`)
with the `markdown` crate.

- A step is rendered inside an `<li>` and a note inside a `<p>`, so keep each one
  as prose: a line starting `- `, `# ` or `1. ` is block syntax and nests a list
  or heading inside the wrapper.
- Links are `[text](url)`. Raw HTML still renders (it is trusted, and it is what
  the scaler injects), but Markdown is the convention.
- A continuation line must be indented by at least one space (three under a
  method step and two under a note line up with the text). An unindented
  continuation line is a build error.
- For oven temperatures, write the instruction out (`200°C (390°F, gas mark 6)`)
  - the previous Rust `celsius(200)` helper is gone.

### Characters that need care

- `|` inside a cell is written `\|`, in a table cell **and** in a frontmatter
  value.
- A leading or trailing space is written `&#32;`, for the same reason (a table
  parser trims them). The corpus contains a few real trailing spaces which the
  API returns, so do not "fix" them.
- `&amp;` writes a literal `&`; a bare `&` is fine, but use the escape if the
  text could otherwise look like an escape.
- An empty frontmatter value (`description:`) means the key is absent.

## The chaos marker

A recipe that is ours (from our own folder, not ported from Kit's or Alex's
upstream Scala) sets `chaos: true` in its frontmatter.

It is not part of the recipe's JSON, so it never reaches the API; the
byte-for-byte comparison *skips* a marked recipe, because ours have no Scala
capture. Do not mark a recipe that has to stay byte-identical to the upstream
Scala.

## Converting a MealStub

When typing up a stub from `crates/core/src/meal_definitions.rs`:

- Keep the stub's `name` and `tags` unless the user changes them.
- Set `created_at` to `2020-04-24`.
- **Delete the stub** from `declared_stubs()` and decrement `DECLARED_STUB_COUNT`
  (both in `meal_definitions.rs`), then update the `assert_eq!(DECLARED_STUB_COUNT, 90)`
  in `crates/core/tests/meal_definitions.rs`. Meals are `recipes()` ++ the
  declared stubs; leaving the stub in duplicates the meal
  (`no_two_meals_share_a_lowercase_name` fails).

## Tags

Pick from existing tags only (`crates/core/src/tag.rs`). The whole vocabulary:

| Category | Tags |
|----------|------|
| Meal type | `Pudding`, `Lunch`, `Baking`, `Soup`, `Christmas`, `Not a Meal` |
| Dietary | `Vegan`, `Vegan-ish`, `Vegetarian`, `Vegetarian-ish`, `Pescatarian`, `Gluten-Free` |
| Dietary, Stephani's | `StephaniUnhealthy`, `Stephani-ish`, `Stephani` |
| Vibe | `Stodge`, `Spicy`, `Cold Weather`, `Hot Weather` |
| Effort | `Slow`, `Quick`, `Scales`, `High Effort`, `Low Effort` |
| Storage | `Freezes`, `Better Next Day` |
| Marker | `AI` - renders the "carefully review this, AI was used" warning on the page |

`It's Made of Meat` also exists, but no recipe sets it today.

Do **not** add `Never Eaten`, `Popular`, `Infrequent`, or `New` - those are
applied automatically from usage data.

Dietary tags are a chain and the parents are inherited: set only the most
specific one. `Vegan` alone yields `Vegan-ish`, `Vegetarian`, `Vegetarian-ish`
and `Pescatarian` in `inherited_tags` automatically. Never list a parent
explicitly. The chain is `Vegan` → `Vegan-ish` → `Vegetarian` → `Vegetarian-ish`
→ `Pescatarian`, and `Gluten-Free` and `Pescatarian` are standalone.

Meat dishes use `Vegetarian-ish` (as the upstream corpus does), not
`Vegetarian`. Baking/puddings use `Pudding` + `Baking` + `Vegetarian` when
egg/dairy only.

## Changing a ported recipe

`crates/core/tests/recipe_corpus.rs` requires every ported recipe to encode byte
for byte as the Scala capture did. Editing a ported recipe's prose or amounts
breaks that on purpose, so record the difference in
`crates/core/tests/fixtures/intentional_differences.txt`:

```
RoastBeetrootDahl notes  # markdown link
```

Name the object and the JSON field(s) that may differ. Everything else in that
recipe is still compared byte for byte, and the test fails both when a difference
is not listed and when a listed difference is no longer needed.

## Conventions

- Match naming and tone of neighbouring recipes; don't over-comment.
- Put ingredient alternatives or cut guidance in the ingredient Notes column,
  not in the method.
- Keep the diff to one file unless editing an existing recipe or converting a
  MealStub.

## Examples in this repo

- Simple savoury: `dahl.md`, `easy_pancakes.md`
- Multi-section: `mexican_polenta_pie.md`, `creamy_leek_croustade.md`
- With source/description/image: `blue_cheese_gnocchi.md`, `beetroot_risotto.md`
- Our own (marked `chaos: true`): `miso_tofu_aubergine.md`, `kimchi_noodles.md`
