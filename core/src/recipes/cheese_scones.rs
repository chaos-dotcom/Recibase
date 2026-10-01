//! `se.reciba.api.recipes.CheeseScones`.

use crate::recipe::RecipeDef;
use crate::tag::Tag;
use chrono::NaiveDate;

pub fn recipe() -> RecipeDef {
    RecipeDef {
        object_name: "CheeseScones".to_string(),
        name: "Cheese Scones".to_string(),
        created_at: NaiveDate::from_ymd_opt(2025, 2, 4).unwrap(),
        permalink_override: None,
        source: Some("https://www.bbcgoodfood.com/user/896076/recipe/classic-cheese-scones".to_string()),
        description: Some("Scones, with cheese. Fill with jams and more".to_string()),
        tagline: None,
        notes: vec![
            "Leave the milk out as it always needs more.".to_string(),
            "Don't roll the dough too thin or they won't rise properly.".to_string(),
        ],
        tags: vec![
            Tag::Vegetarian,
            Tag::HighEffort,
            Tag::Stodge,
        ],
        image: Some(crate::recipe::Image::new("https://i.reciba.se/cheese-scones.jpg")),
        ingredients_blocks: crate::recipe::IngredientsBlock::simple(vec![
            crate::recipe::Ingredient::q("Plain Flour", "208g"),
            crate::recipe::Ingredient::q("Baking Powder", "21g"),
            crate::recipe::Ingredient::q("Cayenne pepper", "pinch"),
            crate::recipe::Ingredient::opt("Butter", Some("55g"), None, Some("chilled")),
            crate::recipe::Ingredient::qp("Mature Cheddar", "120g", "grated"),
            crate::recipe::Ingredient::q("Milk", "90-100ml"),
            crate::recipe::Ingredient::q("Salt", "pinch"),
        ]),
        method: vec![
            format!("Pre-heat the oven to {} and cut baking paper to cover a large baking tray.", crate::utils::int_utils::celsius(180)),
            "Sift the flour, salt, baking powder and cayenne pepper together in a mixing bowl.".to_string(),
            "Add the butter in pieces and combine with your fingertips to make breadcrumbs.".to_string(),
            "Add 100g of the cheese and rub together.".to_string(),
            "Make a well in the centre of the mixture and slowly add the milk, mixing each time. Don't add it all at once as you might not need it. We usually need extra though.".to_string(),
            "Lightly flour a surface and roll the dough to a 2cm thickness. Cut out scone shapes and lay them on the baking tray.".to_string(),
            "Glaze with milk and top with the remaining cheese, leaving a slight gap at the edge of each scone.".to_string(),
            "Bake in the oven for 15-20 minutes then serve with butter and jams".to_string(),
        ],
    }
}
