//! `se.reciba.api.recipes.OlivePestoCasarecce`.
//! chaos-tag: casa-chaos

use crate::recipe::RecipeDef;
use crate::tag::Tag;
use chrono::NaiveDate;

pub fn recipe() -> RecipeDef {
    RecipeDef {
        object_name: "OlivePestoCasarecce".to_string(),
        name: "Olive Pesto Casarecce".to_string(),
        created_at: NaiveDate::from_ymd_opt(2023, 6, 1).unwrap(),
        permalink_override: None,
        source: Some("Waitrose".to_string()),
        description: Some("The herby, salty tang of olives makes a quick and absolutely delicious sauce for pasta. Casarecce are the ideal shape for holding the sauce, but a similar variety will work just as well.".to_string()),
        tagline: None,
        notes: vec![
            "Serves 3".to_string(),
            "Enjoy with a crisp green salad, if liked.".to_string(),
            "Feeding two? Use 175g pasta instead and set aside 1/3 of the pesto in the fridge. It'll taste just as good over the next few days on sandwiches or served with chicken or fish.".to_string(),
        ],
        tags: vec![
            Tag::Quick,
            Tag::Vegetarian,
            Tag::LowEffort,
            Tag::Scales,
        ],
        image: None,
        ingredients_blocks: crate::recipe::IngredientsBlock::simple(vec![
            crate::recipe::Ingredient::qpn("Essential Parmigiano Reggiano", "3 tbsp", "grated", "plus extra to serve"),
            crate::recipe::Ingredient::qp("Garlic", "1 clove", "finely grated"),
            crate::recipe::Ingredient::qp("Unwaxed lemon", "1/2", "zest"),
            crate::recipe::Ingredient::qp("Flat leaf parsley", "3 tbsp", "finely chopped"),
            crate::recipe::Ingredient::opt("Essential Olive Oil", Some("2 tbsp"), None, Some("plus extra for drizzling")),
            crate::recipe::Ingredient::opt("Casarecce pasta", Some("250g"), None, Some("Dried")),
            crate::recipe::Ingredient::q("Fragata Marinated Stoneless Olives with Olive Oil, Garlic & Thyme", "120g pack"),
        ]),
        method: vec![
            "Cook the pasta in a large pan of salted boiling water according to pack instructions.".to_string(),
            "Meanwhile, put the olives in a small food processor and whizz until roughly chopped (alternatively, roughly chop by hand). Add the cheese, garlic, lemon zest, parsley and olive oil. Season and pulse until just combined (or chop the mixture by hand).".to_string(),
            "Drain the pasta and toss with the olive pesto, grating over a little extra cheese and drizzling with a little more oil to serve.".to_string(),
        ],
    }
}
