//! `se.reciba.api.recipes.SpinachTomatoChickpeaCurry`.
//! chaos-tag: casa-chaos

use crate::recipe::RecipeDef;
use crate::tag::Tag;
use chrono::NaiveDate;

pub fn recipe() -> RecipeDef {
    RecipeDef {
        object_name: "SpinachTomatoChickpeaCurry".to_string(),
        name: "Spinach, Tomato + Chickpea Curry".to_string(),
        created_at: NaiveDate::from_ymd_opt(2026, 10, 4).unwrap(),
        permalink_override: None,
        source: None,
        description: None,
        tagline: None,
        notes: vec![
            "Serves 4 as a main course".to_string(),
        ],
        tags: vec![
            Tag::Vegan,
            Tag::Spicy,
            Tag::ColdWeather,
            Tag::LowEffort,
            Tag::Scales,
        ],
        image: None,
        ingredients_blocks: crate::recipe::IngredientsBlock::simple(vec![
            crate::recipe::Ingredient::q("Rapeseed Oil", "3 tablespoons"),
            crate::recipe::Ingredient::q("Black Mustard Seeds", "1/2 teaspoon"),
            crate::recipe::Ingredient::q("Cumin Seeds", "1 teaspoon"),
            crate::recipe::Ingredient::qp("Large Onions", "2", "diced"),
            crate::recipe::Ingredient::qp("Garlic", "5 cloves", "crushed"),
            crate::recipe::Ingredient::qp("Ginger", "2cm", "peeled and grated"),
            crate::recipe::Ingredient::q("Plum Tomatoes", "1 x 400g tin"),
            crate::recipe::Ingredient::qp("Chickpeas", "2 x 400g tin", "drained"),
            crate::recipe::Ingredient::q("Ground Coriander", "1 1/2 teaspoons"),
            crate::recipe::Ingredient::q("Chilli Powder", "1 teaspoon"),
            crate::recipe::Ingredient::q("Ground Turmeric", "1/2 teaspoon"),
            crate::recipe::Ingredient::q("Salt", "1 teaspoon"),
            crate::recipe::Ingredient::qp("Baby Spinach", "500g", "washed"),
        ]),
        method: vec![
            "Put the oil into a large lidded pan over a medium heat and, when hot, add the mustard seeds and cumin seeds. Stir for a minute, or until they pop, then throw in the onions.".to_string(),
            "Fry for 10 to 12 minutes, until they turn translucent and start to caramelise, then add the garlic and ginger. Stir-fry for around 3 minutes, then add the tinned tomatoes, pouring them in with one hand and crushing them with the other. Fill the empty tin a third of the way up with water and add that to the pan too.".to_string(),
            "Cook for 10 minutes, until quite dry and paste-like, then add the chickpeas. Warm them for a couple of minutes, then add the coriander, chilli powder, turmeric and salt. Toss the chickpeas around in the paste, and add the spinach - trying to fold it all in will be like pushing a duvet into a magical handbag, but it will wilt and shrink fairly quickly.".to_string(),
            "Cook for around 5 minutes, until the spinach is soft and tender, and serve with chapattis (see page 288) or basmati rice, and a dollop of yoghurt.".to_string(),
        ],
    }
}
