//! `se.reciba.api.recipes.GreekWraps`.

use crate::recipe::RecipeDef;
use crate::tag::Tag;
use chrono::NaiveDate;

pub fn recipe() -> RecipeDef {
    RecipeDef {
        object_name: "GreekWraps".to_string(),
        name: "Greek Kebab Wraps".to_string(),
        created_at: NaiveDate::from_ymd_opt(2023, 11, 9).unwrap(),
        permalink_override: Some("greek-wraps".to_string()),
        source: None,
        description: Some("Greek style vegetarian kebab wraps".to_string()),
        tagline: None,
        notes: vec!["You can use up other vegetables like courgettes or red onions. Just fry them at the same time as the other veg".to_string()],
        tags: vec![
            Tag::Lunch,
            Tag::Quick,
            Tag::Vegetarian,
        ],
        image: None,
        ingredients_blocks: crate::recipe::IngredientsBlock::simple(vec![
            crate::recipe::Ingredient::q("Vivera Veggie Greek Kebab", "125g"),
            crate::recipe::Ingredient::qp("Red Pepper", "1", "sliced"),
            crate::recipe::Ingredient::qp("Spring Onions", "3", "chopped"),
            crate::recipe::Ingredient::q("Tortilla Wraps", "4"),
            crate::recipe::Ingredient::opt("Cherry Tomatoes", None, Some("halved"), None),
            crate::recipe::Ingredient::new("Sriracha"),
            crate::recipe::Ingredient::new("Mayonnaise"),
            crate::recipe::Ingredient::new("Oil"),
        ]),
        method: vec![
            "Heat some oil in a large frying pan.".to_string(),
            "Throw in the Greek kebab mixture and cook per packet instructions.".to_string(),
            "Mix together a few tablespoons of mayonnaise and some Sriracha to make spicy mayo.".to_string(),
            "Warm the tortillas in a microwave per packet instructions and spread with the spicy mayo".to_string(),
            "A few minutes before the kebab mixture is ready throw in the peppers.".to_string(),
            "Add the spring onions and cherry tomatoes to the pan and cook for a further 30 seconds.".to_string(),
            "Divide up the mixture equally among the tortillas.".to_string(),
        ],
    }
}
