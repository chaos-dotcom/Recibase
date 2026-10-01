//! `se.reciba.api.recipes.TruffleBurgers`.

use crate::recipe::RecipeDef;
use crate::tag::Tag;
use chrono::NaiveDate;

pub fn recipe() -> RecipeDef {
    RecipeDef {
        object_name: "TruffleBurgers".to_string(),
        name: "Truffle Burgers".to_string(),
        created_at: NaiveDate::from_ymd_opt(2020, 4, 24).unwrap(),
        permalink_override: None,
        source: None,
        description: None,
        tagline: None,
        notes: Vec::new(),
        tags: vec![Tag::Vegetarian, Tag::Stodge, Tag::Quick],
        image: None,
        ingredients_blocks: crate::recipe::IngredientsBlock::simple(vec![
            crate::recipe::Ingredient::q("Beyond Meat Burgers", "4"),
            crate::recipe::Ingredient::q("Brioche buns", "4"),
            crate::recipe::Ingredient::q("Manchego", "4 slices"),
            crate::recipe::Ingredient::opt("Jam", None, None, Some("Fig jam is best but strawberry or raspberry also works")),
            crate::recipe::Ingredient::new("Mayonnaise"),
            crate::recipe::Ingredient::new("Truffle oil"),
            crate::recipe::Ingredient::new("Oil"),
        ]),
        method: vec![
            "Lightly toast the brioche buns.".to_string(),
            "Mix the truffle oil and mayonnaise to taste.".to_string(),
            "Spread the truffle mayo on the bottom of each bun and jam on the top.".to_string(),
            "Lightly brush a griddle pan with oil then heat on your hottest hob.".to_string(),
            "Lay out the patties on the griddle and cook for a few minutes until almost browned. Open a window or two and be careful not to let the oil smoke.".to_string(),
            "Turn over and brown the other side".to_string(),
            "Turn over, again, and lay out a slice of Manchego on top of each patty.".to_string(),
            "Lay out the patties on the burger buns. If you fancy shortening your lifespan then drizzle the pan's oil over the patties before adding the bun top.".to_string(),
        ],
    }
}
