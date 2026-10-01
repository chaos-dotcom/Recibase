//! `se.reciba.api.recipes.CranberryRelish`.

use crate::recipe::RecipeDef;
use crate::tag::Tag;
use chrono::NaiveDate;

pub fn recipe() -> RecipeDef {
    RecipeDef {
        object_name: "CranberryRelish".to_string(),
        name: "Cranberry & Cinnamon Relish".to_string(),
        created_at: NaiveDate::from_ymd_opt(2020, 12, 29).unwrap(),
        permalink_override: Some("cranberry-relish".to_string()),
        source: Some("Kit's Mum".to_string()),
        description: None,
        tagline: None,
        notes: vec!["This is best made in advance, to give the flavours time to mix".to_string()],
        tags: vec![
            Tag::Christmas,
            Tag::NonMeal,
        ],
        image: None,
        ingredients_blocks: crate::recipe::IngredientsBlock::simple(vec![
            crate::recipe::Ingredient::qp("Red onions", "2", "thinly sliced"),
            crate::recipe::Ingredient::q("Brown sugar", "3 tbsp"),
            crate::recipe::Ingredient::q("Cranberries", "450g"),
            crate::recipe::Ingredient::q("Redcurrant jelly", "2 tbsp"),
            crate::recipe::Ingredient::q("Cinnamon stick", "1"),
            crate::recipe::Ingredient::q("Vegetable oil", "tbsp"),
        ]),
        method: vec![
            "Gently cook the onions in the oil and sugar for about 20 minutes, stirring frequently, until soft and caramelised.".to_string(),
            "Add the remaining ingredients along with 150ml boiling water.".to_string(),
            "Simmer gently for about 10 minutes until the cranberries are softened but not completely broken down.".to_string(),
            "Add additional sugar if needed, but don't overdo it. The relish should have a tart flavour.".to_string(),
            "Serve warm or cold.".to_string(),
        ],
    }
}
