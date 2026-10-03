//! `se.reciba.api.recipes.CoconutPotatoCurry`.
//! chaos-tag: casa-chaos

use crate::recipe::RecipeDef;
use crate::tag::Tag;
use chrono::NaiveDate;

pub fn recipe() -> RecipeDef {
    RecipeDef {
        object_name: "CoconutPotatoCurry".to_string(),
        name: "Coconut & Potato Curry".to_string(),
        created_at: NaiveDate::from_ymd_opt(2023, 4, 1).unwrap(),
        permalink_override: None,
        source: Some("Waitrose".to_string()),
        description: Some("This vegan, gluten-free curry ticks all the right boxes. It's healthy, great value and all cooked in one pot. You can make it up to 48 hours in advance - just cover and chill, then reheat until piping hot.".to_string()),
        tagline: None,
        notes: vec![
            "Serves 4".to_string(),
            "Some flatbreads or steamed rice would go nicely, too.".to_string(),
            "Leftovers: hold on to those Maris Piper peelings! They're full of fibre and delicious roasted with oil, paprika and a pinch of salt.".to_string(),
        ],
        tags: vec![
            Tag::Vegan,
            Tag::GlutenFree,
            Tag::Scales,
            Tag::LowEffort,
            Tag::Freezes,
        ],
        image: None,
        ingredients_blocks: crate::recipe::IngredientsBlock::simple(vec![
            crate::recipe::Ingredient::q("Essential Vegetable Oil", "1 tbsp"),
            crate::recipe::Ingredient::qp("Onion", "1", "finely sliced"),
            crate::recipe::Ingredient::qp("Garlic", "2 cloves", "finely chopped"),
            crate::recipe::Ingredient::q("Cooks' Ingredients Garam Masala", "1 tsp"),
            crate::recipe::Ingredient::opt("Nigella Seeds", Some("1 1/2 tsp"), None, Some("Optional")),
            crate::recipe::Ingredient::q("Fine Salt", "1/2 tsp"),
            crate::recipe::Ingredient::q("Ground Turmeric", "1/2 tsp"),
            crate::recipe::Ingredient::qp("Waitrose British Maris Piper Potatoes", "700g", "peeled and cut into 2-3cm chunks"),
            crate::recipe::Ingredient::q("Essential Garden Peas", "200g"),
            crate::recipe::Ingredient::q("Essential Coconut Cream", "160ml can"),
            crate::recipe::Ingredient::opt("Coriander Leaves", None, Some("sliced"), None),
            crate::recipe::Ingredient::opt("Green Chilli and Lime Wedges", None, None, Some("To serve (optional)")),
        ]),
        method: vec![
            "Heat the oil in a large saucepan over a medium-high heat. Add the onion and garlic and fry for 2-3 minutes until starting to soften.".to_string(),
            "Meanwhile, rinse the cut potatoes well, shuffling them in a bowl of cold water to release as much starch as possible; set aside.".to_string(),
            "Add the spices and salt to the onions and fry for 1 minute more, then add the potatoes, turning to coat in the spices.".to_string(),
            "Add 250ml cold water, bring to the boil, then lower to a brisk simmer for 15 minutes, stirring regularly to make sure the potatoes cook evenly.".to_string(),
            "Stir in the peas and coconut cream, then simmer for 3-4 minutes more.".to_string(),
            "Serve with the coriander, green chilli and lime wedges, if using.".to_string(),
        ],
    }
}
