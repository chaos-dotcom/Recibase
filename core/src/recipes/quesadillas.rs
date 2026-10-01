//! `se.reciba.api.recipes.Quesadillas`.

use crate::recipe::RecipeDef;
use crate::tag::Tag;
use chrono::NaiveDate;

pub fn recipe() -> RecipeDef {
    RecipeDef {
        object_name: "Quesadillas".to_string(),
        name: "Quesadillas".to_string(),
        created_at: NaiveDate::from_ymd_opt(2022, 11, 18).unwrap(),
        permalink_override: None,
        source: Some("https://www.hellofresh.co.uk/recipes/cheesy-chipotle-bean-quesadillas-5feb6402f4480c042d622a2d".to_string()),
        description: None,
        tagline: None,
        notes: vec![
            "This also works with ancho chilli paste.".to_string(),
            "The original makes a side salad but I have better things to do with my life.".to_string(),
        ],
        tags: vec![
            Tag::Quick,
            Tag::Vegetarian,
            Tag::Spicy,
            Tag::LowEffort,
        ],
        image: None,
        ingredients_blocks: crate::recipe::IngredientsBlock::simple(vec![
            crate::recipe::Ingredient::q("Mixed beans", "1 400ml tin"),
            crate::recipe::Ingredient::opt("Kidney beans", Some("1 400ml tin"), None, Some("or black beans")),
            crate::recipe::Ingredient::opt("Frozen Sweetcorn", None, None, Some("optional, if the mixed beans have no sweetcorn")),
            crate::recipe::Ingredient::opt("Cheddar", Some("60g"), Some("grated"), None),
            crate::recipe::Ingredient::opt("Spring Onions", Some("8"), Some("chopped"), None),
            crate::recipe::Ingredient::q("Chipotle paste", "5 tsp"),
            crate::recipe::Ingredient::q("Tomato Puree", "2 tbsp"),
            crate::recipe::Ingredient::q("Tortillas", "4"),
            crate::recipe::Ingredient::new("Soured Cream"),
            crate::recipe::Ingredient::new("Oil"),
            crate::recipe::Ingredient::new("Salt"),
            crate::recipe::Ingredient::new("Pepper"),
        ]),
        method: vec![
            "Add the beans to a mixing bowl and mush with a fork".to_string(),
            "Add the cheese, spring onions, sweetcorn, chipotle paste and tomato puree to the mixing bowl. Season with salt and pepper to taste. Add more chipotle paste, if desired.".to_string(),
            "Lay out the tortillas and split the mixture evenly across each.".to_string(),
            "Spread the mixture across half of each tortilla, to make folding easier. Fold over each tortilla.".to_string(),
            "Put a frying pan on a high heat with a small drizzle of oil.".to_string(),
            "Fry each tortilla for a minute or two, until browned, then flip and repeat. Keep an eye on the underside to avoid it burning. Set aside once both sides are browned.".to_string(),
            "Serve with some soured cream".to_string(),
        ],
    }
}
