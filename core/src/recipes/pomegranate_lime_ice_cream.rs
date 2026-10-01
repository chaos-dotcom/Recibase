//! `se.reciba.api.recipes.PomegranateLimeIceCream`.

use crate::recipe::RecipeDef;
use crate::tag::Tag;
use chrono::NaiveDate;

pub fn recipe() -> RecipeDef {
    RecipeDef {
        object_name: "PomegranateLimeIceCream".to_string(),
        name: "Pomegranate & Lime Ice Cream".to_string(),
        created_at: NaiveDate::from_ymd_opt(2025, 3, 10).unwrap(),
        permalink_override: Some("pomegranate-lime-ice-cream".to_string()),
        source: Some("Kit".to_string()),
        description: None,
        tagline: None,
        notes: vec![
            crate::ice_cream::generic_notes(),
            "Kit tasting notes: A perfectly enjoyable but fairly tame flavour profile. At least compared to the weirder flavours I've made. I'm glad I added as much lime as I did because it provides a necessary punchy comparison to the subtler pomegranate. I should have used fresh lime juice and zest but I think I went shopping in a bit of a rush.".to_string(),
        ],
        tags: vec![Tag::Pudding],
        image: Some(crate::recipe::Image::new("https://i.reciba.se/pomegranate-lime-ice-cream.jpg")),
        ingredients_blocks: vec![
            crate::ice_cream::generic_ingredients().prefix_ingredients(vec![
                crate::recipe::Ingredient::opt("Pomegranate powder", Some("20g"), None, Some("freeze dried, no added sugar")),
                crate::recipe::Ingredient::q("Lime Juice", "2tsp"),
            ]),
        ],
        method: [
            crate::ice_cream::generic_method_start(),
            vec![
                "Add the mascarpone cheese, pomegranate powder and lime juice. Gently stir in the powder to avoid it going everywhere.".to_string(),
                "Continue whisking until thoroughly mixed.".to_string(),
            ],
            crate::ice_cream::generic_method_end(),
        ]
        .concat(),
    }
}
