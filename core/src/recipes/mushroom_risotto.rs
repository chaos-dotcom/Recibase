//! `se.reciba.api.recipes.MushroomRisotto`.

use crate::recipe::RecipeDef;
use crate::tag::Tag;
use chrono::NaiveDate;

pub fn recipe() -> RecipeDef {
    RecipeDef {
        object_name: "MushroomRisotto".to_string(),
        name: "Mushroom Risotto".to_string(),
        created_at: NaiveDate::from_ymd_opt(2020, 4, 24).unwrap(),
        permalink_override: None,
        source: None,
        description: None,
        tagline: None,
        notes: Vec::new(),
        tags: vec![Tag::Vegetarian, Tag::LowEffort, Tag::Quick],
        image: None,
        ingredients_blocks: crate::recipe::IngredientsBlock::simple(vec![
            crate::recipe::Ingredient::opt("Chestnut mushrooms", Some("250g"), Some("sliced"), None),
            crate::recipe::Ingredient::opt("Dried mushrooms", Some("handful"), None, Some("Porcini, shiitake, wild, etc")),
            crate::recipe::Ingredient::q("Arborio rice", "1 cup"),
            crate::recipe::Ingredient::q("White wine", "A decent slosh"),
            crate::recipe::Ingredient::new("Black pepper"),
            crate::recipe::Ingredient::q("Stock cube", "1"),
            crate::recipe::Ingredient::opt("Water", Some("700ml"), Some("boiling"), Some("Might need to add more")),
            crate::recipe::Ingredient::q("Stilton", "100g"),
            crate::recipe::Ingredient::q("Butter", "knob"),
        ]),
        method: vec![
            "Dissolve the stock cube in the boiling water and add the dried mushrooms, wine and black pepper.".to_string(),
            "Melt the butter in a wide pan and cook the mushrooms over a medium heat.".to_string(),
            "Remove from the pan and set aside.".to_string(),
            "Add the arborio rice to your pan with a small amount of butter and toast for 30 seconds.".to_string(),
            "Gradually stir in the stock, mixing often. Only add more stock when the rice has absorbed the previous lot of stock.".to_string(),
            "When the rice is tender, add the mushrooms and mix thoroughly.".to_string(),
            "Mix in the stilton and allow it to melt through the risotto.".to_string(),
        ],
    }
}
