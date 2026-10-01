//! `se.reciba.api.recipes.MexicanPolentaPie`.

use crate::recipe::RecipeDef;
use crate::tag::Tag;
use chrono::NaiveDate;

pub fn recipe() -> RecipeDef {
    RecipeDef {
        object_name: "MexicanPolentaPie".to_string(),
        name: "Mexican Polenta Pie".to_string(),
        created_at: NaiveDate::from_ymd_opt(2020, 4, 24).unwrap(),
        permalink_override: None,
        source: None,
        description: None,
        tagline: None,
        notes: Vec::new(),
        tags: vec![Tag::Vegetarian, Tag::Slow, Tag::Scales],
        image: None,
        ingredients_blocks: vec![
            crate::recipe::IngredientsBlock::new(Some("Base"), vec![
                crate::recipe::Ingredient::q("Sunflower Oil", "1 tbsp"),
                crate::recipe::Ingredient::qp("Celery", "1 stick", "chopped"),
                crate::recipe::Ingredient::qp("Garlic", "1 large clove", "crushed"),
                crate::recipe::Ingredient::qp("Onions", "150g", "chopped"),
                crate::recipe::Ingredient::qp("Green Pepper", "1/2", "chopped"),
                crate::recipe::Ingredient::q("Cayenne Pepper", "1 tsp"),
                crate::recipe::Ingredient::qp("Red Kidney Beans", "400g", "canned, drained and rinsed"),
                crate::recipe::Ingredient::qp("Green Olives", "12", "stoned, sliced"),
                crate::recipe::Ingredient::qp("Jalapeño Peppers", "1 tbsp", "chopped"),
                crate::recipe::Ingredient::qp("Sweetcorn", "75g", "canned, drained"),
                crate::recipe::Ingredient::qp("Chopped Tomatoes", "400g", "canned"),
                crate::recipe::Ingredient::q("Tomato Purée", "1 tbsp"),
                crate::recipe::Ingredient::new("Salt"),
                crate::recipe::Ingredient::new("Black Pepper"),
            ]),
            crate::recipe::IngredientsBlock::new(Some("Topping"), vec![
                crate::recipe::Ingredient::q("Cornmeal or Polenta", "125g"),
                crate::recipe::Ingredient::q("Plain White Flour", "1 tbsp"),
                crate::recipe::Ingredient::q("Baking Powder", "2 tsp"),
                crate::recipe::Ingredient::qp("Egg", "1", "beaten"),
                crate::recipe::Ingredient::q("Skimmed Milk", "100ml"),
                crate::recipe::Ingredient::qp("Half-fat Mature Cheddar Cheese", "25g", "grated"),
            ]),
        ],
        method: vec![
            format!("Heat the oven to {}. Heat the oil in a saucepan over a high heat. Stir in the celery, garlic, onions and green pepper, bring them to a sizzle, then cover, reduce the heat to low and cook for 10 minutes, or until they have softened. Stir in the cayenne pepper and cook for a further 1-2 minutes.", crate::utils::int_utils::celsius(200)),
            "Stir in the kidney beans, olives, jalapeño peppers, sweetcorn, canned tomatoes, tomato purée and add salt and pepper to taste. Bring the mixture to the boil and simmer for 5 minutes. Then spoon the mixture into a 3 litre ovenproof serving dish.".to_string(),
            "Topping: Mix together the cornmeal or polenta, flour, 1/2 teaspoon of salt and the baking powder, then beat in the egg and milk. The mixture should look like a thick batter; if not, add 1-2 tablespoons of milk.".to_string(),
            "Spoon the topping over the vegetables, sprinkle with the cheese and bake for 40 minutes, or until the topping is risen and golden brown. Leave the pie to stand for 5 minutes before serving.".to_string(),
        ],
    }
}
