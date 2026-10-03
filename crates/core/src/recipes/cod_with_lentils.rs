//! `se.reciba.api.recipes.CodWithLentils`.

use crate::recipe::RecipeDef;
use crate::tag::Tag;
use chrono::NaiveDate;

pub fn recipe() -> RecipeDef {
    RecipeDef {
        object_name: "CodWithLentils".to_string(),
        name: "Cod with Lentils".to_string(),
        created_at: NaiveDate::from_ymd_opt(2023, 3, 17).unwrap(),
        permalink_override: None,
        source: Some("Sand Dollar Café, Aberdeen".to_string()),
        description: None,
        tagline: None,
        notes: vec!["The green lentils are an essential part of the dish, so don't substitute them for red lentils.".to_string()],
        tags: vec![
            Tag::Pescatarian,
            Tag::LowEffort,
            Tag::Quick,
        ],
        image: None,
        ingredients_blocks: crate::recipe::IngredientsBlock::simple(vec![
            crate::recipe::Ingredient::qp("Red Onion", "1", "chopped"),
            crate::recipe::Ingredient::q("Cod Fillets", "2"),
            crate::recipe::Ingredient::q("Green Lentils", "2 400g tin"),
            crate::recipe::Ingredient::new("Butter"),
            crate::recipe::Ingredient::new("Salt"),
            crate::recipe::Ingredient::new("Pepper"),
            crate::recipe::Ingredient::new("Lemon Juice"),
        ]),
        method: vec![
            "Salt and pepper the cod. Wrap in tin foil then cook per packet instructions.".to_string(),
            "Meanwhile, drain the tins of green lentils into a sieve. Rinse and set aside.".to_string(),
            "Heat a knob of butter in a saucepan.".to_string(),
            "Add the onion and cook over a medium heat until soft.".to_string(),
            "Tip in the green lentils and gently heat up. Stir the lentils gently as it's important the lentils don't turn to mush.".to_string(),
            "Season with salt, pepper and a dash of lemon juice. Add more butter if needed.".to_string(),
            "Serve the lentil and onions with the cod on top.".to_string(),
        ],
    }
}
