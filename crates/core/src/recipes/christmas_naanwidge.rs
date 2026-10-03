//! `se.reciba.api.recipes.ChristmasNaanwidge`.

use crate::recipe::RecipeDef;
use crate::tag::Tag;
use chrono::NaiveDate;

pub fn recipe() -> RecipeDef {
    RecipeDef {
        object_name: "ChristmasNaanwidge".to_string(),
        name: "Christmas Naanwidge".to_string(),
        created_at: NaiveDate::from_ymd_opt(2026, 2, 1).unwrap(),
        permalink_override: Some("christmas-naanwidge".to_string()),
        source: Some("Gousto".to_string()),
        description: None,
        tagline: None,
        notes: Vec::new(),
        tags: vec![
            Tag::AI,
            Tag::Christmas,
            Tag::Stodge,
            Tag::Vegetarian,
            Tag::Spicy,
        ],
        image: None,
        ingredients_blocks: crate::recipe::IngredientsBlock::simple(vec![
            crate::recipe::Ingredient::q("Coriander", "10g"),
            crate::recipe::Ingredient::q("Ground turmeric", "1 tsp"),
            crate::recipe::Ingredient::q("Dried chilli flakes", "1/2 tsp"),
            crate::recipe::Ingredient::q("Curry powder", "1 tbsp"),
            crate::recipe::Ingredient::qp("Carrot", "1", "grated"),
            crate::recipe::Ingredient::qp("Parsnip", "1", "peeled into ribbons"),
            crate::recipe::Ingredient::qp("Potatoes", "3", "peeled and diced into small cubes"),
            crate::recipe::Ingredient::qp("Paneer", "200g", "chopped into cubes"),
            crate::recipe::Ingredient::q("Greek-style yoghurt", "100g"),
            crate::recipe::Ingredient::q("Smoked paprika", "1 tsp"),
            crate::recipe::Ingredient::q("Cranberry sauce", "40g"),
            crate::recipe::Ingredient::q("Tamarind paste", "15g"),
            crate::recipe::Ingredient::q("Plain naans", "2"),
            crate::recipe::Ingredient::new("Salt"),
            crate::recipe::Ingredient::new("Pepper"),
            crate::recipe::Ingredient::new("Oil"),
        ]),
        method: vec![
            format!("Preheat the oven to {}.", crate::utils::int_utils::simple_fan_instruction(220)),
            "Add the diced potatoes to one side of a baking tray and sprinkle with half the curry powder, season with a pinch of salt and pepper and drizzle with vegetable oil.".to_string(),
            "Give everything a good mix up and put the tray in the oven for an initial 6-8 min.".to_string(),
            "Stir together the ground turmeric, smoked paprika and chilli flakes in a large bowl with the remaining curry powder, half the Greek-style yoghurt and a pinch of salt and pepper.".to_string(),
            "Add the chopped paneer to the tandoori marinade and give everything a good mix up.".to_string(),
            "After the potatoes have had their initial 6-8 min, add the tandoori paneer to the other side of the baking tray.".to_string(),
            "Return the tray to the oven and cook for a further 10-12 min or until golden and soft.".to_string(),
            "Combine the cranberry sauce, tamarind paste and grated carrot in a bowl to make tamarind & cranberry slaw.".to_string(),
            "Chop the coriander finely, including the stalks (saving a few whole leaves to garnish).".to_string(),
            "Once the golden tandoori paneer is cooked, transfer it to a clean chopping board and reserve the tray.".to_string(),
            "Add the parsnip ribbons to the reserved tray and return it to the oven for 5 min further or until both the potatoes and parsnips are golden and crispy.".to_string(),
            "Put the plain naans in the oven for 3 min or until warmed through.".to_string(),
            "Spread the remaining Greek-style yoghurt over naan, then top with the tamarind & cranberry slaw and the golden tandoori paneer.".to_string(),
            "Sprinkle over the chopped coriander and top with the remaining naan.".to_string(),
            "Cut each naanwich into 4 and garnish with the reserved coriander leaves and crispy parsnip ribbons.".to_string(),
            "Serve with the spiced roast potatoes to the side.".to_string(),
        ],
    }
}
