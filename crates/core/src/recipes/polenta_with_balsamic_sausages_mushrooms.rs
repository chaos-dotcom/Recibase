//! `se.reciba.api.recipes.PolentaWithBalsamicSausagesMushrooms`.
//! chaos-tag: casa-chaos

use crate::recipe::RecipeDef;
use crate::tag::Tag;
use chrono::NaiveDate;

pub fn recipe() -> RecipeDef {
    RecipeDef {
        object_name: "PolentaWithBalsamicSausagesMushrooms".to_string(),
        name: "Polenta with Balsamic Sausages & Mushrooms".to_string(),
        created_at: NaiveDate::from_ymd_opt(2026, 10, 3).unwrap(),
        permalink_override: None,
        source: Some("Waitrose".to_string()),
        description: None,
        tagline: Some("Soft polenta makes a wonderful alternative to mash or pasta.".to_string()),
        notes: vec![
            "Serves 4".to_string(),
            "Sausage dinners call for a smooth, mellow red like this gorgeous Chianti, which has flavours of plum, cherry and dried rosemary. No1 Piccini Organic Chianti Classico Riserva, Italy.".to_string(),
        ],
        tags: vec![
            Tag::VegetarianIsh,
            Tag::Scales,
            Tag::Stodge,
        ],
        image: None,
        ingredients_blocks: crate::recipe::IngredientsBlock::simple(vec![
            crate::recipe::Ingredient::q("Olive Oil", "1 tbsp"),
            crate::recipe::Ingredient::qp("Red Onion", "1 large", "sliced"),
            crate::recipe::Ingredient::q("Fennel Seeds", "1/2 tsp"),
            crate::recipe::Ingredient::q("No.1 Free Range Pork Sausages", "400g pack"),
            crate::recipe::Ingredient::qp("Chestnut Mushrooms", "450g", "sliced"),
            crate::recipe::Ingredient::q("Mazzetti Balsamic Vinegar of Modena Black Label", "2 tbsp"),
            crate::recipe::Ingredient::q("Salt", "1/2 tsp"),
            crate::recipe::Ingredient::q("Whole Milk", "200ml"),
            crate::recipe::Ingredient::q("Polenta Valsugana", "200g"),
            crate::recipe::Ingredient::q("Unsalted Butter", "30g"),
            crate::recipe::Ingredient::opt("Pecorino", Some("20g"), Some("grated"), Some("Or Parmigiano Reggiano")),
        ]),
        method: vec![
            "Heat the oil in your largest frying pan over a medium-high heat. Add the onion and fennel seeds with a pinch of salt. Break up the sausages (including the skins) and add to the pan. Stir, breaking up the sausages a little more, and fry for 1-2 minutes.".to_string(),
            "Turn the heat up to high and add the mushrooms to the pan. Season and fry for about 15 minutes, stirring regularly until the liquid has cooked off and everything is golden and caramelised, making sure the sausages have no pink meat and the juices run clear. Stir through the balsamic, then take off the heat.".to_string(),
            "In a large saucepan, bring 800ml water to the boil. Add the salt and milk, then whisk in the polenta. Lower the heat to medium; cook for 5 minutes, whisking regularly, until soft and smooth.".to_string(),
            "Take off the heat and stir in the butter and cheese. Add a splash of water to loosen the polenta, if liked. Divide the polenta and sausage mixture between plates, adding a handful of rocket, extra grated cheese and a splash of balsamic vinegar, if liked.".to_string(),
        ],
    }
}
