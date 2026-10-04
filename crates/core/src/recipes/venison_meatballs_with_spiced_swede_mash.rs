//! `se.reciba.api.recipes.VenisonMeatballsWithSpicedSwedeMash`.
//! chaos-tag: casa-chaos

use crate::recipe::RecipeDef;
use crate::tag::Tag;
use chrono::NaiveDate;

pub fn recipe() -> RecipeDef {
    RecipeDef {
        object_name: "VenisonMeatballsWithSpicedSwedeMash".to_string(),
        name: "Venison Meatballs with Spiced Swede Mash".to_string(),
        created_at: NaiveDate::from_ymd_opt(2026, 10, 4).unwrap(),
        permalink_override: None,
        source: Some("Waitrose".to_string()),
        description: Some("Ready-made, lean meatballs make an easy dish to serve over the festive period.".to_string()),
        tagline: None,
        notes: vec![
            "Serves 4".to_string(),
            "Prepare 15 minutes; cook 35 minutes.".to_string(),
            "Level up: both the meatballs and the swede mash can be made up to 24 hours in advance and reheated to serve. You may need to add a splash of water to the meatballs to loosen the sauce.".to_string(),
            "Per serving: 1728kJ/415kcals/25g fat/11g saturated fat/20g carbs/16g sugars/8.8g fibre/22g protein/1.3g salt.".to_string(),
        ],
        tags: vec![
            Tag::VegetarianIsh,
            Tag::Stodge,
            Tag::ColdWeather,
            Tag::Scales,
        ],
        image: None,
        ingredients_blocks: crate::recipe::IngredientsBlock::simple(vec![
            crate::recipe::Ingredient::q("Olive Oil", "2 tbsp"),
            crate::recipe::Ingredient::q("British Venison Meatballs", "300g pack 20"),
            crate::recipe::Ingredient::qp("Onion", "1", "sliced"),
            crate::recipe::Ingredient::qp("Garlic", "1 clove", "finely chopped"),
            crate::recipe::Ingredient::qp("British Woodland Mushrooms", "200g pack", "sliced"),
            crate::recipe::Ingredient::qpn("Swede", "1 large", "peeled and cut into 2cm chunks", "about 900g"),
            crate::recipe::Ingredient::q("Ground Allspice", "3/4 tsp"),
            crate::recipe::Ingredient::q("Plain Flour", "1/2 tbsp"),
            crate::recipe::Ingredient::q("Fresh Chicken Stock", "200ml"),
            crate::recipe::Ingredient::q("Unsalted Butter", "40g"),
            crate::recipe::Ingredient::q("Single Cream", "3 tbsp"),
            crate::recipe::Ingredient::qp("Flat Leaf Parsley", "Handful", "roughly chopped"),
        ]),
        method: vec![
            "Heat the oil in a large frying pan over a medium-high heat. Fry the meatballs for 6-8 minutes until browned all over, then scoop out of the pan. Add the onion and fry for 2 minutes, then add the garlic and mushrooms. Fry for 8-10 minutes, stirring regularly, until soft and turning golden.".to_string(),
            "Meanwhile, put the swede in a large saucepan. Cover with cold water and a pinch of salt and set over a high heat. Bring to the boil, then simmer for about 20 minutes or until tender.".to_string(),
            "Stir the allspice and flour into the mushroom mixture, cook for 1 minute, then stir in the stock. Return the meatballs to the pan, lower to a simmer and cook for 10 minutes or until no pink meat remains and the juices run clear. Meanwhile, drain the swede, let it steam dry for a few minutes, then tip back into the pan with the butter. Season, then mash.".to_string(),
            "Stir the cream and half the chopped parsley through the meatballs, then scatter with the remaining parsley and season.".to_string(),
            "Serve with the swede mash.".to_string(),
        ],
    }
}
