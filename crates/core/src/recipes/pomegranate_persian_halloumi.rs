//! `se.reciba.api.recipes.PomegranatePersianHalloumi`.

use crate::recipe::RecipeDef;
use crate::tag::Tag;
use chrono::NaiveDate;

pub fn recipe() -> RecipeDef {
    RecipeDef {
        object_name: "PomegranatePersianHalloumi".to_string(),
        name: "Pomegranate Persian Halloumi".to_string(),
        created_at: NaiveDate::from_ymd_opt(2020, 4, 24).unwrap(),
        permalink_override: None,
        source: Some("Gousto".to_string()),
        description: Some("Halloumi coated in ras el hanout, pan-fried until golden and served with pomegranate couscous and caramelised onions.".to_string()),
        tagline: None,
        notes: Vec::new(),
        tags: vec![
            Tag::LowEffort,
            Tag::Quick,
            Tag::HotWeather,
            Tag::Vegetarian,
            Tag::Scales,
            Tag::StephaniIsh,
        ],
        image: None,
        ingredients_blocks: crate::recipe::IngredientsBlock::simple(vec![
            crate::recipe::Ingredient::qp("Halloumi", "200g", "cut into thick slices"),
            crate::recipe::Ingredient::q("Couscous", "125g"),
            crate::recipe::Ingredient::q("Boiled water", "170ml"),
            crate::recipe::Ingredient::qp("Red onion", "2", "finely sliced"),
            crate::recipe::Ingredient::q("Sultanas", "30g"),
            crate::recipe::Ingredient::q("Pomegranate molasses", "15g"),
            crate::recipe::Ingredient::opt("Pomegranate seeds", Some("10g"), None, Some("optional")),
            crate::recipe::Ingredient::q("Ras el hanout", "1 tbsp"),
            crate::recipe::Ingredient::qp("Mint", "10g", "leaves stripped and roughly chopped"),
            crate::recipe::Ingredient::q("Natural yoghurt", "80g"),
            crate::recipe::Ingredient::new("Olive oil"),
            crate::recipe::Ingredient::new("Salt"),
            crate::recipe::Ingredient::new("Pepper"),
        ]),
        method: vec![
            "Boil a kettle.".to_string(),
            "Add the couscous and sultanas to a bowl, pour over the boiled water, cover and set aside.".to_string(),
            "Combine half the ras el hanout in a bowl with a generous grind of black pepper.".to_string(),
            "Add the sliced halloumi and mix to coat evenly.".to_string(),
            "Heat a large, wide-based pan (preferably non-stick) with 1 tbsp olive oil over a medium-high heat.".to_string(),
            "Once hot, add the sliced red onion and cook for 6-7 min or until soft and lightly caramelised.".to_string(),
            "Fluff the couscous with a fork.".to_string(),
            "Add the pomegranate molasses, caramelised onions (reserve the pan!), 1 tbsp olive oil and the remaining ras el hanout and mix well.".to_string(),
            "Combine the chopped mint in a small bowl with the natural yoghurt.".to_string(),
            "Return the reserved pan to a medium-high heat.".to_string(),
            "Once hot, add the spiced halloumi and cook for 1-2 min on each side or until golden.".to_string(),
            "Serve the halloumi over the pomegranate couscous with a garnish of pomegranate seeds.".to_string(),
        ],
    }
}
