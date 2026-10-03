//! `se.reciba.api.recipes.TangyVegetablePadThai`.

use crate::recipe::RecipeDef;
use crate::tag::Tag;
use chrono::NaiveDate;

pub fn recipe() -> RecipeDef {
    RecipeDef {
        object_name: "TangyVegetablePadThai".to_string(),
        name: "Tangy Vegetable Pad Thai".to_string(),
        created_at: NaiveDate::from_ymd_opt(2020, 4, 24).unwrap(),
        permalink_override: None,
        source: Some("Gousto".to_string()),
        description: Some("Courgette, pepper and rice noodles in a zingy pad Thai sauce.".to_string()),
        tagline: None,
        notes: Vec::new(),
        tags: vec![Tag::LowEffort, Tag::Quick, Tag::Vegan],
        image: None,
        ingredients_blocks: crate::recipe::IngredientsBlock::simple(vec![
            crate::recipe::Ingredient::q("Butternut squash cubes", "160g"),
            crate::recipe::Ingredient::q("Rice noodles", "150g"),
            crate::recipe::Ingredient::qp("Red pepper", "1", "deseeded and cut into thin strips"),
            crate::recipe::Ingredient::q("Courgette", "1"),
            crate::recipe::Ingredient::qp("Garlic", "2 cloves", "finely sliced"),
            crate::recipe::Ingredient::q("Lime", "1"),
            crate::recipe::Ingredient::q("Tamarind paste", "15g"),
            crate::recipe::Ingredient::q("Soy sauce", "30ml"),
            crate::recipe::Ingredient::q("Mirin", "15ml"),
            crate::recipe::Ingredient::q("Sriracha hot chilli sauce", "8ml"),
            crate::recipe::Ingredient::q("Thai basil", "5g"),
            crate::recipe::Ingredient::q("Coriander", "5g"),
            crate::recipe::Ingredient::q("Roasted peanuts", "25g"),
            crate::recipe::Ingredient::new("Salt"),
            crate::recipe::Ingredient::new("Pepper"),
            crate::recipe::Ingredient::new("Sugar"),
            crate::recipe::Ingredient::new("Vegetable oil"),
        ]),
        method: vec![
            format!("Preheat the oven to {} and boil a kettle.", crate::utils::int_utils::simple_fan_instruction(200)),
            "Add the butternut squash cubes to a baking tray with a pinch of salt and pepper and a drizzle of vegetable oil and mix well.".to_string(),
            "Roast for 20-25 min or until cooked with a slight bite.".to_string(),
            "Heat a large, wide-based pan (preferably non-stick) with a generous drizzle of vegetable oil over a medium heat.".to_string(),
            "Once hot, add the pepper strips with a pinch of salt and cook for 3-4 min or until softened.".to_string(),
            "Add the rice noodles to a pot and cover with boiled water.".to_string(),
            "Boil over a high heat for 4-5 min or until softened with a slight bite.".to_string(),
            "Drain the noodles, run them under cold water, then return to the pot with a drizzle of vegetable oil and set aside.".to_string(),
            "Peel lengths off the courgette until you have a pile of ribbons.".to_string(),
            "Add the tamarind paste to a bowl with the soy sauce, mirin, sriracha, the juice of 1/2 the lime and 1 tsp sugar and mix.".to_string(),
            "Once the pepper has softened, add the courgette ribbons and sliced garlic to the pan and cook for 1-2 min until fragrant, then remove from the heat.".to_string(),
            "Crush the roasted peanuts with a rolling pin.".to_string(),
            "Roughly chop the Thai basil and coriander, including the stalks.".to_string(),
            "Cut the remaining lime into wedges.".to_string(),
            "Return the pan to a medium-high heat and add the drained noodles, roasted butternut squash and pad Thai sauce and mix well.".to_string(),
            "Stir through half the chopped Thai basil and coriander.".to_string(),
            "Serve topped with the remaining herbs, crushed peanuts and a lime wedge.".to_string(),
        ],
    }
}
