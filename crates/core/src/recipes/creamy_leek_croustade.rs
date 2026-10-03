//! `se.reciba.api.recipes.CreamyLeekCroustade`.
//! chaos-tag: casa-chaos

use crate::recipe::RecipeDef;
use crate::tag::Tag;
use chrono::NaiveDate;

pub fn recipe() -> RecipeDef {
    RecipeDef {
        object_name: "CreamyLeekCroustade".to_string(),
        name: "Creamy Leek Croustade".to_string(),
        created_at: NaiveDate::from_ymd_opt(2026, 10, 3).unwrap(),
        permalink_override: None,
        source: Some("The Cranks Recipe Book (1982: Dent)".to_string()),
        description: Some(
            "A nutty wholemeal crumb base topped with a creamy leek and tomato sauce.".to_string(),
        ),
        tagline: None,
        notes: Vec::new(),
        tags: vec![
            Tag::Vegetarian,
            Tag::Stodge,
            Tag::ColdWeather,
            Tag::Scales,
        ],
        image: None,
        ingredients_blocks: vec![
            crate::recipe::IngredientsBlock::new(
                Some("Base"),
                vec![
                    crate::recipe::Ingredient::q("Fresh wholemeal breadcrumbs", "175g"),
                    crate::recipe::Ingredient::q("Butter or margarine", "50g"),
                    crate::recipe::Ingredient::qp("Cheddar cheese", "100g", "grated"),
                    crate::recipe::Ingredient::qp("Mixed nuts", "100g", "chopped"),
                    crate::recipe::Ingredient::q("Mixed herbs", "1/2 tsp (2.5 ml)"),
                    crate::recipe::Ingredient::qp("Garlic clove", "1", "crushed"),
                ],
            ),
            crate::recipe::IngredientsBlock::new(
                Some("Sauce"),
                vec![
                    crate::recipe::Ingredient::q("Medium-sized leeks", "3"),
                    crate::recipe::Ingredient::q("Tomatoes", "4"),
                    crate::recipe::Ingredient::q("Butter or margarine", "50g"),
                    crate::recipe::Ingredient::q("Wholemeal flour", "25g"),
                    crate::recipe::Ingredient::q("Milk", "284ml"),
                    crate::recipe::Ingredient::q("Salt & pepper", "to taste"),
                    crate::recipe::Ingredient::q("Fresh wholemeal breadcrumbs", "4 tbsp (60 ml)"),
                ],
            ),
        ],
        method: vec![
            "Base: Put the breadcrumbs in a basin, rub in the butter, then add the remaining ingredients.".to_string(),
            "Press the mixture into an 11 x 7\" (28 x 18 cm) tin.".to_string(),
            "Bake in the oven at 220°C (425°F/Mark 7) for 15-20 minutes, until golden brown.".to_string(),
            "Sauce: Meanwhile, slice the leeks and chop the tomatoes.".to_string(),
            "Melt the butter in a saucepan.".to_string(),
            "Sauté the leeks for 5 minutes, then stir in the flour.".to_string(),
            "Add the milk, stirring constantly, then bring to the boil and reduce the heat to a simmer.".to_string(),
            "Add the remaining ingredients, except the breadcrumbs, and simmer for a few minutes to soften the tomatoes.".to_string(),
            "Check the seasoning.".to_string(),
            "Spoon the vegetable mixture over the base, sprinkle with the breadcrumbs, and heat through in the oven at 180°C (350°F/Mark 4) for 20 minutes.".to_string(),
            "Serve at once.".to_string(),
        ],
    }
}
