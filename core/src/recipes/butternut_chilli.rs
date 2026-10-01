//! `se.reciba.api.recipes.ButternutChilli`.

use crate::recipe::RecipeDef;
use crate::tag::Tag;
use chrono::NaiveDate;

pub fn recipe() -> RecipeDef {
    RecipeDef {
        object_name: "ButternutChilli".to_string(),
        name: "Butternut Chilli".to_string(),
        created_at: NaiveDate::from_ymd_opt(2021, 1, 24).unwrap(),
        permalink_override: None,
        source: Some("Harry".to_string()),
        description: None,
        tagline: None,
        notes: Vec::new(),
        tags: vec![
            Tag::Vegetarian,
            Tag::Spicy,
        ],
        image: None,
        ingredients_blocks: crate::recipe::IngredientsBlock::simple(vec![
            crate::recipe::Ingredient::qp("Onion", "1", "Chopped"),
            crate::recipe::Ingredient::qp("Butternut Squash", "1", "Cubed"),
            crate::recipe::Ingredient::q("Vine Tomatoes", "400g"),
            crate::recipe::Ingredient::q("Red Wine", "150ml"),
            crate::recipe::Ingredient::q("Vegetable stock cube", "1/4"),
            crate::recipe::Ingredient::q("Black turtle beans", "400g tin"),
            crate::recipe::Ingredient::q("Soured Cream", "2 tbsp"),
            crate::recipe::Ingredient::qpn("Piquillo peppers", "6", "Chopped", "or 1 roasted Romano pepper"),
            crate::recipe::Ingredient::qp("Garlic", "1 large clove", "chopped"),
            crate::recipe::Ingredient::q("Cayenne Pepper", "1 tsp"),
            crate::recipe::Ingredient::q("Oregano", "1 tsp"),
            crate::recipe::Ingredient::qp("Red chilli", "1", "finely diced"),
            crate::recipe::Ingredient::qp("Pitted Green Olives", "6", "Chopped"),
            crate::recipe::Ingredient::new("Bay Leaf"),
            crate::recipe::Ingredient::new("Chives"),
            crate::recipe::Ingredient::new("Lemon Juice"),
            crate::recipe::Ingredient::new("Olive Oil"),
        ]),
        method: vec![
            "Cut the lemon into small pieces, each with skin.".to_string(),
            "Pour boiling water over the tomatoes, count to 30, quarter, peel and chop.".to_string(),
            "Soften the onion and garlic in olive oil in a heavy lidded pan.".to_string(),
            "Stir chilli, cayenne, oregano and bay into onion then stir-fry for 1 minute.".to_string(),
            "Add squash, olives, lemon and wine. Simmer, stirring, for 2 min.".to_string(),
            "Add tomatoes, 150ml water and crumbled stock cube. Gently simmer, cover and cook for 20 min.".to_string(),
            "Chop the peppers and add to the pan. Taste for salt, adding extra water if drying out.".to_string(),
            "Cover and cook for a further 20 min until lemon is tender.".to_string(),
            "Drain and rinse the beans. Add to the pot.".to_string(),
            "Reheat now, later or tomorrow. Serve with soured cream and chives.".to_string(),
        ],
    }
}
