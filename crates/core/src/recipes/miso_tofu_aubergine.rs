//! `se.reciba.api.recipes.MisoTofuAubergine`.
//! chaos-tag: casa-chaos

use crate::recipe::RecipeDef;
use crate::tag::Tag;
use chrono::NaiveDate;

pub fn recipe() -> RecipeDef {
    RecipeDef {
        object_name: "MisoTofuAubergine".to_string(),
        name: "Miso Tofu & Aubergine".to_string(),
        created_at: NaiveDate::from_ymd_opt(2026, 10, 3).unwrap(),
        permalink_override: None,
        source: None,
        description: None,
        tagline: None,
        notes: vec![
            "Serves 3-4".to_string(),
            "Prepare 15 minutes; cook 20 minutes.".to_string(),
            "High in protein.".to_string(),
            "Per serving (for 3, not including optional ingredients): 1461kJ/352kcals/23g fat/2.9g saturated fat/12g carbs/8.2g sugars/8.3g fibre/19g protein/1.7g salt; 1 of your 5 a day; vegan.".to_string(),
            "Leftovers: brown rice miso. This aged miso paste is richer and deeper in flavour than white miso and can be used to make a simple miso soup or to enrich marinades and stews. It's a great way to add umami flavour to vegan and veggie dishes, or meaty stews.".to_string(),
        ],
        tags: vec![
            Tag::Vegan,
            Tag::Quick,
            Tag::LowEffort,
            Tag::Scales,
        ],
        image: None,
        ingredients_blocks: vec![
            crate::recipe::IngredientsBlock::new(None, vec![
                crate::recipe::Ingredient::q("Essential Sunflower Oil", "3 tbsp"),
                crate::recipe::Ingredient::opt("Essential Aubergines", Some("2"), Some("trimmed and cut into 2-3cm chunks"), None),
                crate::recipe::Ingredient::q("Clearspring Organic Tofu", "300g pack"),
                crate::recipe::Ingredient::qp("Garlic", "2 cloves", "finely sliced"),
                crate::recipe::Ingredient::qp("Ginger", "15g", "peeled and finely chopped"),
                crate::recipe::Ingredient::opt("Essential Salad Onions", Some("4"), Some("finely sliced"), Some("Greens and whites separated")),
                crate::recipe::Ingredient::opt("Sliced red chilli and steamed rice", None, None, Some("Optional, to serve")),
            ]),
            crate::recipe::IngredientsBlock::new(Some("Miso Sauce"), vec![
                crate::recipe::Ingredient::q("Clearspring Organic Brown Rice Miso", "1 1/2 tbsp"),
                crate::recipe::Ingredient::q("Cooks' Ingredients Shaoxing Rice Wine", "1 1/2 tbsp"),
                crate::recipe::Ingredient::q("Reduced-salt soy sauce", "1 1/2 tbsp"),
                crate::recipe::Ingredient::q("Maple Syrup", "1 tbsp"),
                crate::recipe::Ingredient::q("Cooks' Ingredients Japanese Rice Vinegar", "1 tsp"),
            ]),
        ],
        method: vec![
            "Heat 1 1/2 tbsp oil in a large frying pan that has a lid over a medium-high heat. Add the aubergines and fry, stirring occasionally, for 5 minutes. Add 2 tbsp water to the pan, cover with the lid and steam for 5 minutes, stirring halfway. Tip onto a plate and set aside.".to_string(),
            "Pat the tofu dry with kitchen paper. In a small bowl, mix the miso sauce ingredients with 2 tbsp water. Add the remaining 1 1/2 tbsp oil to the pan and fry the garlic, ginger and salad onion whites for 3-4 minutes until fragrant. Return the aubergines to the pan and fry for 2 minutes. Stir in the miso sauce and simmer for 2-3 minutes.".to_string(),
            "Cut the tofu into 2cm cubes and stir gently into the pan, coating in the sauce and heating until piping hot throughout. Scatter with the salad onion greens and serve with steamed rice and sliced red chilli, if liked.".to_string(),
        ],
    }
}
