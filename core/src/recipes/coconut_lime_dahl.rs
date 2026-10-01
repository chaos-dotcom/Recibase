//! `se.reciba.api.recipes.CoconutLimeDahl`.

use crate::recipe::RecipeDef;
use crate::tag::Tag;
use chrono::NaiveDate;

pub fn recipe() -> RecipeDef {
    RecipeDef {
        object_name: "CoconutLimeDahl".to_string(),
        name: "Coconut Lime Dahl".to_string(),
        created_at: NaiveDate::from_ymd_opt(2021, 12, 26).unwrap(),
        permalink_override: None,
        source: Some("https://greedypanda.co.uk/2021/05/coconut-lime-dal/".to_string()),
        description: None,
        tagline: None,
        notes: vec![
            "Serves 4".to_string(),
            "You can cook this in a rice cooker, using the slow cooker setting, although the lentils may degrade more.".to_string(),
            "The garlic will have a stronger flavour than normal, because it is boiled rather than cooked. Consider adjusting quantity.".to_string(),
        ],
        tags: vec![
            Tag::Vegan,
            Tag::Spicy,
            Tag::LowEffort,
            Tag::Scales,
        ],
        image: None,
        ingredients_blocks: crate::recipe::IngredientsBlock::simple(vec![
            crate::recipe::Ingredient::q("Lentils", "450g"),
            crate::recipe::Ingredient::q("Turmeric", "1 tsp"),
            crate::recipe::Ingredient::qp("Garlic", "2 Cloves", "Diced"),
            crate::recipe::Ingredient::q("Coconut Milk", "1 Tin"),
            crate::recipe::Ingredient::q("Boiling Water", "600ml"),
            crate::recipe::Ingredient::qp("Onion", "1/2", "Chopped"),
            crate::recipe::Ingredient::q("Cumin", "1 tsp"),
            crate::recipe::Ingredient::qp("Lime", "1", "Juiced and zested"),
            crate::recipe::Ingredient::q("Spinach", "9 handfuls"),
            crate::recipe::Ingredient::new("Oil"),
        ]),
        method: vec![
            "Add the lentils, turmeric, garlic, coconut milk and boiling water to a large pan.".to_string(),
            "The water back to the boil, stir, then gently simmer for 40m.".to_string(),
            "Meanwhile, oil a frying pan and gently cook the onions in cumin for a few minutes.".to_string(),
            "Take the frying pan off the heat and leave aside for later.".to_string(),
            "Once the lentils have cooked, mix in the onions, baby spinach and lime juice.".to_string(),
            "Serve garnished with lime zest.".to_string(),
        ],
    }
}
