//! `se.reciba.api.recipes.MascarponeIceCream`.

use crate::recipe::RecipeDef;
use crate::tag::Tag;
use chrono::NaiveDate;

pub fn recipe() -> RecipeDef {
    RecipeDef {
        object_name: "MascarponeIceCream".to_string(),
        name: "Mascarpone Ice Cream".to_string(),
        created_at: NaiveDate::from_ymd_opt(2020, 7, 12).unwrap(),
        permalink_override: None,
        source: Some("George".to_string()),
        description: Some("A simple yet decadent ice cream recipe, no churn needed.".to_string()),
        tagline: Some("Don't eat too much at once...".to_string()),
        notes: vec![
            "You can substitute marmalade for blackberries, cinnamon or any ingredient you fancy. Just be careful not to choose anything too watery, or you'll get ice crystals forming. It's also lovely plain.".to_string(),
            crate::ice_cream::generic_notes(),
        ],
        tags: vec![Tag::Pudding],
        image: Some(crate::recipe::Image::new("https://i.reciba.se/ice-cream.jpg")),
        ingredients_blocks: vec![
            crate::recipe::IngredientsBlock::new(
                None,
                crate::ice_cream::generic_ingredients()
                    .ingredients
                    .into_iter()
                    .chain(vec![crate::recipe::Ingredient::q("Vanilla Essence", "1 tsp")])
                    .collect(),
            ),
        ],
        method: [
            crate::ice_cream::generic_method_start(),
            vec![
                "Add the mascarpone cheese and continue whisking until thoroughly mixed.".to_string(),
            ],
            crate::ice_cream::generic_method_end(),
        ]
        .concat(),
    }
}
