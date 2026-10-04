//! `se.reciba.api.recipes.CoalSmokedAubergineCurry`.
//! chaos-tag: casa-chaos

use crate::recipe::RecipeDef;
use crate::tag::Tag;
use chrono::NaiveDate;

pub fn recipe() -> RecipeDef {
    RecipeDef {
        object_name: "CoalSmokedAubergineCurry".to_string(),
        name: "Coal-Smoked Aubergine Curry".to_string(),
        created_at: NaiveDate::from_ymd_opt(2026, 10, 4).unwrap(),
        permalink_override: None,
        source: None,
        description: None,
        tagline: None,
        notes: vec![
            "Serves 4 to 6 as a side".to_string(),
            "In order to cook this dish safely, it's best to smoke it outside - i.e. when you've burned the coal, place it in the pan and carry it outside, so that no smoke enters your house. Only ever use natural lumpwood charcoal to smoke with, as it's a natural form of wood charcoal.".to_string(),
        ],
        tags: vec![
            Tag::Vegan,
            Tag::GlutenFree,
            Tag::Spicy,
            Tag::ColdWeather,
            Tag::Scales,
        ],
        image: None,
        ingredients_blocks: crate::recipe::IngredientsBlock::simple(vec![
            crate::recipe::Ingredient::opt("Rapeseed Oil", Some("4 tablespoons"), None, Some("Plus 1 teaspoon for smoking")),
            crate::recipe::Ingredient::qp("White Onions", "2 medium", "sliced"),
            crate::recipe::Ingredient::qp("Garlic", "5 cloves", "crushed"),
            crate::recipe::Ingredient::qp("Ginger", "3cm", "peeled and grated"),
            crate::recipe::Ingredient::qp("Aubergines", "3 medium (900g)", "cut into 5cm x 2cm batons"),
            crate::recipe::Ingredient::qp("Ripe Tomatoes", "4 large", "cut into wedges"),
            crate::recipe::Ingredient::q("Chilli Powder", "1 1/4 teaspoons"),
            crate::recipe::Ingredient::q("Salt", "1 teaspoon"),
            crate::recipe::Ingredient::q("Ground Turmeric", "1/3 teaspoon"),
            crate::recipe::Ingredient::q("Ground Coriander", "1 teaspoon"),
            crate::recipe::Ingredient::q("Ground Cumin", "1 teaspoon"),
            crate::recipe::Ingredient::opt("Charcoal", None, None, Some("A piece, around 2cm x 2cm. Chaos Note, I used Liquid Smoke")),
        ]),
        method: vec![
            "Put the 4 tablespoons of oil into a large lidded frying pan over a medium heat. When hot, add the onions and fry for around 10 minutes, until soft and beginning to brown. Add the garlic and ginger and fry for 2 to 3 minutes, until the raw smell of the garlic disappears.".to_string(),
            "Next add the aubergines, along with 6 tablespoons of water, stir and pop the lid on the pan. Cook for around 15 minutes, until the aubergine pieces have collapsed, stirring very occasionally. Add the tomato wedges, chilli powder, salt, turmeric, coriander and cumin, cook for 3 to 4 minutes with the lid off, until the tomatoes become jammy around the edges, then take the pan off the heat.".to_string(),
            "To smoke the curry, place a little heatproof bowl in the centre of the pan. Hold the charcoal in a pair of tongs over a small flame until the edges burn white and red. Then place it carefully in the small bowl, put the lid over the pan and carefully carry outdoors, along with the oil for smoking and a pair of tongs. Place the pan down, open the lid, pour the teaspoon of oil over the hot coal and close the lid again to trap the smoke. For a subtle smoky flavour, smoke the curry for 1 minute. For a nicely smoked flavour, smoke for 2 minutes. Remove the bowl using the tongs and run it under a tap to extinguish the coal.".to_string(),
            "Taste the curry for chilli and salt. Adjust if need be, then serve with rice or buttery naan bread and yoghurt.".to_string(),
        ],
    }
}
