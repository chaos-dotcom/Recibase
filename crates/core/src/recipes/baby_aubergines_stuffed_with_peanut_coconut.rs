//! `se.reciba.api.recipes.BabyAuberginesStuffedWithPeanutCoconut`.
//! chaos-tag: casa-chaos

use crate::recipe::RecipeDef;
use crate::tag::Tag;
use chrono::NaiveDate;

pub fn recipe() -> RecipeDef {
    RecipeDef {
        object_name: "BabyAuberginesStuffedWithPeanutCoconut".to_string(),
        name: "Baby Aubergines Stuffed with Peanut + Coconut".to_string(),
        created_at: NaiveDate::from_ymd_opt(2026, 10, 4).unwrap(),
        permalink_override: None,
        source: None,
        description: None,
        tagline: None,
        notes: vec![
            "Serves 4 as a main course".to_string(),
            "Note: You will need a food processor for this recipe.".to_string(),
        ],
        tags: vec![
            Tag::Vegan,
            Tag::GlutenFree,
            Tag::ColdWeather,
            Tag::Scales,
        ],
        image: None,
        ingredients_blocks: crate::recipe::IngredientsBlock::simple(vec![
            crate::recipe::Ingredient::q("Baby Aubergines", "12 (800g)"),
            crate::recipe::Ingredient::q("Desiccated or Fresh Grated Coconut", "60g"),
            crate::recipe::Ingredient::q("Roasted Unsalted Peanuts", "120g"),
            crate::recipe::Ingredient::q("Fresh Coriander", "40g"),
            crate::recipe::Ingredient::q("Garlic", "8 cloves"),
            crate::recipe::Ingredient::q("Green Finger Chilli", "1"),
            crate::recipe::Ingredient::q("Tomato Puree", "2 tablespoons"),
            crate::recipe::Ingredient::q("Ground Cumin", "1 teaspoon"),
            crate::recipe::Ingredient::q("Ground Turmeric", "3/4 teaspoon"),
            crate::recipe::Ingredient::q("Sugar", "1 teaspoon"),
            crate::recipe::Ingredient::q("Salt", "1 3/4 teaspoons"),
            crate::recipe::Ingredient::q("Rapeseed Oil", "3 tablespoons"),
            crate::recipe::Ingredient::qp("Onion", "1 large", "sliced"),
        ]),
        method: vec![
            "Cut each aubergine in half lengthways, but don't cut through the stem. Roll each one over and cut lengthways again, still keeping the stem intact. Put into a bowl of cold water and set aside.".to_string(),
            "Put a large lidded frying pan over a medium heat and, when hot, toast the coconut and peanuts for 2 to 3 minutes, until the coconut is starting to brown. Tip into a bowl and leave to cool. Put the coriander, garlic, green chilli, tomato puree, cumin, turmeric, sugar and salt into a food processor, along with the cooled peanuts and coconut. Pulse until coarsely ground and fully mixed.".to_string(),
            "Open each aubergine out like a flower and fill with the coconut mixture, using your hands. Roll the aubergine over, open and stuff again, then press closed. If there's any leftover stuffing, add it to the pan later, when you cook the aubergines.".to_string(),
            "Next, put the oil into the frying pan over a medium heat. When hot, add the onion and fry until golden and soft. Add the aubergines and 2 tablespoons of water, turn the heat up high and cook for a couple of minutes, then put the lid on and turn the heat down. Cook for 10 minutes, then gently turn the aubergines and add a splash of water if they're looking dry. Cook for a further 20 minutes, or until nice and tender. Serve with cucumber and mint raita (see page 247), or with a salad, some yoghurt and rice or chapattis (page 288).".to_string(),
        ],
    }
}
