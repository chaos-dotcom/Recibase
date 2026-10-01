//! `se.reciba.api.MealDefinitions`: the meal list, i.e. the recipe corpus plus
//! the hand-written meal stubs.
//!
//! OWNER: workstream W5.
//!
//! The hand-written list is transcribed from
//! `Recibase/src/main/scala/se/reciba/api/recibase/MealDefinitions.scala`
//! (the 90 `MealStub(` literals inside the `++ Set(...)` at the top of the
//! file), in Scala source order. Tag order inside a stub is the order the
//! Scala `Set(...)` literal is written in; the wire order is Scala's `Set`
//! iteration order, applied later by `crate::scala_hash`.

use crate::meal::{MealStub, Source};
use crate::recipes::recipes;
use crate::tag::Tag;
use std::sync::LazyLock;

/// The number of `MealStub(` literals in `MealDefinitions.scala` (the
/// hand-written stubs). `grep -c 'MealStub(' MealDefinitions.scala` = 90:
/// one import, one type annotation and one `MealStub.apply` reference are not
/// followed by `(`, so all 90 hits are stub literals.
pub const DECLARED_STUB_COUNT: usize = 90;

/// `MealDefinitions.mealStubs`.
///
/// Scala builds this as `Recipe.recipes.map(MealStub.apply).toSet ++ Set(...)`,
/// so the recipes are inserted first and a duplicate keeps the entry that was
/// already in the set. The returned slice is the flat union in insertion order;
/// the order it is serialised in comes from `crate::scala_hash`.
pub fn meal_stubs() -> &'static [MealStub] {
    static MEAL_STUBS: LazyLock<Vec<MealStub>> = LazyLock::new(|| {
        let mut stubs = Vec::with_capacity(recipes().len() + declared_stubs().len());
        for recipe in recipes() {
            insert_unique(&mut stubs, MealStub::from_recipe(recipe));
        }
        for stub in declared_stubs() {
            insert_unique(&mut stubs, stub.clone());
        }
        stubs
    });
    &MEAL_STUBS[..]
}

/// The hand-written stubs of `MealDefinitions.mealStubs`, in the order the
/// Scala source declares them (the insertion order for the `Set`).
pub fn declared_stubs() -> &'static [MealStub] {
    static DECLARED: LazyLock<Vec<MealStub>> = LazyLock::new(|| {
        vec![
            MealStub::with_source(
                "Apple & Sausage Filo Casserole",
                vec![Tag::Vegan, Tag::Scales, Tag::Slow],
                Source::Online("https://www.bbc.co.uk/food/recipes/vegan_apple_and_sausage_48574".to_string())
            ),
            MealStub::new(
                "Aubergine & Halloumi Lasagne",
                vec![Tag::Slow, Tag::Vegetarian, Tag::Scales]
            ),
            MealStub::new("Aubergine curry", vec![Tag::Vegan, Tag::Scales, Tag::Slow]),
            MealStub::with_source(
                "Avocado & coconut soup",
                vec![Tag::Vegan, Tag::Soup, Tag::Scales, Tag::Freezes],
                Source::GoogleDrive("170P50UetqQPkrqqYvthtt3FBXsjCAYU_".to_string())
            ),
            MealStub::new(
                "Baked potatoes",
                vec![Tag::Slow, Tag::Vegetarian, Tag::ColdWeather, Tag::LowEffort]
            ),
            MealStub::with_source(
                "Banana Curry",
                vec![Tag::Slow, Tag::Vegan, Tag::Scales, Tag::Freezes],
                Source::Online("https://www.theendlessmeal.com/banana-curry/".to_string())
            ),
            MealStub::with_source(
                "Bean & Broccoli Pasta",
                vec![Tag::VeganIsh, Tag::Scales],
                Source::Online("https://www.themediterraneandish.com/mediterranean-broccoli-pasta-bean/".to_string())
            ),
            MealStub::new("Butternut squash risotto", vec![Tag::Vegan, Tag::HotWeather]),
            MealStub::with_source(
                "Butternut & Blue Cheese Risotto",
                vec![Tag::Vegetarian, Tag::StephaniUnhealthy, Tag::LowEffort, Tag::Scales, Tag::Slow],
                Source::GoogleDrive("1e34QA2s94VJSNVQYxW_hRL3mKjNZtKlS".to_string())
            ),
            MealStub::new("Carbonara", vec![Tag::VegetarianIsh, Tag::Quick, Tag::Scales]),
            MealStub::new(
                "Carrot & Coriander Burgers",
                vec![Tag::Vegetarian, Tag::Slow, Tag::HighEffort]
            ),
            MealStub::with_source(
                "Cashew Curry",
                vec![Tag::Vegan, Tag::Scales],
                Source::Online("https://vegancocotte.com/cashew-curry/".to_string())
            ),
            MealStub::new("Cheese & olive tarts", vec![Tag::Vegetarian]),
            MealStub::with_source(
                "Coconut & egg curry",
                vec![Tag::Vegetarian],
                Source::Online("https://www.bbc.co.uk/food/recipes/whole_eggs_in_coconut_23624".to_string())
            ),
            MealStub::with_source(
                "Cod in tomato sauce",
                vec![Tag::Pescatarian, Tag::Quick],
                Source::GoogleDrive("1jiyXj9imR452VqEE299DlRU1ao5K-gVi".to_string())
            ),
            MealStub::new("Egg & Mozzarella Toasts", vec![Tag::Vegetarian, Tag::Quick]),
            MealStub::new(
                "Fish Pie",
                vec![Tag::Pescatarian, Tag::Scales, Tag::ColdWeather, Tag::Slow]
            ),
            MealStub::new(
                "Fettucine with Dolcelatte and Spinach",
                vec![Tag::Vegetarian, Tag::Quick, Tag::HotWeather, Tag::LowEffort]
            ),
            MealStub::new("Fishcakes", vec![Tag::Pescatarian]),
            MealStub::new(
                "Full Scottish Breakfast",
                vec![Tag::VegetarianIsh, Tag::HighEffort, Tag::Quick, Tag::Stodge]
            ),
            MealStub::new(
                "Garlic Spaghetti",
                vec![Tag::Vegan, Tag::Scales, Tag::Quick, Tag::HotWeather, Tag::LowEffort]
            ),
            MealStub::new("Gnocchi & Tomato Bake", vec![Tag::Vegetarian, Tag::Scales]),
            MealStub::with_source(
                "Gnocchi & Broccoli Bake",
                vec![Tag::Vegetarian, Tag::Scales, Tag::LowEffort, Tag::Quick],
                Source::Online("https://www.bbc.co.uk/food/recipes/gnocchi_pasta_bake_51351".to_string())
            ),
            MealStub::with_source(
                "Goats cheese, leek and spinach pasta bake",
                vec![Tag::Vegetarian, Tag::Scales, Tag::HotWeather],
                Source::Online("https://www.gousto.co.uk/cookbook/recipes/goats-cheese-leek-spinach-pasta-bake".to_string())
            ),
            MealStub::new("Grilled aubergine", vec![Tag::Quick, Tag::Vegan, Tag::LowEffort]),
            MealStub::with_source(
                "Haddock Moqueca",
                vec![Tag::Pescatarian, Tag::Spicy, Tag::Quick, Tag::LowEffort],
                Source::Online("https://www.gousto.co.uk/cookbook/fish-recipes/brazilian-haddock-moqueca-zesty-lime-rice".to_string())
            ),
            MealStub::new(
                "Haggis",
                vec![Tag::Vegetarian, Tag::Slow, Tag::HighEffort, Tag::Scales, Tag::Stodge, Tag::ColdWeather]
            ),
            MealStub::with_source(
                "Harira Soup",
                vec![Tag::Soup, Tag::Scales, Tag::Vegan, Tag::Freezes, Tag::ColdWeather],
                Source::Online("https://www.onegreenplanet.org/vegan-recipe/harira-soup-with-hummus-pitas/".to_string())
            ),
            MealStub::with_source(
                "Honey Mustard Roast Salmon",
                vec![Tag::Stephani, Tag::LowEffort, Tag::Pescatarian, Tag::Scales],
                Source::GoogleDrive("1vhTDuRPfPYHme_T9u2NnTakE1avfdoLk".to_string())
            ),
            MealStub::new(
                "Homemade Pizza",
                vec![Tag::HighEffort, Tag::Slow, Tag::Vegetarian, Tag::Stodge]
            ),
            MealStub::with_source(
                "Jamaican Squash & Coconut Stew",
                vec![Tag::Scales, Tag::Vegan, Tag::Spicy, Tag::Freezes, Tag::BetterNextDay],
                Source::Online("https://www.gousto.co.uk/cookbook/vegan-recipes/jamaican-squash-coconut-stew".to_string())
            ),
            MealStub::with_source(
                "Kedgeree",
                vec![Tag::Pescatarian, Tag::Scales],
                Source::Online("https://www.bbcgoodfood.com/recipes/smoked-haddock-kedgeree".to_string())
            ),
            MealStub::new("Kidney Bean & Vegetable gratin", vec![Tag::VegetarianIsh, Tag::Scales]),
            MealStub::new("Lentil & Vegetable Pilaf", vec![Tag::Vegan, Tag::Scales]),
            MealStub::new("Vegetarian Meatballs", vec![Tag::Scales, Tag::Vegetarian]),
            MealStub::with_source(
                "Mediterranean Fish Stew",
                vec![Tag::Pescatarian],
                Source::Online("https://www.gousto.co.uk/cookbook/fish-recipes/mediterranean-fish-stew-sunny-aioli".to_string())
            ),
            MealStub::with_source(
                "Mediterranean Vegetable Gnocchi",
                vec![Tag::Vegetarian, Tag::Scales, Tag::Quick, Tag::HotWeather],
                Source::Online("https://www.gousto.co.uk/cookbook/vegetarian-recipes/mediterranean-veg-gnocchi-with-basil".to_string())
            ),
            MealStub::with_source(
                "Mexican Tofu with Refried Beans",
                vec![Tag::Vegetarian, Tag::Spicy],
                Source::GoogleDrive("12RZq9w7CGKFZdxvGPUfMVhcjAXm1tdJI".to_string())
            ),
            MealStub::with_source(
                "Mild Paneer Curry",
                vec![Tag::Vegetarian],
                Source::Online("https://www.gousto.co.uk/cookbook/recipes/mild-paneer-curry".to_string())
            ),
            MealStub::with_source(
                "Mozzarella & spinach pancakes",
                vec![Tag::HighEffort, Tag::Vegetarian],
                Source::GoogleDrive("1L7CueEwwXy6ObWx7A2AjIxO4FdsI-YB_".to_string())
            ),
            MealStub::new("Mozzarella Burgers", vec![Tag::Vegetarian, Tag::LowEffort, Tag::Stodge]),
            MealStub::new("Mushroom Lasagne", vec![Tag::Vegetarian, Tag::Slow, Tag::Stodge]),
            MealStub::with_source(
                "Mushroom Soup",
                vec![Tag::Vegan, Tag::Soup, Tag::Slow, Tag::Scales, Tag::Freezes, Tag::ColdWeather],
                Source::GoogleDrive("1mxBjlql91Kxo6lHZL-LnmHmhGlRYThCk".to_string())
            ),
            MealStub::with_source(
                "Mushroom & parsnip rösti pie",
                vec![Tag::Vegetarian, Tag::Slow, Tag::HighEffort],
                Source::GoogleDrive("1-YvovgTwMtwqYvuEBAz3gb2shqaTRtY69i5zJjNsbd8".to_string())
            ),
            MealStub::new(
                "N Bean Chilli",
                vec![Tag::Vegan, Tag::Freezes, Tag::BetterNextDay, Tag::Slow, Tag::Scales]
            ),
            MealStub::with_source(
                "Nutty Sweet Potato & Spinach Pie",
                vec![Tag::Vegetarian, Tag::HighEffort, Tag::ColdWeather],
                Source::GoogleDrive("1dM9T4Bu7Hj3fbLplX3NYKnCFqvuNrFC4tgWhhgWYTUE".to_string())
            ),
            MealStub::with_source(
                "Paneer Butter Masala",
                vec![Tag::Vegetarian, Tag::Quick, Tag::LowEffort, Tag::Spicy, Tag::Scales],
                Source::Online("https://www.gousto.co.uk/cookbook/vegetarian-recipes/paneer-butter-masala-with-coriander-naan".to_string())
            ),
            MealStub::with_source(
                "Paneer Lababdar",
                vec![Tag::Vegetarian, Tag::Spicy, Tag::Scales],
                Source::Online("https://www.indianhealthyrecipes.com/paneer-lababdar-recipe/".to_string())
            ),
            MealStub::new(
                "Pasta & Pesto",
                vec![Tag::VegetarianIsh, Tag::Quick, Tag::Scales, Tag::HotWeather, Tag::LowEffort]
            ),
            MealStub::with_source(
                "Penne with Walnut sauce",
                vec![Tag::Quick, Tag::Vegetarian, Tag::HotWeather, Tag::LowEffort],
                Source::GoogleDrive("1-dIvRInPYyiUT1y6Y6cb7bZpnabubuIe".to_string())
            ),
            MealStub::new(
                "Pepper & goats cheese tart",
                vec![Tag::Vegetarian, Tag::Slow, Tag::Stodge, Tag::HotWeather]
            ),
            MealStub::new("Pies", vec![Tag::LowEffort, Tag::Vegetarian, Tag::Stodge]),
            MealStub::new(
                "Potato gratin",
                vec![Tag::Vegetarian, Tag::Slow, Tag::HighEffort, Tag::Stodge, Tag::Scales, Tag::ColdWeather]
            ),
            MealStub::with_source(
                "Ricotta spinach pitas",
                vec![Tag::Quick, Tag::Vegetarian, Tag::HotWeather, Tag::LowEffort],
                Source::GoogleDrive("1mwrdX7b3hk3AArjnDNWjbyMavuJ6JyI5".to_string())
            ),
            MealStub::with_source(
                "Roast Aubergine & Basil Risotto",
                vec![Tag::Vegetarian],
                Source::Online("https://www.gousto.co.uk/cookbook/recipes/tomato-risotto-with-crispy-roast-aubergine".to_string())
            ),
            MealStub::with_source(
                "Roast Carrot Soup",
                vec![Tag::Vegan, Tag::Scales, Tag::Slow, Tag::Freezes],
                Source::Online("https://cookieandkate.com/roasted-carrot-soup-recipe/#tasty-recipes-35404-jump-target".to_string())
            ),
            MealStub::new("Roast Nut Omelette", vec![Tag::Quick, Tag::Vegetarian]),
            MealStub::new(
                "Roast veg & chickpeas tomato sauce",
                vec![Tag::Vegan, Tag::Freezes, Tag::Slow, Tag::Scales]
            ),
            MealStub::new("Roast vegetable risotto", vec![Tag::Vegetarian, Tag::Slow, Tag::Scales]),
            MealStub::with_source(
                "Satay Sweet Potato Curry",
                vec![Tag::Vegan, Tag::Scales, Tag::Spicy, Tag::Quick, Tag::BetterNextDay],
                Source::Online("https://www.bbc.co.uk/food/recipes/satay_sweet_potato_curry_59527".to_string())
            ),
            MealStub::with_source(
                "Sausage & Bean Casserole",
                vec![Tag::VegetarianIsh, Tag::Scales, Tag::Slow, Tag::ColdWeather],
                Source::Online("https://www.bbcgoodfood.com/recipes/sausage-bean-casserole".to_string())
            ),
            MealStub::new(
                "Sausages & Mash",
                vec![Tag::Vegetarian, Tag::Quick, Tag::Scales, Tag::LowEffort]
            ),
            MealStub::with_source(
                "Scotch Pancakes (authentic)",
                vec![Tag::Pudding, Tag::Quick, Tag::LowEffort, Tag::Scales],
                Source::Online("https://www.reddit.com/r/Scotland/comments/uhaqax/what_do_you_mean_you_dont_know_how_to_make_drop/".to_string())
            ),
            MealStub::with_source(
                "Seitan Tagine",
                vec![Tag::Vegan, Tag::Freezes, Tag::Slow, Tag::Scales],
                Source::Online("https://www.onegreenplanet.org/vegan-recipe/seitan-tagine-with-apricots-and-dates/".to_string())
            ),
            MealStub::with_source(
                "Shahi Paneer",
                vec![Tag::Vegetarian, Tag::Spicy, Tag::Scales],
                Source::Online("https://www.indianhealthyrecipes.com/shahi-paneer-recipe/".to_string())
            ),
            MealStub::with_source(
                "Smoky sausage casserole",
                vec![Tag::VegetarianIsh, Tag::ColdWeather, Tag::Slow, Tag::Scales, Tag::BetterNextDay],
                Source::Online("https://www.bbcgoodfood.com/recipes/smoky-sausage-casserole".to_string())
            ),
            MealStub::with_source(
                "Spiced Parsnip & Apple Soup",
                vec![Tag::Soup, Tag::Scales, Tag::VeganIsh, Tag::Spicy, Tag::Freezes],
                Source::Online("https://www.bbcgoodfood.com/recipes/curried-lentil-parsnip-apple-soup".to_string())
            ),
            MealStub::with_source(
                "Broccoli & Cauliflower Bake",
                vec![Tag::Vegetarian, Tag::Quick, Tag::Scales, Tag::HotWeather, Tag::LowEffort],
                Source::GoogleDrive("1P-i6q_AgZXCtIBWoBIm3fst70VeJNFqK".to_string())
            ),
            MealStub::with_source(
                "Spicy Butternut & Coconut Soup",
                vec![Tag::Soup, Tag::Scales, Tag::Vegan, Tag::Freezes],
                Source::Online("https://www.bbc.co.uk/food/recipes/pumpkin_soup_45815".to_string())
            ),
            MealStub::with_source(
                "Spicy Chilli Paneer Noodles",
                vec![Tag::Vegetarian, Tag::Scales, Tag::Quick, Tag::Spicy],
                Source::Online("https://www.gousto.co.uk/cookbook/vegetarian-recipes/10-min-spicy-chilli-paneer-noodles".to_string())
            ),
            MealStub::new("Stir fry", vec![Tag::Quick, Tag::Vegetarian]),
            MealStub::with_source(
                "Stir fry Teriyaki Mackerel",
                vec![Tag::Pescatarian, Tag::Stephani],
                Source::GoogleDrive("19vVWmaiDP-6ww8vBf-E39jMMDN8XrpfE".to_string())
            ),
            MealStub::new(
                "Supermarket Pizza",
                vec![Tag::LowEffort, Tag::Quick, Tag::Vegetarian, Tag::Stodge]
            ),
            MealStub::with_source(
                "Sweet Potato & Smoked Paprika Soup",
                vec![Tag::Vegetarian, Tag::ColdWeather, Tag::Scales, Tag::Spicy, Tag::Freezes],
                Source::Online("https://www.italianfoodforever.com/2020/11/creamy-sweet-potato-soup/".to_string())
            ),
            MealStub::new(
                "Sweet Potato, Peanut Butter and Coconut Curry",
                vec![Tag::Vegetarian, Tag::HotWeather]
            ),
            MealStub::with_source(
                "Sweetcorn & spinach polenta",
                vec![Tag::VegetarianIsh, Tag::Quick, Tag::Scales, Tag::HotWeather, Tag::LowEffort],
                Source::GoogleDrive("1sbycCsoWJSO8ETeg_AQ-2rHE4Ea6UYvi".to_string())
            ),
            MealStub::new(
                "Thai Green Curry",
                vec![Tag::VeganIsh, Tag::Slow, Tag::Scales, Tag::Spicy]
            ),
            MealStub::with_source(
                "Tofu & cashew nut stir fry",
                vec![Tag::Vegan, Tag::Quick, Tag::HotWeather],
                Source::GoogleDrive("1nNWYsSiKtMubmN3VeXgGpIX8_RniCOlp".to_string())
            ),
            MealStub::new(
                "Tomato & Mozzarella Salad",
                vec![Tag::Vegetarian, Tag::Quick, Tag::HotWeather, Tag::LowEffort]
            ),
            MealStub::new(
                "Tomato sauce",
                vec![Tag::Freezes, Tag::Scales, Tag::Slow, Tag::BetterNextDay, Tag::LowEffort, Tag::VeganIsh]
            ),
            MealStub::new("Tuna & rice peppers", vec![Tag::Pescatarian, Tag::Slow]),
            MealStub::with_source(
                "Tuna Fish Pie",
                vec![Tag::Pescatarian, Tag::Scales],
                Source::Online("https://www.taste.com.au/recipes/tuna-pie-potato-topping/53c34dd0-edf6-4db9-929f-ac2fd4b65667".to_string())
            ),
            MealStub::with_source(
                "Tuna in tomato sauce",
                vec![Tag::Pescatarian],
                Source::GoogleDrive("1nDEnSpUpKG2a3tIL4VklxJrzAVO5GFkX".to_string())
            ),
            MealStub::new("Tuna Pasta", vec![Tag::Pescatarian, Tag::Quick, Tag::LowEffort]),
            MealStub::with_source(
                "Tuna steaks with salsa verde",
                vec![Tag::Pescatarian, Tag::Stephani],
                Source::Online("https://www.bbc.co.uk/food/recipes/tunasteakswithsalsav_74789".to_string())
            ),
            MealStub::with_source(
                "Vegan Brownies (Alt Recipe)",
                vec![Tag::Vegan, Tag::Pudding, Tag::Baking],
                Source::Online("https://www.cheapskatecook.com/eggless-brownies-no-weird-ingredients/".to_string())
            ),
            MealStub::with_source(
                "Vegetable Hotpot with dumplings",
                vec![Tag::Vegetarian, Tag::Slow, Tag::HighEffort, Tag::ColdWeather],
                Source::GoogleDrive("1jsxu4biuRi3ewRQ4g5HDGEpoKw-aMf0e".to_string())
            ),
            MealStub::with_source(
                "Vegetable Sambar",
                vec![Tag::Vegetarian, Tag::Spicy, Tag::Scales],
                Source::Online("https://www.gousto.co.uk/cookbook/vegan-recipes/fragrant-vegetable-sambar-with-coconut-yoghurt".to_string())
            ),
            MealStub::with_source(
                "Veggie Goulash",
                vec![Tag::Vegetarian, Tag::Slow, Tag::Spicy, Tag::Scales, Tag::BetterNextDay, Tag::ColdWeather],
                Source::Online("https://www.gousto.co.uk/cookbook/recipes/veggie-goulash-potato-cakes-sour-cream".to_string())
            ),
            MealStub::with_source(
                "Venetian Style Pasta",
                vec![Tag::Quick, Tag::Vegan, Tag::Scales, Tag::HotWeather, Tag::LowEffort],
                Source::Online("https://www.bbcgoodfood.com/recipes/12135/venetianstyle-pasta".to_string())
            ),
        ]
    });
    &DECLARED[..]
}

/// `MealStub` case-class equality: the name, the tags as a `Set`, the source
/// and the creation date. This is what makes two stubs one element of the
/// Scala `Set` that `mealStubs` is.
pub fn same_meal(a: &MealStub, b: &MealStub) -> bool {
    a.name == b.name
        && a.source == b.source
        && a.created_at == b.created_at
        && tags_as_set_eq(&a.tags, &b.tags)
}

/// Compares two tag lists as Scala `Set`s: order and repeats are irrelevant.
fn tags_as_set_eq(a: &[Tag], b: &[Tag]) -> bool {
    let mut a = a.to_vec();
    a.sort();
    a.dedup();
    let mut b = b.to_vec();
    b.sort();
    b.dedup();
    a == b
}

/// `Set.+`: adding a stub that is already present keeps the present one.
fn insert_unique(stubs: &mut Vec<MealStub>, stub: MealStub) {
    if !stubs.iter().any(|existing| same_meal(existing, &stub)) {
        stubs.push(stub);
    }
}
