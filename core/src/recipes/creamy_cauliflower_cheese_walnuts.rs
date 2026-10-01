//! `se.reciba.api.recipes.CreamyCauliflowerCheeseWalnuts`.

use crate::recipe::RecipeDef;
use crate::tag::Tag;
use chrono::NaiveDate;

pub fn recipe() -> RecipeDef {
    RecipeDef {
        object_name: "CreamyCauliflowerCheeseWalnuts".to_string(),
        name: "Creamy cauliflower cheese with walnuts".to_string(),
        created_at: NaiveDate::from_ymd_opt(2020, 4, 24).unwrap(),
        permalink_override: Some("creamy-cauliflower-cheese".to_string()),
        source: Some("https://docs.google.com/document/d/1A0bgFOwirLW2mct8KNrxYdk4OEsGZtY23mFdgOljHFA".to_string()),
        description: None,
        tagline: None,
        notes: Vec::new(),
        tags: vec![
            Tag::Vegetarian,
            Tag::Quick,
            Tag::LowEffort,
        ],
        image: None,
        ingredients_blocks: crate::recipe::IngredientsBlock::simple(vec![
            crate::recipe::Ingredient::qp("Cauliflower", "1", "cut into 1cm pieces"),
            crate::recipe::Ingredient::opt("Creme Fraiche", Some("300g"), None, Some("or Cream Cheese")),
            crate::recipe::Ingredient::q("Dijon Mustard", "1 tsp"),
            crate::recipe::Ingredient::qp("Blue Cheese", "125g", "crumbled"),
            crate::recipe::Ingredient::qp("Walnuts", "25g", "roughly chopped"),
            crate::recipe::Ingredient::qp("Cheddar Cheese", "50g", "grated"),
            crate::recipe::Ingredient::new("Salt"),
            crate::recipe::Ingredient::new("Black pepper"),
        ]),
        method: vec![
            "Steam the cauliflower until tender, drain, then place in a grill-suitable dish".to_string(),
            "Mix the cream cheese and mustard with the cauliflower, then stir in the blue cheese. Season with a little salt if necessary and plenty of pepper.".to_string(),
            "Scatter the walnuts on top, then cover with the cheddar (this helps to prevent the walnuts from burning).".to_string(),
            "Place under a preheated hot grill for 10-15 minutes, or until the top is golden brown and the inside hot and bubbling.".to_string(),
        ],
    }
}
