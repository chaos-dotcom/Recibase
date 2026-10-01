//! `se.reciba.api.recipes.LentilSpinachStew`.

use crate::recipe::RecipeDef;
use crate::tag::Tag;
use chrono::NaiveDate;

pub fn recipe() -> RecipeDef {
    RecipeDef {
        object_name: "LentilSpinachStew".to_string(),
        name: "Lentil & Spinach Stew".to_string(),
        created_at: NaiveDate::from_ymd_opt(2022, 3, 26).unwrap(),
        permalink_override: None,
        source: Some("https://drive.google.com/file/d/1f247M9Y9DLk4T6NzMng1B6GRvhoaAX74C28UTyxk_Jo/view".to_string()),
        description: None,
        tagline: None,
        notes: Vec::new(),
        tags: vec![
            Tag::Vegan,
            Tag::Scales,
        ],
        image: None,
        ingredients_blocks: crate::recipe::IngredientsBlock::simple(vec![
            crate::recipe::Ingredient::qp("Garlic", "1 clove", "diced"),
            crate::recipe::Ingredient::qp("Onion", "1", "roughly chopped"),
            crate::recipe::Ingredient::qp("Carrots", "2-3", "chopped"),
            crate::recipe::Ingredient::opt("Salad Tomatoes", None, Some("chopped"), None),
            crate::recipe::Ingredient::opt("Cream of coconut", None, Some("grated"), None),
            crate::recipe::Ingredient::new("Frozen Spinach"),
            crate::recipe::Ingredient::q("Lentils", "150g"),
            crate::recipe::Ingredient::q("Water", "400ml"),
            crate::recipe::Ingredient::new("Worcestershire Sauce"),
            crate::recipe::Ingredient::q("Stock cube", "1"),
            crate::recipe::Ingredient::new("Honey"),
            crate::recipe::Ingredient::new("Chili flakes"),
            crate::recipe::Ingredient::new("Cinnamon"),
            crate::recipe::Ingredient::new("Nutmeg"),
            crate::recipe::Ingredient::new("Olive Oil"),
        ]),
        method: vec![
            "Put out frozen spinach so it can defrost.".to_string(),
            "Boil the kettle and make stock.".to_string(),
            "Heat the olive oil in a pan and cook the carrots and garlic with a little bit of honey for a few minutes.".to_string(),
            "Add the onions and cook for a few more minutes.".to_string(),
            "Add the tomatoes, stock, grated cream of coconut, a dash of Worcestershire sauce and the spices.".to_string(),
            "Bring to simmer then add the lentils. You’ll need to possibly add a bit of water as the lentils simmer.".to_string(),
            "Simmer with the lid on for however long lentils take (15-20 minutes?).".to_string(),
            "Add frozen spinach.".to_string(),
        ],
    }
}
