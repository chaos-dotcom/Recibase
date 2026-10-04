//! `se.reciba.api.recipes.LemonPeaRisotto`.
//! chaos-tag: casa-chaos

use crate::recipe::RecipeDef;
use crate::tag::Tag;
use chrono::NaiveDate;

pub fn recipe() -> RecipeDef {
    RecipeDef {
        object_name: "LemonPeaRisotto".to_string(),
        name: "Lemon & Pea Risotto".to_string(),
        created_at: NaiveDate::from_ymd_opt(2026, 10, 3).unwrap(),
        permalink_override: None,
        source: Some("Waitrose".to_string()),
        description: None,
        tagline: None,
        notes: vec![
            "Serves 4".to_string(),
            "Prepare 10 minutes; cook 30 minutes.".to_string(),
            "Cook's tip: For extra flavour, grill slices of prosciutto crudo until crisp, then snap into shards once cool and scatter over the risotto. Toasted, chopped hazelnuts would be a lovely addition, too.".to_string(),
            "Per serving: 1721kJ/411kcals/16g fat/9.4g saturated fat/48g carbs/5.8g sugars/7.1g fibre/14g protein/3.3g salt".to_string(),
        ],
        tags: vec![Tag::Vegetarian, Tag::LowEffort, Tag::HotWeather],
        image: None,
        ingredients_blocks: crate::recipe::IngredientsBlock::simple(vec![
            crate::recipe::Ingredient::opt("Chicken or vegetable stock", Some("850ml"), None, Some("Made up with 2 stock cubes")),
            crate::recipe::Ingredient::q("Essential Unsalted Butter", "50g"),
            crate::recipe::Ingredient::qp("Garlic", "1 clove", "finely chopped"),
            crate::recipe::Ingredient::opt("Cooks' Ingredients Unwaxed Lemon", Some("1"), Some("zest grated"), Some("juice of 1/2; 1/2 cut into wedges")),
            crate::recipe::Ingredient::q("Arborio risotto rice", "200g"),
            crate::recipe::Ingredient::q("Frozen Essential Petits Pois", "400g"),
            crate::recipe::Ingredient::qp("Parmigiano Reggiano", "50g", "finely grated"),
            crate::recipe::Ingredient::opt("Lemon thyme", Some("1/4 x 20g pack"), None, Some("leaves picked")),
        ]),
        method: vec![
            "Put the stock in a small saucepan, bring to a simmer and keep warm on the lowest heat. Melt 25g butter in a deep frying or sauté pan, add the garlic and most of the grated lemon zest, cook gently for a few minutes until fragrant. Stir in the rice and toss in the garlicky lemon butter. Begin to add the stock, a ladleful at a time, stirring constantly and allowing the rice to absorb the stock before adding more, until the rice is tender and the risotto is creamy (20-25 minutes).".to_string(),
            "Meanwhile, bring a small pan of salted water to the boil and cook the peas for 4 minutes; drain. Set 1/2 aside, then transfer the rest to a blender (or use a stick blender) and whizz with the lemon juice until smooth.".to_string(),
            "Stir the pea purée into the cooked risotto along with the remaining 25g butter, most of the cheese, the lemon thyme and the reserved peas. Spoon the risotto into bowls and top with extra cheese and the reserved lemon zest. Serve with the lemon wedges alongside for squeezing over.".to_string(),
        ],
    }
}
