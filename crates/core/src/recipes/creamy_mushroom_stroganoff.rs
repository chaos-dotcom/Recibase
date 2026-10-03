//! `se.reciba.api.recipes.CreamyMushroomStroganoff`.

use crate::recipe::RecipeDef;
use crate::tag::Tag;
use chrono::NaiveDate;

pub fn recipe() -> RecipeDef {
    RecipeDef {
        object_name: "CreamyMushroomStroganoff".to_string(),
        name: "Mushroom Stroganoff".to_string(),
        created_at: NaiveDate::from_ymd_opt(2020, 4, 24).unwrap(),
        permalink_override: None,
        source: Some("http://allrecipes.co.uk/recipe/6460/creamy-mushroom-stroganoff.aspx".to_string()),
        description: None,
        tagline: None,
        notes: vec!["You can substitute Shiitake or Porchini mushrooms for their dried equivalents.\nPots of dried forest mushrooms also work. Try softening the mushrooms in the stock.".to_string()],
        tags: vec![
            Tag::Vegetarian,
            Tag::Scales,
            Tag::LowEffort,
            Tag::Quick,
        ],
        image: None,
        ingredients_blocks: crate::recipe::IngredientsBlock::simple(vec![
            crate::recipe::Ingredient::new("Butter"),
            crate::recipe::Ingredient::qp("Large Onion", "1", "sliced"),
            crate::recipe::Ingredient::qp("Brown Mushrooms", "250g", "sliced"),
            crate::recipe::Ingredient::q("Shiitake or Porchini Mushrooms", "Some"),
            crate::recipe::Ingredient::q("Vegetable or Mushroom stock cube", "1"),
            crate::recipe::Ingredient::q("Soured Cream", "350ml"),
            crate::recipe::Ingredient::q("Plain Flour", "3 tbsp"),
            crate::recipe::Ingredient::new("Salt"),
            crate::recipe::Ingredient::new("Black Pepper"),
            crate::recipe::Ingredient::qpn("Garlic", "1 clove", "diced", "Optional"),
            crate::recipe::Ingredient::opt("Nutmeg", None, None, Some("Optional")),
            crate::recipe::Ingredient::opt("Brandy", None, None, Some("Optional")),
            crate::recipe::Ingredient::opt("Marjoram", None, None, Some("Optional")),
            crate::recipe::Ingredient::opt("Worcestershire sauce", None, None, Some("Optional")),
        ]),
        method: vec![
            "Chop the shiitake or porchini mushrooms into large pieces".to_string(),
            "Boil the kettle then use as little water as possible to disolve the cube in a jug.".to_string(),
            "Melt butter in a large frying pan then soften the onion with the garlic.".to_string(),
            "Turn up the heat and add the brown mushrooms. Cook for a minute or so.".to_string(),
            "Add the Shiitake or Porchini Mushrooms and cook for a minute more.".to_string(),
            "Turn down the heat then stir in the soured cream, being careful not to let it boil.".to_string(),
            "Stir in the vegetable stock until you have a creamy sauce that isn't too watery. You might not need all the stock. Sift in plain flour, a small bit at a time, if needed.".to_string(),
            "Add salt, black pepper, nutmeg, Worcestershire sauce, marjoram and brandy to taste.".to_string(),
            "Serve with rice.".to_string(),
        ],
    }
}
