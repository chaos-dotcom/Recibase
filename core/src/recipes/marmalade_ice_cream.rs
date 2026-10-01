//! `se.reciba.api.recipes.MarmaladeIceCream`.

use crate::recipe::RecipeDef;
use crate::tag::Tag;
use chrono::NaiveDate;

pub fn recipe() -> RecipeDef {
    RecipeDef {
        object_name: "MarmaladeIceCream".to_string(),
        name: "Marmalade Ice Cream".to_string(),
        created_at: NaiveDate::from_ymd_opt(2020, 7, 12).unwrap(),
        permalink_override: None,
        source: Some("George".to_string()),
        description: Some("A sweet and tart ice cream recipe, no churn needed.".to_string()),
        tagline: None,
        notes: vec![
            "You can substitute marmalade for blackberries, cinnamon or any ingredient you fancy. Just be careful not to choose anything too watery, or you'll get ice crystals forming. It's also lovely plain.".to_string(),
            crate::ice_cream::generic_notes(),
        ],
        tags: vec![Tag::Pudding],
        image: Some(crate::recipe::Image::new("https://i.reciba.se/ice-cream.jpg")),
        ingredients_blocks: vec![
            crate::ice_cream::generic_ingredients().prefix_ingredients(vec![
                crate::recipe::Ingredient::opt("Marmalade", Some("125g"), None, Some("any premium brand")),
            ]),
        ],
        method: [
            crate::ice_cream::generic_method_start(),
            vec![
                "Add the mascarpone cheese and marmalade and continue whisking until thoroughly mixed.".to_string(),
            ],
            crate::ice_cream::generic_method_end(),
        ]
        .concat(),
    }
}
