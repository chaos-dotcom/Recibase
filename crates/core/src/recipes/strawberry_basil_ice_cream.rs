//! `se.reciba.api.recipes.StrawberryBasilIceCream`.

use crate::recipe::RecipeDef;
use crate::tag::Tag;
use chrono::NaiveDate;

pub fn recipe() -> RecipeDef {
    RecipeDef {
        object_name: "StrawberryBasilIceCream".to_string(),
        name: "Strawberry, Basil & Black Pepper Ice Cream".to_string(),
        created_at: NaiveDate::from_ymd_opt(2025, 2, 14).unwrap(),
        permalink_override: Some("strawberry-basil-ice-cream".to_string()),
        source: Some("Kit".to_string()),
        description: None,
        tagline: None,
        notes: vec![
            crate::ice_cream::generic_notes(),
            "I vibed the quantity of pepper. A few grinder twists wasn't enough so I added a healthy dose the second time. If I make this again I'll measure it in tsp. Consider using a fine spice mill or grinding with a mortar and pestal, because my supermarket grinder left large pieces.".to_string(),
            "There was only 1/4tsp of vanilla essence left the first time I made this. I think 1/2-1tsp would work best.".to_string(),
            "While the food colouring is optional the mixture is a concrete grey by default, so I'd recommend it! I've doubled the quantity of food colouring when writing this up because the mixture was still a bit grey, but YMMV.".to_string(),
            "Stephani tasting notes: Interesting and complex. Most liked so far. Strong basil flavour with moments of strawberry and black pepper.".to_string(),
            "Kit tasting notes: smells like pesto, tastes juicy and fruity with strawberry dominating the basil. Pepper a bit weak. Might improve after a few days in the freezer but I should consider soaking the pepper in the mascarpone for a day in the fridge before making ice cream. Consider adding a dash of lime juice to balance the strawberry sweetness, like <a href=\"https://peterleymanorfarm.co.uk/strawberry-basil-and-black-pepper-ice-cream/\">this recipe</a> does.".to_string(),
        ],
        tags: vec![Tag::Pudding],
        image: Some(crate::recipe::Image::new("https://i.reciba.se/strawberry-basil-ice-cream.jpg")),
        ingredients_blocks: vec![
            crate::ice_cream::generic_ingredients().prefix_ingredients(vec![
                crate::recipe::Ingredient::opt("Strawberry powder", Some("12.5g"), None, Some("freeze dried, no added sugar")),
                crate::recipe::Ingredient::q("Fresh Basil", "15g"),
                crate::recipe::Ingredient::new("Black pepper"),
                crate::recipe::Ingredient::opt("Green Food Colouring", Some("1tsp"), None, Some("optional")),
            ]),
        ],
        method: [
            crate::ice_cream::generic_method_start(),
            vec![
                "Separate the basil leaves from their stems and put in a blender alongside a dollop of the mascarpone. Puree with a food blender.".to_string(),
                "Add the basil puree and the remaining mascarpone to the whisked egg yolks. Grind in a large dose of black pepper.".to_string(),
                "Mix in the strawberry powder enough that it won't spray everywhere, then whisk all the ingredients together.".to_string(),
            ],
            crate::ice_cream::generic_method_end(),
        ]
        .concat(),
    }
}
