//! `se.reciba.api.recipes.SeafoodLasagne`.

use crate::recipe::RecipeDef;
use crate::tag::Tag;
use chrono::NaiveDate;

pub fn recipe() -> RecipeDef {
    RecipeDef {
        object_name: "SeafoodLasagne".to_string(),
        name: "Seafood Lasagne".to_string(),
        created_at: NaiveDate::from_ymd_opt(2020, 4, 24).unwrap(),
        permalink_override: None,
        source: None,
        description: None,
        tagline: None,
        notes: Vec::new(),
        tags: vec![
            Tag::Pescatarian,
            Tag::Slow,
            Tag::Stodge,
            Tag::ColdWeather,
        ],
        image: None,
        ingredients_blocks: crate::recipe::IngredientsBlock::simple(vec![
            crate::recipe::Ingredient::q("Semi-skimmed milk", "1L"),
            crate::recipe::Ingredient::opt("Mixed seafood", Some("400g"), None, Some("you can buy a pie mix or pick and mix your own")),
            crate::recipe::Ingredient::q("Garlic clove", "1"),
            crate::recipe::Ingredient::q("Plain flour", "50g"),
            crate::recipe::Ingredient::q("English mustard", "1 tsp"),
            crate::recipe::Ingredient::q("Fresh lasagne sheets", "400g"),
            crate::recipe::Ingredient::q("Tarragon", "2 tbsp"),
            crate::recipe::Ingredient::q("Baby spinach", "large handful"),
            crate::recipe::Ingredient::qp("Cheddar", "150g", "Grated"),
            crate::recipe::Ingredient::qp("Parmesan", "75g", "Grated"),
            crate::recipe::Ingredient::new("Butter"),
        ]),
        method: vec![
            format!("Preheat the oven to {}.", crate::utils::int_utils::celsius(180)),
            "Melt a large knob of butter in a saucepan with a low heat.".to_string(),
            "Add garlic and pepper then cook for a short while.".to_string(),
            "Sift in flour while stirring then cook for 2-3 minutes.".to_string(),
            "Slowly add milk over a medium heat.".to_string(),
            "Turn down the heat then add mustard and tarragon.".to_string(),
            "Place a spoonful or two of the sauce into a greased ovenproof dish to just cover the bottom. Cover with a couple of lasagne sheets.".to_string(),
            "Stir the fish, seafood and spinach to the remaining sauce and stir until well combined.".to_string(),
            "Spoon some of the fish sauce mixture onto the lasagne sheets and top with another layer of lasagne sheets. Repeat this layering of fish sauce mixture and lasagne until all of the sauce and pasta have been used up. The final layer should be lasagne sheets.".to_string(),
            "Sprinkle the top with cheddar cheese and parmesan, then transfer to the oven to bake for 40 minutes, or until the fish is cooked and the topping is golden-brown.".to_string(),
            "Serve at the table in its dish with a bowl of green salad and some garlic bread.".to_string(),
        ],
    }
}
