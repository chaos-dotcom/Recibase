//! `se.reciba.api.recipes.BlueCheeseGnocchi`.

use crate::recipe::RecipeDef;
use crate::tag::Tag;
use chrono::NaiveDate;

pub fn recipe() -> RecipeDef {
    RecipeDef {
        object_name: "BlueCheeseGnocchi".to_string(),
        name: "Blue Cheese Gnocchi".to_string(),
        created_at: NaiveDate::from_ymd_opt(2020, 4, 24).unwrap(),
        permalink_override: None,
        source: None,
        description: Some("Gnocchi with creme fraiche and spinach topped with grilled blue stilton.".to_string()),
        tagline: Some("Fill the void with cheese".to_string()),
        notes: vec!["You can also use fresh tagliatelle rather than gnocchi, although you'll need a lot more creme fraiche.".to_string()],
        tags: vec![
            Tag::Stodge,
            Tag::Quick,
            Tag::VegetarianIsh,
            Tag::ColdWeather,
            Tag::LowEffort,
        ],
        image: None,
        ingredients_blocks: crate::recipe::IngredientsBlock::simple(vec![
            crate::recipe::Ingredient::q("Gnocchi", "500g"),
            crate::recipe::Ingredient::qp("Parmesan", "~70g", "grated"),
            crate::recipe::Ingredient::q("Creme Fraiche", "150ml"),
            crate::recipe::Ingredient::q("Spinach", "200g"),
            crate::recipe::Ingredient::qp("Stilton", "200g", "diced/crumbled"),
            crate::recipe::Ingredient::opt("Pimento Stuffed olives", None, None, Some("Optional")),
            crate::recipe::Ingredient::opt("Cherry Tomatoes", None, None, Some("Optional")),
            crate::recipe::Ingredient::opt("Fresh bread", None, None, Some("Optional")),
        ]),
        method: vec![
            "Tear up the spinach into a colander".to_string(),
            "Place the gnocchi in a pan of boiling water and simmer until the gnocchi start to float to the surface".to_string(),
            "Drain the gnocchi into the colander, over the spinach. Shake to displace any trapped water ".to_string(),
            "Empty the gnocchi/spinach into an ovenproof dish and mix in the parmesan and creme fraiche".to_string(),
            "Sprinkle crumbled stilton over the top".to_string(),
            "Grill on a medium/high heat until the stilton is bubbling and golden".to_string(),
            "Serve on fresh bread with cherry tomatoes, olives, and anything else you can think of.".to_string(),
        ],
    }
}
