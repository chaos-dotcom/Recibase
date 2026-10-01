//! `se.reciba.api.recipes.Mead`.

use crate::recipe::RecipeDef;
use crate::tag::Tag;
use chrono::NaiveDate;

pub fn recipe() -> RecipeDef {
    RecipeDef {
        object_name: "Mead".to_string(),
        name: "Mead".to_string(),
        created_at: NaiveDate::from_ymd_opt(2020, 7, 25).unwrap(),
        permalink_override: None,
        source: Some("https://www.diynatural.com/homemade-mead-honey-mead-recipe/".to_string()),
        description: None,
        tagline: Some("A medieval classic".to_string()),
        notes: vec![
            "This recipe requires a large pan, 2 ~1L flip top bottles, funnel, airlock and rubber bung.Sterilise all equipment before use to avoid comtaminating your mead.".to_string(),
            "I'd recommend doubling or tripling the quantities to make this worth the effort.".to_string(),
        ],
        tags: vec![Tag::NonMeal],
        image: None,
        ingredients_blocks: crate::recipe::IngredientsBlock::simple(vec![
            crate::recipe::Ingredient::opt("Honey", Some("340g"), None, Some("A premium, ideally local, brand")),
            crate::recipe::Ingredient::q("Tap Water", "416ml"),
            crate::recipe::Ingredient::q("Boiling Water", "630ml"),
            crate::recipe::Ingredient::q("Champagne Yeast", "1/3 tsp"),
        ]),
        method: vec![
            "Pour the honey and boiling water into a large pan. Use some of the water to carefully rinse out the jar.".to_string(),
            "Turn up the heat and bring to just below boiling.".to_string(),
            "Turn down the heat and simmer for 30 minutes, stirring frequently. Skim off any scum that forms.".to_string(),
            "Turn off the heat and leave to cool until it reaches 40C.".to_string(),
            "Pour into your bottle using the funnel then leave to cool until 32C.".to_string(),
            "Add the yeast, cap the bottle, and shake thoroughly.".to_string(),
            "Top up the bottle with tap water, leaving a few inches headroom to allow for bubbling.".to_string(),
            "Fix the bung and airlock to the bottle, then add some water to the airlock.".to_string(),
            "Leave in a cool, dark, location for 6 weeks.".to_string(),
            "Remove the airlock and bung then refrigerate for one week. This will settle out the yeast.".to_string(),
            "Transfer to another bottle, pouring carefully as to leave behind the yeasty sediment.".to_string(),
        ],
    }
}
