//! `se.reciba.api.recipes.SweetChilliFetaPasta`.

use crate::recipe::RecipeDef;
use crate::tag::Tag;
use chrono::NaiveDate;

pub fn recipe() -> RecipeDef {
    RecipeDef {
        object_name: "SweetChilliFetaPasta".to_string(),
        name: "Sweet chilli feta pasta".to_string(),
        created_at: NaiveDate::from_ymd_opt(2020, 4, 24).unwrap(),
        permalink_override: None,
        source: None,
        description: None,
        tagline: None,
        notes: vec![
            "Try to find large bottles of sweet chilli sauce. You can get about a litre for the same price as one of the tiny bottles. This recipe needs quite a bit.".to_string(),
            "There are basically no quantities. Add what seems right.".to_string(),
        ],
        tags: vec![
            Tag::Quick,
            Tag::Pescatarian,
            Tag::Scales,
            Tag::LowEffort,
        ],
        image: None,
        ingredients_blocks: crate::recipe::IngredientsBlock::simple(vec![
            crate::recipe::Ingredient::new("Pine nuts"),
            crate::recipe::Ingredient::new("Cashew nuts"),
            crate::recipe::Ingredient::new("Brazil nuts"),
            crate::recipe::Ingredient::opt("Feta", Some("1 block, approx. 200g"), Some("cut into 1cm cubes"), None),
            crate::recipe::Ingredient::opt("Tuna chunks", Some("1 80g tin"), Some("drained"), Some("Optional")),
            crate::recipe::Ingredient::new("Sweet chilli sauce"),
            crate::recipe::Ingredient::new("Black pepper"),
            crate::recipe::Ingredient::opt("Fusilli", None, None, Some("Spaghetti also works well here")),
        ]),
        method: vec![
            "Boil water and cook the pasta until al dente or softer. Drain and set aside.".to_string(),
            "Meanwhile, add oil to a large, deep pan along with the pine, cashew and brazil nuts and roast over a high heat until brown.".to_string(),
            "Remove pan from the heat and allow to cool for a moment. Add half the feta to the pan and melt over a medium heat.".to_string(),
            "Add the tuna (if used), black pepper and sweet chilli sauce to the pan and warm.".to_string(),
            "Add the pasta and remaining feta and mix thoroughly.".to_string(),
            "Serve hot.".to_string(),
        ],
    }
}
