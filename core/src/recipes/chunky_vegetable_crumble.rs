//! `se.reciba.api.recipes.ChunkyVegetableCrumble`.

use crate::recipe::RecipeDef;
use crate::tag::Tag;
use chrono::NaiveDate;

pub fn recipe() -> RecipeDef {
    RecipeDef {
        object_name: "ChunkyVegetableCrumble".to_string(),
        name: "Chunky Vegetable Crumble".to_string(),
        created_at: NaiveDate::from_ymd_opt(2020, 4, 24).unwrap(),
        permalink_override: Some("vegetable-crumble".to_string()),
        source: Some("Vegetarian Cookery Bible (2012: Reader's Digest)".to_string()),
        description: None,
        tagline: None,
        notes: vec![
            "Most brands of Worcestershire Sauce contain anchovies and are therefore not vegetarian. There are some that are, however.".to_string(),
            "Honey can be added with the carrots for extra sweetness.".to_string(),
        ],
        tags: vec![
            Tag::VegetarianIsh,
            Tag::Slow,
            Tag::HighEffort,
            Tag::Scales,
            Tag::ColdWeather,
        ],
        image: None,
        ingredients_blocks: crate::recipe::IngredientsBlock::simple(vec![
            crate::recipe::Ingredient::qp("Onion", "1", "sliced"),
            crate::recipe::Ingredient::qp("Garlic", "3 minimum", "finely chopped or crushed"),
            crate::recipe::Ingredient::qp("Carrots", "3", "cut into 2cm chunks"),
            crate::recipe::Ingredient::qp("Parsnips", "2", "cut into 2cm chunks"),
            crate::recipe::Ingredient::qpn("Baby turnips", "350g", "quartered", "Optional. Never been added"),
            crate::recipe::Ingredient::qpn("New potatoes", "5-6", "cut into 2cm chunks", "Whatever number seems reasonable for the pan size"),
            crate::recipe::Ingredient::opt("Vegetable stock", Some("450ml"), None, Some("Use less for a thicker dish (preferred)")),
            crate::recipe::Ingredient::q("Worcestershire sauce", "generous dash"),
            crate::recipe::Ingredient::q("Tomato purée", "1 tbsp"),
            crate::recipe::Ingredient::opt("Bay leaves", Some("2"), None, Some("Optional. Never been added")),
            crate::recipe::Ingredient::qp("Butter beans", "1 410g tin", "drained and rinsed"),
            crate::recipe::Ingredient::opt("Thyme", Some("1-2 tsp"), None, Some("Optional")),
            crate::recipe::Ingredient::new("Salt"),
            crate::recipe::Ingredient::new("Pepper"),
            crate::recipe::Ingredient::q("Plain Flour", "85g"),
            crate::recipe::Ingredient::qpn("Butter", "30g", "diced", "Should be cool, not room temp."),
            crate::recipe::Ingredient::qpn("Mature cheddar cheese", "75g", "coarsely grated", "Always use more"),
            crate::recipe::Ingredient::q("Sunflower seeds", "30g"),
        ]),
        method: vec![
            format!("Preheat oven to {}.", crate::utils::int_utils::celsius(190)),
            "Heat oil in a large saucepan. Add the onion and garlic and cook until soft.".to_string(),
            "Add the carrots, parsnips, turnips (if used) and potatoes and cook briefly.".to_string(),
            "Stir in the stock, Worcestershire sauce, tomato purée and bay leaves (if used).".to_string(),
            "Bring to the boil and simmer for 20 minutes, stirring occasionally.".to_string(),
            "Put the flour in a mixing bowl and rub in the butter to make the crumble topping. Sprinkle over 1 1/2 tbsp water and mix with fork to make large crumbs. Stir in the cheese and sunflower seeds.".to_string(),
            "Stir the butter beans into the vegetable mixture and cook for further 5-7 minutes or until vegetables are tender.".to_string(),
            "Remove the bay leaves (if added).".to_string(),
            "Blend, mash or puree approx. 1 ladleful of the vegetable mixture.".to_string(),
            "Stir in parsley (if used) and the salt and pepper.".to_string(),
            "Pour the mixture into a suitable ovenproof dish.".to_string(),
            "Sprinkle the crumble mixture evenly over the top.".to_string(),
            "Bake for 20 minutes or until golden brown.".to_string(),
        ],
    }
}
