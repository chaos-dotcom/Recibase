//! `se.reciba.api.recipes.LemonFetaPasta`.

use crate::recipe::RecipeDef;
use crate::tag::Tag;
use chrono::NaiveDate;

pub fn recipe() -> RecipeDef {
    RecipeDef {
        object_name: "LemonFetaPasta".to_string(),
        name: "Lemon Feta Pasta".to_string(),
        created_at: NaiveDate::from_ymd_opt(2020, 4, 24).unwrap(),
        permalink_override: None,
        source: None,
        description: None,
        tagline: None,
        notes: Vec::new(),
        tags: vec![
            Tag::Quick,
            Tag::Pescatarian,
            Tag::LowEffort,
            Tag::Scales,
        ],
        image: None,
        ingredients_blocks: crate::recipe::IngredientsBlock::simple(vec![
            crate::recipe::Ingredient::new("Pine nuts"),
            crate::recipe::Ingredient::new("Cashew nuts"),
            crate::recipe::Ingredient::new("Brazil nuts"),
            crate::recipe::Ingredient::qp("Feta", "1 block, approx. 200g", "cut into 1cm cubes"),
            crate::recipe::Ingredient::qpn("Tuna", "1 80g tin", "drained", "Optional"),
            crate::recipe::Ingredient::opt("Olives", None, None, Some("Pimento or green preferred")),
            crate::recipe::Ingredient::opt("Lemon juice", None, None, Some("Either fresh or bottled")),
            crate::recipe::Ingredient::opt("Thyme", None, None, Some("Either fresh or dried")),
            crate::recipe::Ingredient::opt("Fusilli", None, None, Some("Spaghetti also works well here")),
        ]),
        method: vec![
            "Boil water and cook the pasta until al dente or softer. Drain and set aside.".to_string(),
            "Meanwhile, add oil to a large, deep pan along with the pine, cashew and brazil nuts and roast over a high heat until brown.".to_string(),
            "Remove pan from the heat and allow to cool for a moment. Add half the feta to the pan and melt over a medium heat.".to_string(),
            "Add the tuna (if used), thyme, olives and lemon juice to the pan and warm.".to_string(),
            "Add the pasta and remaining feta and mix thoroughly.".to_string(),
            "Serve hot.".to_string(),
        ],
    }
}
