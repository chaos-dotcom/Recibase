//! `se.reciba.api.recipes.BeetrootRisotto`.

use crate::recipe::RecipeDef;
use crate::tag::Tag;
use chrono::NaiveDate;

pub fn recipe() -> RecipeDef {
    RecipeDef {
        object_name: "BeetrootRisotto".to_string(),
        name: "Roast Beetroot Risotto".to_string(),
        created_at: NaiveDate::from_ymd_opt(2020, 4, 24).unwrap(),
        permalink_override: Some("beetroot-risotto".to_string()),
        source: None,
        description: None,
        tagline: None,
        notes: vec!["<a href=\"https://t.sci1.uk/risotto-calculator/\">An arborio rice/water ratio calculator</a>".to_string()],
        tags: vec![
            Tag::Vegetarian,
            Tag::Slow,
            Tag::HotWeather,
        ],
        image: Some(crate::recipe::Image::new("https://i.reciba.se/beetroot-risotto.jpg")),
        ingredients_blocks: crate::recipe::IngredientsBlock::simple(vec![
            crate::recipe::Ingredient::qpn("Beetroot", "5", "peeled and cut into 1-2cm pieces", "Quantity is approximate"),
            crate::recipe::Ingredient::q("Arborio rice", "1 cup"),
            crate::recipe::Ingredient::q("White wine", "A decent slosh"),
            crate::recipe::Ingredient::new("Thyme"),
            crate::recipe::Ingredient::q("Stock cube", "1"),
            crate::recipe::Ingredient::qpn("Water", "700ml", "boiling", "might need to add more"),
            crate::recipe::Ingredient::q("Soft Goats Cheese", "up to 75g"),
        ]),
        method: vec![
            format!("Preheat the oven to {}.", crate::utils::int_utils::celsius(200)),
            "Place the peeled beetroot on a baking tray, drizzle with oil and put in the oven for 20 minutes or until soft.".to_string(),
            "If the beetroot came with stems and leaves, cut these into small pieces and set aside.".to_string(),
            "Dissolve the stock cube in the boiling water and add the wine and thyme.".to_string(),
            "Put the arborio rice in a pan over a medium heat and gradually stir in the stock, mixing often and not adding more until the previous lot of water has been absorbed.".to_string(),
            "About half-way through making the rice, add the beetroot stalks and leaves to the rice pan.".to_string(),
            "When the rice is tender, add the beetroot and mix thoroughly.".to_string(),
            "Mix in the goats cheese.".to_string(),
        ],
    }
}
