//! `se.reciba.api.recipes.TurmericGingerIceCream`.

use crate::recipe::RecipeDef;
use crate::tag::Tag;
use chrono::NaiveDate;

pub fn recipe() -> RecipeDef {
    RecipeDef {
        object_name: "TurmericGingerIceCream".to_string(),
        name: "Turmeric & Ginger Ice Cream".to_string(),
        created_at: NaiveDate::from_ymd_opt(2022, 4, 9).unwrap(),
        permalink_override: None,
        source: Some("https://www.sugarlovespices.com/turmeric-ginger-honey-no-churn-ice-cream/".to_string()),
        description: Some("A rich, spicy and earthy ice cream. No churn needed.".to_string()),
        tagline: None,
        notes: vec![
            "The spice mix isn't set in stone. Taste test the mixture before freezing and adjust according to preference.".to_string(),
            "You can use up the egg whites by making <a href=\"https://www.bbcgoodfood.com/recipes/easy-chocolate-mousse\" rel=\"nofollow\">chocolate mousse</a>.".to_string(),
            "For reference the picture is actually marmalade ice cream. It seems I never took a picture when I made this, though I remember it being delicious.".to_string(),
        ],
        tags: vec![Tag::Pudding],
        image: Some(crate::recipe::Image::new("https://i.reciba.se/ice-cream.jpg")),
        ingredients_blocks: crate::recipe::IngredientsBlock::simple(vec![
            crate::recipe::Ingredient::q("Mascarpone", "460g"),
            crate::recipe::Ingredient::q("Eggs", "4"),
            crate::recipe::Ingredient::q("Icing Sugar", "120g"),
            crate::recipe::Ingredient::q("Honey", "1-2 tbsp"),
            crate::recipe::Ingredient::q("Turmeric", "1 1/2 tsp"),
            crate::recipe::Ingredient::qp("Fresh Ginger", "1 tsp", "minced"),
            crate::recipe::Ingredient::q("Vanilla Extract", "1/2 tsp"),
            crate::recipe::Ingredient::qp("Cardamon", "1/2 tsp", "remove the pods and grind the seeds"),
            crate::recipe::Ingredient::q("Cinnamon", "1 tsp"),
            crate::recipe::Ingredient::q("Chilli Powder", "a pinch"),
        ]),
        method: [
            crate::ice_cream::generic_method_start(),
            vec![
                "Add the mascarpone cheese then continue whisking.".to_string(),
                "Gently stir in all the remaining ingredients, to avoid aerosolising the powdered spices.".to_string(),
                "Whisk until thoroughly mixed.".to_string(),
            ],
            crate::ice_cream::generic_method_end(),
        ]
        .concat(),
    }
}
