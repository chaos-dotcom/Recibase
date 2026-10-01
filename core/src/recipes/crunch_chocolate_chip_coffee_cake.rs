//! `se.reciba.api.recipes.CrunchChocolateChipCoffeeCake`.

use crate::recipe::RecipeDef;
use crate::tag::Tag;
use chrono::NaiveDate;

pub fn recipe() -> RecipeDef {
    RecipeDef {
        object_name: "CrunchChocolateChipCoffeeCake".to_string(),
        name: "Crunch Chocolate Chip Coffee Cake".to_string(),
        created_at: NaiveDate::from_ymd_opt(2020, 4, 24).unwrap(),
        permalink_override: Some("coffee-cake".to_string()),
        source: None,
        description: None,
        tagline: None,
        notes: Vec::new(),
        tags: vec![
            Tag::Pudding,
            Tag::Baking,
            Tag::Vegetarian,
        ],
        image: None,
        ingredients_blocks: vec![
            crate::recipe::IngredientsBlock::new(Some("Cake"), vec![
                crate::recipe::Ingredient::q("Plain Flour", "375g"),
                crate::recipe::Ingredient::q("Baking Powder", "2 tsp"),
                crate::recipe::Ingredient::q("Baking Soda", "1/2 tsp"),
                crate::recipe::Ingredient::q("Salt", "1/4 tsp"),
                crate::recipe::Ingredient::q("Butter", "180g"),
                crate::recipe::Ingredient::qp("Cream Cheese", "250g", "soft"),
                crate::recipe::Ingredient::q("Granulated Sugar", "300g"),
                crate::recipe::Ingredient::q("Vanilla Extract", "1 tsp"),
                crate::recipe::Ingredient::q("Eggs", "3 large"),
                crate::recipe::Ingredient::q("Milk", "180ml"),
            ]),
            crate::recipe::IngredientsBlock::new(Some("Topping"), vec![
                crate::recipe::Ingredient::q("Dark Brown Sugar", "100g"),
                crate::recipe::Ingredient::q("Plain Flour", "75g"),
                crate::recipe::Ingredient::q("Butter", "60g"),
                crate::recipe::Ingredient::q("Dark Chocolate Chips", "180g"),
                crate::recipe::Ingredient::q("Walnuts", "60g"),
            ]),
        ],
        method: vec![
            format!("Preheat the oven to {}.", crate::utils::int_utils::celsius(180)),
            "Butter and flour a 13 x 9-inch (32 x 23-cm) baking pan.".to_string(),
            "Topping: Stir the sugar and flour in a medium bowl. Use a pastry blender to cut in the butter until the mixture resembles fine crumbs. Stir in the chocolate chips and walnuts.".to_string(),
            "Cake: Mix the flour, baking powder, baking soda and salt in a large bowl.".to_string(),
            "Beat the butter, cream cheese, sugar and vanilla in a large bowl with an electric mixer at medium speed until creamy.".to_string(),
            "Add the eggs, one at a time, beating until just blended after each addition.".to_string(),
            "With the mixer at low speed, gradually beat in the dry ingredients, alternating with the milk.".to_string(),
            "Spoon the batter into the prepared pan. Sprinkle with the topping.".to_string(),
            "Bake until a toothpick inserted into the centre comes out clean, 50-60 minutes.".to_string(),
            "Cool the cake completely in the pan on a wire rack.".to_string(),
        ],
    }
}
