//! `core/src/meal_definitions.rs` against the Scala source it is transcribed
//! from (`Recibase/src/main/scala/se/reciba/api/recibase/MealDefinitions.scala`)
//! and against the duplicate-name check `MealsController` runs when its
//! companion object initialises.
//!
//! OWNER: workstream W5.

use recibase_core::meal::{MealStub, Source};
use recibase_core::meal_definitions::{declared_stubs, meal_stubs, same_meal, DECLARED_STUB_COUNT};
use recibase_core::recipes::recipes;
use recibase_core::tag::Tag;

/// The tags of a stub as a Scala `Set` would compare them: sorted, de-duplicated.
fn tag_set(stub: &MealStub) -> Vec<Tag> {
    let mut tags = stub.tags.clone();
    tags.sort();
    tags.dedup();
    tags
}

/// `name + tags + source + createdAt`, the four fields of the `MealStub` case
/// class, with the tags taken as a `Set`.
fn stub_key(stub: &MealStub) -> String {
    format!(
        "{}|{:?}|{:?}|{:?}",
        stub.name,
        tag_set(stub),
        stub.source,
        stub.created_at
    )
}

#[test]
fn declared_stub_count_is_the_number_of_scala_stub_literals() {
    // `MealStub(` occurs 90 times in MealDefinitions.scala, and every one of
    // those is a stub literal. The three other mentions of `MealStub` in that
    // file - the import, the `Set[MealStub]` type annotation and
    // `MealStub.apply` - are not followed by `(`.
    assert_eq!(DECLARED_STUB_COUNT, 90);
    assert_eq!(declared_stubs().len(), DECLARED_STUB_COUNT);
}

#[test]
fn no_two_meals_share_a_lowercase_name() {
    // MealsController's class-initialisation check throws
    // "Duplicate meal names: ..." when two meals differ only in case.
    let mut seen: Vec<String> = Vec::new();
    for meal in meal_stubs() {
        let key = meal.name.to_lowercase();
        assert!(
            !seen.contains(&key),
            "duplicate meal name (lowercased): {}",
            meal.name
        );
        seen.push(key);
    }
    assert_eq!(seen.len(), meal_stubs().len());
}

#[test]
fn recibase_sourced_meals_point_at_their_recipe_permalink() {
    let mut seen = 0usize;
    for meal in meal_stubs() {
        if let Some(Source::Recibase(permalink)) = &meal.source {
            seen += 1;
            let recipe = recipes()
                .iter()
                .find(|r| r.name == meal.name)
                .unwrap_or_else(|| panic!("no recipe named {}", meal.name));
            assert_eq!(&recipe.permalink(), permalink, "permalink of {}", meal.name);
            assert_eq!(
                Some(recipe.created_at),
                meal.created_at,
                "created_at of {}",
                meal.name
            );
            assert_eq!(
                tag_set(&MealStub::from_recipe(recipe)),
                tag_set(meal),
                "tags of {}",
                meal.name
            );
        }
    }
    assert_eq!(
        seen,
        recipes().len(),
        "one Recibase-sourced meal per recipe"
    );
}

#[test]
fn meal_stubs_is_one_entry_per_recipe_plus_the_declared_stubs() {
    let all = meal_stubs();
    let declared = declared_stubs();
    let corpus = recipes();

    assert_eq!(
        all.len(),
        corpus.len() + declared.len(),
        "the union must neither lose nor invent entries"
    );

    for recipe in corpus {
        assert!(
            all.iter()
                .any(|m| m.name == recipe.name
                    && m.source == Some(Source::Recibase(recipe.permalink()))),
            "recipe stub missing: {}",
            recipe.name
        );
    }
    for stub in declared {
        assert!(
            all.iter().any(|m| same_meal(m, stub)),
            "declared stub missing: {}",
            stub.name
        );
    }
}

#[test]
fn declared_stubs_never_use_a_recibase_source_and_have_no_created_at() {
    // Every hand-written stub is `MealStub(name, tags)` with an `Online` or
    // `GoogleDrive` source, or no source at all; `Recibase(...)` and
    // `createdAt` are only ever produced by `MealStub.apply(recipe)`.
    for stub in declared_stubs() {
        assert!(
            !matches!(stub.source, Some(Source::Recibase(_))),
            "declared stub with a Recibase source: {}",
            stub.name
        );
        assert_eq!(
            stub.created_at, None,
            "declared stub with a createdAt: {}",
            stub.name
        );
    }
}

#[test]
fn declared_stubs_are_pairwise_distinct() {
    // The hand-written entries are the elements of one Scala `Set(...)`
    // literal: a repeated element would silently disappear from that set.
    let mut keys: Vec<String> = declared_stubs().iter().map(stub_key).collect();
    let before = keys.len();
    keys.sort();
    keys.dedup();
    assert_eq!(keys.len(), before, "the Scala Set literal repeats a stub");
}

/// Twenty stubs copied out of the Scala text, in the order they appear there:
/// indices 0, 4, 9, 14, 19, 24, 29, 34, 39, 44, 49, 54, 59, 64, 69, 74, 79,
/// 84, 87 and 89 of the 90 `MealStub(...)` literals. The source is the Scala
/// constructor name plus its argument, or `None` for the two-argument
/// `MealStub(name, tags)` form.
#[rustfmt::skip]
const SAMPLE: &[(&str, &[Tag], Option<(&str, &str)>)] = &[
    (
        "Apple & Sausage Filo Casserole",
        &[Tag::Vegan, Tag::Scales, Tag::Slow],
        Some(("Online", "https://www.bbc.co.uk/food/recipes/vegan_apple_and_sausage_48574")),
    ),
    (
        "Baked potatoes",
        &[Tag::Slow, Tag::Vegetarian, Tag::ColdWeather, Tag::LowEffort],
        None,
    ),
    (
        "Carbonara",
        &[Tag::VegetarianIsh, Tag::Quick, Tag::Scales],
        None,
    ),
    (
        "Cod in tomato sauce",
        &[Tag::Pescatarian, Tag::Quick],
        Some(("GoogleDrive", "1jiyXj9imR452VqEE299DlRU1ao5K-gVi")),
    ),
    (
        "Full Scottish Breakfast",
        &[Tag::VegetarianIsh, Tag::HighEffort, Tag::Quick, Tag::Stodge],
        None,
    ),
    (
        "Grilled aubergine",
        &[Tag::Quick, Tag::Vegan, Tag::LowEffort],
        None,
    ),
    (
        "Homemade Pizza",
        &[Tag::HighEffort, Tag::Slow, Tag::Vegetarian, Tag::Stodge],
        None,
    ),
    (
        "Vegetarian Meatballs",
        &[Tag::Scales, Tag::Vegetarian],
        None,
    ),
    (
        "Mozzarella & spinach pancakes",
        &[Tag::HighEffort, Tag::Vegetarian],
        Some(("GoogleDrive", "1L7CueEwwXy6ObWx7A2AjIxO4FdsI-YB_")),
    ),
    (
        "N Bean Chilli",
        &[Tag::Vegan, Tag::Freezes, Tag::BetterNextDay, Tag::Slow, Tag::Scales],
        None,
    ),
    (
        "Penne with Walnut sauce",
        &[Tag::Quick, Tag::Vegetarian, Tag::HotWeather, Tag::LowEffort],
        Some(("GoogleDrive", "1-dIvRInPYyiUT1y6Y6cb7bZpnabubuIe")),
    ),
    (
        "Roast Aubergine & Basil Risotto",
        &[Tag::Vegetarian],
        Some(("Online", "https://www.gousto.co.uk/cookbook/recipes/tomato-risotto-with-crispy-roast-aubergine")),
    ),
    (
        "Satay Sweet Potato Curry",
        &[Tag::Vegan, Tag::Scales, Tag::Spicy, Tag::Quick, Tag::BetterNextDay],
        Some(("Online", "https://www.bbc.co.uk/food/recipes/satay_sweet_potato_curry_59527")),
    ),
    (
        "Shahi Paneer",
        &[Tag::Vegetarian, Tag::Spicy, Tag::Scales],
        Some(("Online", "https://www.indianhealthyrecipes.com/shahi-paneer-recipe/")),
    ),
    (
        "Spicy Chilli Paneer Noodles",
        &[Tag::Vegetarian, Tag::Scales, Tag::Quick, Tag::Spicy],
        Some(("Online", "https://www.gousto.co.uk/cookbook/vegetarian-recipes/10-min-spicy-chilli-paneer-noodles")),
    ),
    (
        "Sweet Potato, Peanut Butter and Coconut Curry",
        &[Tag::Vegetarian, Tag::HotWeather],
        None,
    ),
    (
        "Tomato sauce",
        &[Tag::Freezes, Tag::Scales, Tag::Slow, Tag::BetterNextDay, Tag::LowEffort, Tag::VeganIsh],
        None,
    ),
    (
        "Tuna steaks with salsa verde",
        &[Tag::Pescatarian, Tag::Stephani],
        Some(("Online", "https://www.bbc.co.uk/food/recipes/tunasteakswithsalsav_74789")),
    ),
    (
        "Vegetable Sambar",
        &[Tag::Vegetarian, Tag::Spicy, Tag::Scales],
        Some(("Online", "https://www.gousto.co.uk/cookbook/vegan-recipes/fragrant-vegetable-sambar-with-coconut-yoghurt")),
    ),
    (
        "Venetian Style Pasta",
        &[Tag::Quick, Tag::Vegan, Tag::Scales, Tag::HotWeather, Tag::LowEffort],
        Some(("Online", "https://www.bbcgoodfood.com/recipes/12135/venetianstyle-pasta")),
    ),
];

#[test]
fn sample_of_twenty_stubs_matches_the_scala_text() {
    let mut kinds = Vec::new();
    for (name, tags, source) in SAMPLE {
        let stub = declared_stubs()
            .iter()
            .find(|s| s.name == *name)
            .unwrap_or_else(|| panic!("no declared stub named {}", name));
        assert_eq!(stub.tags.as_slice(), *tags, "tags of {}", name);
        let expected = match source {
            None => None,
            Some(("Online", url)) => Some(Source::Online((*url).to_string())),
            Some(("GoogleDrive", id)) => Some(Source::GoogleDrive((*id).to_string())),
            Some(other) => panic!("bad sample source {:?}", other),
        };
        assert_eq!(stub.source, expected, "source of {}", name);
        kinds.push(source.map(|(kind, _)| kind));
    }
    assert_eq!(SAMPLE.len(), 20);
    assert!(
        kinds.contains(&None),
        "the sample must cover a stub with no source"
    );
    assert!(
        kinds.contains(&Some("Online")),
        "the sample must cover an Online source"
    );
    assert!(
        kinds.contains(&Some("GoogleDrive")),
        "the sample must cover a GoogleDrive source"
    );
}
