//! `se.reciba.api.recipes.ChipotleMacNCheese`.

use crate::recipe::RecipeDef;
use crate::tag::Tag;
use chrono::NaiveDate;

pub fn recipe() -> RecipeDef {
    RecipeDef {
        object_name: "ChipotleMacNCheese".to_string(),
        name: "Chipotle Mac 'n' Cheese".to_string(),
        created_at: NaiveDate::from_ymd_opt(2026, 2, 1).unwrap(),
        permalink_override: Some("chipotle-mac-n-cheese".to_string()),
        source: Some("Gousto".to_string()),
        description: None,
        tagline: None,
        notes: Vec::new(),
        tags: vec![
            Tag::AI,
            Tag::Vegetarian,
            Tag::Scales,
            Tag::Stodge,
        ],
        image: None,
        ingredients_blocks: crate::recipe::IngredientsBlock::simple(vec![
            crate::recipe::Ingredient::qp("Yellow pepper", "1", "cut into thin strips"),
            crate::recipe::Ingredient::qp("Red pepper", "1", "cut into thin strips"),
            crate::recipe::Ingredient::qp("Red onion", "1", "sliced"),
            crate::recipe::Ingredient::q("Macaroni", "150g"),
            crate::recipe::Ingredient::q("Vegetable stock cube", "1"),
            crate::recipe::Ingredient::qp("Spring onion", "1", "finely sliced"),
            crate::recipe::Ingredient::q("Smoked paprika", "2 tsp"),
            crate::recipe::Ingredient::q("Chipotle paste", "40g"),
            crate::recipe::Ingredient::q("Tomato paste", "1 tbsp"),
            crate::recipe::Ingredient::qp("Cheddar cheese", "80g", "grated"),
            crate::recipe::Ingredient::q("Panko breadcrumbs", "30g"),
            crate::recipe::Ingredient::q("Creme fraiche", "200g"),
            crate::recipe::Ingredient::new("Salt"),
            crate::recipe::Ingredient::new("Vegetable oil"),
        ]),
        method: vec![
            format!("Preheat the oven to {}.", crate::utils::int_utils::simple_fan_instruction(220)),
            "Heat a wide-based pan (preferably non-stick with a matching lid) with a drizzle of vegetable oil over a medium heat.".to_string(),
            "Once hot, add the peppers and cook, covered, for 4-5 min.".to_string(),
            "Meanwhile, boil a kettle.".to_string(),
            "Add the sliced onion to the pan with a pinch of salt and cook, covered, for a further 4-5 min.".to_string(),
            "While the onion is cooking, add the macaroni to a pot of boiled water with a large pinch of salt and bring to the boil over a high heat.".to_string(),
            "Cook for 7 min or until it's cooked with a slight bite.".to_string(),
            "Once done, drain the macaroni, reserving 250ml of the starchy pasta water.".to_string(),
            "Combine the panko breadcrumbs, smoked paprika, a pinch of salt and a generous drizzle of vegetable oil.".to_string(),
            "Dissolve the vegetable stock cube in the reserved pasta water.".to_string(),
            "Once the onion has softened, add the chipotle paste, tomato paste, drained macaroni, creme fraiche, grated cheddar and the starchy pasta stock.".to_string(),
            "Give everything a good old mix up.".to_string(),
            "Transfer the chipotle mac 'n' cheese to an oven-proof dish. Ideally one small dish per person.".to_string(),
            "Top with the smoky panko breadcrumbs.".to_string(),
            "Put the dish in the oven for 10 min or until bubbling and crispy.".to_string(),
            "Remove the chipotle mac 'n' cheese from the oven and leave to stand until cooled slightly.".to_string(),
            "Garnish with the sliced spring onion.".to_string(),
        ],
    }
}
