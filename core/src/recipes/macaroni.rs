//! `se.reciba.api.recipes.Macaroni`.

use crate::recipe::RecipeDef;
use crate::tag::Tag;
use chrono::NaiveDate;

pub fn recipe() -> RecipeDef {
    RecipeDef {
        object_name: "Macaroni".to_string(),
        name: "Macaroni".to_string(),
        created_at: NaiveDate::from_ymd_opt(2020, 4, 24).unwrap(),
        permalink_override: None,
        source: None,
        description: None,
        tagline: None,
        notes: vec!["We largely vibe the ingredients for this recipe.".to_string()],
        tags: vec![Tag::Scales, Tag::Vegetarian, Tag::ColdWeather, Tag::Stodge],
        image: None,
        ingredients_blocks: crate::recipe::IngredientsBlock::simple(vec![
            crate::recipe::Ingredient::q("Butter", "25g"),
            crate::recipe::Ingredient::q("Plain Flour", "2 tbsp"),
            crate::recipe::Ingredient::q("Milk", "1/2 pint (or more)"),
            crate::recipe::Ingredient::new("Medium Cheddar Cheese"),
            crate::recipe::Ingredient::q("Macaroni Pasta", "275g"),
        ]),
        method: vec![
            "Grate the cheese.".to_string(),
            "Melt the butter then stir in the flour a tablespoon at a time until you have a paste."
                .to_string(),
            "Cook the flour paste for a few minutes, stirring constantly.".to_string(),
            "Add milk a dribble at a time until it's fairly runny again.".to_string(),
            "Add in the grated cheese and bring up to the simmer then turn the heat right down."
                .to_string(),
            "In a second pan, boil slightly salted water then add and cook the macaroni."
                .to_string(),
            "Check texture then drain thoroughly and add to the sauce and mix. Serve.".to_string(),
        ],
    }
}
