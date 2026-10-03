//! `se.reciba.api.recipes.ParsnipGingerSoup`.

use crate::recipe::RecipeDef;
use crate::tag::Tag;
use chrono::NaiveDate;

pub fn recipe() -> RecipeDef {
    RecipeDef {
        object_name: "ParsnipGingerSoup".to_string(),
        name: "Parsnip & Ginger Soup".to_string(),
        created_at: NaiveDate::from_ymd_opt(2020, 4, 24).unwrap(),
        permalink_override: Some("parsnip-and-ginger-soup".to_string()),
        source: None,
        description: None,
        tagline: None,
        notes: Vec::new(),
        tags: vec![Tag::Soup, Tag::VeganIsh, Tag::LowEffort],
        image: None,
        ingredients_blocks: crate::recipe::IngredientsBlock::simple(vec![
            crate::recipe::Ingredient::qp("Parnsips", "450g", "Diced"),
            crate::recipe::Ingredient::qp("Onion", "1", "Diced"),
            crate::recipe::Ingredient::opt("Potato", Some("1"), None, Some("Optional, subtitute for flour")),
            crate::recipe::Ingredient::qp("Orange", "1", "Grate the rind and juice"),
            crate::recipe::Ingredient::qp("Fresh Ginger", "2.5cm", "Grated"),
            crate::recipe::Ingredient::new("Stock cube"),
            crate::recipe::Ingredient::new("Butter"),
            crate::recipe::Ingredient::opt("Single Cream", Some("250ml"), None, Some("Optional")),
        ]),
        method: vec![
            "Melt the butter in a large pan then stir in the parsnips, onion, flour, ginger and orange rind.".to_string(),
            "Dissolve the stock in 500ml - 1L of water then mix in and bring to the boil.".to_string(),
            "Simmer for 20 minutes or so, until the parsnips are soft.".to_string(),
            "Blend the mixture to your desired consistency.".to_string(),
            "Stir in the orange juice and reheat, without boiling.".to_string(),
            "Add cream, if desired.".to_string(),
        ],
    }
}
