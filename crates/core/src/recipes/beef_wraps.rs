//! `se.reciba.api.recipes.BeefWraps`.

use crate::recipe::RecipeDef;
use crate::tag::Tag;
use chrono::NaiveDate;

pub fn recipe() -> RecipeDef {
    RecipeDef {
        object_name: "BeefWraps".to_string(),
        name: "Beef Wraps".to_string(),
        created_at: NaiveDate::from_ymd_opt(2025, 2, 3).unwrap(),
        permalink_override: None,
        source: None,
        description: Some("Spiced vegetarian beef wraps with black beans".to_string()),
        tagline: None,
        notes: vec!["You can use up other vegetables like courgettes or red onions. Just fry them at the same time as the other veg.".to_string()],
        tags: vec![
            Tag::Lunch,
            Tag::Quick,
            Tag::Vegetarian,
        ],
        image: None,
        ingredients_blocks: crate::recipe::IngredientsBlock::simple(vec![
            crate::recipe::Ingredient::q("Vegetarian Beef Strips", "175g"),
            crate::recipe::Ingredient::qp("Red Pepper", "1", "sliced"),
            crate::recipe::Ingredient::qp("Spring Onions", "3", "chopped"),
            crate::recipe::Ingredient::q("Tortilla Wraps", "4"),
            crate::recipe::Ingredient::opt("Cherry Tomatoes", None, Some("halved"), None),
            crate::recipe::Ingredient::q("Black Beans", "1 400g tin"),
            crate::recipe::Ingredient::new("Cayenne pepper"),
            crate::recipe::Ingredient::new("Smoked Paprika"),
            crate::recipe::Ingredient::new("Mayonnaise"),
            crate::recipe::Ingredient::new("Oil"),
        ]),
        method: vec![
            "Heat some oil in a large frying pan.".to_string(),
            "Throw in the beef strips along with the cayenne pepper and paprika. Cook per packet instructions.".to_string(),
            "Warm the tortillas in a microwave per packet instructions and spread with some mayonnaise".to_string(),
            "A few minutes before the beef strips are ready throw in the peppers.".to_string(),
            "Add the spring onions, cherry tomatoes and black beans to the pan and cook for a further 30 seconds.".to_string(),
            "Divide up the mixture equally among the tortillas.".to_string(),
            "Wrap up and serve.".to_string(),
        ],
    }
}
