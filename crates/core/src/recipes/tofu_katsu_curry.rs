//! `se.reciba.api.recipes.TofuKatsuCurry`.

use crate::recipe::RecipeDef;
use crate::tag::Tag;
use chrono::NaiveDate;

pub fn recipe() -> RecipeDef {
    RecipeDef {
        object_name: "TofuKatsuCurry".to_string(),
        name: "Tofu Katsu Curry".to_string(),
        created_at: NaiveDate::from_ymd_opt(2020, 4, 24).unwrap(),
        permalink_override: None,
        source: Some("Gousto".to_string()),
        description: Some("Crispy breaded tofu with homemade katsu curry sauce, sticky rice and roasted Tenderstem broccoli.".to_string()),
        tagline: None,
        notes: Vec::new(),
        tags: vec![Tag::Vegan, Tag::Quick, Tag::Scales],
        image: None,
        ingredients_blocks: crate::recipe::IngredientsBlock::simple(vec![
            crate::recipe::Ingredient::q("Plain tofu", "280g"),
            crate::recipe::Ingredient::q("White long grain rice", "130g"),
            crate::recipe::Ingredient::q("Tenderstem broccoli", "160g"),
            crate::recipe::Ingredient::q("Panko breadcrumbs", "40g"),
            crate::recipe::Ingredient::q("Vegan mayonnaise", "25ml"),
            crate::recipe::Ingredient::qp("Fresh root ginger", "15g", "peeled and finely chopped or grated"),
            crate::recipe::Ingredient::q("Curry powder", "1 tbsp"),
            crate::recipe::Ingredient::q("Plain flour", "2 tbsp"),
            crate::recipe::Ingredient::q("Soy sauce", "15ml"),
            crate::recipe::Ingredient::q("Mango chutney", "20g"),
            crate::recipe::Ingredient::new("Salt"),
            crate::recipe::Ingredient::new("Vegetable oil"),
        ]),
        method: vec![
            format!("Preheat the oven to {}.", crate::utils::int_utils::simple_fan_instruction(220)),
            "Drain the tofu, pat it dry with kitchen paper and slice it lengthways into strips.".to_string(),
            "Add the mayonnaise and panko breadcrumbs to separate plates and season the breadcrumbs with a pinch of salt.".to_string(),
            "Toss the tofu strips in the mayonnaise until lightly coated, then press them into the breadcrumbs firmly to evenly coat.".to_string(),
            "Add the breaded tofu to a baking tray lined with non-stick baking paper.".to_string(),
            "Drizzle generously with vegetable oil and bake for 10 min.".to_string(),
            "Add the Tenderstem broccoli to the tray, drizzle with oil and return to the oven for 10-15 min or until the tofu is golden and crispy and the broccoli is tender.".to_string(),
            "Add the rice to a pot with a lid with 225ml cold water and bring to the boil over a high heat.".to_string(),
            "Once boiling, reduce the heat to very low and cook, covered, for 12-15 min or until all the water has absorbed and the rice is cooked.".to_string(),
            "Once done, stir vigorously to release the starch, then remove from the heat and set aside (lid on) to steam until serving.".to_string(),
            "Meanwhile, boil a kettle.".to_string(),
            "Heat a large, wide-based pan (preferably non-stick) with 2 tbsp vegetable oil over a medium heat.".to_string(),
            "Once hot, add the chopped ginger and cook for 3 min or until fragrant.".to_string(),
            "Add the curry powder and flour and cook for 1 min.".to_string(),
            "Gradually whisk in 300ml boiled water and cook for 5-6 min or until thickened.".to_string(),
            "Once the sauce has thickened, stir in the soy sauce and mango chutney.".to_string(),
            "Serve the tofu katsu over the katsu sauce with the sticky rice and roasted Tenderstem broccoli.".to_string(),
        ],
    }
}
