//! `se.reciba.api.recipes.ButternutSquashPadThai`.

use crate::recipe::RecipeDef;
use crate::tag::Tag;
use chrono::NaiveDate;

pub fn recipe() -> RecipeDef {
    RecipeDef {
        object_name: "ButternutSquashPadThai".to_string(),
        name: "Butternut Squash Pad Thai".to_string(),
        created_at: NaiveDate::from_ymd_opt(2026, 2, 1).unwrap(),
        permalink_override: Some("butternut-squash-pad-thai".to_string()),
        source: Some("Gousto".to_string()),
        description: None,
        tagline: None,
        notes: Vec::new(),
        tags: vec![
            Tag::AI,
            Tag::Spicy,
            Tag::Vegan,
            Tag::Quick,
        ],
        image: None,
        ingredients_blocks: crate::recipe::IngredientsBlock::simple(vec![
            crate::recipe::Ingredient::qp("Butternut squash", "120g", "cut into 2cm cubes"),
            crate::recipe::Ingredient::q("Lime", "1"),
            crate::recipe::Ingredient::q("Red chilli", "1"),
            crate::recipe::Ingredient::qp("Red pepper", "1", "cut into thin strips"),
            crate::recipe::Ingredient::qp("Garlic", "1 cloves", "finely sliced"),
            crate::recipe::Ingredient::q("Roasted peanuts", "25g"),
            crate::recipe::Ingredient::q("Soy sauce", "2 tbsp"),
            crate::recipe::Ingredient::q("Tamarind paste", "3 tsp"),
            crate::recipe::Ingredient::q("Toasted sesame oil", "1 tsp"),
            crate::recipe::Ingredient::q("Tenderstem broccoli", "80g"),
            crate::recipe::Ingredient::q("Red curry paste", "4 tsp"),
            crate::recipe::Ingredient::q("Sriracha hot chilli sauce", "1 tbsp"),
            crate::recipe::Ingredient::q("Fried onions", "3 tsp"),
            crate::recipe::Ingredient::q("Thai rice noodles", "200g"),
            crate::recipe::Ingredient::new("Salt"),
            crate::recipe::Ingredient::new("Sugar"),
            crate::recipe::Ingredient::new("Vegetable oil"),
        ]),
        method: vec![
            format!("Preheat the oven to {} and boil a kettle.", crate::utils::int_utils::simple_fan_instruction(200)),
            "Add the butternut squash cubes to a bowl with the red curry paste and a drizzle of vegetable oil and mix it up, then add to a tray.".to_string(),
            "Put the tray in the oven for 15-20 min or until cooked with a slight bite.".to_string(),
            "Whilst the butternut squash is cooking, add the Thai rice noodles to a bowl and cover with boiled water.".to_string(),
            "Set aside for 12-15 min or until softened.".to_string(),
            "Once softened, drain the noodles reserving a cup of starchy noodle water.".to_string(),
            "Drizzle with a little vegetable oil and set aside.".to_string(),
            "Heat a large, wide-based pan (preferably non-stick) with a generous drizzle of vegetable oil over a medium heat.".to_string(),
            "Once hot, add the sliced red peppers and sliced garlic with a pinch of salt and cook for 5-6 min or until softened.".to_string(),
            "Cut the Tenderstem broccoli in half.".to_string(),
            "Once the peppers have softened, add the halved Tenderstem broccoli with a splash of boiled water and cook for 4-5 min or until the broccoli is cooked with a slight bite.".to_string(),
            "Add the soy sauce to a bowl with the Sriracha hot chilli sauce, tamarind paste and the juice of the limes.".to_string(),
            "Add sugar and water and stir it all together.".to_string(),
            "Crush the peanuts in their bag with a rolling pin.".to_string(),
            "Slice the red chillies finely.".to_string(),
            "Once the butternut squash cubes are done, add them to the pan and reduce the heat to low.".to_string(),
            "Add the pad Thai sauce and stir through the drained noodles.".to_string(),
            "Add a splash of the reserved noodle water if it's looking a little dry.".to_string(),
            "Serve the butternut squash pad Thai and garnish with the crushed peanuts, crispy fried onions and sliced red chilli.".to_string(),
        ],
    }
}
