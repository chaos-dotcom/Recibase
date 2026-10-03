//! `se.reciba.api.recipes.BirthdayCake`.

use crate::recipe::RecipeDef;
use crate::tag::Tag;
use chrono::NaiveDate;

pub fn recipe() -> RecipeDef {
    RecipeDef {
        object_name: "BirthdayCake".to_string(),
        name: "Birthday Cake (classic)".to_string(),
        created_at: NaiveDate::from_ymd_opt(2023, 8, 8).unwrap(),
        permalink_override: None,
        source: Some("Kit's Mum".to_string()),
        description: Some("My mum's classic madeira cake recipe. Made with buttercream icing and topped with Smarties!".to_string()),
        tagline: None,
        notes: vec!["You can optionally slice off the top of the cake, to provide a flat surface for easier decoration.".to_string()],
        tags: vec![
            Tag::Pudding,
            Tag::Baking,
        ],
        image: Some(crate::recipe::Image::new("https://i.reciba.se/birthday-cake.jpg")),
        ingredients_blocks: crate::recipe::IngredientsBlock::simple(vec![
            crate::recipe::Ingredient::q("Butter", "5oz"),
            crate::recipe::Ingredient::q("Caster Sugar", "5oz"),
            crate::recipe::Ingredient::q("Eggs", "3"),
            crate::recipe::Ingredient::q("Self raising flour", "8oz"),
            crate::recipe::Ingredient::q("Lemon essence", "1/4 tsp"),
            crate::recipe::Ingredient::q("Unsalted Butter", "100g"),
            crate::recipe::Ingredient::q("Icing Sugar", "200g"),
            crate::recipe::Ingredient::q("Milk", "1 tbsp"),
            crate::recipe::Ingredient::q("Smarties", "136g"),
            crate::recipe::Ingredient::opt("Vanilla Extract", Some("1 tsp"), None, Some("buy vanilla extract, not vanilla flavouring")),
            crate::recipe::Ingredient::opt("Marzipan", None, None, Some("optional, if you want to decorate with lettering")),
        ]),
        method: vec![
            "Pre-heat the oven to 165-180 C (325-350 F, gas mark 3-4).".to_string(),
            "Cream the salted butter and sugar.".to_string(),
            "Lightly whisk the eggs and add to the mixture.".to_string(),
            "Add the lemon essence.".to_string(),
            "Fold in the flour.".to_string(),
            "Put the mixture in a deep 6\" or 7\" cake tin.".to_string(),
            "Bake in the oven for 1 1/4 hours.".to_string(),
            "Remove from the oven, leave to cool.".to_string(),
            "Optional: Slice horizontally in two, to allow for an icing filling.".to_string(),
            "For the icing: beat the unsalted butter and sugar together until smooth.".to_string(),
            "Beat in the milk and vanilla extract.".to_string(),
            "Carefully spread it over the cake and in the middle of the two halves (if applicable).".to_string(),
            "Decorate with smarties.".to_string(),
            "Optional: Roll out the marzipan, create message with letter cutters and add to top of cake.".to_string(),
        ],
    }
}
