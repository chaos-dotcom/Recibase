//! `se.reciba.api.recipes.OvenBakedShakshukaRoastedPeppersTomatoesChilliWithEggs`.
//! chaos-tag: casa-chaos

use crate::recipe::RecipeDef;
use crate::tag::Tag;
use chrono::NaiveDate;

pub fn recipe() -> RecipeDef {
    RecipeDef {
        object_name: "OvenBakedShakshukaRoastedPeppersTomatoesChilliWithEggs".to_string(),
        name: "Oven Baked Shakshuka: Roasted Peppers, Tomatoes & Chilli with Eggs".to_string(),
        created_at: NaiveDate::from_ymd_opt(2026, 10, 4).unwrap(),
        permalink_override: None,
        source: None,
        description: None,
        tagline: None,
        notes: vec![
            "Serves 4".to_string(),
            "Prepare 15 minutes; cook 45 minutes.".to_string(),
            "Note: The eggs will take more or less time depending on whether they're fridge cold.".to_string(),
        ],
        tags: vec![Tag::Vegetarian, Tag::LowEffort, Tag::Scales],
        image: None,
        ingredients_blocks: vec![
            crate::recipe::IngredientsBlock::new(None, vec![
                crate::recipe::Ingredient::qp("Red Onion", "1", "roughly chopped"),
                crate::recipe::Ingredient::qp("Red Peppers", "2", "roughly chopped"),
                crate::recipe::Ingredient::qp("Yellow Peppers", "2", "roughly chopped"),
                crate::recipe::Ingredient::qp("Vine Tomatoes", "300g", "quartered"),
                crate::recipe::Ingredient::qp("Red Chillies", "2", "deseeded and roughly chopped"),
                crate::recipe::Ingredient::qp("Garlic", "2 cloves", "crushed"),
                crate::recipe::Ingredient::q("Olive Oil", "1 tablespoon"),
                crate::recipe::Ingredient::q("Sea Salt", "1 teaspoon"),
                crate::recipe::Ingredient::q("Ground Cumin", "1 teaspoon"),
                crate::recipe::Ingredient::q("Ground Coriander", "1 teaspoon"),
                crate::recipe::Ingredient::q("Smoked Paprika", "1 1/2 teaspoons"),
                crate::recipe::Ingredient::q("Chopped Tomatoes", "1 x 400g tin"),
                crate::recipe::Ingredient::q("Free-range Eggs", "4"),
                crate::recipe::Ingredient::opt("Za'atar", Some("1 tablespoon"), None, Some("Optional")),
                crate::recipe::Ingredient::opt("Coriander", None, Some("freshly chopped"), None),
            ]),
            crate::recipe::IngredientsBlock::new(Some("To Serve"), vec![
                crate::recipe::Ingredient::new("Buttered Toast or Pitta Breads"),
            ]),
        ],
        method: vec![
            "Preheat the oven to 180°C fan/200°C/gas 6.".to_string(),
            "Mix the onion, peppers, tomatoes, chillies and garlic with the oil, salt and spices in a large roasting tin, then transfer to the oven and roast for 30 minutes.".to_string(),
            "Lower the temperature to 160°C fan/180°C/gas 4. Squash the cooked tomatoes down well with a wooden spoon, then add the tinned tomatoes and mix everything together. Make four indentations in the tomato mixture, crack an egg into each, then return to the oven for a further 10 minutes, or until the eggs are just cooked to your liking.".to_string(),
            "Scatter with the za'atar, if using, and the freshly chopped coriander. Serve with lots of hot buttered toast or pitta breads.".to_string(),
        ],
    }
}
