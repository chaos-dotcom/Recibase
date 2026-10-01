//! `se.reciba.api.recipes.GoanKingPrawnBalchao`.

use crate::recipe::RecipeDef;
use crate::tag::Tag;
use chrono::NaiveDate;

pub fn recipe() -> RecipeDef {
    RecipeDef {
        object_name: "GoanKingPrawnBalchao".to_string(),
        name: "Goan King Prawn Balchão".to_string(),
        created_at: NaiveDate::from_ymd_opt(2026, 2, 6).unwrap(),
        permalink_override: Some("goan-king-prawn-balchao".to_string()),
        source: Some("Gousto".to_string()),
        description: None,
        tagline: None,
        notes: Vec::new(),
        tags: vec![
            Tag::Pescatarian,
            Tag::LowEffort,
            Tag::Quick,
            Tag::Spicy,
            Tag::Scales,
        ],
        image: None,
        ingredients_blocks: crate::recipe::IngredientsBlock::simple(vec![
            crate::recipe::Ingredient::qp("Red onions", "2", "finely diced"),
            crate::recipe::Ingredient::qp("Garlic", "3 cloves", "roughly chopped"),
            crate::recipe::Ingredient::qp("Fresh root ginger", "15g", "peeled and roughly chopped"),
            crate::recipe::Ingredient::q("Ground coriander", "1 tsp"),
            crate::recipe::Ingredient::q("Ground cumin", "1 tsp"),
            crate::recipe::Ingredient::q("Dried chilli flakes", "1/2 tsp"),
            crate::recipe::Ingredient::q("Cayenne pepper", "1/2 tsp"),
            crate::recipe::Ingredient::q("Tomato paste", "32g"),
            crate::recipe::Ingredient::q("King prawns", "171g"),
            crate::recipe::Ingredient::q("Tamarind paste", "15g"),
            crate::recipe::Ingredient::q("Cider vinegar", "30ml"),
            crate::recipe::Ingredient::q("Sugar", "1 tsp"),
            crate::recipe::Ingredient::q("Baby leaf spinach", "80g"),
            crate::recipe::Ingredient::q("Basmati rice", "130g"),
            crate::recipe::Ingredient::new("Salt"),
            crate::recipe::Ingredient::new("Vegetable oil"),
            crate::recipe::Ingredient::q("Water", "300ml"),
        ]),
        method: vec![
            "Fry the onion in a wide pan with a generous drizzle of vegetable oil over a medium heat for 10-12 min or until softened.".to_string(),
            "Rinse the rice and cook in the water for 10-12 minutes.".to_string(),
            "Add the chopped garlic, ginger, ground coriander, ground cumin, chilli flakes, cayenne pepper and a generous pinch of salt to a pestle & mortar and grind to a smooth paste. Add 2 tbsp vegetable oil and give everything a good mix up.".to_string(),
            "Once the onion has softened, add the Balchao spice paste and tomato paste to the pan and cook for 4-5 min or until fragrant.".to_string(),
            "Add the king prawns, tamarind paste, cider vinegar and 1 tsp sugar and cook for 2-3 min or until the prawns are almost cooked through.".to_string(),
            "Add the baby leaf spinach with 60ml cold water and cook for a further 2-3 min or until the spinach has just wilted and the sauce is thickened to a curry-like consistency.".to_string(),
            "Serve the Goan prawn Balchao curry over the basmati rice.".to_string(),
        ],
    }
}
