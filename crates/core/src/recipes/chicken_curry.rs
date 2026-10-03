//! `se.reciba.api.recipes.ChickenCurry`.

use crate::recipe::RecipeDef;
use crate::tag::Tag;
use chrono::NaiveDate;

pub fn recipe() -> RecipeDef {
    RecipeDef {
        object_name: "ChickenCurry".to_string(),
        name: "Chicken Curry (WIP)".to_string(),
        created_at: NaiveDate::from_ymd_opt(2023, 1, 8).unwrap(),
        permalink_override: Some("chicken-curry".to_string()),
        source: Some("H + S".to_string()),
        description: None,
        tagline: None,
        notes: vec!["\nThis is H's spice blend. Alternative spice profile by S:\nLots of cumin, smoked paprika (sweet) and cinnamon. A medium amount of turmeric and nutmeg. A couple tsp cloves. No sugar. Only salt after cooking.\nI might re-jig the method to simplify but I want to make it a few more times.  \n".to_string()],
        tags: vec![
            Tag::Slow,
            Tag::Freezes,
            Tag::Scales,
            Tag::Spicy,
            Tag::Stephani,
        ],
        image: None,
        ingredients_blocks: crate::recipe::IngredientsBlock::simple(vec![
            crate::recipe::Ingredient::qp("Chicken Breasts", "4", "chopped into 3cm pieces"),
            crate::recipe::Ingredient::qp("Large Onions", "2", "chopped"),
            crate::recipe::Ingredient::qp("Garlic", "6 cloves", "diced"),
            crate::recipe::Ingredient::qp("Green Beans", "220g", "topped, tailed and halved"),
            crate::recipe::Ingredient::qp("Chestnut Mushrooms", "485g", "chopped"),
            crate::recipe::Ingredient::opt("Double Cream", Some("75ml"), None, Some("add more if desired")),
            crate::recipe::Ingredient::q("Sweetcorn", "1 large tin"),
            crate::recipe::Ingredient::q("Lemongrass", "2 stems"),
            crate::recipe::Ingredient::new("Kaffir lime leaves"),
            crate::recipe::Ingredient::new("Salt"),
            crate::recipe::Ingredient::new("Sugar"),
            crate::recipe::Ingredient::new("Smoked Paprika"),
            crate::recipe::Ingredient::new("Cumin"),
            crate::recipe::Ingredient::new("Cloves"),
            crate::recipe::Ingredient::new("Nutmeg"),
            crate::recipe::Ingredient::new("Turmeric"),
            crate::recipe::Ingredient::opt("Red Peppers", Some("2"), None, Some("Optional")),
            crate::recipe::Ingredient::opt("Chillies", None, None, Some("Optional")),
            crate::recipe::Ingredient::opt("Naan Bread", None, None, Some("Optional")),
        ]),
        method: vec![
            "Heat the wok over a hot flame.".to_string(),
            "Fry the onions. Add the chicken halfway through then throw in the garlic and (optional) peppers.".to_string(),
            "Add the smoked paprika, cumin, nutmeg, cloves and turmeric. Primarily paprika and cumin. Cook the spices for a moment.".to_string(),
            "Transfer to a large pan.".to_string(),
            "Add the cream, sweetcorn (including brine), sugar and salt to the large pan. Add a little water, sparingly, if it looks dry.".to_string(),
            "Slice the lemongrass leaves lengthways, excluding the root, so they stay together. Crush the lemongrass with the side of knife to release the flavour.".to_string(),
            "Add the kaffir lime leaves and lemongrass to the large pan, then bring to a simmer.".to_string(),
            "Meanwhile, fry the green beans and mushrooms in the wok.".to_string(),
            "Transfer the green beans and mushrooms to the large pan.".to_string(),
            "Continue simmering to reduce most of the liquid.".to_string(),
            "Heat the naans and serve alongside.".to_string(),
            "Do cute victory dance with butt wiggle.".to_string(),
        ],
    }
}
