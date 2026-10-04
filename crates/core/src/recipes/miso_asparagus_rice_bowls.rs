//! `se.reciba.api.recipes.MisoAsparagusRiceBowls`.
//! chaos-tag: casa-chaos

use crate::recipe::RecipeDef;
use crate::tag::Tag;
use chrono::NaiveDate;

pub fn recipe() -> RecipeDef {
    RecipeDef {
        object_name: "MisoAsparagusRiceBowls".to_string(),
        name: "Miso Asparagus Rice Bowls".to_string(),
        created_at: NaiveDate::from_ymd_opt(2026, 10, 3).unwrap(),
        permalink_override: None,
        source: None,
        description: None,
        tagline: None,
        notes: vec![
            "Serves 2".to_string(),
            "Prepare 10 minutes; cook 10 minutes.".to_string(),
            "Cook's tip: white miso. Miso is a fermented soya-bean paste that's frequently found in Japanese cooking. Rich in umami, it can be used for miso soup, added to marinades or blended with butter to serve on grilled vegetables.".to_string(),
            "Good Health: low in sat fat / source of protein.".to_string(),
            "Per serving (not including optional ingredients): 1836kJ/437kcals/14.3g fat/2g saturated fat/56.1g carbs/8.5g sugars/6.9g fibre/17.6g protein/2.1g salt; 1 of your 5 a day; vegan.".to_string(),
        ],
        tags: vec![
            Tag::Vegan,
            Tag::Quick,
            Tag::LowEffort,
            Tag::Lunch,
        ],
        image: None,
        ingredients_blocks: crate::recipe::IngredientsBlock::simple(vec![
            crate::recipe::Ingredient::q("Sunflower or Vegetable Oil", "1 1/2 tbsp"),
            crate::recipe::Ingredient::q("White Miso Paste", "1 tbsp"),
            crate::recipe::Ingredient::q("Lemon Juice", "1 tbsp"),
            crate::recipe::Ingredient::q("Maple Syrup or Agave Nectar", "1 tsp"),
            crate::recipe::Ingredient::qp("Ginger", "About 5g", "peeled and finely grated"),
            crate::recipe::Ingredient::q("Jumbo Asparagus", "450g pack"),
            crate::recipe::Ingredient::opt("Kenji Sushi Rice", Some("250g pack"), None, Some("Microwaveable")),
            crate::recipe::Ingredient::qp("Organic Silken Tofu", "150g pack", "cut into cubes"),
            crate::recipe::Ingredient::opt("Shichimi Togarashi", None, None, Some("Or sesame seeds, with Cooks' Ingredients Crispy Fried Onions, to serve (optional)")),
        ]),
        method: vec![
            "In a small bowl, whisk together 1 tbsp of the miso, lemon juice, maple syrup (or agave nectar) and the oil. Add a splash of water to loosen, if needed, then stir in the ginger.".to_string(),
            "Set a large frying pan over a high heat. Toss the asparagus with the rest of the oil, then add to the smoking-hot pan. Leave for a minute, then cook for 5-8 minutes, tossing from time to time until charred and tender, season with salt. Meanwhile, heat the rice according to pack instructions.".to_string(),
            "Spoon the rice into bowls, top with the tofu and asparagus, then drizzle with the miso-ginger dressing. Finish with shichimi togarashi (or toasted sesame seeds) and crispy fried onions, if liked.".to_string(),
        ],
    }
}
