//! `se.reciba.api.recipes.BirthdayCake2`.

use crate::recipe::RecipeDef;
use crate::tag::Tag;
use chrono::NaiveDate;

pub fn recipe() -> RecipeDef {
    RecipeDef {
        object_name: "BirthdayCake2".to_string(),
        name: "Birthday Cake".to_string(),
        created_at: NaiveDate::from_ymd_opt(2024, 11, 21).unwrap(),
        permalink_override: None,
        source: Some("https://www.bbc.co.uk/games/embed/food-interactive-sponge-cake-calculator?units=metric&cakeType=layer_cake&tinShape=round&mode=tinSize&value=18cm&flavouring=lemon&icing=buttercream_fill_cover".to_string()),
        description: Some("Madeira cake with buttercream icing and topped with Smarties".to_string()),
        tagline: None,
        notes: vec![
            "This is based on the BBC cake calculator for my 2 x 18cm round cake tins with the addition of Smarties and marzipan lettering.".to_string(),
            "You always add too little icing to the middle of the cake and have a lot left over.".to_string(),
        ],
        tags: vec![
            Tag::Pudding,
            Tag::Baking,
        ],
        image: Some(crate::recipe::Image::new("https://i.reciba.se/birthday-cake.jpg")),
        ingredients_blocks: vec![
            crate::recipe::IngredientsBlock::new(Some("Cake"), vec![
                crate::recipe::Ingredient::qp("Unsalted Butter", "200g", "Softened"),
                crate::recipe::Ingredient::q("Caster Sugar", "200g"),
                crate::recipe::Ingredient::q("Eggs", "2 medium"),
                crate::recipe::Ingredient::qp("Lemon", "1/2", "zest only"),
                crate::recipe::Ingredient::q("Milk", "2 tbsp"),
                crate::recipe::Ingredient::q("Plain Flour", "186g"),
                crate::recipe::Ingredient::q("Baking Powder", "3 tsp"),
            ]),
            crate::recipe::IngredientsBlock::new(Some("Topping"), vec![
                crate::recipe::Ingredient::qp("Unsalted butter", "150g", "softened"),
                crate::recipe::Ingredient::q("Icing Sugar", "300g"),
                crate::recipe::Ingredient::q("Vanilla Extract", "1 tsp"),
                crate::recipe::Ingredient::q("Milk", "1 tbsp"),
                crate::recipe::Ingredient::opt("Smarties", Some("200g"), None, Some("optional")),
                crate::recipe::Ingredient::opt("Cocoa powder", Some("4.5 tsp"), None, Some("add if you want chocolate icing")),
                crate::recipe::Ingredient::opt("Marzipan", None, None, Some("optional, if you want to decorate with lettering")),
            ]),
        ],
        method: vec![
            "Preheat the oven to 180C/160C (Fan)/Gas 4.".to_string(),
            "Using your butter's wrapping, grease 2 x 18cm round cake tins with butter. Line the bottom of the tins with a circle of baking paper.".to_string(),
            "In a large bowl, cream together the butter and sugar using an electric mixer until the mixture is pale and fluffy.".to_string(),
            "Beat in the eggs one at a time, mixing until the egg is completely incorporated into the batter before adding the next. Add a tablespoon of flour if the mixture curdles.".to_string(),
            "Mix in the lemon zest.".to_string(),
            "Fold in the flour and baking powder using a large metal spoon until no traces of flour are visible. Gently fold in the milk to loosen the mixture.".to_string(),
            "Spoon the mixture into the prepared cake tins, spreading evenly with a spatula. Make a slight dip in the centre with the tip of the spatula if you don't want the cake to be domed in the middle.".to_string(),
            "Bake for 25 minutes, or until the cakes spring back when the centre is pressed gently with a finger.".to_string(),
            "Remove from the oven and take the cakes out of the tins after about 10 minutes. Place the cakes on a wire rack to cool completely.".to_string(),
            "To make the buttercream icing, sift half the icing sugar into a bowl.".to_string(),
            "Add the softened butter and (optional) cocoa powder then beat until light and fluffy.".to_string(),
            "Beat in the remaining icing sugar then beat in the vanilla extract and the milk.".to_string(),
            "Place one of the cakes upside down onto a cake board or stand. Using a palette knife or spatula, spread the icing onto the bottom layer of the cake, then place the second layer on top.".to_string(),
            "Decorate with smarties and marzipan lettering, if desired.".to_string(),
        ],
    }
}
