//! `se.reciba.api.recipes.VeganBrownies`.

use crate::recipe::RecipeDef;
use crate::tag::Tag;
use chrono::NaiveDate;

pub fn recipe() -> RecipeDef {
    RecipeDef {
        object_name: "VeganBrownies".to_string(),
        name: "Vegan Brownies (WIP)".to_string(),
        created_at: NaiveDate::from_ymd_opt(2022, 7, 1).unwrap(),
        permalink_override: Some("vegan-brownies".to_string()),
        source: Some("https://reciba.se/brownies".to_string()),
        description: Some("Version 4 of my vegan alt recipe, based on Linda McCartney's classic brownies".to_string()),
        tagline: None,
        notes: vec![
            "This recipe is all about the chocolate, so use high quality 70%+ dark cooking chocolate. I use Willie's Cacao Chocolate Drops but other vegan cooking chocolates are available. Always check the ingredients as some contain milk.".to_string(),
            "If your vegan butter is unsalted then use a full teaspoon of salt. I reduced the salt quantity to 1/2 tsp because Vitalite is quite salty. The vegan version uses way less butter, but is still failing keep its fluffy texture, so I might reduce the Vitalite further.".to_string(),
            "I've successfully made a gluten free (but non-vegan) version using potato flour and the taste is indistinguishable. The vegan gf versions failed to set, but that was using a much older version of the recipe. I'll try again with gf flour once I've perfected the vegan recipe.".to_string(),
            "If you come up with any improvements please <a href=\"mailto:kittsville@gmail.com\">email me</a> or DM me on Twitter: <a href=\"https://twitter.com/kittsville\">@kittsville</a>.".to_string(),
        ],
        tags: vec![Tag::Pudding, Tag::Baking, Tag::Vegan],
        image: Some(crate::recipe::Image::new("https://i.reciba.se/brownies.jpg")),
        ingredients_blocks: crate::recipe::IngredientsBlock::simple(vec![
            crate::recipe::Ingredient::q("Vitalite", "200g"),
            crate::recipe::Ingredient::opt("Dark Chocolate", Some("300g"), Some("Broken into pieces"), None),
            crate::recipe::Ingredient::opt("Aquafaba", Some("214ml"), None, Some("Collect the water from 2 tins of chickpeas")),
            crate::recipe::Ingredient::q("Cream of tartar", "1/4 tsp"),
            crate::recipe::Ingredient::q("Granulated Sugar", "450g"),
            crate::recipe::Ingredient::q("Vanilla Extract", "1 tbsp"),
            crate::recipe::Ingredient::q("Plain Flour", "200g"),
            crate::recipe::Ingredient::q("Salt", "1/2 tsp"),
        ]),
        method: vec![
            "Line a 34cm x 25cm x 6cm roasting tin with grease proof baking paper".to_string(),
            format!("Pre-heat the oven to {}", crate::utils::int_utils::celsius(180)),
            "Melt the Vitalite and chocolate together in a heat-proof bowl suspended over a saucepan of barely simmering water.".to_string(),
            "Whisk the aquafaba, cream of tartar, sugar and vanilla extract together in a bowl until the mixture is thick and creamy and coats the back of a spoon.".to_string(),
            "Once the Vitalite and chocolate have melted, remove from the heat and gently fold in the aquafaba mixture. Be careful not to go knock the air out of the aquafaba.".to_string(),
            "Sift in the flour and salt then gently fold in the flour until completely mixed.".to_string(),
            "Pour into your lined tin, ensuring the mixture is evenly distributed. Bake in the oven for 20 - 25 minutes, or until the whole of the top has formed a little brown crust that has started to crack. This giant brownie should not wobble, but should remain a little gooey on the inside.".to_string(),
            "Leave to cool for 20 minutes in the pan before lifting out and cutting into portions.".to_string(),
        ],
    }
}
