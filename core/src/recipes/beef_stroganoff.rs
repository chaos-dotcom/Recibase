//! `se.reciba.api.recipes.BeefStroganoff`.

use crate::recipe::RecipeDef;
use crate::tag::Tag;
use chrono::NaiveDate;

pub fn recipe() -> RecipeDef {
    RecipeDef {
        object_name: "BeefStroganoff".to_string(),
        name: "Beef Stroganoff".to_string(),
        created_at: NaiveDate::from_ymd_opt(2020, 4, 24).unwrap(),
        permalink_override: None,
        source: Some("Kit's Dad".to_string()),
        description: None,
        tagline: None,
        notes: Vec::new(),
        tags: vec![
            Tag::Scales,
            Tag::Slow,
        ],
        image: None,
        ingredients_blocks: crate::recipe::IngredientsBlock::simple(vec![
            crate::recipe::Ingredient::opt("Beef", Some("500g"), None, Some("Not casserole (too tough) or fillet (too expensive)")),
            crate::recipe::Ingredient::q("Garlic Clove", "1"),
            crate::recipe::Ingredient::q("Onion", "1"),
            crate::recipe::Ingredient::q("Mushrooms", "5-6"),
            crate::recipe::Ingredient::q("Soured Cream", "350ml"),
            crate::recipe::Ingredient::opt("Beef or Pork Stock Cube", None, None, Some("Or 1 tbsp Bovril")),
            crate::recipe::Ingredient::q("Salt", "Pinch"),
            crate::recipe::Ingredient::q("Pepper", "Pinch"),
            crate::recipe::Ingredient::q("Paprika", "Pinch"),
            crate::recipe::Ingredient::q("Butter", "Wedge"),
        ]),
        method: vec![
            "Cover meat with clingfilm and hit with a rolling pin to flatten it.".to_string(),
            "Cut into thin strips and sprinkle with salt, pepper and paprika.".to_string(),
            "Slice mushrooms and place them to one side.".to_string(),
            "Slice the onion and crush the garlic then brown in melted butter.".to_string(),
            "Once softened take out and put on a plate to one side.".to_string(),
            "Brown the meat in batches then add the mushrooms and the Bovril or stock cube, diluted in a cup of hot water.".to_string(),
            "Put the onions back and add the soured cream then bring the mixture to just under boiling.".to_string(),
            "Simmer for 40 minutes then serve with rice or pasta, ideally rice.".to_string(),
        ],
    }
}
