//! `se.reciba.api.recipes.HalloumiWraps`.

use crate::recipe::RecipeDef;
use crate::tag::Tag;
use chrono::NaiveDate;

pub fn recipe() -> RecipeDef {
    RecipeDef {
        object_name: "HalloumiWraps".to_string(),
        name: "Halloumi Wraps".to_string(),
        created_at: NaiveDate::from_ymd_opt(2023, 11, 9).unwrap(),
        permalink_override: None,
        source: None,
        description: Some("Greek style halloumi wraps".to_string()),
        tagline: None,
        notes: vec![
            "You can use up other vegetables like courgettes or red onions. Just fry them at the same time as the other veg.".to_string(),
            "A single packet of halloumi is slightly too little but two is slightly too much. Ideally cook this for an odd number of people or find a recipe to use up the remainder.".to_string(),
        ],
        tags: vec![
            Tag::Lunch,
            Tag::Quick,
            Tag::Vegetarian,
        ],
        image: None,
        ingredients_blocks: crate::recipe::IngredientsBlock::simple(vec![
            crate::recipe::Ingredient::qp("Halloumi", "225g", "sliced"),
            crate::recipe::Ingredient::qp("Red Pepper", "1", "sliced"),
            crate::recipe::Ingredient::qp("Spring Onions", "3", "chopped"),
            crate::recipe::Ingredient::q("Tortilla Wraps", "4"),
            crate::recipe::Ingredient::opt("Cherry Tomatoes", None, Some("halved"), None),
            crate::recipe::Ingredient::new("Yoghurt"),
            crate::recipe::Ingredient::opt("Garlic", None, Some("diced"), None),
            crate::recipe::Ingredient::new("Oil"),
        ]),
        method: vec![
            "Heat some oil on a griddle pan over a high heat.".to_string(),
            "Fry the halloumi slices on the griddle until browned. Turn once.".to_string(),
            "Mix together a few tablespoons of yoghurt and the garlic.".to_string(),
            "Warm the tortillas in a microwave per packet instructions and spread with the garlic yoghurt.".to_string(),
            "A few minutes before the halloumi is ready heat a small frying pan over a medium heat and cook the red pepper.".to_string(),
            "Add the spring onions and cherry tomatoes to the pan and cook for a further 30 seconds.".to_string(),
            "Divide up the halloumi and veg mixture equally among the tortillas.".to_string(),
        ],
    }
}
