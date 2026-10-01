//! `se.reciba.api.recipes.ChilliConCarne`.

use crate::recipe::RecipeDef;
use crate::tag::Tag;
use chrono::NaiveDate;

pub fn recipe() -> RecipeDef {
    RecipeDef {
        object_name: "ChilliConCarne".to_string(),
        name: "Chilli con Carne".to_string(),
        created_at: NaiveDate::from_ymd_opt(2020, 4, 24).unwrap(),
        permalink_override: Some("chilli-con-carne".to_string()),
        source: Some("Kit's Dad".to_string()),
        description: None,
        tagline: None,
        notes: Vec::new(),
        tags: vec![
            Tag::VegetarianIsh,
            Tag::Freezes,
            Tag::BetterNextDay,
            Tag::Scales,
            Tag::Spicy,
        ],
        image: None,
        ingredients_blocks: crate::recipe::IngredientsBlock::simple(vec![
            crate::recipe::Ingredient::new("Oil"),
            crate::recipe::Ingredient::new("Salt"),
            crate::recipe::Ingredient::new("Pepper"),
            crate::recipe::Ingredient::q("Chilli Powder", "1 tsp"),
            crate::recipe::Ingredient::q("Crushed or whole dried chillies", "1 tsp"),
            crate::recipe::Ingredient::q("Red Pepper", "1"),
            crate::recipe::Ingredient::new("Frozen Sweetcorn"),
            crate::recipe::Ingredient::new("Paprika"),
            crate::recipe::Ingredient::new("Tomato Puree"),
            crate::recipe::Ingredient::q("Peeled plum tomatoes", "1 400g tin"),
            crate::recipe::Ingredient::new("Italian Herbs"),
            crate::recipe::Ingredient::new("Brown Sauce"),
            crate::recipe::Ingredient::new("Beef stock cube or Bovril"),
            crate::recipe::Ingredient::new("Rice"),
            crate::recipe::Ingredient::new("Olive Oil"),
            crate::recipe::Ingredient::q("Onion", "1"),
            crate::recipe::Ingredient::q("Garlic Clove", "1"),
            crate::recipe::Ingredient::q("Kidney Beans", "1 400g tin"),
            crate::recipe::Ingredient::opt("Honey", Some("1 tbsp"), None, Some("Optional")),
            crate::recipe::Ingredient::opt("Dark cooking chocolate", None, None, Some("Optional")),
            crate::recipe::Ingredient::q("Mince", "500g"),
        ]),
        method: vec![
            "Slice onions, crush garlic then brown in 2 tsp olive oil.".to_string(),
            "Add the pepper and cook for a minute.".to_string(),
            "Add the mince in batches and brown then stir in chilli powder and paprika.".to_string(),
            "Add the stock cube/Bovril dissolved in a small amount of water and stir.".to_string(),
            "Then add 2 inches or so of tomato purée and stir again, followed by the tinned tomatoes.".to_string(),
            "Simmer for half an hour then check for taste.".to_string(),
            "Add chilli powder or tomato purée depending on flavour, also salt/pepper if needed.".to_string(),
            "Stir in the sweetcorn.".to_string(),
            "A tablespoon of honey or a few pieces of dark chocolate can improve the flavour.".to_string(),
            "Serve with rice (preferably) or pasta.".to_string(),
        ],
    }
}
