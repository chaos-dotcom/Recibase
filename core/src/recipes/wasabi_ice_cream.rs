//! `se.reciba.api.recipes.WasabiIceCream`.

use crate::recipe::RecipeDef;
use crate::tag::Tag;
use chrono::NaiveDate;

pub fn recipe() -> RecipeDef {
    RecipeDef {
        object_name: "WasabiIceCream".to_string(),
        name: "Wasabi Ice Cream".to_string(),
        created_at: NaiveDate::from_ymd_opt(2021, 6, 12).unwrap(),
        permalink_override: None,
        source: Some("Kit".to_string()),
        description: Some("A simple yet decadent spicy ice cream recipe, no churn needed.".to_string()),
        tagline: None,
        notes: vec![
            "You can substitute wasabi for blackberries or any ingredient you fancy. Just be careful not to choose anything too watery, or you'll get ice crystals forming. It's also lovely plain.".to_string(),
            crate::ice_cream::generic_notes(),
        ],
        tags: vec![Tag::Pudding],
        image: Some(crate::recipe::Image::new("https://i.reciba.se/wasabi-ice-cream.jpg")),
        ingredients_blocks: vec![
            crate::ice_cream::generic_ingredients().prefix_ingredients(vec![
                crate::recipe::Ingredient::q("Wasabi paste", "2 tsp"),
                crate::recipe::Ingredient::opt("Green food colouring", Some("1/2 tsp"), None, Some("optional")),
            ]),
        ],
        method: [
            crate::ice_cream::generic_method_start(),
            vec![
                "Add the mascarpone cheese and food colouring, then continue whisking.".to_string(),
                "Add the wasabi, one teaspon at a time. Whisk thoroughly and taste each time. This makes for a hot flavour, so you may only need 1.".to_string(),
            ],
            crate::ice_cream::generic_method_end(),
        ]
        .concat(),
    }
}
