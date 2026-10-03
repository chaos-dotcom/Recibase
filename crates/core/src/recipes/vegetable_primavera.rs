//! `se.reciba.api.recipes.VegetablePrimavera`.

use crate::recipe::RecipeDef;
use crate::tag::Tag;
use chrono::NaiveDate;

pub fn recipe() -> RecipeDef {
    RecipeDef {
        object_name: "VegetablePrimavera".to_string(),
        name: "Vegetable Primavera".to_string(),
        created_at: NaiveDate::from_ymd_opt(2020, 4, 24).unwrap(),
        permalink_override: None,
        source: Some("Vegetarian Cookery Bible (2012: Reader's Digest)".to_string()),
        description: None,
        tagline: None,
        notes: vec![
            "The mustard is a key part of the dish, so the quantity here is only a suggestion. Alex likes to use a lot.".to_string(),
            "Most combinations of small or baby vegetables work, as does e.g. tenderstem broccoli. Carrot sticks are a suitable replacement for baby carrots, too. Use whatever you have or can get. Aim for 3-4 types. Suggestions include: baby sweetcorn, green beans, baby carrots, mange tout.".to_string(),
            "Many types or tortellini or ravioli work for this dish.".to_string(),
        ],
        tags: vec![
            Tag::Vegetarian,
            Tag::Quick,
            Tag::HotWeather,
            Tag::LowEffort,
        ],
        image: None,
        ingredients_blocks: crate::recipe::IngredientsBlock::simple(vec![
            crate::recipe::Ingredient::opt("Baby vegetables", Some("3-4 varieties"), None, Some("See notes for suggestions")),
            crate::recipe::Ingredient::opt("Tortellini or ravioli", Some("400g"), None, Some("Choose your own flavour")),
            crate::recipe::Ingredient::opt("Olive oil", Some("1 tbsp"), None, Some("Extra virgin preferred")),
            crate::recipe::Ingredient::opt("Lemon", Some("1/2"), None, Some("Or use lemon juice")),
            crate::recipe::Ingredient::opt("Wholegrain mustard", Some("1-2 tbsp"), None, None),
            crate::recipe::Ingredient::new("Salt"),
            crate::recipe::Ingredient::new("Black pepper"),
        ]),
        method: vec![
            "Prepare and steam the vegetables, in batches if necessary. Do not mix vegetables that cook quickly with those that cook slowly. Remove when still slightly crisp.".to_string(),
            "Cook the pasta and drain.".to_string(),
            "Heat the oil in the pan and add the lemon juice, mustard, salt and pepper.".to_string(),
            "Mix in the vegetables followed by the pasta.".to_string(),
            "Serve on warmed plates.".to_string(),
        ],
    }
}
