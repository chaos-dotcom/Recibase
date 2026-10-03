//! `se.reciba.api.recipes.Pancakes`.

use crate::recipe::RecipeDef;
use crate::tag::Tag;
use chrono::NaiveDate;

pub fn recipe() -> RecipeDef {
    RecipeDef {
        object_name: "Pancakes".to_string(),
        name: "Pancakes".to_string(),
        created_at: NaiveDate::from_ymd_opt(2025, 3, 4).unwrap(),
        permalink_override: None,
        source: Some("Eth".to_string()),
        description: Some("Incredibly simple crepe style pancakes".to_string()),
        tagline: None,
        notes: Vec::new(),
        tags: vec![
            Tag::Vegetarian,
            Tag::Scales,
            Tag::Pudding,
            Tag::Quick,
            Tag::LowEffort,
        ],
        image: None,
        ingredients_blocks: crate::recipe::IngredientsBlock::simple(vec![
            crate::recipe::Ingredient::q("Flour", "200g"),
            crate::recipe::Ingredient::q("Milk", "400ml"),
            crate::recipe::Ingredient::new("Butter"),
        ]),
        method: vec![
            "Whisk together ingredients.".to_string(),
            "Heat a large frying pan over a high flame.".to_string(),
            "Throw in a small piece of butter. It should immediately start bubbling.".to_string(),
            "Pour a small amount of the mixture into the pan. Just enough to go to cover the bottom.".to_string(),
            "Flip once the underside is lightly browned and brown the remaining side.".to_string(),
            "Serve with sweet toppings.".to_string(),
        ],
    }
}
