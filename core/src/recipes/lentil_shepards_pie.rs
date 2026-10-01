//! `se.reciba.api.recipes.LentilShepardsPie`.

use crate::recipe::RecipeDef;
use crate::tag::Tag;
use chrono::NaiveDate;

pub fn recipe() -> RecipeDef {
    RecipeDef {
        object_name: "LentilShepardsPie".to_string(),
        name: "Veggie Shepherd's Pie".to_string(),
        created_at: NaiveDate::from_ymd_opt(2020, 4, 24).unwrap(),
        permalink_override: None,
        source: None,
        description: None,
        tagline: None,
        notes: Vec::new(),
        tags: vec![
            Tag::Vegetarian,
            Tag::Slow,
            Tag::HighEffort,
            Tag::Scales,
            Tag::ColdWeather,
        ],
        image: None,
        ingredients_blocks: crate::recipe::IngredientsBlock::simple(vec![
            crate::recipe::Ingredient::qp("Onions", "1", "Diced"),
            crate::recipe::Ingredient::qp("Carrots", "4", "Diced"),
            crate::recipe::Ingredient::qpn("Celery", "1 head", "Chopped", "Optional"),
            crate::recipe::Ingredient::qp("Garlic", "4 Cloves", "Finely chopped"),
            crate::recipe::Ingredient::qp("Chestnut mushrooms", "200g", "sliced"),
            crate::recipe::Ingredient::q("Red Lentils", "230g"),
            crate::recipe::Ingredient::new("Butter"),
            crate::recipe::Ingredient::opt("Bay Leaf", Some("2"), None, Some("Optional")),
            crate::recipe::Ingredient::q("Thyme", "1 tbsp"),
            crate::recipe::Ingredient::opt("Red wine", Some("100ml"), None, Some("Optional")),
            crate::recipe::Ingredient::q("Stock cube", "1"),
            crate::recipe::Ingredient::q("Tomato purée", "3 tbsp"),
            crate::recipe::Ingredient::opt("King Edwards Potatoes", Some("500g"), None, Some("Other floury potatoes will do")),
            crate::recipe::Ingredient::q("Butter", "85g"),
            crate::recipe::Ingredient::q("Milk", "100ml"),
            crate::recipe::Ingredient::qp("Cheddar", "50g", "Grated"),
        ]),
        method: vec![
            "To make the sauce, heat the butter in a pan, then gently fry the onions, carrots, celery and garlic for 15 mins until soft and golden.".to_string(),
            "Turn up the heat, add the mushrooms, then cook for 4 mins more.".to_string(),
            "Add the herbs, lentils, wine and stock. It's important that you do not season with salt at this stage.".to_string(),
            "Simmer for 40-50 mins until the lentils are very soft.".to_string(),
            "Take off the heat and stir in the tomato purée. Season to taste.".to_string(),
            "While the lentils are cooking, tip the potatoes into a pan of water, then boil for about 15 mins until tender.".to_string(),
            "Drain the potatoes well, then mash with the butter and milk.".to_string(),
            "Pour the lentil mixture into a casserole dish and top with the mash potatoes.".to_string(),
            "Use a fork to rake the surface of the mash potatoes. This will help golden the dish.".to_string(),
            "Scatter with grated cheddar then bake in a 190C/fan 170C oven for 30 minutes, until golden.".to_string(),
        ],
    }
}
