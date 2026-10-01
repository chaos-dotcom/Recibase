//! `se.reciba.api.recipes.ToadInTheHole`.

use crate::recipe::RecipeDef;
use crate::tag::Tag;
use chrono::NaiveDate;

pub fn recipe() -> RecipeDef {
    RecipeDef {
        object_name: "ToadInTheHole".to_string(),
        name: "Toad in the Hole".to_string(),
        created_at: NaiveDate::from_ymd_opt(2021, 9, 1).unwrap(),
        permalink_override: Some("toad-in-the-hole".to_string()),
        source: Some("Jeremy".to_string()),
        description: None,
        tagline: None,
        notes: vec![
            "If you're using vegetarian sausages don't fully cook them beforehand, otherwise they get too dry.".to_string(),
        ],
        tags: vec![Tag::Vegetarian, Tag::LowEffort],
        image: Some(crate::recipe::Image::new("https://i.reciba.se/toad-in-the-hole.jpg")),
        ingredients_blocks: crate::recipe::IngredientsBlock::simple(vec![
            crate::recipe::Ingredient::q("Sausages", "6"),
            crate::recipe::Ingredient::q("Plain Flour", "150g"),
            crate::recipe::Ingredient::q("Eggs", "2"),
            crate::recipe::Ingredient::q("Milk", "125ml"),
            crate::recipe::Ingredient::q("Water", "125ml"),
            crate::recipe::Ingredient::q("Salt", "A pinch"),
            crate::recipe::Ingredient::q("Gravy granules", "7 tsp"),
            crate::recipe::Ingredient::new("Red wine"),
            crate::recipe::Ingredient::q("Marmite", "1/2 tsp"),
        ]),
        method: vec![
            "Cook the sausages. Then turn the oven up to 200C.".to_string(),
            "Sift the flour into a mixing bowl.".to_string(),
            "Add the salt, eggs, water and milk.".to_string(),
            "Whisk the ingredients for a few minutes.".to_string(),
            "Brush a 20cm by 26cm by 4cm baking tray with oil.".to_string(),
            "Warm the baking tray in the oven for 4 minutes".to_string(),
            "Evenly pour half the batter into the baking tray.".to_string(),
            "Place the sausages in the batter.".to_string(),
            "Pour the remainder of the batter over the sausages.".to_string(),
            "Bake in the oven for 25-30 minutes. You MUST NOT open the oven while it is cooking.".to_string(),
            "Meanwhile add the gravy granules and a splash of red wine to a measuring jug.".to_string(),
            "Top up the jug with boiling water to 400ml.".to_string(),
            "Stir in the marmite. Add more water if the gravy is too thick.".to_string(),
            "Serve the toad in the hole with lashings of gravy.".to_string(),
        ],
    }
}
