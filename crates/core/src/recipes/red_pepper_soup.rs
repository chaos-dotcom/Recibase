//! `se.reciba.api.recipes.RedPepperSoup`.

use crate::recipe::RecipeDef;
use crate::tag::Tag;
use chrono::NaiveDate;

pub fn recipe() -> RecipeDef {
    RecipeDef {
        object_name: "RedPepperSoup".to_string(),
        name: "Red Pepper & Apple Soup".to_string(),
        created_at: NaiveDate::from_ymd_opt(2020, 4, 24).unwrap(),
        permalink_override: Some("red-pepper-soup".to_string()),
        source: None,
        description: None,
        tagline: None,
        notes: Vec::new(),
        tags: vec![
            Tag::Soup,
            Tag::Scales,
            Tag::VeganIsh,
            Tag::LowEffort,
        ],
        image: None,
        ingredients_blocks: crate::recipe::IngredientsBlock::simple(vec![
            crate::recipe::Ingredient::q("Butter", "Knob"),
            crate::recipe::Ingredient::q("Onion", "1"),
            crate::recipe::Ingredient::q("Red Pepper", "1"),
            crate::recipe::Ingredient::q("Apple", "1"),
            crate::recipe::Ingredient::q("Carrot", "1"),
            crate::recipe::Ingredient::q("Water", "600ml"),
            crate::recipe::Ingredient::new("Stock Cube"),
            crate::recipe::Ingredient::new("Mixed Herbs"),
            crate::recipe::Ingredient::opt("Extra vegetables", None, None, Some("Optional")),
        ]),
        method: vec![
            "Chop up everything and place everything but the onions in a bowl.".to_string(),
            "Boil the water and add the stock cube and stir it in.".to_string(),
            "Melt the butter then add the onions and soften then under a low heat for a minute or so.".to_string(),
            "Add all the vegetables then the water and herbs.".to_string(),
            "Turn up the heat and boil for a minute then turn down the heat and simmer it for 30 minutes. Wait until cool and blend.".to_string(),
        ],
    }
}
