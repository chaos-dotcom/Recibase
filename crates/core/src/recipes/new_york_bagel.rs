//! `se.reciba.api.recipes.NewYorkBagel`.

use crate::recipe::RecipeDef;
use crate::tag::Tag;
use chrono::NaiveDate;

pub fn recipe() -> RecipeDef {
    RecipeDef {
        object_name: "NewYorkBagel".to_string(),
        name: "New York Bagels".to_string(),
        created_at: NaiveDate::from_ymd_opt(2021, 9, 13).unwrap(),
        permalink_override: None,
        source: Some("https://www.thevegspace.co.uk/recipe-four-fabulously-filling-bagel-toppings/".to_string()),
        description: Some("A vegetarian take on the classic pastrami bagel".to_string()),
        tagline: None,
        notes: Vec::new(),
        tags: vec![Tag::Lunch],
        image: None,
        ingredients_blocks: crate::recipe::IngredientsBlock::simple(vec![
            crate::recipe::Ingredient::q("Bagels", "4"),
            crate::recipe::Ingredient::opt("Squeaky Bean Deli Pastrami", Some("180g"), None, Some("2 packets")),
            crate::recipe::Ingredient::opt("Cooked Beetroot", Some("180g"), None, Some("ideally pre-grated sweet & smoky")),
            crate::recipe::Ingredient::new("Pickles"),
            crate::recipe::Ingredient::opt("Cheese", None, None, Some("ideally something fancy like Comté")),
            crate::recipe::Ingredient::opt("Mayonnaise", None, None, Some("optional")),
        ]),
        method: vec![
            "Halve and lightly toast the bagels.".to_string(),
            "Slice the beetroot, pickles and cheese.".to_string(),
            "Layer the bottom half of each bagel with mayo, pastrami, pickles, beetroot and then cheese. Make sure the cheese covers everything so it doesn't burn.".to_string(),
            "Grill the bottom halves of the bagels until the cheese is bubbly.".to_string(),
            "Optionally spread the top half of each bagel with mayo.".to_string(),
            "Combine the top and bottom halves of the bagels and serve.".to_string(),
        ],
    }
}
