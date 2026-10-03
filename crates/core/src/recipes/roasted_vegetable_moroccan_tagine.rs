//! `se.reciba.api.recipes.RoastedVegetableMoroccanTagine`.

use crate::recipe::RecipeDef;
use crate::tag::Tag;
use chrono::NaiveDate;

pub fn recipe() -> RecipeDef {
    RecipeDef {
        object_name: "RoastedVegetableMoroccanTagine".to_string(),
        name: "Roasted Vegetable Moroccan Tagine".to_string(),
        created_at: NaiveDate::from_ymd_opt(2022, 3, 27).unwrap(),
        permalink_override: None,
        source: Some("https://www.onegreenplanet.org/vegan-recipe/delicious-roasted-veggie-moroccan-tagine/".to_string()),
        description: None,
        tagline: None,
        notes: Vec::new(),
        tags: vec![Tag::Vegan, Tag::Scales, Tag::Slow],
        image: None,
        ingredients_blocks: crate::recipe::IngredientsBlock::simple(vec![
            crate::recipe::Ingredient::qp("Aubergine", "1", "cut into 2cm chunks"),
            crate::recipe::Ingredient::qp("Carrots", "2", "cut into quarters"),
            crate::recipe::Ingredient::qp("Pepper", "1", "cut into quarters"),
            crate::recipe::Ingredient::qp("Sweet potato", "1", "cut into 3cm chunks"),
            crate::recipe::Ingredient::qp("Garlic", "4 cloves", "peel the skin but leave whole"),
            crate::recipe::Ingredient::qp("Broccoli", "1/2", "cut into florets"),
            crate::recipe::Ingredient::qp("Onion", "1", "roughly chopped"),
            crate::recipe::Ingredient::q("Rosemary", "1/2 tsp"),
            crate::recipe::Ingredient::q("Ground Cumin", "1/2 tsp"),
            crate::recipe::Ingredient::q("Ground Coriander", "1/2 tsp"),
            crate::recipe::Ingredient::q("Turmeric Powder", "1/2 tsp"),
            crate::recipe::Ingredient::qp("Chickpeas", "1 tin", "drained"),
            crate::recipe::Ingredient::q("Tinned Tomatoes", "1 tin"),
            crate::recipe::Ingredient::q("Harissa Paste", "1 tsp"),
            crate::recipe::Ingredient::opt("Honey", Some("1 tsp"), None, Some("can use maple syrup instead")),
            crate::recipe::Ingredient::q("Stock Cube", "1"),
            crate::recipe::Ingredient::opt("Couscous", Some("200g"), None, Some("can serve with rice instead")),
            crate::recipe::Ingredient::new("Olive Oil"),
            crate::recipe::Ingredient::new("Salt"),
        ]),
        method: vec![
            format!("Preheat the oven to {}.", crate::utils::int_utils::celsius(200)),
            "Spread the aubergine, carrots, pepper and sweet potato across a large roasting tin. Drizzle with oil and sprinkle with salt and rosemary.".to_string(),
            "Roast the vegetables for 20m or until lightly browned. Set aside once they're ready.".to_string(),
            "Boil some water and make 240ml of stock. Mix in the chopped tomatoes, harissa paste, cumin, ground coriander, turmeric and honey. Set aside so the flavours can develop.".to_string(),
            "Wait until the vegetables have been roasting for 15 minutes.".to_string(),
            "Heat some olive oil in a large pan. Everything will eventually be added to this pan, so make sure it's big enough.".to_string(),
            "Add the onions, garlic and broccoli to the pan and saute over a medium heat for 5-10 minutes, or until the garlic and onions are slightly browned.".to_string(),
            "Add the roast vegetables and chickpeas to the pan.".to_string(),
            "Cover the pan and simmer for 15-20 minutes.".to_string(),
            "Cook the couscous, per packet instructions.".to_string(),
            "Serve the tagine with a side of couscous.".to_string(),
        ],
    }
}
