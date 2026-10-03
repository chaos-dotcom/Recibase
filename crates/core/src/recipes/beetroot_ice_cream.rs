//! `se.reciba.api.recipes.BeetrootIceCream`.

use crate::recipe::RecipeDef;
use crate::tag::Tag;
use chrono::NaiveDate;

pub fn recipe() -> RecipeDef {
    RecipeDef {
        object_name: "BeetrootIceCream".to_string(),
        name: "Beetroot Ice Cream".to_string(),
        created_at: NaiveDate::from_ymd_opt(2021, 6, 12).unwrap(),
        permalink_override: None,
        source: Some("Kit".to_string()),
        description: Some("A simple yet decadent earthy ice cream recipe, no churn needed.".to_string()),
        tagline: None,
        notes: vec![
            "Consider adding a dash of lime to balance the beetroot flavour.".to_string(),
            "You can substitute beetroot for blackberries or any ingredient you fancy. Just be careful not to choose anything too watery, or you'll get ice crystals forming. It's also lovely plain.".to_string(),
            crate::ice_cream::generic_notes(),
        ],
        tags: vec![Tag::Pudding],
        image: Some(crate::recipe::Image::new("https://i.reciba.se/beetroot-ice-cream.jpg")),
        ingredients_blocks: vec![crate::ice_cream::generic_ingredients().prefix_ingredients(vec![crate::recipe::Ingredient::q("Beetroot Powder", "25g")])],
        method: [
            crate::ice_cream::generic_method_start(),
            vec![
                "Add the mascarpone cheese and beetroot powder. Gently stir in the powder to avoid it going everywhere.".to_string(),
                "Continue whisking until thoroughly mixed.".to_string(),
            ],
            crate::ice_cream::generic_method_end(),
        ].concat(),
    }
}
