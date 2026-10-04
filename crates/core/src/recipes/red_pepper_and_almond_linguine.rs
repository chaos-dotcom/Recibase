//! `se.reciba.api.recipes.RedPepperAndAlmondLinguine`.
//! chaos-tag: casa-chaos

use crate::recipe::RecipeDef;
use crate::tag::Tag;
use chrono::NaiveDate;

pub fn recipe() -> RecipeDef {
    RecipeDef {
        object_name: "RedPepperAndAlmondLinguine".to_string(),
        name: "Red Pepper & Almond Linguine".to_string(),
        created_at: NaiveDate::from_ymd_opt(2026, 10, 3).unwrap(),
        permalink_override: None,
        source: None,
        description: None,
        tagline: None,
        notes: vec![
            "Serves 2".to_string(),
            "Prepare 15 minutes; cook 20 minutes.".to_string(),
            "Good Health: low in sat fat / source of protein.".to_string(),
            "Cook's tip: keep basil fresh for longer by treating it like a bunch of flowers. For best results, trim the ends of the stalks and keep in a glass of water at room temperature.".to_string(),
            "Per serving: 2178kJ/519kcals, 17.8g fat (4.8g saturated), 67.1g carbs (13.2g sugars), 9.9g fibre, 17.5g protein, 0.9g salt - 2 of your 5 a day.".to_string(),
        ],
        tags: vec![
            Tag::Quick,
            Tag::Vegetarian,
            Tag::LowEffort,
            Tag::Scales,
        ],
        image: None,
        ingredients_blocks: crate::recipe::IngredientsBlock::simple(vec![
            crate::recipe::Ingredient::qp("Fresh medium tomatoes", "400g", "roughly chopped"),
            crate::recipe::Ingredient::q("Toasted flaked almonds", "20g"),
            crate::recipe::Ingredient::qp("Basil", "1/2 x 25g pack", "leaves only"),
            crate::recipe::Ingredient::q("De Cecco Linguine", "150g"),
            crate::recipe::Ingredient::q("Sherry vinegar", "2 tsp"),
            crate::recipe::Ingredient::q("Olive oil", "4 tsp"),
            crate::recipe::Ingredient::qp("Manchego", "25g", "finely grated"),
            crate::recipe::Ingredient::qp("Red peppers", "2", "deseeded and cut into small chunks"),
            crate::recipe::Ingredient::qp("Garlic", "2 cloves", "chopped"),
            crate::recipe::Ingredient::q("Cooks' Ingredients Crushed Red Chillies", "1/2 tsp"),
        ]),
        method: vec![
            "Bring a saucepan of salted water to the boil and cook the pasta for 10-12 minutes until tender. Meanwhile, heat 2 tsp oil in a frying pan and fry the peppers, turning frequently, for 12 minutes until softened and lightly browned.".to_string(),
            "Tip half of the peppers onto a plate. Leave the rest in the frying pan and add the garlic, chillies, half of the tomatoes, half of the almonds and a pinch of salt. Cook for a further 4-5 minutes until the mixture is softened. Reserve a few small basil leaves and tear the rest into pieces. Add to the sauce with the vinegar and remaining 2 tsp olive oil. Blend until smooth.".to_string(),
            "Thoroughly drain the pasta and return to the pan with the sauce and the reserved pepper and tomatoes. Heat for 1-2 minutes. Serve scattered with the manchego, almonds and reserved basil leaves.".to_string(),
        ],
    }
}
