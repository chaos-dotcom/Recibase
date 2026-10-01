//! `se.reciba.api.recipes.VegetarianMexican`.

use crate::recipe::RecipeDef;
use crate::tag::Tag;
use chrono::NaiveDate;

pub fn recipe() -> RecipeDef {
    RecipeDef {
        object_name: "VegetarianMexican".to_string(),
        name: "Simple Vegetarian Mexican".to_string(),
        created_at: NaiveDate::from_ymd_opt(2023, 8, 23).unwrap(),
        permalink_override: Some("vegetarian-mexican".to_string()),
        source: None,
        description: Some("Low spoons vegetarian Mexican".to_string()),
        tagline: None,
        notes: Vec::new(),
        tags: vec![Tag::Vegetarian, Tag::LowEffort, Tag::Stephani],
        image: None,
        ingredients_blocks: crate::recipe::IngredientsBlock::simple(vec![
            crate::recipe::Ingredient::opt("Black Beans", Some("2 tins"), Some("drained"), None),
            crate::recipe::Ingredient::q("Frozen sweetcorn", "260g"),
            crate::recipe::Ingredient::q("Squash", "1"),
            crate::recipe::Ingredient::opt("Garlic", None, Some("finely chopped"), None),
            crate::recipe::Ingredient::opt("Cheese", None, None, Some("ideally Sainsbury's 4 cheese mix or a herb/spiced cheese")),
            crate::recipe::Ingredient::new("Tortillas"),
            crate::recipe::Ingredient::new("Cinnamon"),
            crate::recipe::Ingredient::new("Cumin"),
            crate::recipe::Ingredient::new("Salt"),
            crate::recipe::Ingredient::opt("Soured Cream", None, None, Some("optional")),
        ]),
        method: vec![
            format!("Preheat the oven to {}.", crate::utils::int_utils::celsius(200)),
            "Peel the squash and chop into 2cm chunks.".to_string(),
            "Spread on a baking tray and mix with oil, cinnamon and cumin. Roast for 35 minutes.".to_string(),
            "Defrost the sweetcorn then combine in a large bowl with the black beans and garlic.".to_string(),
            "Once roasted, add the squash to the bowl along with the cheese. Taste and add salt or more spices.".to_string(),
            "Serve with tortillas".to_string(),
        ],
    }
}
