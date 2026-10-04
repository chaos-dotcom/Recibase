---
name: add-recibase-recipe
description: Add or edit Recibase recipes in the Rust port. Use when the user asks to add a recipe, convert recipe text into code, type up a MealStub, or work in crates/core/src/recipes/.
---

# Add a Recibase Recipe

## Overview

Recipes are Rust modules in `crates/core/src/recipes/`, one file per Scala
`case object ... extends Recipe`, each exposing `pub fn recipe() -> RecipeDef`.

This repo is a byte-for-byte port of the Scala Recibase: every ported recipe's
JSON must equal the Scala capture (`crates/core/tests/recipe_corpus.rs`).
Recipes that are *ours* (pulled from our own folder, not ported from the
upstream Scala) carry a **chaos tag** so they can be told apart from that parity
corpus — see below.

## Workflow

1. Read 1–2 nearby recipes for style (e.g. `beef_stroganoff.rs`,
   `birthday_cake2.rs`).
2. Create `snake_case.rs` matching the `object_name` (e.g. `BeefStroganoff` →
   `beef_stroganoff.rs`).
3. Fill in `pub fn recipe() -> RecipeDef` (template below).
4. Set `created_at` with `NaiveDate::from_ymd_opt(year, month, day).unwrap()`;
   when converting a `MealStub`, use `(2020, 4, 24)`.
5. Pick tags from `Tag` in `crates/core/src/tag.rs`.
6. Verify: `cargo build -p recibase-core` and `cargo test --workspace`.

## Chaos tag

A recipe that is ours (from our own folder, not ported from Kit's or Alex's
upstream Scala) is marked with a chaos tag: a line in the module doc comment.

```rust
//! `se.reciba.api.recipes.ExampleRecipe`.
//! chaos-tag: casa-chaos
```

The marker is the literal `chaos-tag:` (`recibase_core::recipe::CHAOS_TAG`). It is
a **comment**, so it is never serialised and the byte-for-byte comparison ignores
it; `crates/core/tests/recipe_corpus.rs` additionally *skips* a tagged recipe, so
ours do not need a Scala capture.

The filename must equal `snake_case(object_name)` (e.g. `ExampleRecipe` →
`example_recipe.rs`) so the tag can be found. Do not tag a recipe that has to stay
byte-identical to the upstream Scala.

## Converting a MealStub

When typing up a stub from `crates/core/src/meal_definitions.rs`:

- Keep the stub's `name` and `tags` unless the user changes them.
- Set `created_at` to `NaiveDate::from_ymd_opt(2020, 4, 24).unwrap()`.
- **Delete the stub** from `declared_stubs()` and decrement `DECLARED_STUB_COUNT`
  (both in `meal_definitions.rs`), then update the `assert_eq!(DECLARED_STUB_COUNT, 90)`
  in `crates/core/tests/meal_definitions.rs`. Meals are `recipes()` ++ the
  declared stubs; leaving the stub in duplicates the meal
  (`no_two_meals_share_a_lowercase_name` fails).

## Template

```rust
//! `se.reciba.api.recipes.ExampleRecipe`.

use crate::recipe::RecipeDef;
use crate::tag::Tag;
use chrono::NaiveDate;

pub fn recipe() -> RecipeDef {
    RecipeDef {
        object_name: "ExampleRecipe".to_string(),
        name: "Example Recipe".to_string(),
        created_at: NaiveDate::from_ymd_opt(2026, 7, 8).unwrap(),
        permalink_override: None,
        source: None,
        description: None,
        tagline: None,
        notes: Vec::new(),
        tags: vec![Tag::Scales, Tag::LowEffort],
        image: None,
        ingredients_blocks: crate::recipe::IngredientsBlock::simple(vec![
            crate::recipe::Ingredient::q("Onion", "1"),
            crate::recipe::Ingredient::q("Salt", "Pinch"),
        ]),
        method: vec![
            format!("Preheat the oven to {}.", crate::utils::int_utils::celsius(180)),
            "Serve immediately.".to_string(),
        ],
    }
}
```

## Required fields (`RecipeDef`)

| Field | Notes |
|-------|-------|
| `object_name` | Scala case object name; the `edit` link filename |
| `name` | Display name; the permalink is derived from it automatically unless overridden |
| `created_at` | `NaiveDate::from_ymd_opt(y, m, d).unwrap()` |
| `tags` | `vec![Tag::…]` — insertion order is kept; the wire order is applied by `scala_hash` |
| `ingredients_blocks` | `Vec<IngredientsBlock>` |
| `method` | `Vec<String>`, one step per string |

## Optional fields

- `source` — attribution string (`Some("Gousto".to_string())`), not a URL; URLs
  belong on MealStubs as `Source::Online(...)`.
- `description`, `tagline` — `Option<String>`.
- `notes` — `Vec<String>` of extra tips (HTML links allowed).
- `image` — `Some(crate::recipe::Image::new("https://i.reciba.se/…"))`.
- `permalink_override` — `Some("custom-slug".to_string())` only when the Scala
  overrides `permalink` (or the auto slug is wrong); otherwise `None`, which
  makes `RecipeDef::permalink()` use `from_raw_string(name)`.

Ice cream recipes reuse the shared mixin in `crates/core/src/ice_cream.rs`
(`generic_ingredients`, `generic_notes`, `generic_method_start`,
`generic_method_end`) — there is no `IceCream` trait.

## Ingredients

### Single section

```rust
ingredients_blocks: crate::recipe::IngredientsBlock::simple(vec![
    crate::recipe::Ingredient::q("Onion", "1"),
    crate::recipe::Ingredient::q("Plain Flour", "200g"),
]),
```

### Multiple sections

Order sections as the user requests (e.g. cake before topping):

```rust
ingredients_blocks: vec![
    crate::recipe::IngredientsBlock::new(Some("Cake"), vec![
        crate::recipe::Ingredient::q("Plain Flour", "375g"),
    ]),
    crate::recipe::IngredientsBlock::new(Some("Topping"), vec![
        crate::recipe::Ingredient::q("Walnuts", "60g"),
    ]),
],
```

### Ingredient constructors

`Ingredient` fields are `name: String` and `quantity` / `prep` / `notes` as
`Option<String>`. Use the helpers where they fit; fall back to `opt` for any
other combination:

```rust
Ingredient::q("Onion", "1")                                // name + quantity
Ingredient::qp("Cream Cheese", "250g", "soft")             // + prep
Ingredient::qpn("Beetroot", "5", "peeled", "approximate")  // + notes
Ingredient::opt("Honey", Some("1 tbsp"), None, Some("Optional"))
Ingredient::new("Salt")                                    // name only
```

**Common mistake:** `q`/`qp`/`qpn` take `&str`, not `Option`, and there is no
named-argument form. For an ingredient with a missing field use `opt`:
`Ingredient::opt("Stock Cube", None, None, Some("Or 1 tbsp Bovril"))`. Do not
invent a quantity you do not have.

## Method steps

- One step per `vec!` element.
- Use British spelling (`centre`, `flavour`) to match existing recipes.
- Write fractions as `1/2`, `1/4` — not `½`.
- Prefix steps by section when helpful: `"Topping: …"`, `"Cake: …"`.
- For oven temperatures use `crate::utils::int_utils`:

```rust
use crate::utils::int_utils::{celsius, simple_fan_instruction};
format!("Preheat the oven to {}.", celsius(180)) // "180°C (355°F, gas mark 4)"
```

## Tags

Pick from existing tags only (`crates/core/src/tag.rs`). The whole vocabulary:

| Category | Tags |
|----------|------|
| Meal type | `Pudding`, `Lunch`, `Baking`, `Soup`, `Christmas`, `NonMeal` |
| Dietary | `Vegan`, `VeganIsh`, `Vegetarian`, `VegetarianIsh`, `Pescatarian`, `GlutenFree` |
| Vibe | `Stodge`, `Spicy`, `ColdWeather`, `HotWeather` |
| Effort | `Slow`, `Quick`, `Scales`, `HighEffort`, `LowEffort` |
| Storage | `Freezes`, `BetterNextDay` |
| Marker | `AI` — renders the "carefully review this, AI was used" warning on the page |

Do **not** add `NeverEaten`, `Popular`, `Infrequent`, or `New` — those are
applied automatically from usage data.

Dietary tags are a chain and the parents are inherited: set only the most
specific one. `Tag::Vegan` alone yields `VeganIsh`, `Vegetarian`,
`VegetarianIsh` and `Pescatarian` in `inherited_tags` automatically
(`Tag::all_parent_tags`); never list a parent explicitly. The chain is
`Vegan` → `VeganIsh` → `Vegetarian` → `VegetarianIsh` → `Pescatarian`, and
`GlutenFree` and `Pescatarian` are standalone (no parent).

Meat dishes use `VegetarianIsh` (as the upstream corpus does), not
`Vegetarian`.

Baking/puddings use `Pudding` + `Baking` + `Vegetarian` when egg/dairy only.

## Conventions

- Match naming and tone of neighbouring recipes; don't over-comment.
- Put ingredient alternatives or cut guidance in the ingredient `notes`, not in
  `method`.
- Keep the diff to one new file unless editing an existing recipe or converting
  a MealStub.
- `object_name` = Scala case object = filename stem of the `edit` link.

## Examples in this repo

- Simple savoury: `beef_stroganoff.rs`, `chilli_con_carne.rs`
- Multi-section baking: `crunch_chocolate_chip_coffee_cake.rs`, `birthday_cake2.rs`
- With source/notes/image: `brownies.rs`
- Ice cream: `pistachio_ice_cream.rs` (+ the shared `ice_cream.rs` mixin)
