//! `se.reciba.api.recipes.SquashSweetPotatoLentilStew`.
//! chaos-tag: casa-chaos

use crate::recipe::RecipeDef;
use crate::tag::Tag;
use chrono::NaiveDate;

pub fn recipe() -> RecipeDef {
    RecipeDef {
        object_name: "SquashSweetPotatoLentilStew".to_string(),
        name: "Squash, Sweet Potato & Lentil Stew".to_string(),
        created_at: NaiveDate::from_ymd_opt(2026, 10, 3).unwrap(),
        permalink_override: None,
        source: Some("Waitrose".to_string()),
        description: None,
        tagline: None,
        notes: vec![
            "Serves 4".to_string(),
            "Prepare 15 minutes; cook 35 minutes.".to_string(),
            "Good Health: 7 plant varieties / low fat / high in fibre / source of protein.".to_string(),
            "Per serving: 1133kJ/270kcals/8.5g fat/1.3g saturated fat/34g carbs/17.1g sugars/10.5g fibre/9.1g protein/2.6g salt; vegan, 4 of your 5 a day.".to_string(),
        ],
        tags: vec![
            Tag::Vegan,
            Tag::ColdWeather,
            Tag::LowEffort,
            Tag::Scales,
            Tag::BetterNextDay,
        ],
        image: None,
        ingredients_blocks: crate::recipe::IngredientsBlock::simple(vec![
            crate::recipe::Ingredient::q("Cooks' Ingredients Sweet Potato & Butternut Squash", "2 x 350g packs"),
            crate::recipe::Ingredient::opt("Sundried Tomatoes", Some("6"), Some("chopped"), Some("In oil, plus 2 tbsp of the oil")),
            crate::recipe::Ingredient::qp("Onion", "1", "finely chopped"),
            crate::recipe::Ingredient::q("Salt", "1/2 tsp"),
            crate::recipe::Ingredient::opt("Cavolo Nero", Some("200g pack"), None, Some("Stalks finely sliced, leaves shredded")),
            crate::recipe::Ingredient::qp("Garlic", "2 cloves", "finely chopped"),
            crate::recipe::Ingredient::qp("Celery", "1 stalk", "finely chopped"),
            crate::recipe::Ingredient::q("Ground Coriander", "1 tsp"),
            crate::recipe::Ingredient::opt("Lentils", Some("400g can"), None, Some("Drained and rinsed")),
            crate::recipe::Ingredient::q("Vegan Stock", "750ml"),
        ]),
        method: vec![
            "Preheat the oven to 220°C, gas mark 7. On a large baking sheet, toss the sweet potato and squash with 1 tbsp oil from the sundried tomato jar. Season, spread out evenly and roast for 35 minutes, stirring halfway.".to_string(),
            "Meanwhile, heat another 1 tbsp oil from the sundried tomato jar in a large saucepan over a medium-high heat. Fry the onion and salt for 2-3 minutes. Add the cavolo nero stalks, garlic and celery. Fry, stirring regularly, for 10 minutes.".to_string(),
            "Add the sundried tomatoes and ground coriander; fry for 2 minutes. Stir in the lentils and stock, then simmer for 5 minutes. Add the shredded cavolo nero and cook for a final 5 minutes. Take off the heat and tumble in the roasted sweet potato and squash. Divide between bowls and top with a little grated cheese or cheese alternative, if liked.".to_string(),
        ],
    }
}
