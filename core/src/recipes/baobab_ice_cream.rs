//! `se.reciba.api.recipes.BaobabIceCream`.

use crate::recipe::RecipeDef;
use crate::tag::Tag;
use chrono::NaiveDate;

pub fn recipe() -> RecipeDef {
    RecipeDef {
        object_name: "BaobabIceCream".to_string(),
        name: "Baobab Ice Cream".to_string(),
        created_at: NaiveDate::from_ymd_opt(2021, 6, 12).unwrap(),
        permalink_override: None,
        source: Some("Kit".to_string()),
        description: Some("A simple yet decadent zesty and tart ice cream recipe, no churn needed.".to_string()),
        tagline: None,
        notes: vec![
            "It got very thick while mixing and was a bit flakey rather than soft so maybe try adding more egg yokes next time.".to_string(),
            "You can substitute baobab for wasabi, marmalade or any ingredient you fancy. Just be careful not to choose anything too watery, or you'll get ice crystals forming. It's also lovely plain.".to_string(),
            crate::ice_cream::generic_notes(),
            "Tasting notes: Flavour is sweet with a sparkling sharpness that was zesty but with a starchy depth.".to_string(),
            "Stephani says: It has a citrus type flavour but with a bready depth, like a very meaty fruit instead of normal citrus fruit texture.".to_string(),
        ],
        tags: vec![Tag::Pudding],
        image: Some(crate::recipe::Image::new("https://i.reciba.se/baobab-ice-cream.jpg")),
        ingredients_blocks: vec![crate::ice_cream::generic_ingredients().prefix_ingredients(vec![crate::recipe::Ingredient::q("Baobab powder", "20g")])],
        method: [
            crate::ice_cream::generic_method_start(),
            vec![
                "Add the mascarpone cheese and baobab powder. Gently stir in the powder to avoid it going everywhere.".to_string(),
                "Continue whisking until thoroughly mixed.".to_string(),
            ],
            crate::ice_cream::generic_method_end(),
        ].concat(),
    }
}
