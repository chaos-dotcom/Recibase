//! `se.reciba.api.recipes.EggTapas`.

use crate::recipe::RecipeDef;
use crate::tag::Tag;
use chrono::NaiveDate;

pub fn recipe() -> RecipeDef {
    RecipeDef {
        object_name: "EggTapas".to_string(),
        name: "Egg Tapas".to_string(),
        created_at: NaiveDate::from_ymd_opt(2026, 2, 6).unwrap(),
        permalink_override: Some("egg-tapas".to_string()),
        source: Some("Gousto".to_string()),
        description: None,
        tagline: None,
        notes: Vec::new(),
        tags: vec![
            Tag::Vegetarian,
            Tag::Spicy,
        ],
        image: None,
        ingredients_blocks: crate::recipe::IngredientsBlock::simple(vec![
            crate::recipe::Ingredient::qp("Brown onion", "1", "thinly sliced"),
            crate::recipe::Ingredient::q("Eggs", "2"),
            crate::recipe::Ingredient::qp("Red pepper", "1", "thinly sliced"),
            crate::recipe::Ingredient::qp("Garlic clove", "1", "finely chopped"),
            crate::recipe::Ingredient::q("Spring onion", "1"),
            crate::recipe::Ingredient::q("Coriander", "10g"),
            crate::recipe::Ingredient::q("Stock Cube", "1"),
            crate::recipe::Ingredient::q("Mayonnaise", "4 tbsp"),
            crate::recipe::Ingredient::q("Chipotle paste", "5 tsp"),
            crate::recipe::Ingredient::qp("White potatoes", "4", "cut into bite-sized pieces"),
            crate::recipe::Ingredient::q("Cayenne pepper", "1/2 tsp"),
            crate::recipe::Ingredient::qp("Cannellini beans", "1 tin (400g)", "drained and rinsed"),
            crate::recipe::Ingredient::new("Olive oil"),
            crate::recipe::Ingredient::new("Pepper"),
            crate::recipe::Ingredient::new("Salt"),
            crate::recipe::Ingredient::new("Sugar"),
        ]),
        method: vec![
            format!("Preheat the oven to {}.", crate::utils::int_utils::simple_fan_instruction(220)),
            "Add the potatoes to a baking tray, drizzle with olive oil, season with cayenne pepper, salt and pepper.".to_string(),
            "Put the tray in the oven for 25-30 min or until golden and crisp.".to_string(),
            "Heat a wide frying pan with a drizzle of olive oil over a medium-high heat.".to_string(),
            "Add the sliced pepper and onion with a pinch of both salt and sugar. Cook for 7-9 min or until they've softened.".to_string(),
            "Once softened, add the chopped garlic and cook for a further 1 min.".to_string(),
            "Make 200ml of vegetable stock then stir in half of the chipotle paste.".to_string(),
            "Add the stock and beans to the softened pepper and onion and cook for 10-15 min over a low heat or until reduced to a thick stew.".to_string(),
            "Trim, then slice the spring onion finely. Chop the coriander finely, including the stalks.".to_string(),
            "Combine the mayonnaise with the chopped spring onion, remaining chipotle paste, a drizzle of olive oil and half of the chopped coriander. Season with a pinch of both salt and pepper and set aside.".to_string(),
            "Once your potatoes are almost cooked: Heat another pan with a matching lid (large enough to fit 2 eggs) with 1 tbsp olive oil over a medium-low heat.".to_string(),
            "Crack the eggs into a bowl, and once the pan is hot, add the eggs in one go. Cover with a lid and cook for 2-3 min or until done to your liking, then remove from the heat and season with salt and pepper.".to_string(),
            "Stir the remaining chopped coriander through the pepper and bean stew.".to_string(),
            "Serve the eggs over the stew with the crispy potatoes and smoky mayo to the side.".to_string(),
        ],
    }
}
