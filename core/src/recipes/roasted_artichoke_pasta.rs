//! `se.reciba.api.recipes.RoastedArtichokePasta`.

use crate::recipe::RecipeDef;
use crate::tag::Tag;
use chrono::NaiveDate;

pub fn recipe() -> RecipeDef {
    RecipeDef {
        object_name: "RoastedArtichokePasta".to_string(),
        name: "Roasted Artichoke Pasta".to_string(),
        created_at: NaiveDate::from_ymd_opt(2023, 8, 8).unwrap(),
        permalink_override: None,
        source: Some("Stephani".to_string()),
        description: None,
        tagline: None,
        notes: vec![
            "Ingredients are per person, so scale it appropriately.".to_string(),
            format!("12 minutes for tomatoes and 27 for (Waitrose) artichokes at {} in your oven. Can probably -2 to +5 on artichokes without much trouble.", crate::utils::int_utils::celsius(200)),
            "Waiting for the pasta and vegetables to cool before mixing with the ricotta is optional. It depends how to feel about heat affecting the ricotta's texture.".to_string(),
        ],
        tags: vec![
            Tag::Vegetarian,
            Tag::Scales,
            Tag::LowEffort,
            Tag::StephaniUnhealthy,
        ],
        image: None,
        ingredients_blocks: crate::recipe::IngredientsBlock::simple(vec![
            crate::recipe::Ingredient::q("Sun-dried Tomatoes", "100g"),
            crate::recipe::Ingredient::q("Roasted Artichokes", "140g"),
            crate::recipe::Ingredient::opt("Pine Nuts", None, None, Some("optional")),
            crate::recipe::Ingredient::q("Ricotta", "80-125g"),
            crate::recipe::Ingredient::opt("Pasta", None, None, Some("preferably fresh")),
            crate::recipe::Ingredient::opt("Mixed Herbs", None, None, Some("I use a Herbes de Provence blend")),
            crate::recipe::Ingredient::new("Salt"),
        ]),
        method: vec![
            format!("Preheat the oven to {}.", crate::utils::int_utils::celsius(200)),
            "Find a roasting tray with space for the artichokes and sun-dried tomatoes.".to_string(),
            "Drain the jar of artichokes and lay them on the tray. Roast for 20-30 minutes.".to_string(),
            "Drain the jar of sun-dried tomatoes and add them to the tray for the last 8-12 minutes.".to_string(),
            "While the other ingredients are roasting add the ricotta and herbs to a mixing bowl. Mix well.".to_string(),
            "If you're adding pine nuts then gently fry them in oil in a small frying pan.".to_string(),
            "Cook the pasta per packet instructions then drain and leave to cool.".to_string(),
            "Take the baking tray out of the oven and add all ingredients to the mixing bowl. Mix well and serve.".to_string(),
        ],
    }
}
