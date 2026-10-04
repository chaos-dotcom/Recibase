//! `se.reciba.api.recipes.SimpleVegetableStirFry`.
//! chaos-tag: casa-chaos

use crate::recipe::RecipeDef;
use crate::tag::Tag;
use chrono::NaiveDate;

pub fn recipe() -> RecipeDef {
    RecipeDef {
        object_name: "SimpleVegetableStirFry".to_string(),
        name: "Simple Vegetable Stir Fry".to_string(),
        created_at: NaiveDate::from_ymd_opt(2026, 10, 3).unwrap(),
        permalink_override: None,
        source: None,
        description: None,
        tagline: None,
        notes: vec![
            "Serves 2".to_string(),
            "Prepare 10 minutes; cook 10 minutes.".to_string(),
            "Good Health: low in sat fat / high in protein / high in vitamin B12 / 14 plant varieties.".to_string(),
            "Per serving: 2187kJ/522kcals/20g fat/1.9g saturated fat/58g carbs/7.5g sugars/8.1g fibre/21g protein/2.3g salt/0.6µg B12; 1 of your 5 a day, vegan.".to_string(),
            "Leftovers: ketjap manis. This sweetened soy sauce originates from Indonesia, where it's used in dishes such as nasi goreng. The addition of the sugar (traditionally jaggery or palm sugar) gives a thick, syrupy texture. Add it to stir fries or marinades for richness and depth.".to_string(),
        ],
        tags: vec![Tag::Vegan, Tag::Quick, Tag::LowEffort],
        image: None,
        ingredients_blocks: crate::recipe::IngredientsBlock::simple(vec![
            crate::recipe::Ingredient::q("Cooks' Ingredients Ketjap Manis", "1 tbsp"),
            crate::recipe::Ingredient::q("Soy Sauce", "1 tbsp"),
            crate::recipe::Ingredient::q("Cooks' Ingredients Shaoxing Rice Wine", "1 1/2 tbsp"),
            crate::recipe::Ingredient::q("Vegetable or Sunflower Oil", "2 tbsp"),
            crate::recipe::Ingredient::qp("THIS is Super Superfood Superblock", "150g", "cut into 1cm cubes"),
            crate::recipe::Ingredient::qp("Garlic", "2 cloves", "finely sliced"),
            crate::recipe::Ingredient::qp("Ginger", "20g", "finely grated"),
            crate::recipe::Ingredient::q("Sweet & Tender Vegetable Stir Fry", "210g pack"),
            crate::recipe::Ingredient::opt("Cooked White Rice", Some("300g"), None, Some("To serve")),
        ]),
        method: vec![
            "In a small bowl, mix the ketjap manis, soy sauce and Shaoxing rice wine; set aside.".to_string(),
            "Heat the oil in a wok or large frying pan over a high heat. When hot, add the Super Superfood cubes and stir fry for 2-3 minutes until golden in places. Add the garlic, ginger and red onion from the stir-fry pack and fry for 1 minute.".to_string(),
            "Add the remaining vegetables from the pack and stir fry for 1-2 minutes, then add the sauce and cook for another 1-2 minutes until all the vegetables are tender and the cubes are hot throughout. Serve immediately with the rice.".to_string(),
        ],
    }
}
