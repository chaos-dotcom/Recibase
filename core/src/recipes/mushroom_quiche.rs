//! `se.reciba.api.recipes.MushroomQuiche`.

use crate::recipe::RecipeDef;
use crate::tag::Tag;
use chrono::NaiveDate;

pub fn recipe() -> RecipeDef {
    RecipeDef {
        object_name: "MushroomQuiche".to_string(),
        name: "Mushroom Quiche".to_string(),
        created_at: NaiveDate::from_ymd_opt(2020, 4, 24).unwrap(),
        permalink_override: None,
        source: None,
        description: None,
        tagline: None,
        notes: vec![
            "Take the pastry out of the fridge 40 mins before use!".to_string(),
        ],
        tags: vec![Tag::Stodge, Tag::Slow, Tag::Vegetarian],
        image: None,
        ingredients_blocks: crate::recipe::IngredientsBlock::simple(vec![
            crate::recipe::Ingredient::qp("Chestnut mushrooms", "250g", "sliced"),
            crate::recipe::Ingredient::opt("Dried mushrooms", Some("handful"), None, Some("Porcini, shiitake, wild, etc")),
            crate::recipe::Ingredient::qp("Red onion", "1", "sliced"),
            crate::recipe::Ingredient::opt("Parmesan", None, Some("Grated"), None),
            crate::recipe::Ingredient::q("Mascarpone", "2 tbsp"),
            crate::recipe::Ingredient::q("Eggs", "2"),
            crate::recipe::Ingredient::new("Salt"),
            crate::recipe::Ingredient::new("Black pepper"),
            crate::recipe::Ingredient::q("Wholegrain Mustard", "2 tsp"),
            crate::recipe::Ingredient::q("Shortcrust pastry sheet", "230g"),
        ]),
        method: vec![
            "Place the dried mushrooms in a small amount of boiling water to rehydrate.".to_string(),
            "Add some oil to a wide pan and soften the onion.".to_string(),
            "Add the mushrooms to the pan and cook until soft.".to_string(),
            "Add the mascarpone, eggs, mustard, salt and black pepper to a mixing bowl and beat together.".to_string(),
            "Add the mushroom mixture and dried mushrooms to the mixing bowl, including the water used to rehydrate the mushrooms if desired.".to_string(),
            "Place the pastry in a 9 inch flan dish, rerolling it if necessary.".to_string(),
            "Pour the mixture into the dish and top with the grated parmesan.".to_string(),
            "Bake in the oven at 200C/gas 6 for 20 minutes.".to_string(),
        ],
    }
}
