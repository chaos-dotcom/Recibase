//! `se.reciba.api.recipes.SmokyChickpeaStew`.

use crate::recipe::RecipeDef;
use crate::tag::Tag;
use chrono::NaiveDate;

pub fn recipe() -> RecipeDef {
    RecipeDef {
        object_name: "SmokyChickpeaStew".to_string(),
        name: "Smoky Sweet potato and chickpea stew".to_string(),
        created_at: NaiveDate::from_ymd_opt(2020, 7, 12).unwrap(),
        permalink_override: None,
        source: Some("https://www.budgetbytes.com/smoky-potato-chickpea-stew/".to_string()),
        description: Some("A simple yet hearty vegan stew".to_string()),
        tagline: None,
        notes: Vec::new(),
        tags: vec![Tag::Vegan, Tag::Scales, Tag::Freezes],
        image: None,
        ingredients_blocks: crate::recipe::IngredientsBlock::simple(vec![
            crate::recipe::Ingredient::qp("Sweet Potatoes", "620g", "chopped in 1cm cubes"),
            crate::recipe::Ingredient::qp("Garlic Cloves", "4", "finely chopped or crushed"),
            crate::recipe::Ingredient::opt("Carrots", Some("2"), Some("diced"), Some("optional")),
            crate::recipe::Ingredient::qp("Onions", "2", "chopped"),
            crate::recipe::Ingredient::qp("Red pepper", "1", "sliced"),
            crate::recipe::Ingredient::qp("Fresh ginger", "2 tsp", "grated or finely chopped"),
            crate::recipe::Ingredient::opt("Spinach", Some("a large handful"), Some("chopped"), Some("alternatively kale")),
            crate::recipe::Ingredient::qp("Sundried tomatoes", "285g", "chopped"),
            crate::recipe::Ingredient::opt("Vegetarian chorizo sausages", Some("6"), None, Some("optional")),
            crate::recipe::Ingredient::q("Kidney Beans", "1 400g tin"),
            crate::recipe::Ingredient::q("Chopped tomatoes", "1 400g tin"),
            crate::recipe::Ingredient::q("Chickpeas", "2 400g tins"),
            crate::recipe::Ingredient::q("Curry powder", "2 tsps"),
            crate::recipe::Ingredient::q("Smoked paprika", "several teaspoons"),
            crate::recipe::Ingredient::new("Stock cube"),
            crate::recipe::Ingredient::new("Olive oil"),
        ]),
        method: vec![
            "If you're adding the veggie chorizo sausages, preheat the oven per packet instructions.".to_string(),
            "Add some olive oil to a large soup/stew pot.".to_string(),
            "Add the onion, carrots, garlic and ginger to the pot and saute on a medium heat until soft.".to_string(),
            "Meanwhile, disolve the stock cube in boiling water and drain the chickpeas.".to_string(),
            "Add the smoked paprika and curry powder and cook the spices for a minute or two.".to_string(),
            "Add the potatoes, chickpeas, red pepper, sundried tomatoes and chopped tomatoes.".to_string(),
            "Turn up the heat to bring to a boil, then turn down to a light simmer for 45m, sturring occasionaly.".to_string(),
            "Meanwhile, cook the veggie sausages and allow to cool, then slice into 2cm pieces.".to_string(),
            "Add the sausages and kidney beans then simmer for a further 5-10m".to_string(),
            "Throw in the spinach shortly before serving".to_string(),
        ],
    }
}
