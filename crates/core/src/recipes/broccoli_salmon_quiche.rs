//! `se.reciba.api.recipes.BroccoliSalmonQuiche`.

use crate::recipe::RecipeDef;
use crate::tag::Tag;
use chrono::NaiveDate;

pub fn recipe() -> RecipeDef {
    RecipeDef {
        object_name: "BroccoliSalmonQuiche".to_string(),
        name: "Broccoli & Salmon Quiche".to_string(),
        created_at: NaiveDate::from_ymd_opt(2020, 4, 24).unwrap(),
        permalink_override: None,
        source: None,
        description: None,
        tagline: None,
        notes: vec!["Consider using the remaining broccoli stalks and stilton in a broccoli and stilton soup.".to_string()],
        tags: vec![
            Tag::Stodge,
            Tag::Slow,
            Tag::Pescatarian,
        ],
        image: None,
        ingredients_blocks: crate::recipe::IngredientsBlock::simple(vec![
            crate::recipe::Ingredient::qpn("Broccoli Florets", "100g", "Cut to 1-3cm pieces", "Save the stalks for a soup"),
            crate::recipe::Ingredient::qp("Smoked Salmon", "120g", "Cut into 2cm pieces"),
            crate::recipe::Ingredient::q("Stilton", "50g"),
            crate::recipe::Ingredient::q("Shortcrust Pastry Sheet", "230g"),
            crate::recipe::Ingredient::q("Eggs", "3"),
            crate::recipe::Ingredient::q("Mascarpone Cheese", "2 tbsp"),
            crate::recipe::Ingredient::new("Pepper"),
        ]),
        method: vec![
            format!("Pre-heat the oven to {}", crate::utils::int_utils::celsius(200)),
            "Add the mascarpone cheese to a mixing bowl. It may need a few seconds in the microwave to warm up.".to_string(),
            "Beat in the two eggs.".to_string(),
            "Mix in the broccoli and salmon pieces".to_string(),
            "Lay the shortcrust pastry sheet over a 9 inch flan dish and gently push into the edges.".to_string(),
            "Snip off any excess pastry escaping the dish. You can save this to bake mini sweet treats.".to_string(),
            "Pour the mixture into the pastry dish, spreading it evenly.".to_string(),
            "Crumble stilton over the top of the quiche.".to_string(),
            "Bake in the oven for 20 minutes".to_string(),
        ],
    }
}
