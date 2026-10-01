//! `se.reciba.api.recipes.NutRoast`.

use crate::recipe::RecipeDef;
use crate::tag::Tag;
use chrono::NaiveDate;

pub fn recipe() -> RecipeDef {
    RecipeDef {
        object_name: "NutRoast".to_string(),
        name: "Nut Roast".to_string(),
        created_at: NaiveDate::from_ymd_opt(2024, 12, 25).unwrap(),
        permalink_override: None,
        source: None,
        description: Some("A hearty and filling vegetarian Christmas main. Serve with gravy and roast veg.".to_string()),
        tagline: None,
        notes: vec![
            "We use Waitrose mixed roasted nuts, which is a mixture of brazil nuts, almonds, hazelnuts and macadamia nuts. The pre-roasting really adds to the flavour, so if you buy raw nuts do consider roasting them youself.".to_string(),
            "You can prepare the dry ingredients several days in advance, to reduce stress on the day.".to_string(),
            "This dish is very filling you you'll only need 2 slices per person.".to_string(),
            "The ingredient quantities are largely vibe based, depending how Alex feels each year.".to_string(),
            "The number of eggs needed varies each year, depending how dry the mixture is. Always buy more than needed.".to_string(),
        ],
        tags: vec![
            Tag::Vegetarian,
            Tag::HighEffort,
            Tag::Christmas,
            Tag::ColdWeather,
        ],
        image: Some(crate::recipe::Image::new("https://i.reciba.se/nut-roast.jpg")),
        ingredients_blocks: crate::recipe::IngredientsBlock::simple(vec![
            crate::recipe::Ingredient::q("Mixed Roasted Nuts", "500g"),
            crate::recipe::Ingredient::opt("Rosemary", Some("5 sprigs"), Some("finely chopped"), None),
            crate::recipe::Ingredient::opt("Cranberries", Some("100g"), Some("rinsed"), None),
            crate::recipe::Ingredient::q("Eggs", "4-6"),
            crate::recipe::Ingredient::opt("Onion", Some("1"), Some("finely chopped"), None),
            crate::recipe::Ingredient::opt("Chestnut mushrooms", Some("250g"), Some("finely chopped"), None),
            crate::recipe::Ingredient::opt("Brown crusty bread", Some("2 slices"), None, Some("use more slices if they're not thick. Older/stale bread works better")),
            crate::recipe::Ingredient::q("Puff Pastry", "2 sheets"),
            crate::recipe::Ingredient::new("Salt"),
            crate::recipe::Ingredient::new("Oil"),
        ]),
        method: vec![
            format!("Pre-heat the oven to {}", crate::utils::int_utils::celsius(200)),
            "Smash up the nuts with a large mortar and pestle. If you don't have one you can smash up the nuts using a metal mixing bowl and the end of a rolling pin.".to_string(),
            "Transfer the mixture to a large mixing bowl.".to_string(),
            "Toast the bread then blend it into breadcrumbs and add to the mixture.".to_string(),
            "Heat some oil in a large frying pan then cook the onions and mushrooms for a few minutes.".to_string(),
            "Take the pan off the heat and leave to cool.".to_string(),
            "Add the mushrooms, onion, rosemary and some salt to the bowl. Mix together.".to_string(),
            "Mix in the eggs. Add as many as is necessary for the mixture to stick together.".to_string(),
            "Roll out one of the sheets of puff pasty on a large baking tray.".to_string(),
            "Spoon the mixture onto the pastry sheet, leaving a few cm at the edges.".to_string(),
            "Fold the edges of the pastry up over the mixture.".to_string(),
            "Slice the second pastry sheet into 1cm wide diagonal strips.".to_string(),
            "Lay the strips over the mixture in an interlacing lattice. Connect them to the folded-up edges of the first pastry sheet to fully encase the mixture.".to_string(),
            "Beat an egg in a cup then brush the pastry. If you used up all your eggs then brush with milk instead.".to_string(),
            "Bake in the oven for 40 minutes or until golden brown.".to_string(),
            "Serve with gravy, roasted vegetables and cranberry relish.".to_string(),
        ],
    }
}
