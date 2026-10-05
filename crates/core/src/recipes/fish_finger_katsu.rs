//! `se.reciba.api.recipes.FishFingerKatsu`.
//! chaos-tag: casa-chaos

use crate::recipe::RecipeDef;
use crate::tag::Tag;
use chrono::NaiveDate;

pub fn recipe() -> RecipeDef {
    RecipeDef {
        object_name: "FishFingerKatsu".to_string(),
        name: "Fish Finger Katsu".to_string(),
        created_at: NaiveDate::from_ymd_opt(2023, 5, 1).unwrap(),
        permalink_override: None,
        source: Some("Waitrose".to_string()),
        description: None,
        tagline: None,
        notes: vec![
            "Serves 2".to_string(),
            "Chiu chow chilli oil: use this condiment to add a spicy kick to all kinds of dishes. Prefer things a bit milder? Omit it from the recipe or swap for 1/4 tsp Cooks' Ingredients Peppery Pul Biber.".to_string(),
        ],
        tags: vec![
            Tag::Pescatarian,
            Tag::Quick,
            Tag::LowEffort,
            Tag::Scales,
        ],
        image: None,
        ingredients_blocks: vec![
            crate::recipe::IngredientsBlock::new(None, vec![
                crate::recipe::Ingredient::q("Waitrose 6 Chunky Cod Fish Fingers", "330g pack"),
                crate::recipe::Ingredient::q("New Kenji Sushi Rice", "250g pack"),
                crate::recipe::Ingredient::q("Waitrose Katsu Curry Sauce", "140g pack"),
            ]),
            crate::recipe::IngredientsBlock::new(Some("Cucumber Salad"), vec![
                crate::recipe::Ingredient::q("Chinese Rice Vinegar", "1 tbsp"),
                crate::recipe::Ingredient::q("Reduced Salt Soy Sauce", "2 tsp"),
                crate::recipe::Ingredient::q("Toasted Sesame Oil", "1/2 tsp"),
                crate::recipe::Ingredient::opt("Chiu Chow Chilli Oil", Some("1/2 tsp"), None, Some("Or 1/4 tsp chilli flakes")),
                crate::recipe::Ingredient::qp("Essential Cucumber", "170g", "roughly chopped"),
                crate::recipe::Ingredient::qp("Essential Radish", "100g", "quartered"),
            ]),
        ],
        method: vec![
            format!("Preheat the oven to {} and put a baking tray inside to heat up. Transfer the fish fingers to the hot tray and bake in the oven for 12-14 minutes, or longer until cooked through.", crate::utils::int_utils::celsius(200)),
            "Meanwhile, make the cucumber salad. In a bowl, mix the rice vinegar, soy sauce, sesame oil and chilli oil (or flakes). Add the cucumber and radishes, then toss to coat.".to_string(),
            "Microwave the sushi rice according to pack instructions; set aside. Heat the katsu sauce according to pack instructions. Divide the rice between plates, top with the fish fingers and spoon over the katsu sauce. Serve with the cucumber and radish salad.".to_string(),
        ],
    }
}
