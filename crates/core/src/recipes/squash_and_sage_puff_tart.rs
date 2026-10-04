//! `se.reciba.api.recipes.SquashAndSagePuffTart`.
//! chaos-tag: casa-chaos

use crate::recipe::RecipeDef;
use crate::tag::Tag;
use chrono::NaiveDate;

pub fn recipe() -> RecipeDef {
    RecipeDef {
        object_name: "SquashAndSagePuffTart".to_string(),
        name: "Squash & Sage Puff Tart".to_string(),
        created_at: NaiveDate::from_ymd_opt(2026, 10, 4).unwrap(),
        permalink_override: None,
        source: Some("Waitrose".to_string()),
        description: Some("This stunning tart would make a brilliant meat-free main on Christmas Day.".to_string()),
        tagline: None,
        notes: vec![
            "Serves 6".to_string(),
            "Prepare 15 minutes (plus cooling); cook 45 minutes.".to_string(),
            "Per serving: 1692kJ/407kcals/30.2g fat/16.6g saturated fat/25.3g carbs/4.7g sugars/7g fibre/7g protein/1g salt.".to_string(),
        ],
        tags: vec![
            Tag::Vegetarian,
            Tag::Christmas,
            Tag::ColdWeather,
            Tag::LowEffort,
            Tag::Scales,
        ],
        image: None,
        ingredients_blocks: crate::recipe::IngredientsBlock::simple(vec![
            crate::recipe::Ingredient::qp("Butternut squash", "500g", "peeled and deseeded"),
            crate::recipe::Ingredient::q("Olive oil", "1 tbsp"),
            crate::recipe::Ingredient::qpn("Garlic", "3 cloves", "whole", "unpeeled"),
            crate::recipe::Ingredient::q("Salt", "1/2 tsp"),
            crate::recipe::Ingredient::q("Jus-Rol All Butter Puff Pastry Sheet", "320g"),
            crate::recipe::Ingredient::q("No.1 French Crème Fraîche", "200ml tub"),
            crate::recipe::Ingredient::qpn("Parmigiano Reggiano", "25g", "finely grated", "plus a little extra"),
            crate::recipe::Ingredient::q("Sage leaves", "1/2 x 20g pack"),
            crate::recipe::Ingredient::q("Pumpkin seeds", "1-2 tbsp"),
        ]),
        method: vec![
            "Preheat the oven to 200°C, gas mark 6, and arrange 2 shelves in the oven. Slice the squash as thinly as you can, then toss with the oil, garlic cloves and salt in a large roasting tin.".to_string(),
            "Unroll the pastry and trim away the excess paper, then set it (still on its paper) on a baking sheet. Score a 2cm border around the edge and pierce the middle all over with a fork. Put on the lower shelf of the oven and the squash tin on the upper shelf. Bake both for 20 minutes, then set aside to cool for 5 minutes (or longer if preparing in advance).".to_string(),
            "Squeeze the garlic from the skins and mash with the crème fraîche. Stir in the 25g cheese and season. Spread this mixture over the base of the pastry (pressing it down if it has risen a lot), then arrange the squash on top. Grate over a little more cheese, then scatter with the sage and pumpkin seeds. Roast for about 25 minutes until golden. Serve warm.".to_string(),
        ],
    }
}
