//! `se.reciba.api.recipes.SquashGnocchiGratin`.

use crate::recipe::RecipeDef;
use crate::tag::Tag;
use chrono::NaiveDate;

pub fn recipe() -> RecipeDef {
    RecipeDef {
        object_name: "SquashGnocchiGratin".to_string(),
        name: "Squash and Gnocchi Gratin".to_string(),
        created_at: NaiveDate::from_ymd_opt(2024, 1, 30).unwrap(),
        permalink_override: Some("squash-gnocchi-gratin".to_string()),
        source: None,
        description: None,
        tagline: None,
        notes: vec![
            "Using 200g feta felt like a bit too much so I've noted it down as 100g. Can adjust if that's too little.".to_string(),
            "You can use pumpkin instead of squash.".to_string(),
            "If you have a suitable casserole dish you can bake both the squash and the final mixture in the same dish, to save on washing.".to_string(),
        ],
        tags: vec![
            Tag::Vegetarian,
            Tag::Slow,
            Tag::LowEffort,
            Tag::Stephani,
            Tag::Scales,
        ],
        image: None,
        ingredients_blocks: crate::recipe::IngredientsBlock::simple(vec![
            crate::recipe::Ingredient::qp("Squash", "1", "chopped into 1 1/2cm cubes"),
            crate::recipe::Ingredient::qp("Rosemary", "1 sprig", "needles chopped"),
            crate::recipe::Ingredient::q("Chilli Flakes", "1 tsp"),
            crate::recipe::Ingredient::q("Gnocchi", "500g"),
            crate::recipe::Ingredient::qp("Kale", "125g", "chopped"),
            crate::recipe::Ingredient::q("Creme fraiche", "200ml"),
            crate::recipe::Ingredient::q("Feta", "100g"),
            crate::recipe::Ingredient::q("Panko breadcrumbs ", "100g"),
            crate::recipe::Ingredient::new("Salt"),
            crate::recipe::Ingredient::new("Olive Oil"),
            crate::recipe::Ingredient::new("Black pepper"),
        ]),
        method: vec![
            "Pre-heat the oven to 200C/fan 180C/gas 6.".to_string(),
            "Lay out the squash on a baking tray, cover with olive oil then scatter with rosemary, chilli flakes and salt.".to_string(),
            "Bake for 25 minutes until beginning to soften.".to_string(),
            "Meanwhile, blanche the gnocchi and kale by covering with hot water for 2 minutes and then draining.".to_string(),
            "Tip the gnocchi and kale into a casserole dish and add the creme fraiche.".to_string(),
            "Take the squash out of the oven and mix into the casserole dish.".to_string(),
            "Crumble feta over the mixture.".to_string(),
            "Bake in the oven for 30 minutes until the top is golden brown.".to_string(),
        ],
    }
}
