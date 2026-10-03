//! `se.reciba.api.recipes.BasaPathia`.

use crate::recipe::RecipeDef;
use crate::tag::Tag;
use chrono::NaiveDate;

pub fn recipe() -> RecipeDef {
    RecipeDef {
        object_name: "BasaPathia".to_string(),
        name: "Basa Pathia".to_string(),
        created_at: NaiveDate::from_ymd_opt(2026, 2, 1).unwrap(),
        permalink_override: Some("basa-pathia".to_string()),
        source: Some("Gousto".to_string()),
        description: None,
        tagline: None,
        notes: Vec::new(),
        tags: vec![
            Tag::AI,
            Tag::Pescatarian,
            Tag::Spicy,
        ],
        image: None,
        ingredients_blocks: crate::recipe::IngredientsBlock::simple(vec![
            crate::recipe::Ingredient::qp("Brown onion", "1", "finely chopped"),
            crate::recipe::Ingredient::q("Cumin seeds", "1 tsp"),
            crate::recipe::Ingredient::q("Ground cumin", "1 tsp"),
            crate::recipe::Ingredient::q("Cayenne pepper", "1/2 tsp"),
            crate::recipe::Ingredient::q("Curry powder", "1/2 tbsp"),
            crate::recipe::Ingredient::q("Coriander", "5g"),
            crate::recipe::Ingredient::q("Basa fillets", "200g"),
            crate::recipe::Ingredient::q("Tamarind paste", "15g"),
            crate::recipe::Ingredient::q("Ground turmeric", "1/2 tsp"),
            crate::recipe::Ingredient::q("Tomato paste", "32g"),
            crate::recipe::Ingredient::q("Basmati rice", "100g"),
            crate::recipe::Ingredient::qp("Garlic Clove", "2", "finely chopped"),
            crate::recipe::Ingredient::qp("Fresh root ginger", "15g", "finely chopped"),
            crate::recipe::Ingredient::q("Naan", "2"),
            crate::recipe::Ingredient::new("Salt"),
            crate::recipe::Ingredient::new("Sugar"),
            crate::recipe::Ingredient::new("Oil"),
        ]),
        method: vec![
            format!("Preheat the oven to {}.", crate::utils::int_utils::simple_fan_instruction(200)),
            "Heat a wide-based pan (preferably non-stick) with a drizzle of vegetable oil over a medium heat.".to_string(),
            "Once hot, add the chopped onion with a generous pinch of salt and cook for 6-8 minutes or until softened.".to_string(),
            "Meanwhile, heat a pot with a matching lid with a generous drizzle of vegetable oil over a medium heat.".to_string(),
            "Once hot, add the cumin seeds and cook for 1-2 minutes or until sizzling.".to_string(),
            "Once the cumin seeds start to sizzle, carefully add the basmati rice and stir it all together.".to_string(),
            "Add the ground turmeric and 250ml cold water with a pinch of salt to the pot and bring to the boil over a high heat.".to_string(),
            "Once boiling, reduce the heat to very low and cook, covered, for 10-12 minutes or until the water has absorbed and the rice is cooked.".to_string(),
            "Once cooked, remove from the heat and keep covered until serving.".to_string(),
            "Add the ginger and garlic to the pan and cook for 1-2 minutes further or until fragrant.".to_string(),
            "Once fragrant, add the ground cumin, curry powder and cayenne pepper with the tomato paste and stir it all together.".to_string(),
            "Add the tamarind paste with 150ml boiled water and 2 tsp sugar and cook for 4-5 minutes or until the sauce is beginning to thicken.".to_string(),
            "Add the chopped basa fillets and cook for 3-4 more minutes or until the fish is cooked through.".to_string(),
            "Heat up the naans per packet instruction.".to_string(),
            "Garnish with coriander.".to_string(),
        ],
    }
}
