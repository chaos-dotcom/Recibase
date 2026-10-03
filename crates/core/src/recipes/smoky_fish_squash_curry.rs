//! `se.reciba.api.recipes.SmokyFishSquashCurry`.

use crate::recipe::RecipeDef;
use crate::tag::Tag;
use chrono::NaiveDate;

pub fn recipe() -> RecipeDef {
    RecipeDef {
        object_name: "SmokyFishSquashCurry".to_string(),
        name: "Smoky fish and squash curry".to_string(),
        created_at: NaiveDate::from_ymd_opt(2020, 4, 24).unwrap(),
        permalink_override: Some("smoky-fish-curry".to_string()),
        source: None,
        description: None,
        tagline: None,
        notes: Vec::new(),
        tags: vec![Tag::Pescatarian, Tag::Spicy],
        image: None,
        ingredients_blocks: crate::recipe::IngredientsBlock::simple(vec![
            crate::recipe::Ingredient::qp("Onion", "1", "diced"),
            crate::recipe::Ingredient::qp("Garlic", "1 clove", "finely chopped"),
            crate::recipe::Ingredient::opt("Smoked fish", Some("200g"), Some("cut into bite-size pieces"), Some("Either haddock or basa")),
            crate::recipe::Ingredient::opt("Coriander", Some("5g"), None, Some("Optional")),
            crate::recipe::Ingredient::opt("Red chilli", Some("1"), Some("sliced"), Some("Replace with chilli flakes or powder")),
            crate::recipe::Ingredient::opt("Creamed coconut", Some("50g"), None, Some("More is better")),
            crate::recipe::Ingredient::opt("Ginger", Some("30g"), Some("finely chopped or grated"), Some("Use ground ginger for speed")),
            crate::recipe::Ingredient::q("Stock cube", "1"),
            crate::recipe::Ingredient::q("Ground coriander", "1 tsp"),
            crate::recipe::Ingredient::q("Ground turmeric", "1 tsp"),
            crate::recipe::Ingredient::qpn("Squash", "200g", "peeled and cut into bite-size pieces", "alternatively use sweet potato"),
            crate::recipe::Ingredient::qp("Water", "350ml", "boiling"),
            crate::recipe::Ingredient::opt("Naan Bread", None, None, Some("Optional")),
        ]),
        method: vec![
            "Soften the onion in a pan.".to_string(),
            "Make the stock and add the creamed coconut.".to_string(),
            "Add the garlic, ginger, ground coriander, ground turmeric, chilli and the squash and mix well.".to_string(),
            "Pour in the stock and cook for 15-20 mins or until the squash is soft and the stock has thickened.".to_string(),
            "Add the smoked fish and cook for 5 mins or until cooked through.".to_string(),
            "Serve and garnish with corriander.".to_string(),
        ],
    }
}
