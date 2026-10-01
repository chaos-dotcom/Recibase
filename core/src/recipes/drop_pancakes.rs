//! `se.reciba.api.recipes.DropPancakes`.

use crate::recipe::RecipeDef;
use crate::tag::Tag;
use chrono::NaiveDate;

pub fn recipe() -> RecipeDef {
    RecipeDef {
        object_name: "DropPancakes".to_string(),
        name: "Scotch Pancakes (BBC)".to_string(),
        created_at: NaiveDate::from_ymd_opt(2026, 2, 17).unwrap(),
        permalink_override: Some("scotch-pancakes".to_string()),
        source: Some("https://www.bbcgoodfood.com/recipes/drop-pancakes".to_string()),
        description: None,
        tagline: None,
        notes: vec![
            "You need very little butter to cook the pancakes. Too much and they won't cook properly".to_string(),
        ],
        tags: vec![
            Tag::Vegetarian,
            Tag::Scales,
            Tag::Pudding,
            Tag::Quick,
            Tag::LowEffort,
        ],
        image: None,
        ingredients_blocks: crate::recipe::IngredientsBlock::simple(vec![
            crate::recipe::Ingredient::q("Plain flour", "200g"),
            crate::recipe::Ingredient::q("Baking powder", "1/2 tbsp"),
            crate::recipe::Ingredient::q("Fine salt", "1/2 tsp"),
            crate::recipe::Ingredient::q("Golden caster sugar", "50g"),
            crate::recipe::Ingredient::qp("Egg", "1", "beaten"),
            crate::recipe::Ingredient::q("Milk", "200ml"),
            crate::recipe::Ingredient::opt("Butter", None, None, Some("for cooking")),
        ]),
        method: vec![
            "Sieve the plain flour into a bowl, then add the baking powder, golden caster sugar and fine salt.".to_string(),
            "Whisk the beaten egg and milk together in a jug, then pour into the bowl and whisk for a few minutes until you have a smooth, thick batter.".to_string(),
            "Add a little butter to a large frying pan.".to_string(),
            "Heat the pan over medium heat, and once hot drop 2 tablespoons of the batter into the pan to make small pancakes — you should be able to cook about 4-5 at a time.".to_string(),
            "Cook the pancakes for 2-3 minutes until the edges are set and bubbles rise in the centre. Flip and cook for another 2-3 minutes until golden brown and cooked through. Repeat with the remaining batter, adding more butter when needed.".to_string(),
            "Serve the drop pancakes topped with extra butter and a drizzle of maple syrup alongside seasonal fruit, if you like.".to_string(),
        ],
    }
}
