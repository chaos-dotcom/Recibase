//! `se.reciba.api.recipes.BakedRigatoniAubergine`.

use crate::recipe::RecipeDef;
use crate::tag::Tag;
use chrono::NaiveDate;

pub fn recipe() -> RecipeDef {
    RecipeDef {
        object_name: "BakedRigatoniAubergine".to_string(),
        name: "Baked Rigatoni with Aubergine".to_string(),
        created_at: NaiveDate::from_ymd_opt(2020, 4, 24).unwrap(),
        permalink_override: None,
        source: Some("Vegetarian Cookery Bible (2012: Reader's Digest)".to_string()),
        description: None,
        tagline: None,
        notes: vec!
            ["Traditional parmesan is not vegetarian".to_string()],
        tags: vec![
            Tag::Vegetarian,
            Tag::Slow,
            Tag::Scales,
        ],
        image: None,
        ingredients_blocks: crate::recipe::IngredientsBlock::simple(vec![
            crate::recipe::Ingredient::qp("Onion", "1", "chopped"),
            crate::recipe::Ingredient::qp("Garlic", "2-3 cloves", "finely chopped or crushed"),
            crate::recipe::Ingredient::new("Red wine"),
            crate::recipe::Ingredient::q("Chopped tomatoes", "2 400g tins"),
            crate::recipe::Ingredient::qp("Sun-dried tomatoes", "5", "drained and chopped."),
            crate::recipe::Ingredient::qp("Aubergine", "1", "cut into 1cm cubes"),
            crate::recipe::Ingredient::qpn("Oregano", "2 tbsp", "chopped", "Fresh or dried"),
            crate::recipe::Ingredient::opt("Rigatoni", Some("225g"), None, Some("Or penne or other chunky pasta tube")),
            crate::recipe::Ingredient::q("Breadcrumbs", "30g"),
            crate::recipe::Ingredient::opt("Parmesan", Some("30g"), None, Some("Or a ball of mozzarella")),
            crate::recipe::Ingredient::new("Salt"),
            crate::recipe::Ingredient::new("Pepper"),
        ]),
        method: vec![
            format!("Preheat the oven to {}.", crate::utils::int_utils::celsius(200)),
            "Heat oil or butter in a saucepan. Add the onion and garlic and cook until soft.".to_string(),
            "Add the wine, followed by the tins of chopped tomatoes, the sun-dried tomatoes, aubergine, oregano, salt and pepper.".to_string(),
            "Bring to the boil, cover and simmer for 15-20 minutes.".to_string(),
            "Cook the pasta until al dente. Drain well.".to_string(),
            "Tip the pasta and sauce into an suitable ovenproof dish. Mix together thoroughly.".to_string(),
            "Cover with the breadcrumbs and parmesan (or mozzarella).".to_string(),
            "Bake for 15-20 minutes until bubbling and the top is golden brown.".to_string(),
        ],
    }
}
