//! `se.reciba.api.recipes.IndianPatties`.

use crate::recipe::RecipeDef;
use crate::tag::Tag;
use chrono::NaiveDate;

pub fn recipe() -> RecipeDef {
    RecipeDef {
        object_name: "IndianPatties".to_string(),
        name: "Indian Patties".to_string(),
        created_at: NaiveDate::from_ymd_opt(2020, 4, 24).unwrap(),
        permalink_override: None,
        source: Some("Vegetarian Cookery Bible (2012: Reader's Digest)".to_string()),
        description: Some("Vegetarian burger patties made with red lentils, spinach, mint and spices.".to_string()),
        tagline: None,
        notes: vec![
            "Work well served with sweet chilli sauce and/or mayonnaise.".to_string(),
            "Don't waste time finely chopping the ingredients as they're blended anyway.".to_string(),
            "If you don't have a blender you can chop the garlic, chilli and ginger finely then cook the onion at the same time.".to_string(),
            "Can take a long time.".to_string(),
        ],
        tags: vec![
            Tag::Vegetarian,
            Tag::HighEffort,
            Tag::Slow,
            Tag::Spicy,
        ],
        image: None,
        ingredients_blocks: crate::recipe::IngredientsBlock::simple(vec![
            crate::recipe::Ingredient::q("Red lentils", "225g"),
            crate::recipe::Ingredient::q("Whole cloves", "1-2"),
            crate::recipe::Ingredient::opt("Coriander seeds", Some("2 tsp"), None, Some("Or ground coriander")),
            crate::recipe::Ingredient::q("Cumin seeds", "1-2 tsp"),
            crate::recipe::Ingredient::q("Black peppercorns", "1 tsp"),
            crate::recipe::Ingredient::qp("Garlic", "2-3 cloves", "chopped or crushed"),
            crate::recipe::Ingredient::qp("Ginger", "1cm piece", "peeled and chopped/grated"),
            crate::recipe::Ingredient::qp("Onion", "1", "diced"),
            crate::recipe::Ingredient::q("Spinach", "225g"),
            crate::recipe::Ingredient::new("Coriander leaves"),
            crate::recipe::Ingredient::new("Mint leaves"),
            crate::recipe::Ingredient::qp("Chillies", "1-2", "chopped, seeds removed if desired"),
            crate::recipe::Ingredient::new("Ground cinnamon"),
            crate::recipe::Ingredient::q("Egg", "1"),
            crate::recipe::Ingredient::new("Salt"),
        ]),
        method: vec![
            "Set the lentils aside in boiling water to soften.".to_string(),
            "Meanwhile, dry-roast the cloves, cumin seeds, coriander seeds and black peppercorns in a pan. Grid thoroughly with a mortar and pestle and set aside.".to_string(),
            "Add the chillies, garlic and ginger to the pan with a little oil and soften. Add the (drained) lentils, along with the spinach, coriander and mint and cook for 5-10 minutes.".to_string(),
            "Add the ground spices, cinnamon, egg and salt and mix.".to_string(),
            "Blend the mixture thoroughly.".to_string(),
            "Add the onion and pulse blender until finely chopped but not pureed.".to_string(),
            "Form the mixture into ~5cm patties and brush with oil on both sides.".to_string(),
            "Place them on a baking tray under the grill until brown and crisp on both sides, turning as necessary.".to_string(),
            "Serve hot.".to_string(),
        ],
    }
}
