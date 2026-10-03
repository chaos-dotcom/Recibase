//! `se.reciba.api.recipes.HoisinDuckWraps`.

use crate::recipe::RecipeDef;
use crate::tag::Tag;
use chrono::NaiveDate;

pub fn recipe() -> RecipeDef {
    RecipeDef {
        object_name: "HoisinDuckWraps".to_string(),
        name: "Hoisin Duck Wraps".to_string(),
        created_at: NaiveDate::from_ymd_opt(2023, 3, 29).unwrap(),
        permalink_override: Some("duck-wraps".to_string()),
        source: None,
        description: Some("To the tune of Top Cat: Duck Wraps! They're vegetarian, Duck Wraps!".to_string()),
        tagline: None,
        notes: vec![
            "You can use up other vegetables like courgettes or red onions. Just fry them at the same time as the other veg.".to_string(),
            "This tastes best with hoisin sauce but mayonnaise works as a decent substitute.".to_string(),
        ],
        tags: vec![
            Tag::Lunch,
            Tag::Quick,
            Tag::Vegetarian,
        ],
        image: None,
        ingredients_blocks: crate::recipe::IngredientsBlock::simple(vec![
            crate::recipe::Ingredient::q("Vegetarian Shredded Hoisin Duck", "200g"),
            crate::recipe::Ingredient::qp("Red Pepper", "1", "sliced"),
            crate::recipe::Ingredient::qp("Spring Onions", "3", "chopped"),
            crate::recipe::Ingredient::q("Tortilla Wraps", "4"),
            crate::recipe::Ingredient::opt("Cherry Tomatoes", None, Some("halved"), None),
            crate::recipe::Ingredient::opt("Hoisin sauce", None, None, Some("Optional")),
            crate::recipe::Ingredient::new("Mayonnaise"),
            crate::recipe::Ingredient::new("Oil"),
        ]),
        method: vec![
            "Heat some oil in a large frying pan.".to_string(),
            "Throw in the shredded hoisin duck and cook per packet instructions.".to_string(),
            "Warm the tortillas in a microwave per packet instructions and spread with some mayonnaise".to_string(),
            "A few minutes before the duck mixture is ready throw in the peppers.".to_string(),
            "Add the spring onions and cherry tomatoes to the pan and cook for a further 30 seconds.".to_string(),
            "Divide up the shredded hoisin duck and vegetables equally among the tortillas.".to_string(),
            "Drizzle with hoisin sauce then wrap up and serve.".to_string(),
        ],
    }
}
