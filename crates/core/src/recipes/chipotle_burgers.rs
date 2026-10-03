//! `se.reciba.api.recipes.ChipotleBurgers`.

use crate::recipe::RecipeDef;
use crate::tag::Tag;
use chrono::NaiveDate;

pub fn recipe() -> RecipeDef {
    RecipeDef {
        object_name: "ChipotleBurgers".to_string(),
        name: "Chipotle Burgers".to_string(),
        created_at: NaiveDate::from_ymd_opt(2022, 10, 16).unwrap(),
        permalink_override: None,
        source: Some("https://www.honestburgers.co.uk/food/burgers/bacon-plant/".to_string()),
        description: None,
        tagline: None,
        notes: vec![
            "This version is only vegetarian, though you could follow the original Honest Burger version and use vegan mayo/cheese.".to_string(),
            "This actually works better with ancho chilli paste but \"ancho burgers\" doesn't have the same ring to it.".to_string(),
        ],
        tags: vec![
            Tag::Vegetarian,
            Tag::Spicy,
            Tag::Stodge,
            Tag::Quick,
        ],
        image: None,
        ingredients_blocks: crate::recipe::IngredientsBlock::simple(vec![
            crate::recipe::Ingredient::q("Beyond Meat Burgers", "4"),
            crate::recipe::Ingredient::q("Brioche buns", "4"),
            crate::recipe::Ingredient::q("Applewood Smoked Cheddar", "4 slices"),
            crate::recipe::Ingredient::q("Vegan Streaky Bacon Rashers", "105g"),
            crate::recipe::Ingredient::opt("Ketchup", None, None, Some("A premium brand like Sauce Shop")),
            crate::recipe::Ingredient::new("Mayonnaise"),
            crate::recipe::Ingredient::opt("Chipotle paste", Some("30g"), None, Some("or ancho chilli paste")),
            crate::recipe::Ingredient::new("Oil"),
        ]),
        method: vec![
            "Lightly toast the brioche buns.".to_string(),
            "Mix the chipotle paste and mayonnaise to taste.".to_string(),
            "Spread the chipotle mayo on the bottom of each bun and ketchup on the top.".to_string(),
            "Lightly brush a griddle pan with oil then heat on your hottest hob.".to_string(),
            "Lay out the patties on the griddle and cook for a few minutes until almost browned. Open a window or two and be careful not to let the oil smoke.".to_string(),
            "Turn over and brown the other side".to_string(),
            "Turn over, again, and lay out a slice of smoked cheddar on top of each patty.".to_string(),
            "Lay out the patties on the burger buns. If you fancy shortening your lifespan then drizzle the pan's oil over the patties before adding the bun top.".to_string(),
        ],
    }
}
