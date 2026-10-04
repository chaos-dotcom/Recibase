//! `se.reciba.api.recipes.QuickRoastedFennelAndBulgurWheatWithMozzarellaFigsPomegranateAndDill`.
//! chaos-tag: casa-chaos

use crate::recipe::RecipeDef;
use crate::tag::Tag;
use chrono::NaiveDate;

pub fn recipe() -> RecipeDef {
    RecipeDef {
        object_name: "QuickRoastedFennelAndBulgurWheatWithMozzarellaFigsPomegranateAndDill".to_string(),
        name: "Quick Roasted Fennel & Bulgur Wheat with Mozzarella, Figs, Pomegranate & Dill".to_string(),
        created_at: NaiveDate::from_ymd_opt(2026, 10, 4).unwrap(),
        permalink_override: None,
        source: Some("Alice Hart".to_string()),
        description: Some("This is a beautiful dish, and perfect for a lunchbox the next day too. Do as Alice Hart suggests and buy the shorter, fatter bulbs of fennel - they have a better flavour.".to_string()),
        tagline: None,
        notes: vec![
            "Serves 4".to_string(),
            "Prepare 10 minutes; cook 20 minutes.".to_string(),
            "Note: Take the mozzarella, figs and pomegranate out of the fridge an hour before you want them, to let them come up to room temperature.".to_string(),
        ],
        tags: vec![
            Tag::Vegetarian,
            Tag::Quick,
            Tag::LowEffort,
            Tag::Lunch,
        ],
        image: None,
        ingredients_blocks: vec![
            crate::recipe::IngredientsBlock::new(None, vec![
                crate::recipe::Ingredient::qp("Fennel", "300g", "thinly sliced"),
                crate::recipe::Ingredient::q("Bulgur Wheat", "300g"),
                crate::recipe::Ingredient::qp("Garlic", "2 cloves", "crushed"),
                crate::recipe::Ingredient::qp("Vegetable Stock", "600ml", "boiling"),
                crate::recipe::Ingredient::qp("Figs", "2-3", "quartered"),
                crate::recipe::Ingredient::qp("Mozzarella", "250g", "roughly torn"),
                crate::recipe::Ingredient::qp("Pomegranate", "1", "seeds only"),
                crate::recipe::Ingredient::qp("Fresh Dill", "20g", "roughly chopped"),
            ]),
            crate::recipe::IngredientsBlock::new(Some("Dressing"), vec![
                crate::recipe::Ingredient::qp("Orange", "1", "zest plus 1 tablespoon juice"),
                crate::recipe::Ingredient::q("Extra Virgin Olive Oil", "1 tablespoon"),
                crate::recipe::Ingredient::qp("Spring Onions", "4", "thinly sliced"),
                crate::recipe::Ingredient::q("Sea Salt", "1 teaspoon"),
                crate::recipe::Ingredient::new("Freshly Ground Black Pepper"),
            ]),
        ],
        method: vec![
            "Preheat the oven to 200°C fan/220°C/gas 7.".to_string(),
            "Mix the fennel, bulgur wheat, garlic and boiling vegetable stock in a roasting tin, then transfer to the oven and cook for 20 minutes.".to_string(),
            "Meanwhile, whisk the orange zest, juice, extra virgin olive oil, spring onions, sea salt and black pepper together. Pour this dressing over the cooked bulgur wheat and fennel and mix well. Taste and adjust the seasoning as needed.".to_string(),
            "Scatter with the fig quarters, mozzarella, pomegranate seeds and chopped dill and serve hot.".to_string(),
        ],
    }
}
