//! `se.reciba.api.recipes.DukatenCookies`.

use crate::recipe::RecipeDef;
use crate::tag::Tag;
use chrono::NaiveDate;

pub fn recipe() -> RecipeDef {
    RecipeDef {
        object_name: "DukatenCookies".to_string(),
        name: "Dukaten Cookies".to_string(),
        created_at: NaiveDate::from_ymd_opt(2024, 12, 24).unwrap(),
        permalink_override: None,
        source: Some("Stephani".to_string()),
        description: Some("Dukatenplätzchen, rum flavoured German Christmas cookies".to_string()),
        tagline: None,
        notes: vec!["If you don't have a small enough biscuit cutter you can use some wider shot glasses. The cookies should have a 4cm diameter.".to_string()],
        tags: vec![
            Tag::Pudding,
            Tag::Baking,
        ],
        image: Some(crate::recipe::Image::new("https://i.reciba.se/dukaten-cookies.jpg")),
        ingredients_blocks: vec![
            crate::recipe::IngredientsBlock::new(Some("Biscuits"), vec![
                crate::recipe::Ingredient::q("Plain Flour", "250g"),
                crate::recipe::Ingredient::q("Baking Powder", "1tsp"),
                crate::recipe::Ingredient::q("Sugar", "75g"),
                crate::recipe::Ingredient::q("Vanilla Extract", "1/2 tsp"),
                crate::recipe::Ingredient::q("Egg", "1"),
                crate::recipe::Ingredient::opt("Milk", Some("1 tbsp"), None, Some("or more if the mixture is too dry")),
                crate::recipe::Ingredient::opt("Butter", Some("125g"), None, Some("or margarine")),
            ]),
            crate::recipe::IngredientsBlock::new(Some("Filling"), vec![
                crate::recipe::Ingredient::q("Butter", "125g"),
                crate::recipe::Ingredient::q("Icing Sugar", "130g"),
                crate::recipe::Ingredient::q("Cocoa", "1 heaped tablespoon"),
                crate::recipe::Ingredient::q("Rum flavouring", "1 top"),
                crate::recipe::Ingredient::q("Egg", "1"),
            ]),
            crate::recipe::IngredientsBlock::new(Some("Frosting"), vec![
                crate::recipe::Ingredient::q("Powdered Sugar", "100g"),
                crate::recipe::Ingredient::opt("Cocoa", Some("1 heaped tablespoon"), None, Some("not a measuring spoon")),
                crate::recipe::Ingredient::qp("Water", "1-2 tbsp", "hot"),
                crate::recipe::Ingredient::qp("Butter", "28g", "melted"),
            ]),
        ],
        method: vec![
            format!("Pre-heat the oven to {}", crate::utils::int_utils::celsius(190)),
            "Rub a large baking tray with butter or margarine.".to_string(),
            "Combine all the biscuit ingredients in order.".to_string(),
            "Mix together then knead until smooth. If the mixture is too soft then put it in the fridge or freezer for a moment.".to_string(),
            "Roll out thin and cut into 4cm circles.".to_string(),
            "Lay out on the baking tray and bake for 10 minutes or until a light golden colour.".to_string(),
            "Put to one side to cool.".to_string(),
            "While the biscuits are baking you can also make the filling and frosting:".to_string(),
            "Make the filling by combining all the ingredients with a food mixer.".to_string(),
            "Make the frosting by mixing all the ingredients until smooth".to_string(),
            "Once the biscuits are cool lay out half of them on a sheet of baking paper. Leave room between the biscuits and the edge for the frosting to ooze.".to_string(),
            "Place a drop of the filling on top of each biscuit then flatten the mixture by placing down another biscuit on top.".to_string(),
            "Once all the biscuits are filled drizzle the frosting over one side of each biscuit.".to_string(),
            "Transfer the biscuits to the fridge to set the frosting and filling.".to_string(),
        ],
    }
}
