//! `se.reciba.api.recipes.SaagPaneer`.

use crate::recipe::RecipeDef;
use crate::tag::Tag;
use chrono::NaiveDate;

pub fn recipe() -> RecipeDef {
    RecipeDef {
        object_name: "SaagPaneer".to_string(),
        name: "Saag Paneer".to_string(),
        created_at: NaiveDate::from_ymd_opt(2020, 4, 24).unwrap(),
        permalink_override: None,
        source: None,
        description: None,
        tagline: None,
        notes: vec![
            "All quantities are very approximate. Some ingredients are probably also missing as I really just make this up as I go...".to_string(),
            "For frying the paneer, the goal is something like a very shallow shallow fry, with more oil than would be used for pan frying but less than most shallow frying. A sautee pan is ideal for this.".to_string(),
        ],
        tags: vec![
            Tag::Vegetarian,
            Tag::HotWeather,
            Tag::Spicy,
            Tag::LowEffort,
        ],
        image: None,
        ingredients_blocks: crate::recipe::IngredientsBlock::simple(vec![
            crate::recipe::Ingredient::opt("Paneer", Some("1 block"), Some("cut into 2-3 cm cubes"), None),
            crate::recipe::Ingredient::q("Ground turmeric", "2 tsp"),
            crate::recipe::Ingredient::opt("Chilli powder", Some("1/2 tsp"), None, None),
            crate::recipe::Ingredient::opt("Onion", Some("1"), Some("diced, but not too finely"), None),
            crate::recipe::Ingredient::opt("Ground cumin", Some("1/2 tsp"), None, Some("Probably optional")),
            crate::recipe::Ingredient::opt("Ground cinnamon", Some("1/2 tsp"), None, Some("Can probably replace this and the cumin with garam masala")),
            crate::recipe::Ingredient::opt("Tomatoes", Some("2"), Some("diced"), Some("Tins of chopped tomatoes are far too wet for this. Also optional.")),
            crate::recipe::Ingredient::opt("Spinach", Some("1 large bag minimum"), Some("torn up"), Some("Can also use frozen spinach. Either way, a lot is needed.")),
            crate::recipe::Ingredient::opt("Peas", None, None, Some("Optional")),
        ]),
        method: vec![
            "Add oil, turmeric and chilli powder to a wide, deep pan.".to_string(),
            "Add the paneer and fry on a high heat, turning occasionally until crispy and brown on multiple sides. Remove and set aside.".to_string(),
            "Add the onions, cumin and cinnamon and cook in the remaining oil until very soft. At some point also add the tomatoes.".to_string(),
            "Add the spinach (in batches if necessary) and wilt.".to_string(),
            "Add peas if used.".to_string(),
            "Add the fried paneer to the pan and heat, stirring carefully so as not to break the paneer.".to_string(),
            "Serve with rice or naan bread.".to_string(),
        ],
    }
}
