//! `se.reciba.api.recipes.RussianMushroomJulienne`.

use crate::recipe::RecipeDef;
use crate::tag::Tag;
use chrono::NaiveDate;

pub fn recipe() -> RecipeDef {
    RecipeDef {
        object_name: "RussianMushroomJulienne".to_string(),
        name: "Russian Mushroom Julienne".to_string(),
        created_at: NaiveDate::from_ymd_opt(2020, 4, 24).unwrap(),
        permalink_override: None,
        source: Some("https://livelaughrowe.com/russian-dish-mushroom-julienne/".to_string()),
        description: None,
        tagline: None,
        notes: Vec::new(),
        tags: vec![Tag::Vegetarian, Tag::Stodge],
        image: None,
        ingredients_blocks: crate::recipe::IngredientsBlock::simple(vec![
            crate::recipe::Ingredient::opt("Mushrooms", Some("250g"), Some("thinly sliced"), None),
            crate::recipe::Ingredient::opt("Onion", None, Some("thinly sliced"), None),
            crate::recipe::Ingredient::new("White wine"),
            crate::recipe::Ingredient::q("Soured cream", "150ml"),
            crate::recipe::Ingredient::q("Double cream", "120ml"),
            crate::recipe::Ingredient::opt("Mozzarella cheese", Some("240g"), Some("roughly chopped"), None),
            crate::recipe::Ingredient::new("Butter"),
        ]),
        method: vec![
            format!("Preheat the oven to {}.", crate::utils::int_utils::celsius(190)),
            "In a medium pan, melt the butter then saute the mushrooms and onions on a medium heat, until they've given off liquid.".to_string(),
            "Transfer the mixture to a medium casserole dish.".to_string(),
            "Using the same pan, melt a knob of butter and add a slosh of white wine.".to_string(),
            "Simmer on a medium heat for a couple of minutes, then stir in the double cream and soured cream.".to_string(),
            "Bring it to boil and pour over the mushrooms. Stir the mixture.".to_string(),
            "Sprinkle generously with cheese and bake for about 10 minutes (until the cheese is melted and starts to golden).".to_string(),
        ],
    }
}
