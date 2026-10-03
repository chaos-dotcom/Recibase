//! `se.reciba.api.recipes.CheddarLeekOrzotto`.
//! chaos-tag: casa-chaos

use crate::recipe::RecipeDef;
use crate::tag::Tag;
use chrono::NaiveDate;

pub fn recipe() -> RecipeDef {
    RecipeDef {
        object_name: "CheddarLeekOrzotto".to_string(),
        name: "Cheddar & Leek Orzotto".to_string(),
        created_at: NaiveDate::from_ymd_opt(2024, 4, 1).unwrap(),
        permalink_override: None,
        source: Some("Waitrose".to_string()),
        description: Some("A pleasingly simple twist on risotto that's quicker to make but just as gratifying as the original.".to_string()),
        tagline: None,
        notes: vec![
            "Serves 2".to_string(),
            "Prepare 10 minutes + standing; cook 15 minutes.".to_string(),
            "Cook's tip: use fresh stock if you can, its flavour will really come through in the orzotto.".to_string(),
        ],
        tags: vec![Tag::Vegetarian, Tag::Scales, Tag::Quick, Tag::LowEffort],
        image: None,
        ingredients_blocks: crate::recipe::IngredientsBlock::simple(vec![
            crate::recipe::Ingredient::q("Orzo pasta", "150g"),
            crate::recipe::Ingredient::opt("Stock", Some("400-500ml"), None, Some("Chicken or vegetable")),
            crate::recipe::Ingredient::q("Essential Frozen Petits Pois", "100g"),
            crate::recipe::Ingredient::qp("Castello Tickler Mature Cheddar", "80g", "finely grated"),
            crate::recipe::Ingredient::qpn("Leek", "1 large", "washed and trimmed", "about 250g"),
            crate::recipe::Ingredient::q("Essential Unsalted Butter", "20g"),
            crate::recipe::Ingredient::q("Olive oil", "Splash"),
            crate::recipe::Ingredient::qp("Garlic", "1 large clove", "finely chopped"),
        ]),
        method: vec![
            "Halve the leek lengthways, then halve again into quarters and finely slice.".to_string(),
            "Heat the butter and olive oil in a medium-large saucepan over a medium-high heat.".to_string(),
            "Add the leek, chopped garlic and a pinch of salt and cook, stirring regularly, for 5 minutes until soft but without any colour.".to_string(),
            "Add the orzo to the pan and stir through the leek. Cook, stirring, for 2 minutes.".to_string(),
            "Add 400ml stock to the pan, bring to the boil, then reduce the heat to a gentle simmer.".to_string(),
            "Cook for about 4 minutes, stirring regularly, then stir in the peas.".to_string(),
            "Cook for another 4 minutes, adding a little more stock if it looks dry at any point (it should be a loose consistency).".to_string(),
            "Taste the orzo; it's ready when tender but with just a little bite.".to_string(),
            "Take off the heat and stir in 2/3 of the cheddar, then let stand for 2 minutes.".to_string(),
            "Sprinkle the remaining cheese over the top and serve with a grinding of black pepper.".to_string(),
        ],
    }
}
