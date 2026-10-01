//! `se.reciba.api.recipes.Spanakopita`.

use crate::recipe::RecipeDef;
use crate::tag::Tag;
use chrono::NaiveDate;

pub fn recipe() -> RecipeDef {
    RecipeDef {
        object_name: "Spanakopita".to_string(),
        name: "Spanakopita".to_string(),
        created_at: NaiveDate::from_ymd_opt(2024, 5, 12).unwrap(),
        permalink_override: None,
        source: Some("Minna".to_string()),
        description: None,
        tagline: None,
        notes: vec![
            "The frozen spinach quantity is approximate as I measured an old bag that had a lot of ice in it.".to_string(),
            "Iceland frozen spinach is better than other supermarkets' as they include whole leaves, while others are completely shredded.".to_string(),
            "I used approximately 1/2tsp fennel seeds the first time and it wasn't enough, so I'm hoping 1tsp is enough.".to_string(),
            "I didn't make this with onion or garlic the first time.".to_string(),
        ],
        tags: vec![Tag::Vegetarian, Tag::Slow],
        image: None,
        ingredients_blocks: crate::recipe::IngredientsBlock::simple(vec![
            crate::recipe::Ingredient::q("Frozen spinach", "300g"),
            crate::recipe::Ingredient::qp("Fresh Parsley", "30g", "chopped"),
            crate::recipe::Ingredient::qp("Onion", "1", "diced"),
            crate::recipe::Ingredient::qp("Garlic", "2 cloves", "diced"),
            crate::recipe::Ingredient::qp("Feta", "200g", "diced"),
            crate::recipe::Ingredient::q("Eggs", "2"),
            crate::recipe::Ingredient::q("Fillo Pastry", "125g"),
            crate::recipe::Ingredient::q("Fennel Seeds", "1 tsp"),
            crate::recipe::Ingredient::new("Salt"),
            crate::recipe::Ingredient::new("Pepper"),
            crate::recipe::Ingredient::new("Olive Oil"),
        ]),
        method: vec![
            format!("Preheat the oven to {}.", crate::utils::int_utils::celsius(160)),
            "Defrost the spinach and squeeze out as much moisture as you can through a sieve.".to_string(),
            "Mix every ingredient except the oil and pastry in a bowl.".to_string(),
            "Brush a 9 1/2″ X 13″ baking dish with olive oil".to_string(),
            "Line the baking dish with 2/3rds of the fillo pastry sheets. Add them two at a time then brush the layer with olive oil.".to_string(),
            "Evenly spread the spinach and feta filling over the fillo crust.".to_string(),
            "Layer the rest of the fillo pasty sheets on top, using the same method as before.".to_string(),
            "Brush the very top layer with olive oil, and sprinkle with just a few drops of water.".to_string(),
            "Fold the flaps or excess from the sides, you can crumble them a little. Brush the folded sides well with olive oil. Cut Spanakopita part-way through into squares, or leave the cutting to later.".to_string(),
            "Bake for 45-60 minutes, or until the fillo crust is crisp and golden brown. Remove from the oven. Finish cutting into squares and serve.".to_string(),
            "Optionally serve with greek yoghurt or creme fraiche.".to_string(),
        ],
    }
}
