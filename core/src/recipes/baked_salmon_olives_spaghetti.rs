//! `se.reciba.api.recipes.BakedSalmonOlivesSpaghetti`.

use crate::recipe::RecipeDef;
use crate::tag::Tag;
use chrono::NaiveDate;

pub fn recipe() -> RecipeDef {
    RecipeDef {
        object_name: "BakedSalmonOlivesSpaghetti".to_string(),
        name: "Baked Salmon with Olives and Spaghetti".to_string(),
        created_at: NaiveDate::from_ymd_opt(2020, 4, 24).unwrap(),
        permalink_override: Some("salmon-olive-spaghetti".to_string()),
        source: None,
        description: Some("Baked salmon served on a bed of spaghetti, onions and olives.".to_string()),
        tagline: None,
        notes: vec![
            "Cover the baking tray in foil for easier cleaning later.".to_string(),
            "Use the bigger pan.".to_string(),
        ],
        tags: vec![
            Tag::Pescatarian,
            Tag::HotWeather,
        ],
        image: None,
        ingredients_blocks: crate::recipe::IngredientsBlock::simple(vec![
            crate::recipe::Ingredient::opt("Salmon fillets", Some("1 per person"), None, Some("Approximately 1 inch wide")),
            crate::recipe::Ingredient::opt("Capers", None, Some("finely chopped"), None),
            crate::recipe::Ingredient::new("Olive oil"),
            crate::recipe::Ingredient::qp("Onion", "1", "sliced"),
            crate::recipe::Ingredient::new("Pimento-stuffed olives"),
            crate::recipe::Ingredient::new("Lemon juice"),
            crate::recipe::Ingredient::opt("Thyme", None, None, Some("Dried or fresh")),
            crate::recipe::Ingredient::new("Spaghetti"),
            crate::recipe::Ingredient::opt("Cherry tomatoes", None, Some("halved"), Some("Optional")),
        ]),
        method: vec![
            format!("Preheat the oven at {}.", crate::utils::int_utils::celsius(160)),
            "Cook the spaghetti in a saucepan.".to_string(),
            "Meanwhile, rub the salmon fillets with the olive oil, lemon juice and the chopped capers. Sprinkle with salt.".to_string(),
            "Place in the oven and bake for 10-15 minutes or until cooked through. If you are unsure, check the thickest part of the salmon with a fork.".to_string(),
            "Drain the spaghetti and set aside.".to_string(),
            "In the same pan, heat a small amount of oil and cook the onions, leaving them softened but still crisp. Add the olives, tomatoes, thyme and lemon juice and warm.".to_string(),
            "Add the spaghetti and toss. Keep warm.".to_string(),
            "Serve the salmon fillets on top of the spaghetti mixture.".to_string(),
        ],
    }
}
