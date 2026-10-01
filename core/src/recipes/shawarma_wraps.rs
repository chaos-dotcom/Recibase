//! `se.reciba.api.recipes.ShawarmaWraps`.

use crate::recipe::RecipeDef;
use crate::tag::Tag;
use chrono::NaiveDate;

pub fn recipe() -> RecipeDef {
    RecipeDef {
        object_name: "ShawarmaWraps".to_string(),
        name: "Shawarma Wraps".to_string(),
        created_at: NaiveDate::from_ymd_opt(2026, 3, 27).unwrap(),
        permalink_override: None,
        source: Some("Sylvia".to_string()),
        description: Some("Roasted sweet potato, red pepper and veggie mince with eastern spices in a wrap".to_string()),
        tagline: None,
        notes: vec![
            "It might be useful mix the vegetables with oil and spices in a large bowl.".to_string(),
        ],
        tags: vec![Tag::VeganIsh, Tag::Scales],
        image: None,
        ingredients_blocks: vec![
            crate::recipe::IngredientsBlock::new(None, vec![
                crate::recipe::Ingredient::qp("Sweet Potato", "1", "cut into 2cm wedges"),
                crate::recipe::Ingredient::qp("Red Pepper", "1", "sliced"),
                crate::recipe::Ingredient::qp("Red Onion", "1", "thinly sliced"),
                crate::recipe::Ingredient::qp("Vivera Plant-Based Shawarma Kebab", "1", "175g"),
                crate::recipe::Ingredient::qp("Garlic", "3 cloves", "sliced"),
                crate::recipe::Ingredient::new("Wraps"),
                crate::recipe::Ingredient::opt("Siracha Sauce", None, None, Some("Optional")),
                crate::recipe::Ingredient::opt("Yoghurt", None, None, Some("Optional")),
                crate::recipe::Ingredient::opt("Mayonaise", None, None, Some("Optional")),
                crate::recipe::Ingredient::qp("Fresh coriander", "10g", "chopped"),
                crate::recipe::Ingredient::new("Olive Oil"),
                crate::recipe::Ingredient::new("Salt"),
                crate::recipe::Ingredient::new("Pepper"),
            ]),
            crate::recipe::IngredientsBlock::new(Some("Shawarma Spice mix"), vec![
                crate::recipe::Ingredient::q("Cumin", "1/2 tsp"),
                crate::recipe::Ingredient::q("Ground Coriander", "1/2 tsp"),
                crate::recipe::Ingredient::q("Smoked Paprika", "1/2 tsp"),
                crate::recipe::Ingredient::q("Ground Cinnamon", "1/4 tsp"),
                crate::recipe::Ingredient::q("Allspice", "1/4 tsp"),
                crate::recipe::Ingredient::q("Cayenne Pepper", "1/4 tsp"),
            ]),
        ],
        method: vec![
            format!("Preheat the oven to {}.", crate::utils::int_utils::celsius(200)),
            "Start with sweet potatoes because they take the longest to roast.".to_string(),
            "Spread the sweet potatoes out on a large roasting tray then drizzle with olive oil, the spice mix, salt and pepper. Roast for 30-40 minutes.".to_string(),
            "Repeat the process for the red pepper, red onion and garlic. Roasting them for 20-30 minutes.".to_string(),
            "Shortly before serving, heat up the shawarma kebab pieces in the oven for 5 minutes.".to_string(),
            "Microwave the wraps per packet instructions.".to_string(),
            "If you're using mayonaise then mix it with siracha.".to_string(),
            "Spread siracha mayonaise or yoghurt on the wraps then add the roasted vegetables and mince. Garnish with fresh coriander.".to_string(),
        ],
    }
}
