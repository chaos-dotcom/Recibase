//! `se.reciba.api.recipes.PaneerJalfrezi`.

use crate::recipe::RecipeDef;
use crate::tag::Tag;
use chrono::NaiveDate;

pub fn recipe() -> RecipeDef {
    RecipeDef {
        object_name: "PaneerJalfrezi".to_string(),
        name: "Paneer Jalfrezi".to_string(),
        created_at: NaiveDate::from_ymd_opt(2020, 4, 24).unwrap(),
        permalink_override: None,
        source: Some("Gousto".to_string()),
        description: None,
        tagline: None,
        notes: Vec::new(),
        tags: vec![
            Tag::Vegetarian,
            Tag::Quick,
            Tag::Spicy,
            Tag::LowEffort,
        ],
        image: None,
        ingredients_blocks: crate::recipe::IngredientsBlock::simple(vec![
            crate::recipe::Ingredient::qp("Onion", "1", "sliced"),
            crate::recipe::Ingredient::qp("Green pepper", "1", "sliced"),
            crate::recipe::Ingredient::q("Chilli flakes", "1/2 tsp"),
            crate::recipe::Ingredient::qp("Ginger", "15g", "finely chopped or grated"),
            crate::recipe::Ingredient::qp("Paneer", "1 block (225g)", "cut into bite-sized cubes"),
            crate::recipe::Ingredient::qp("Garlic", "2-3 cloves", "finely chopped or crushed"),
            crate::recipe::Ingredient::qp("Salad Tomatoes", "2", "chopped"),
            crate::recipe::Ingredient::q("Curry powder", "1 tbsp"),
            crate::recipe::Ingredient::q("Tomato purée", "32g"),
            crate::recipe::Ingredient::q("Brown sugar", "2 tsp"),
            crate::recipe::Ingredient::q("Stock cube", "1"),
            crate::recipe::Ingredient::q("Water", "300ml"),
        ]),
        method: vec![
            "Fry the paneer until brown, turning to achieve an even colouring then set aside.".to_string(),
            "Soften the onion in the pan with the sugar and a pich of salt until brown and starting to caramelise.".to_string(),
            "Add the sliced pepper and cook for 5-8 mins.".to_string(),
            "Disolve the stock cube in water.".to_string(),
            "Add the garlic, tomatoes, tomato purée, ginger, chilli flakes, curry poweder, stock and paneer to the pan and stir.".to_string(),
            "Cook for 5-10 mins on a medium heat until thickened.".to_string(),
        ],
    }
}
