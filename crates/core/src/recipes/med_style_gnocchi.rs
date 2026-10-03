//! `se.reciba.api.recipes.MedStyleGnocchi`.
//! chaos-tag: casa-chaos

use crate::recipe::RecipeDef;
use crate::tag::Tag;
use chrono::NaiveDate;

pub fn recipe() -> RecipeDef {
    RecipeDef {
        object_name: "MedStyleGnocchi".to_string(),
        name: "Med-style Gnocchi".to_string(),
        created_at: NaiveDate::from_ymd_opt(2023, 9, 1).unwrap(),
        permalink_override: None,
        source: Some("Waitrose".to_string()),
        description: Some("A flavour-packed, one-pan recipe that is on the table in just 35 minutes.".to_string()),
        tagline: None,
        notes: vec![
            "Serves 4".to_string(),
            "Basil: this classic Mediterranean herb goes with anything tomato-based - add it generously to most pizza and pasta dishes. It's also great blended with olive oil as an instant dressing for grilled chicken and bean salads.".to_string(),
        ],
        tags: vec![
            Tag::Vegetarian,
            Tag::Quick,
            Tag::LowEffort,
            Tag::Scales,
        ],
        image: None,
        ingredients_blocks: crate::recipe::IngredientsBlock::simple(vec![
            crate::recipe::Ingredient::qp("No.1 Red Choice Tomatoes", "300g", "quartered"),
            crate::recipe::Ingredient::q("Gnocchi", "500g"),
            crate::recipe::Ingredient::qp("Garlic", "2 cloves", "crushed"),
            crate::recipe::Ingredient::qp("Nonpareille Capers", "1 tbsp", "drained and rinsed"),
            crate::recipe::Ingredient::q("Essential Olive Oil", "2 1/2 tbsp"),
            crate::recipe::Ingredient::qp("Tenderstem Broccoli", "200g", "trimmed and cut into 3cm pieces"),
            crate::recipe::Ingredient::qp("Essential Mozzarella", "150g ball", "roughly torn"),
            crate::recipe::Ingredient::qpn("Basil Leaves", "Handful", "roughly torn", "To serve"),
        ]),
        method: vec![
            format!("Preheat the oven to {}. In a large roasting tin (about 21cm x 27cm), gently toss together the tomatoes, gnocchi, garlic, capers and 2 tbsp olive oil. Spread out evenly, season and roast for 12 minutes.", crate::utils::int_utils::celsius(200)),
            "Toss the broccoli with the remaining 1/2 tbsp oil and season. Add to the gnocchi tin and scatter with the torn mozzarella. Roast for a final 10-12 minutes.".to_string(),
            "Give everything a light stir and scatter with the basil to serve.".to_string(),
        ],
    }
}
