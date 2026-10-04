//! `se.reciba.api.recipes.WarmingSweetPotatoMushroomPolentaWithTomatoes`.
//! chaos-tag: casa-chaos

use crate::recipe::RecipeDef;
use crate::tag::Tag;
use chrono::NaiveDate;

pub fn recipe() -> RecipeDef {
    RecipeDef {
        object_name: "WarmingSweetPotatoMushroomPolentaWithTomatoes".to_string(),
        name: "Warming Sweet Potato & Mushroom Polenta with Tomatoes".to_string(),
        created_at: NaiveDate::from_ymd_opt(2026, 10, 4).unwrap(),
        permalink_override: None,
        source: Some("Alice Hart's The New Vegetarian".to_string()),
        description: None,
        tagline: None,
        notes: vec![
            "Serves 2".to_string(),
            "Prepare 10 minutes; cook 55 minutes.".to_string(),
        ],
        tags: vec![
            Tag::Vegan,
            Tag::ColdWeather,
            Tag::LowEffort,
            Tag::Scales,
            Tag::Stodge,
        ],
        image: None,
        ingredients_blocks: vec![
            crate::recipe::IngredientsBlock::new(None, vec![
                crate::recipe::Ingredient::q("Polenta", "150g"),
                crate::recipe::Ingredient::q("Vegetable Stock", "400ml"),
                crate::recipe::Ingredient::q("Olive Oil", "3 tablespoons"),
                crate::recipe::Ingredient::qp("Sweet Potatoes", "300g", "peeled and cut into 5mm chunks"),
                crate::recipe::Ingredient::new("Freshly Ground Black Pepper"),
                crate::recipe::Ingredient::qp("Mini Portobello or Chestnut Mushrooms", "300g", "sliced"),
                crate::recipe::Ingredient::qp("Cherry Tomatoes", "250g", "halved"),
                crate::recipe::Ingredient::qp("Garlic", "2 cloves", "crushed"),
                crate::recipe::Ingredient::q("Sea Salt", "1 teaspoon"),
                crate::recipe::Ingredient::new("Freshly Ground Black Pepper"),
            ]),
            crate::recipe::IngredientsBlock::new(Some("Dressing"), vec![
                crate::recipe::Ingredient::qp("Fresh Flat-Leaf Parsley", "15g", "finely chopped"),
                crate::recipe::Ingredient::qp("Lemon", "1", "juice only"),
                crate::recipe::Ingredient::q("Extra Virgin Olive Oil", "2 tablespoons"),
                crate::recipe::Ingredient::q("Chilli Flakes", "1/2 teaspoon"),
            ]),
        ],
        method: vec![
            "Preheat the oven to 180°C fan/200°C/gas 6.".to_string(),
            "Line a roasting tin with baking paper, then tip in the polenta, vegetable stock, 2 tablespoons of olive oil and the sweet potatoes. Stir, season well with black pepper, then transfer to the oven and cook, uncovered, for 40 minutes.".to_string(),
            "Meanwhile, mix together the parsley, lemon juice, extra virgin olive oil and chilli flakes for the dressing.".to_string(),
            "Stir the mushrooms, tomatoes, garlic, salt, pepper and another tablespoon of olive oil together and set aside.".to_string(),
            "Once the polenta has had 40 minutes, take the tin out of the oven and give it a good stir. Top with the mushroom and tomato mixture and return to the oven for a further 15 minutes, after which the mushrooms should be softened and the polenta crisp. Serve with the dressing alongside and a green salad.".to_string(),
        ],
    }
}
