//! `se.reciba.api.recipes.ScrambledEggs`.

use crate::recipe::RecipeDef;
use crate::tag::Tag;
use chrono::NaiveDate;

pub fn recipe() -> RecipeDef {
    RecipeDef {
        object_name: "ScrambledEggs".to_string(),
        name: "Scrambled Eggs".to_string(),
        created_at: NaiveDate::from_ymd_opt(2020, 4, 24).unwrap(),
        permalink_override: None,
        source: None,
        description: Some("Soft, buttery scrambled eggs.".to_string()),
        tagline: None,
        notes: vec![
            "Variants:\nPaprika - add a generous amount of (smoked) paprika along with the salt/pepper.\nSriracha - mix in a dash of sriracha for a spicier dish. Can be combined with the paprika.\nStilton - crumble and melt blue stilton into the butter.\nSpring onions - lightly fry a sliced spring onion in the butter. Also works with garlic.".to_string(),
        ],
        tags: vec![
            Tag::Lunch,
            Tag::Vegetarian,
            Tag::LowEffort,
            Tag::Quick,
        ],
        image: None,
        ingredients_blocks: crate::recipe::IngredientsBlock::simple(vec![
            crate::recipe::Ingredient::opt("Butter", None, None, Some("Use a decent amount")),
            crate::recipe::Ingredient::q("Eggs", "2"),
            crate::recipe::Ingredient::new("Milk"),
            crate::recipe::Ingredient::new("Salt"),
            crate::recipe::Ingredient::new("Black pepper"),
        ]),
        method: vec![
            "Melt the butter in a (very) small saucepan.".to_string(),
            "When bubbling, crack in the eggs and mix thoroughly.".to_string(),
            "Add a dash of milk, along with the salt and pepper.".to_string(),
            "Cook the eggs over a medium heat, stirring to prevent sticking.".to_string(),
            "When the egg mixture reaches the desired consistency (ideally soft enough to pour), remove from heat.".to_string(),
            "Serve on two slices of toast.".to_string(),
        ],
    }
}
