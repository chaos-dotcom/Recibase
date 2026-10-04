//! `se.reciba.api.recipes.SummertimeMacNCheese`.
//! chaos-tag: casa-chaos

use crate::recipe::RecipeDef;
use crate::tag::Tag;
use chrono::NaiveDate;

pub fn recipe() -> RecipeDef {
    RecipeDef {
        object_name: "SummertimeMacNCheese".to_string(),
        name: "Summertime Mac 'n' Cheese".to_string(),
        created_at: NaiveDate::from_ymd_opt(2023, 7, 1).unwrap(),
        permalink_override: None,
        source: Some("Waitrose".to_string()),
        description: None,
        tagline: None,
        notes: vec![
            "Serves 4".to_string(),
            "Smart swap: if you have peas in your freezer, you can use them instead of the spinach. Just cook them with the pasta at the same time as the sweetcorn.".to_string(),
        ],
        tags: vec![
            Tag::Vegetarian,
            Tag::Scales,
            Tag::Stodge,
            Tag::Quick,
        ],
        image: None,
        ingredients_blocks: crate::recipe::IngredientsBlock::simple(vec![
            crate::recipe::Ingredient::q("Essential Unsalted Dairy Butter", "30g"),
            crate::recipe::Ingredient::q("Essential Plain Flour", "30g"),
            crate::recipe::Ingredient::q("Skimmed Milk", "500ml"),
            crate::recipe::Ingredient::q("Dried Penne or Fusilli Pasta", "350g"),
            crate::recipe::Ingredient::q("Essential Frozen Sweetcorn", "200g"),
            crate::recipe::Ingredient::q("Dijon Mustard", "1 tsp"),
            crate::recipe::Ingredient::q("Salt", "1/2 tsp"),
            crate::recipe::Ingredient::qp("Essential Mature Cheddar", "200g", "grated"),
            crate::recipe::Ingredient::q("Baby Spinach", "120g pack"),
            crate::recipe::Ingredient::opt("Green Salad", None, None, Some("To serve (optional)")),
        ]),
        method: vec![
            "Preheat the grill to medium-high. Heat the butter in a large saucepan over a medium-high heat. When foaming, add the flour and stir for 2 minutes over the heat to form a paste. Add the milk a little at a time, stirring as you go, to make a smooth sauce. Meanwhile, bring a large pan of salted water to the boil and cook the pasta for 2 minutes less than pack instructions, adding the sweetcorn for the last 2 minutes.".to_string(),
            "Once all the milk is incorporated into the flour, stir in the mustard and salt then simmer for 4-5 minutes, stirring regularly, until slightly thickened and glossy. Lower the heat and stir in 3/4 of the Cheddar until melted.".to_string(),
            "Drain the pasta and sweetcorn and add to the cheese sauce with the spinach. Stir over a low heat for 1 minute until the spinach has just wilted slightly. Tip into an ovenproof dish, scatter over the remaining cheese and grill for 3-5 minutes until golden on top. Serve with a green salad, if liked.".to_string(),
        ],
    }
}
