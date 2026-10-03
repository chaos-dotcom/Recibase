//! The `IceCream` mixin: shared content prepended/appended to ice cream recipes.

use crate::recipe::{Ingredient, IngredientsBlock};

pub fn generic_ingredients() -> IngredientsBlock {
    IngredientsBlock::new(
        None,
        vec![
            Ingredient::q("Mascarpone", "230g"),
            Ingredient::q("Eggs", "2"),
            Ingredient::q("Icing Sugar", "60g"),
        ],
    )
}

pub fn generic_notes() -> String {
    concat!(
        "I tend to double up this recipe to make enough for a few days.\n",
        "\n",
        "You can use up the egg whites by making ",
        "<a href=\"https://www.bbcgoodfood.com/recipes/easy-chocolate-mousse\" rel=\"nofollow\">",
        "chocolate mousse</a>."
    )
    .to_string()
}

pub fn generic_method_start() -> Vec<String> {
    vec![
        "Carefully separate the egg yolks using your hands or spoons. Put aside the egg whites for another recipe.".to_string(),
        "Whisk the egg yolks and sugar in a bowl, with an electric mixer, until thick and light in colour.".to_string(),
    ]
}

pub fn generic_method_end() -> Vec<String> {
    vec![
        "Decant into a freezer suitable dish and freeze for at least 6 hours.".to_string(),
    ]
}
