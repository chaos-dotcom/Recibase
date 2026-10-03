//! `se.reciba.api.recipes.PistachioIceCream`.

use crate::recipe::RecipeDef;
use crate::tag::Tag;
use chrono::NaiveDate;

pub fn recipe() -> RecipeDef {
    RecipeDef {
        object_name: "PistachioIceCream".to_string(),
        name: "Pistachio Ice Cream".to_string(),
        created_at: NaiveDate::from_ymd_opt(2021, 6, 12).unwrap(),
        permalink_override: None,
        source: Some("Kit".to_string()),
        description: Some("A simple yet decadent Pistachio ice cream recipe, no churn needed.".to_string()),
        tagline: None,
        notes: vec![
            "You can substitute pistachios for blackberries or any ingredient you fancy. Just be careful not to choose anything too watery, or you'll get ice crystals forming. It's also lovely plain.".to_string(),
            crate::ice_cream::generic_notes(),
        ],
        tags: vec![Tag::Pudding],
        image: Some(crate::recipe::Image::new("https://i.reciba.se/pistachio-ice-cream.jpg")),
        ingredients_blocks: vec![
            crate::ice_cream::generic_ingredients().prefix_ingredients(vec![
                crate::recipe::Ingredient::q("Pistachios", "175g"),
            ]),
        ],
        method: [
            vec![
                "Crush or blend the pistachios to form a paste. Consider setting aside 3-4 pistachios as a garnish.".to_string(),
            ],
            crate::ice_cream::generic_method_start(),
            vec![
                "Add the mascarpone cheese and pistachios, then continue whisking.".to_string(),
                "Decant into a freezer suitable dish, garnish with the remaining pistachios, then freeze for at least 6 hours.".to_string(),
            ],
        ]
        .concat(),
    }
}
