//! `se.reciba.api.recipes.PeanutButterBiscuits`.

use crate::recipe::RecipeDef;
use crate::tag::Tag;
use chrono::NaiveDate;

pub fn recipe() -> RecipeDef {
    RecipeDef {
        object_name: "PeanutButterBiscuits".to_string(),
        name: "Peanut Butter Biscuits".to_string(),
        created_at: NaiveDate::from_ymd_opt(2021, 1, 24).unwrap(),
        permalink_override: None,
        source: Some("Alex's Mum".to_string()),
        description: None,
        tagline: None,
        notes: Vec::new(),
        tags: vec![Tag::Baking],
        image: Some(crate::recipe::Image::new("https://i.reciba.se/peanut-butter-biscuits.jpg")),
        ingredients_blocks: crate::recipe::IngredientsBlock::simple(vec![
            crate::recipe::Ingredient::q("Crunchy peanut butter", "250g"),
            crate::recipe::Ingredient::q("Light Brown soft sugar", "200g"),
            crate::recipe::Ingredient::q("Egg", "1 medium"),
        ]),
        method: vec![
            format!("Preheat oven {}.", crate::utils::int_utils::celsius(180)),
            "Line 2 baking trays with parchment.".to_string(),
            "Beat all the ingredients together in a bowl until well combined.".to_string(),
            "Scoop out tablespoons and roll them into balls.".to_string(),
            "Arrange on baking trays, well spaced out".to_string(),
            "Flatten with a fork.".to_string(),
            "Bake for 12 minutes. Leave to cool for 5 minutes.".to_string(),
            "Transfer to a rack to cool completely.".to_string(),
        ],
    }
}
