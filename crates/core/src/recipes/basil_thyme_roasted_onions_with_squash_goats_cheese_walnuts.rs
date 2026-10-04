//! `se.reciba.api.recipes.BasilThymeRoastedOnionsWithSquashGoatsCheeseWalnuts`.
//! chaos-tag: casa-chaos

use crate::recipe::RecipeDef;
use crate::tag::Tag;
use chrono::NaiveDate;

pub fn recipe() -> RecipeDef {
    RecipeDef {
        object_name: "BasilThymeRoastedOnionsWithSquashGoatsCheeseWalnuts".to_string(),
        name: "Basil & Thyme Roasted Onions with Squash, Goat's Cheese & Walnuts".to_string(),
        created_at: NaiveDate::from_ymd_opt(2026, 10, 4).unwrap(),
        permalink_override: None,
        source: None,
        description: Some("This is such a lovely autumnal dish and looks beautiful when brought to the table: it's the favourite of this book's designer, Pene. Serve with a green salad and good bread.".to_string()),
        tagline: None,
        notes: vec![
            "Serves 4".to_string(),
            "Prepare 10 minutes; cook 50 minutes.".to_string(),
            "Note: if your onions are large, give them 50 minutes in the oven before topping with the goat's cheese.".to_string(),
        ],
        tags: vec![
            Tag::Vegetarian,
            Tag::Slow,
            Tag::ColdWeather,
            Tag::Scales,
        ],
        image: None,
        ingredients_blocks: vec![
            crate::recipe::IngredientsBlock::new(None, vec![
                crate::recipe::Ingredient::qp("Butternut squash", "600g", "peeled and cut into 1cm chunks"),
                crate::recipe::Ingredient::qp("Onions", "4 medium", "halved"),
                crate::recipe::Ingredient::q("Olive oil", "1 tablespoon"),
                crate::recipe::Ingredient::q("Sea salt", "1 teaspoon"),
                crate::recipe::Ingredient::new("Freshly ground black pepper"),
                crate::recipe::Ingredient::qp("Garlic", "4 cloves", "unpeeled and halved"),
                crate::recipe::Ingredient::q("Fresh basil leaves", "8"),
                crate::recipe::Ingredient::q("Fresh thyme sprigs", "A handful"),
                crate::recipe::Ingredient::opt("Soft goat's cheese", Some("125g"), None, Some("Without rind")),
                crate::recipe::Ingredient::qp("Toasted walnuts", "A handful", "roughly broken"),
            ]),
            crate::recipe::IngredientsBlock::new(Some("Dressing"), vec![
                crate::recipe::Ingredient::qp("Fresh basil", "30g", "very finely chopped"),
                crate::recipe::Ingredient::q("Olive oil", "3 tablespoons"),
                crate::recipe::Ingredient::q("Sea salt", "1 teaspoon"),
                crate::recipe::Ingredient::q("Lemon juice", "1/2 tablespoon"),
            ]),
        ],
        method: vec![
            "Preheat the oven to 200°C fan/220°C/gas 7. Arrange the squash and onions in a single layer in a roasting tin or large lasagne dish, the onions cut side up. Mix everything well with the olive oil, then scatter over the salt and black pepper.".to_string(),
            "Top each halved onion with half a clove of garlic, a basil leaf and a sprig of thyme and scatter the remaining thyme over the squash. Transfer to the oven and roast for 40 minutes.".to_string(),
            "Remove the dish from the oven and roughly break the goat's cheese over the onions and squash in large chunks, making sure each onion gets a piece of cheese over it. Scatter over the walnuts, then return to the oven for a further 10 minutes, until the onions are soft through when pierced with a fork.".to_string(),
            "Meanwhile, mix the chopped basil, olive oil, sea salt and lemon juice together. Once the onions are ready, pour over the dressing, and serve hot.".to_string(),
        ],
    }
}
