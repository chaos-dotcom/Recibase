//! `se.reciba.api.recipes.Brownies`.

use crate::recipe::RecipeDef;
use crate::tag::Tag;
use chrono::NaiveDate;

pub fn recipe() -> RecipeDef {
    RecipeDef {
        object_name: "Brownies".to_string(),
        name: "Brownies".to_string(),
        created_at: NaiveDate::from_ymd_opt(2022, 1, 9).unwrap(),
        permalink_override: None,
        source: Some("https://www.louisegorrod.com/buttercup-archive/2011/07/chocolate-brownies.html".to_string()),
        description: Some("I originally got this Linda McCartney recipe from my dad, although I've since re-found it on Louise Gorrod's website. I've mirrored it here both for my own notes and because websites inevitably disappear when you least expect them to.".to_string()),
        tagline: None,
        notes: vec![
            "This recipe is all about the chocolate, so use high quality 70% dark cooking chocolate. I use Willie's Cacao Chocolate Drops or Menier Swiss Dark Chocolate.".to_string(),
            "You can use gluten free flour and the taste is indistinguishable. I've been working on a <a href=\"https://reciba.se/vegan-brownies\">vegan version</a> using aquafaba.".to_string(),
        ],
        tags: vec![
            Tag::Pudding,
            Tag::Baking,
            Tag::Vegetarian,
        ],
        image: Some(crate::recipe::Image::new("https://i.reciba.se/brownies.jpg")),
        ingredients_blocks: crate::recipe::IngredientsBlock::simple(vec![
            crate::recipe::Ingredient::q("Unsalted Butter", "300g"),
            crate::recipe::Ingredient::qp("Dark Chocolate", "300g", "Broken into pieces"),
            crate::recipe::Ingredient::q("Eggs", "5 large"),
            crate::recipe::Ingredient::q("Granulated Sugar", "450g"),
            crate::recipe::Ingredient::q("Vanilla Extract", "1 tbsp"),
            crate::recipe::Ingredient::q("Plain Flour", "200g"),
            crate::recipe::Ingredient::q("Salt", "1 tsp"),
        ]),
        method: vec![
            "Line a 34cm x 25cm x 6cm roasting tin with grease proof baking paper".to_string(),
            format!("Pre-heat the oven to {}", crate::utils::int_utils::celsius(180)),
            "Melt the butter and chocolate together in a heat-proof bowl suspended over a saucepan of barely simmering water.".to_string(),
            "Whisk the eggs, sugar and vanilla extract together in a bowl until the mixture is thick and creamy and coats the back of a spoon.".to_string(),
            "Once the butter and chocolate have melted, remove from the heat and gently fold in the egg mixture. Be careful not to go knock the air out of the egg mixture.".to_string(),
            "Sift in the flour and salt then gently fold in the flour until completely mixed.".to_string(),
            "Pour into your lined tin, ensuring the mixture is evenly distributed. Bake in the oven for 20 - 25 minutes, or until the whole of the top has formed a little brown crust that has started to crack. This giant brownie should not wobble, but should remain a little gooey on the inside.".to_string(),
            "Leave to cool for 20 minutes in the pan before lifting out and cutting into portions.".to_string(),
        ],
    }
}
