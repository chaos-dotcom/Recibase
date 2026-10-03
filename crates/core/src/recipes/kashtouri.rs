//! `se.reciba.api.recipes.Kashtouri`.

use crate::recipe::RecipeDef;
use crate::tag::Tag;
use chrono::NaiveDate;

pub fn recipe() -> RecipeDef {
    RecipeDef {
        object_name: "Kashtouri".to_string(),
        name: "Kashtouri".to_string(),
        created_at: NaiveDate::from_ymd_opt(2020, 4, 24).unwrap(),
        permalink_override: None,
        source: Some("Vegetarian Cookery Bible (2012: Reader's Digest)".to_string()),
        description: Some("An Egyptian dish of rice, macaroni and lentils in a spicy tomato sauce.".to_string()),
        tagline: None,
        notes: Vec::new(),
        tags: vec![
            Tag::Vegan,
            Tag::Scales,
            Tag::HotWeather,
            Tag::LowEffort,
        ],
        image: None,
        ingredients_blocks: crate::recipe::IngredientsBlock::simple(vec![
            crate::recipe::Ingredient::q("Risotto rice", "150g"),
            crate::recipe::Ingredient::q("Macaroni", "150g"),
            crate::recipe::Ingredient::opt("Red lentils", Some("150g"), None, Some("Or 1 400g tin green lentils")),
            crate::recipe::Ingredient::qp("Onion", "1", "finely chopped"),
            crate::recipe::Ingredient::opt("Garlic", None, Some("finely chopped or crushed"), None),
            crate::recipe::Ingredient::q("Chopped tomatoes", "1 400g tin"),
            crate::recipe::Ingredient::q("Cayenne pepper", "1-2 tsp"),
            crate::recipe::Ingredient::q("Ground coriander", "1 tsp"),
            crate::recipe::Ingredient::new("Lemon juice"),
            crate::recipe::Ingredient::new("Salt"),
            crate::recipe::Ingredient::new("Black pepper"),
        ]),
        method: vec![
            "Bring water to a boil and fill a large saucepan. Add the rice and simmer for 5 minutes.".to_string(),
            "Add the macaroni and lentils and simmer for 10 minutes or until rice, macaroni and lentils are tender.".to_string(),
            "Drain and set aside.".to_string(),
            "Meanwhile, heat some oil in a large saucepan and cook onions and garlic until softened. Stir in cayenne pepper, coriander, chopped tomatoes and lemon juice. Add salt and pepper to taste.".to_string(),
            "Simmer for 5-10 minutes, stirring occasionally.".to_string(),
            "Add rice, macaroni and lentils to the mixture.".to_string(),
            "Serve hot.".to_string(),
        ],
    }
}
