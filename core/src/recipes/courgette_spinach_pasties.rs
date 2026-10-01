//! `se.reciba.api.recipes.CourgetteSpinachPasties`.

use crate::recipe::RecipeDef;
use crate::tag::Tag;
use chrono::NaiveDate;

pub fn recipe() -> RecipeDef {
    RecipeDef {
        object_name: "CourgetteSpinachPasties".to_string(),
        name: "Courgette and spinach pasties".to_string(),
        created_at: NaiveDate::from_ymd_opt(2020, 4, 24).unwrap(),
        permalink_override: None,
        source: None,
        description: None,
        tagline: None,
        notes: vec!["We previously used goats cheese but feta is better at bringing out the courgette's flavour.".to_string()],
        tags: vec![
            Tag::Slow,
            Tag::Vegetarian,
            Tag::Stodge,
        ],
        image: None,
        ingredients_blocks: crate::recipe::IngredientsBlock::simple(vec![
            crate::recipe::Ingredient::q("Courgettes", "2"),
            crate::recipe::Ingredient::qp("Feta", "200g", "diced"),
            crate::recipe::Ingredient::qp("Spinach", "3 handfulls", "roughly chopped"),
            crate::recipe::Ingredient::q("All Butter Puff Pastry", "320g"),
            crate::recipe::Ingredient::opt("Plain Flour", None, None, Some("to thicken mixture")),
            crate::recipe::Ingredient::new("Nutmeg"),
            crate::recipe::Ingredient::new("Lemon Juice"),
            crate::recipe::Ingredient::new("Salt"),
            crate::recipe::Ingredient::new("Pepper"),
        ]),
        method: vec![
            "Grate the courgettes using a fine grater, then strain the result in a sieve.".to_string(),
            "Mix the grated courgette, feta, lemon juice, nutmeg and spinach in a pan on a low heat.".to_string(),
            "Bring to a very gentle simmer then turn off the heat.".to_string(),
            "Sift in a small amount of flour to thicken the mixure.".to_string(),
            "Leave the mixture to cool for a few minutes and lay out the pastry, cutting it into two pieces.".to_string(),
            "Place half the mixture on one side of each pastry, leaving at least a cm at the edges".to_string(),
            "Fold the other side of each pastry over the top of the mixture. Merge the edges of the pastry by pressing down with the tines of a fork.".to_string(),
            "Make a 2-3cm slice in the top of each pastry, to let out steam.".to_string(),
            "Bake in the oven at 220C/200C fan/gas 7 for 20 minutes.".to_string(),
        ],
    }
}
