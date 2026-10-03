//! `se.reciba.api.recipes.RoastedVegetableTart`.

use crate::recipe::RecipeDef;
use crate::tag::Tag;
use chrono::NaiveDate;

pub fn recipe() -> RecipeDef {
    RecipeDef {
        object_name: "RoastedVegetableTart".to_string(),
        name: "Roasted Vegetable Tart".to_string(),
        created_at: NaiveDate::from_ymd_opt(2020, 4, 24).unwrap(),
        permalink_override: None,
        source: None,
        description: None,
        tagline: None,
        notes: Vec::new(),
        tags: vec![Tag::Vegetarian, Tag::HotWeather, Tag::Slow],
        image: None,
        ingredients_blocks: crate::recipe::IngredientsBlock::simple(vec![
            crate::recipe::Ingredient::q("Aubergines", "400g"),
            crate::recipe::Ingredient::q("Courgettes", "400g"),
            crate::recipe::Ingredient::qp("Red Pepper", "1", "thickly sliced"),
            crate::recipe::Ingredient::qp("Yellow Pepper", "1", "thickly sliced"),
            crate::recipe::Ingredient::qp("Red Onions", "150g", "thickly sliced"),
            crate::recipe::Ingredient::qp("Garlic", "1 clove", "chopped"),
            crate::recipe::Ingredient::qp("Fresh Rosemary or Thyme", "1 tsp", "chopped, plus extra fresh sprigs for topping"),
            crate::recipe::Ingredient::q("Olive Oil", "3 tbsp"),
            crate::recipe::Ingredient::new("Salt"),
            crate::recipe::Ingredient::new("Black Pepper"),
            crate::recipe::Ingredient::qp("Filo Pastry", "4 sheets", "about 125g altogether"),
            crate::recipe::Ingredient::qp("Feta", "150g", "diced"),
        ]),
        method: vec![
            format!("Heat the oven to {}. Cut the aubergines and courgettes into 1cm slices. Arrange all the vegetables in a single layer in a roasting tin, scatter the garlic and rosemary or thyme over them, then drizzle with 2 tablespoons of the olive oil. Season to taste.", crate::utils::int_utils::celsius(200)),
            "Roast the vegetables for 40-60 minutes until they have softened and browned.".to_string(),
            "Meanwhile, place a baking sheet in the oven to warm. Line a 20-23cm loose-based tart tin with the sheets of filo pastry, brushing each layer with oil before adding the next. Crumple up any overhanging edges to form a rim. Place the tin on the baking sheet and bake the pastry case for 5-8 minutes until golden brown.".to_string(),
            format!("Reduce the oven to {}. Spoon the roasted vegetables into the pastry case and scatter the cheese and chopped rosemary and thyme evenly over the top. Return the tart to the oven for 10 minutes, or until the cheese has just melted. Cut into quarters and serve.", crate::utils::int_utils::celsius(160)),
        ],
    }
}
