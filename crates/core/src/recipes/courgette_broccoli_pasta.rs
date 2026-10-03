//! `se.reciba.api.recipes.CourgetteBroccoliPasta`.

use crate::recipe::RecipeDef;
use crate::tag::Tag;
use chrono::NaiveDate;

pub fn recipe() -> RecipeDef {
    RecipeDef {
        object_name: "CourgetteBroccoliPasta".to_string(),
        name: "Courgette & Broccoli Pasta".to_string(),
        created_at: NaiveDate::from_ymd_opt(2020, 4, 24).unwrap(),
        permalink_override: None,
        source: Some("Gousto".to_string()),
        description: None,
        tagline: None,
        notes: vec![
            "If tagliatelle is unavailable try spaghetti.".to_string(),
            "You can use garlic paste instead if you add it to the stock, rather than cooking it with the courgettes.".to_string(),
            "Try adding a tablespoon of creme fraiche or soured cream, for a richer sauce.".to_string(),
        ],
        tags: vec![
            Tag::Quick,
            Tag::Vegetarian,
            Tag::HotWeather,
        ],
        image: None,
        ingredients_blocks: crate::recipe::IngredientsBlock::simple(vec![
            crate::recipe::Ingredient::qp("Courgette", "1", "roughly chopped"),
            crate::recipe::Ingredient::qp("Tenderstem broccoli", "120g", "halved"),
            crate::recipe::Ingredient::q("Fresh tagliatelle", "190g"),
            crate::recipe::Ingredient::qp("Garlic", "3 cloves", "finely chopped"),
            crate::recipe::Ingredient::q("Baby spinach", "80g"),
            crate::recipe::Ingredient::opt("Parmesan", None, Some("grated"), None),
            crate::recipe::Ingredient::new("Lemon juice"),
            crate::recipe::Ingredient::new("Dried chilli flakes"),
            crate::recipe::Ingredient::opt("Flaked almonds", Some("handful"), None, Some("optional")),
            crate::recipe::Ingredient::opt("Soured cream", Some("1 tbsp"), None, Some("optional")),
            crate::recipe::Ingredient::new("Vegetable stock"),
            crate::recipe::Ingredient::new("Olive oil"),
        ]),
        method: vec![
            "Dissolve the stock in 200ml of boiling water".to_string(),
            "Heat some olive oil on a high heat in a wide pan".to_string(),
            "Fry the broccoli stalks for a minute".to_string(),
            "Add the broccoli florets and fry for a few more minutes, until browned".to_string(),
            "Remove the broccoli from the pan and set aside".to_string(),
            "Cook the courgettes and garlic for 4-5 minutes or until starting to soften, then turn off the heat".to_string(),
            "Cook the tagliatelle per packet instructions".to_string(),
            "Drain the tagliatelle and add it to the courgette pan".to_string(),
            "Add the lemon juice, chilli flakes, parmesan, flaked almonds, spinach, soured cream (optional), stock and broccoli to the pan".to_string(),
            "Cook on a medium heat for 2 minutes until the sauce has thickened and the spinach wilted".to_string(),
        ],
    }
}
