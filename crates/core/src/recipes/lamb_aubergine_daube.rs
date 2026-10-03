//! `se.reciba.api.recipes.LambAubergineDaube`.

use crate::recipe::RecipeDef;
use crate::tag::Tag;
use chrono::NaiveDate;

pub fn recipe() -> RecipeDef {
    RecipeDef {
        object_name: "LambAubergineDaube".to_string(),
        name: "Lamb and Aubergine Daube".to_string(),
        created_at: NaiveDate::from_ymd_opt(2020, 4, 24).unwrap(),
        permalink_override: None,
        source: Some("The Times - Dinner Tonight".to_string()),
        description: None,
        tagline: None,
        notes: Vec::new(),
        tags: vec![Tag::Spicy],
        image: None,
        ingredients_blocks: crate::recipe::IngredientsBlock::simple(vec![
            crate::recipe::Ingredient::q("Aubergine", "350g"),
            crate::recipe::Ingredient::q("Onions", "2"),
            crate::recipe::Ingredient::q("Lamb Neck Fillet", "750g"),
            crate::recipe::Ingredient::q("Chopped Tomatoes", "1 400g tin"),
            crate::recipe::Ingredient::q("Chicken Stock", "250ml"),
            crate::recipe::Ingredient::q("Cumin Seed", "1/2 tsp"),
            crate::recipe::Ingredient::q("Fennel Seed", "1/2 tsp"),
            crate::recipe::Ingredient::q("Dried Chilli Flakes", "1/2 tsp"),
            crate::recipe::Ingredient::qp("Ginger", "10g", "peeled and grated"),
            crate::recipe::Ingredient::new("Olive Oil"),
            crate::recipe::Ingredient::new("Cinamon Stick"),
            crate::recipe::Ingredient::new("Salt"),
            crate::recipe::Ingredient::new("Lemon"),
        ]),
        method: vec![
            "Cut the aubergine into kebab sized chunks then immerse in water diluted with 2tbsp. Leave 20 for minutes. Rinse and drain.".to_string(),
            "Finely chop the onions. Pulverise the cumin, fennel, chilli flakes and ginger to make a corse paste.".to_string(),
            "Cut the lamb into kebab sized pieces.".to_string(),
            "Heat the oil in a large frying pan and stir fry the aubergine, tossing until beginning to soften but not completely cooked. Remove from the pan.".to_string(),
            "Add a dash more oil the cook the onions for 5 minutes, stirring frequently.".to_string(),
            "Stir in the cinnamon stick and spice paste.".to_string(),
            "Toss for a couple of minutes then push the onions to one side of the pan and brown the lamb.".to_string(),
            "Add the aubergine back along with the chopped tomatoes and stock.".to_string(),
            "Bring to a gentle simmer then cover and cook for 45 minutes or until lamb is tender.".to_string(),
            "Season with salt and lemon.".to_string(),
        ],
    }
}
