//! `se.reciba.api.recipes.Dahl`.

use crate::recipe::RecipeDef;
use crate::tag::Tag;
use chrono::NaiveDate;

pub fn recipe() -> RecipeDef {
    RecipeDef {
        object_name: "Dahl".to_string(),
        name: "Dahl".to_string(),
        created_at: NaiveDate::from_ymd_opt(2020, 4, 24).unwrap(),
        permalink_override: None,
        source: Some("Vegetarian Cookery Bible (2012: Reader's Digest)".to_string()),
        description: Some("An Indian dish of lentils infused with spices.".to_string()),
        tagline: None,
        notes: vec!["Roasted nuts can also be added. Cook them until brown with a little oil in the small pan before adding the butter, cumin and onion.".to_string()],
        tags: vec![
            Tag::Vegan,
            Tag::Scales,
            Tag::Spicy,
            Tag::LowEffort,
        ],
        image: None,
        ingredients_blocks: crate::recipe::IngredientsBlock::simple(vec![
            crate::recipe::Ingredient::q("Red lentils", "350g"),
            crate::recipe::Ingredient::qpn("Water", "800ml", "boiling", "Quantity is important!"),
            crate::recipe::Ingredient::opt("Ground turmeric", Some("1 tsp"), None, Some("Use 2-3 tsp instead")),
            crate::recipe::Ingredient::q("Chilli powder", "1/2 tsp"),
            crate::recipe::Ingredient::qpn("Ginger", "1 cm piece", "peeled and finely chopped", "Replace with ground ginger if necessary"),
            crate::recipe::Ingredient::qp("Garlic", "2 cloves", "finely chopped"),
            crate::recipe::Ingredient::opt("Garam masala", Some("1/2 tsp"), None, Some("Use at least 1 tsp")),
            crate::recipe::Ingredient::new("Salt"),
            crate::recipe::Ingredient::q("Butter", "25g approx"),
            crate::recipe::Ingredient::q("Ground cumin", "1 tsp"),
            crate::recipe::Ingredient::qp("Onion", "1", "diced"),
        ]),
        method: vec![
            "Boil the kettle and rinse the lentils.".to_string(),
            "Put the lentils in a saucepan and cover with the boiling water.".to_string(),
            "Add turmeric and chilli powder then bring the pan to the boil.".to_string(),
            "Add the garlic and ginger to the pan.".to_string(),
            "Cook the lentils for approx. 10 minutes.".to_string(),
            "Add the garam masala.".to_string(),
            "Heat the butter and cumin in a small pan and add the onions. Cook until soft.".to_string(),
            "Mix the onions into the dahl and serve.".to_string(),
        ],
    }
}
