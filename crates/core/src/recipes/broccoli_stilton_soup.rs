//! `se.reciba.api.recipes.BroccoliStiltonSoup`.

use crate::recipe::RecipeDef;
use crate::tag::Tag;
use chrono::NaiveDate;

pub fn recipe() -> RecipeDef {
    RecipeDef {
        object_name: "BroccoliStiltonSoup".to_string(),
        name: "Broccoli & Stilton soup".to_string(),
        created_at: NaiveDate::from_ymd_opt(2024, 1, 17).unwrap(),
        permalink_override: Some("broccoli-stilton-soup".to_string()),
        source: None,
        description: Some("A rich and nutritious winter warmer".to_string()),
        tagline: None,
        notes: vec!["The original recipe uses a 1:2 stilton to broccoli ratio rather than our decadent 1:1 ratio. If you'd prefer not to get gout then stick with the original ratio.".to_string()],
        tags: vec![
            Tag::Slow,
            Tag::Vegetarian,
            Tag::Scales,
            Tag::Soup,
            Tag::LowEffort,
            Tag::Freezes,
            Tag::BetterNextDay,
            Tag::ColdWeather,
        ],
        image: None,
        ingredients_blocks: crate::recipe::IngredientsBlock::simple(vec![
            crate::recipe::Ingredient::q("Broccoli", "2 heads"),
            crate::recipe::Ingredient::q("Stilton", "220g"),
            crate::recipe::Ingredient::qp("Onion", "1 onion", "roughly chopped"),
            crate::recipe::Ingredient::qp("Garlic", "3 cloves", "diced"),
            crate::recipe::Ingredient::q("Stock Cube", "1"),
            crate::recipe::Ingredient::q("Boiled water", "1L"),
            crate::recipe::Ingredient::q("Butter", "1 knob"),
            crate::recipe::Ingredient::new("Ground Nutmeg"),
            crate::recipe::Ingredient::new("Lemon Juice"),
            crate::recipe::Ingredient::new("Salt"),
        ]),
        method: vec![
            "Cut off the broccoli florets and chop the stalk into 2cm pieces.".to_string(),
            "Heat a large pan over a medium heat then melt the knob of butter.".to_string(),
            "Add the garlic and onions then soften for a few minutes.".to_string(),
            "Add the broccoli stalks and cook for another minute.".to_string(),
            "Add the broccoli florets, stock cube and enough water to cover all but the last inch of the mixture.".to_string(),
            "Add a few drops of lemon juice and a small sprinkle of nutmeg.".to_string(),
            "Bring to the boil then cover and simmer for 20 minutes.".to_string(),
            "Roughly chop the stilton.".to_string(),
            "Turn off the heat and blend.".to_string(),
            "Put back over a low heat and add the stilton. Stir until melted. Be careful: the mixture will now stick more easily and possibly spit.".to_string(),
            "Add more lemon juice and possibly salt to taste.".to_string(),
            "Serve with fresh bread.".to_string(),
        ],
    }
}
