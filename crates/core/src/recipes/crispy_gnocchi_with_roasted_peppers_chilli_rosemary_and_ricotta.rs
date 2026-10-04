//! `se.reciba.api.recipes.CrispyGnocchiWithRoastedPeppersChilliRosemaryAndRicotta`.
//! chaos-tag: casa-chaos

use crate::recipe::RecipeDef;
use crate::tag::Tag;
use chrono::NaiveDate;

pub fn recipe() -> RecipeDef {
    RecipeDef {
        object_name: "CrispyGnocchiWithRoastedPeppersChilliRosemaryAndRicotta".to_string(),
        name: "Crispy Gnocchi with Roasted Peppers, Chilli, Rosemary & Ricotta".to_string(),
        created_at: NaiveDate::from_ymd_opt(2026, 10, 4).unwrap(),
        permalink_override: None,
        source: None,
        description: Some("The gnocchi with mozzarella and tomatoes from the first Roasting Tin book was so popular that I decided to revisit it, as there are never too many ways to eat crispy gnocchi. This version, with roasted red peppers and rosemary, is a lovely alternative. Use a very large and ideally metal roasting tin, for maximum crunch on the potatoes.".to_string()),
        tagline: None,
        notes: vec![
            "Serves 2 generously".to_string(),
            "Prepare 15 minutes; cook 30 minutes.".to_string(),
        ],
        tags: vec![
            Tag::Vegetarian,
            Tag::LowEffort,
            Tag::Scales,
            Tag::Stodge,
            Tag::ColdWeather,
        ],
        image: None,
        ingredients_blocks: crate::recipe::IngredientsBlock::simple(vec![
            crate::recipe::Ingredient::q("Gnocchi", "500g"),
            crate::recipe::Ingredient::qp("Mixed red, yellow and baby peppers", "500g", "roughly chopped"),
            crate::recipe::Ingredient::qp("Cherry tomatoes", "200g", "halved"),
            crate::recipe::Ingredient::q("Olive oil", "2 tablespoons"),
            crate::recipe::Ingredient::q("Bay leaves", "2"),
            crate::recipe::Ingredient::q("Garlic", "2 cloves"),
            crate::recipe::Ingredient::q("Chilli flakes", "1 teaspoon"),
            crate::recipe::Ingredient::q("Fresh rosemary", "2 large sprigs"),
            crate::recipe::Ingredient::q("Sea salt", "1 teaspoon"),
            crate::recipe::Ingredient::new("Freshly ground black pepper"),
            crate::recipe::Ingredient::q("Ricotta", "4 tablespoons"),
            crate::recipe::Ingredient::qp("Parsley", "A handful", "freshly chopped"),
        ]),
        method: vec![
            "Preheat the oven to 200°C fan/220°C/gas 7. Tip the gnocchi into a large bowl, then pour a kettleful of boiling water over it and leave to stand for 2 minutes before draining well.".to_string(),
            "Tip the gnocchi into a roasting tin along with everything except the ricotta. Mix well - make sure you've used a tin big enough for everything to fit in one layer. Transfer to the oven and cook for 30 minutes, until the gnocchi is crisp and golden.".to_string(),
            "Taste and season with salt and pepper as needed, dollop on the ricotta and scatter with the parsley before serving hot.".to_string(),
        ],
    }
}
