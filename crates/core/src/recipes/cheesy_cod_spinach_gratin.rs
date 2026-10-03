//! `se.reciba.api.recipes.CheesyCodSpinachGratin`.

use crate::recipe::RecipeDef;
use crate::tag::Tag;
use chrono::NaiveDate;

pub fn recipe() -> RecipeDef {
    RecipeDef {
        object_name: "CheesyCodSpinachGratin".to_string(),
        name: "Cheesy Cod and Spinach Gratin".to_string(),
        created_at: NaiveDate::from_ymd_opt(2020, 4, 24).unwrap(),
        permalink_override: Some("cheesy-cod".to_string()),
        source: None,
        description: None,
        tagline: None,
        notes: Vec::new(),
        tags: vec![Tag::Pescatarian, Tag::LowEffort, Tag::Quick],
        image: Some(crate::recipe::Image::new(
            "https://i.reciba.se/cod-spinach-gratin.jpg",
        )),
        ingredients_blocks: crate::recipe::IngredientsBlock::simple(vec![
            crate::recipe::Ingredient::q("Cod", "2 110g fillets"),
            crate::recipe::Ingredient::q("Stock cube", "1"),
            crate::recipe::Ingredient::qp("Spinach", "80g", "torn or chopped"),
            crate::recipe::Ingredient::q("Panko breadcrumbs", "30g"),
            crate::recipe::Ingredient::qp("Cheddar", "40g", "grated"),
            crate::recipe::Ingredient::q("Soft cheese", "50g"),
            crate::recipe::Ingredient::qp("Water", "150ml", "boiling"),
            crate::recipe::Ingredient::opt("Crispy Potato Slices", None, None, Some("optional")),
        ]),
        method: vec![
            format!(
                "Heat the oven to {}.",
                crate::utils::int_utils::celsius(200)
            ),
            "Dissolve the stock cube and soft cheese in the boiling water.".to_string(),
            "Place the spinach in an ovenproof dish and pour over the cheesy stock.".to_string(),
            "Place the cod fillets on top of the spinach.".to_string(),
            "Sprinkle the cheddar and breadcrumbs over the cod.".to_string(),
            "Put in the oven for 15-20 mins or until the fish is cooked through.".to_string(),
        ],
    }
}
