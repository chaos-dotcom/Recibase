//! `se.reciba.api.recipes.ParsnipLentilLasagne`.

use crate::recipe::RecipeDef;
use crate::tag::Tag;
use chrono::NaiveDate;

pub fn recipe() -> RecipeDef {
    RecipeDef {
        object_name: "ParsnipLentilLasagne".to_string(),
        name: "Parsnip and Lentil Lasagne".to_string(),
        created_at: NaiveDate::from_ymd_opt(2020, 4, 24).unwrap(),
        permalink_override: Some("parsnip-and-lentil-lasagne".to_string()),
        source: None,
        description: None,
        tagline: None,
        notes: Vec::new(),
        tags: vec![
            Tag::Slow,
            Tag::HighEffort,
            Tag::Scales,
            Tag::Vegetarian,
        ],
        image: None,
        ingredients_blocks: crate::recipe::IngredientsBlock::simple(vec![
            crate::recipe::Ingredient::opt("Lasagne sheets", Some("200g"), None, Some("12 sheets")),
            crate::recipe::Ingredient::opt("Soft Goats cheese", Some("150g"), None, Some("or ricotta")),
            crate::recipe::Ingredient::q("Feta", "200g"),
            crate::recipe::Ingredient::opt("Milk", Some("100ml"), None, Some("Full fat or semi-skimmed")),
            crate::recipe::Ingredient::qp("Parsnips", "400g", "sliced into sticks"),
            crate::recipe::Ingredient::qp("Red Onion", "1 large", "thinly sliced"),
            crate::recipe::Ingredient::q("Red lentils", "100g"),
            crate::recipe::Ingredient::qp("Red peppers", "2 large", "diced"),
            crate::recipe::Ingredient::qp("Carrot", "1 large", "thinly sliced"),
            crate::recipe::Ingredient::q("Vegetable Stock", "300ml"),
            crate::recipe::Ingredient::q("Passata", "250ml"),
            crate::recipe::Ingredient::opt("Kidney Beans", Some("2 400g tins"), Some("drained and rinsed"), None),
            crate::recipe::Ingredient::opt("Sunflower Oil", None, None, Some("or olive oil")),
            crate::recipe::Ingredient::new("Bay Leaf"),
            crate::recipe::Ingredient::new("Salt"),
            crate::recipe::Ingredient::new("Pepper"),
            crate::recipe::Ingredient::opt("Nutmeg", None, Some("grated"), Some("Optional")),
        ]),
        method: vec![
            format!("Preheat the oven the {}.", crate::utils::int_utils::celsius(190)),
            "Heat the oil in a large saucepan.".to_string(),
            "Add the onion and soften for about 10 minutes.".to_string(),
            "Add the lentils, red peppers, carrot, vegetable stock, bay leaf and pasatta.".to_string(),
            "Bring the mixture to the boil, then reduce the heat and simmer for 25 minutes or until the lentils/vegetables are soft.".to_string(),
            "Oil and season the parsnips then roast in the oven for about 25 minutes.".to_string(),
            "Remove the bay leaf and partially puree with a hand blender.".to_string(),
            "Season with salt/pepper to taste then mix in the beans.".to_string(),
            "Spoon 1/4 of the sauce over the bottom of a large greaseproof dish then cover with lasagne sheets.".to_string(),
            "Add two more layers like so: spread half the sauce on top of the lasagne sheets, arrange half the roast parsnips in the sauce then finally cover with lasagne sheets.".to_string(),
            "Put the goats cheese in a bowl and stir in the milk until smooth. If the mixture is too firm/cold give it a second or two in the microwave. Season with pepper.".to_string(),
            "Spoon the goats cheese mixture over the dish then scatter with crumbled feta and nutmeg.".to_string(),
            "Bake for 40 minutes or until bubbly and golden.".to_string(),
            "Take out of the oven and leave to stand for 5 minutes before serving.".to_string(),
        ],
    }
}
