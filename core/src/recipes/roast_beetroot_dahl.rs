//! `se.reciba.api.recipes.RoastBeetrootDahl`.

use crate::recipe::RecipeDef;
use crate::tag::Tag;
use chrono::NaiveDate;

pub fn recipe() -> RecipeDef {
    RecipeDef {
        object_name: "RoastBeetrootDahl".to_string(),
        name: "Roast Beetroot Dahl".to_string(),
        created_at: NaiveDate::from_ymd_opt(2022, 2, 20).unwrap(),
        permalink_override: None,
        source: Some("Hello Fresh".to_string()),
        description: None,
        tagline: None,
        notes: vec![
            "The beetroot quantity is guestimated. Will refine over time.".to_string(),
            "We use a modified version of <a href=\"https://www.theflavorbender.com/sri-lankan-roasted-curry-powder/\">this</a> Sri Lankan Curry power recipe. Will write down our version some time.".to_string(),
        ],
        tags: vec![Tag::Vegan, Tag::Spicy, Tag::Scales],
        image: None,
        ingredients_blocks: crate::recipe::IngredientsBlock::simple(vec![
            crate::recipe::Ingredient::qp("Beetroot", "500g", "cut into 2cm chunks"),
            crate::recipe::Ingredient::qp("Onion", "1", "thinly sliced"),
            crate::recipe::Ingredient::qp("Spring Onions", "2", "chopped"),
            crate::recipe::Ingredient::qp("Lime", "1", "zested and juiced"),
            crate::recipe::Ingredient::qp("Garlic", "2 cloves", "chopped"),
            crate::recipe::Ingredient::opt("Flatbreads", Some("4"), None, Some("Optional")),
            crate::recipe::Ingredient::q("Sri Lankan Curry Powder", "a few tablespoons"),
            crate::recipe::Ingredient::q("Stock cube", "1"),
            crate::recipe::Ingredient::q("Coconut Milk", "1 400ml tin"),
            crate::recipe::Ingredient::q("Red Lentils", "150g"),
            crate::recipe::Ingredient::q("Water", "200ml"),
            crate::recipe::Ingredient::new("Oil"),
        ]),
        method: vec![
            format!("Preheat the oven to {}.", crate::utils::int_utils::celsius(200)),
            "Spread the beetroot on a baking tray. Drizzle with oil and season with salt and pepper.".to_string(),
            "Roast the beetroot for 20-25 minutes then set aside.".to_string(),
            "Boil the water and make stock.".to_string(),
            "Heat a large saucepan with some oil. Add the onions and soften for a few minutes.".to_string(),
            "Stir in the curry powder and garlic then cook for a minute.".to_string(),
            "Add the coconut milk, lentils and stock, then bring to the boil.".to_string(),
            "Turn down the heat and simmer for 20-25 minutes or until the lentils are soft. Stir every few minutes.".to_string(),
            "Mix the roasted beetroot and lime juice into the lentils.".to_string(),
            "Lightly toast the flatbreads.".to_string(),
            "Serve with a garnish of lime zest and spring onions, with the flatbreads on the side.".to_string(),
        ],
    }
}
