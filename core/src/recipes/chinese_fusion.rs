//! `se.reciba.api.recipes.ChineseFusion`.

use crate::recipe::RecipeDef;
use crate::tag::Tag;
use chrono::NaiveDate;

pub fn recipe() -> RecipeDef {
    RecipeDef {
        object_name: "ChineseFusion".to_string(),
        name: "Chinese Fusion with Hoisin".to_string(),
        created_at: NaiveDate::from_ymd_opt(2023, 9, 8).unwrap(),
        permalink_override: Some("chinese-fusion".to_string()),
        source: Some("Stephani".to_string()),
        description: None,
        tagline: None,
        notes: vec![
            "You can add the salt during cooking rather than at the end.".to_string(),
            "Yes I know, using a wok on a medium heat, blah blah blah.".to_string(),
        ],
        tags: vec![Tag::Stephani],
        image: None,
        ingredients_blocks: crate::recipe::IngredientsBlock::simple(vec![
            crate::recipe::Ingredient::q("Butternut Squash", "1"),
            crate::recipe::Ingredient::opt(
                "Lamb Mince",
                Some("500g"),
                None,
                Some("alternatively use turkey thigh mince, chicken thigh fillet also works"),
            ),
            crate::recipe::Ingredient::q("Leeks", "2 large"),
            crate::recipe::Ingredient::q("Shiitake mushrooms", "125g"),
            crate::recipe::Ingredient::q("Garlic", "4 cloves"),
            crate::recipe::Ingredient::q("Jar Ginger", "2 fork-fulls"),
            crate::recipe::Ingredient::qp("Sliced Water chestnuts", "1 tin", "drained"),
            crate::recipe::Ingredient::qp("Sliced bamboo", "1 tin", "drained"),
            crate::recipe::Ingredient::new("Five spice"),
            crate::recipe::Ingredient::new("Rich hoisin sauce"),
            crate::recipe::Ingredient::new("Noodles"),
            crate::recipe::Ingredient::new("Oil"),
            crate::recipe::Ingredient::new("Salt"),
        ]),
        method: vec![
            format!(
                "Preheat the oven to {}.",
                crate::utils::int_utils::celsius(200)
            ),
            "Peel butternut squash and cut into 3-4cm cubes.".to_string(),
            "Place on roasting tray, mix with oil and roast for 80-120 minutes.".to_string(),
            "Slice leeks into 7mm slices. Finely chop the garlic.".to_string(),
            "Add some oil to a large wok and heat over a medium flame.".to_string(),
            "Fry the leeks, garlic and ginger in the wok until reduced.".to_string(),
            "Add the mince and five spice then brown.".to_string(),
            "Add mushrooms and cook until the water is almost gone.".to_string(),
            "Mix in the butternut squash, water chestnuts and sliced bamboo.".to_string(),
            "Cook the noodles per packet instructions.".to_string(),
            "Serve on top of the noodles. Add salt and hoisin sauce to taste.".to_string(),
        ],
    }
}
