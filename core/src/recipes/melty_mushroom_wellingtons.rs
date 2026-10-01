//! `se.reciba.api.recipes.MeltyMushroomWellingtons`.

use crate::recipe::RecipeDef;
use crate::tag::Tag;
use chrono::NaiveDate;

pub fn recipe() -> RecipeDef {
    RecipeDef {
        object_name: "MeltyMushroomWellingtons".to_string(),
        name: "Melty Mushroom Wellingtons".to_string(),
        created_at: NaiveDate::from_ymd_opt(2020, 4, 24).unwrap(),
        permalink_override: None,
        source: None,
        description: None,
        tagline: None,
        notes: vec![
            "Take the pastry out of the fridge ten minutes before use.".to_string(),
            "Makes two large pies.".to_string(),
        ],
        tags: vec![
            Tag::Slow,
            Tag::Vegetarian,
            Tag::Stodge,
            Tag::ColdWeather,
        ],
        image: None,
        ingredients_blocks: crate::recipe::IngredientsBlock::simple(vec![
            crate::recipe::Ingredient::opt("Chestnut mushrooms", Some("250g"), Some("half sliced, half diced"), None),
            crate::recipe::Ingredient::opt("Spinach", None, Some("torn up"), None),
            crate::recipe::Ingredient::qp("Stilton", "220g", "finely chopped"),
            crate::recipe::Ingredient::opt("Garlic", None, Some("finely chopped"), None),
            crate::recipe::Ingredient::q("Butter", "knob"),
            crate::recipe::Ingredient::new("Black pepper"),
            crate::recipe::Ingredient::q("Puff pastry", "1 sheet"),
        ]),
        method: vec![
            format!("Preheat the oven at {}.", crate::utils::int_utils::celsius(200)),
            "Melt the butter in wide pan.".to_string(),
            "Cook the garlic and black pepper.".to_string(),
            "Add the mushrooms and cook over a medium heat until softened. Add the spinach and wilt.".to_string(),
            "Mix in the stilton.".to_string(),
            "Meanwhile place the puff pastry on a chopping board and cut it in half along the short axis. For each half, score it in the middle along the short axis.".to_string(),
            "Place a large spoonful of the mushroom mixture on one side of the scored pastries.".to_string(),
            "Fold over the other half of the pasty and seal with a fork.".to_string(),
            "Cut a slit in the top to allow steam to escape.".to_string(),
            "Place the pies on a baking tray in the oven for 20 minutes or until golden brown.".to_string(),
        ],
    }
}
