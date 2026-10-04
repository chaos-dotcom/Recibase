//! `se.reciba.api.recipes.ProsciuttoCabbageBeanStew`.
//! chaos-tag: casa-chaos

use crate::recipe::RecipeDef;
use crate::tag::Tag;
use chrono::NaiveDate;

pub fn recipe() -> RecipeDef {
    RecipeDef {
        object_name: "ProsciuttoCabbageBeanStew".to_string(),
        name: "Prosciutto, Cabbage & Bean Stew".to_string(),
        created_at: NaiveDate::from_ymd_opt(2023, 11, 1).unwrap(),
        permalink_override: None,
        source: Some("Principe".to_string()),
        description: Some("A simple one-pan dish packed with veg and topped with crispy prosciutto. It's lovely served with crusty bread, or try stirring in your choice of small pasta shapes as it cooks.".to_string()),
        tagline: None,
        notes: vec![
            "Savoy cabbage is particularly effective at soaking up the flavour in stews and stir fries. You can also shred it finely, then cook with lardons for a simple side dish, or stir it into mashed potato with salad onions to make colcannon.".to_string(),
        ],
        tags: vec![
            Tag::ItsMadeOfMeat,
            Tag::VegetarianIsh,
            Tag::Stodge,
            Tag::Quick,
            Tag::ColdWeather,
            Tag::LowEffort,
            Tag::Scales,
        ],
        image: None,
        ingredients_blocks: crate::recipe::IngredientsBlock::simple(vec![
            crate::recipe::Ingredient::q("Olive Oil", "1 1/2 tbsp"),
            crate::recipe::Ingredient::q("Principe Italian Prosciutto Crudo", "70g"),
            crate::recipe::Ingredient::qp("Onion", "1", "finely chopped"),
            crate::recipe::Ingredient::qp("Celery", "1 stalk", "finely chopped"),
            crate::recipe::Ingredient::qp("Garlic", "1 clove", "finely chopped"),
            crate::recipe::Ingredient::qp("Essential Savoy Cabbage", "1/2", "shredded"),
            crate::recipe::Ingredient::opt("Thyme", Some("4 sprigs"), None, Some("optional")),
            crate::recipe::Ingredient::qp("Essential Cannellini Beans", "400g can", "drained and rinsed"),
            crate::recipe::Ingredient::opt("White Wine", Some("100ml"), None, Some("optional")),
            crate::recipe::Ingredient::q("Cooks' Ingredients Chicken Stock", "200ml"),
            crate::recipe::Ingredient::q("Essential Single Cream", "2 tbsp"),
        ]),
        method: vec![
            "Heat 1 tbsp oil over a medium-high heat in a large saute pan or shallow casserole.".to_string(),
            "Add 4 slices of prosciutto and cook for 1-2 minutes on each side until crisp; set aside.".to_string(),
            "Add the remaining 1/2 tbsp oil to the pan and fry the onion, celery and garlic for 8-10 minutes, stirring regularly, until softened.".to_string(),
            "Chop the remaining prosciutto, add to the pan and fry for 1 minute.".to_string(),
            "Add the cabbage and thyme (if using) and cook, stirring, for another 3-4 minutes.".to_string(),
            "Stir in the beans, then add the wine, if using. Simmer briskly until almost all the liquid has evaporated.".to_string(),
            "Add the chicken stock and simmer for 2-3 minutes more, then stir in the single cream and take off the heat.".to_string(),
            "Season (remember the ham is salty) and serve topped with the crispy prosciutto. Remove any thyme sprigs before eating.".to_string(),
        ],
    }
}
