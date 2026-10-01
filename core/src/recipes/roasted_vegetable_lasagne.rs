//! `se.reciba.api.recipes.RoastedVegetableLasagne`.

use crate::recipe::RecipeDef;
use crate::tag::Tag;
use chrono::NaiveDate;

pub fn recipe() -> RecipeDef {
    RecipeDef {
        object_name: "RoastedVegetableLasagne".to_string(),
        name: "Roasted Vegetable Lasagne".to_string(),
        created_at: NaiveDate::from_ymd_opt(2020, 4, 24).unwrap(),
        permalink_override: None,
        source: Some("https://www.bbcgoodfood.com/recipes/10603/roasted-vegetable-lasagne".to_string()),
        description: None,
        tagline: None,
        notes: vec![
            "This takes a while to make so leave lots of time.".to_string(),
            "Don't worry if the roasted vegetables are ready early. Just take them out and put them to one side, or decant them onto a plate.".to_string(),
        ],
        tags: vec![
            Tag::Slow,
            Tag::HighEffort,
            Tag::Vegetarian,
            Tag::Scales,
        ],
        image: None,
        ingredients_blocks: crate::recipe::IngredientsBlock::simple(vec![
            crate::recipe::Ingredient::q("Red Peppers", "3"),
            crate::recipe::Ingredient::q("Aubergines", "2"),
            crate::recipe::Ingredient::qp("Onions", "2", "diced"),
            crate::recipe::Ingredient::q("Garlic Cloves", "2"),
            crate::recipe::Ingredient::qp("Carrot", "1", "roughly chopped"),
            crate::recipe::Ingredient::q("Tomato Puree", "2 tbsp"),
            crate::recipe::Ingredient::new("Red or White Wine"),
            crate::recipe::Ingredient::q("Chopped Tomatoes", "3 tins"),
            crate::recipe::Ingredient::opt("Dried mixed herbs", Some("handful"), None, Some("Italian herb blends or just plain oregano works")),
            crate::recipe::Ingredient::q("Butter", "knob"),
            crate::recipe::Ingredient::new("Plain Flour"),
            crate::recipe::Ingredient::new("Milk"),
            crate::recipe::Ingredient::q("Fresh Lasagne sheets", "300g"),
            crate::recipe::Ingredient::opt("Mozzarella", Some("125g"), None, Some("I'd buy extra just to be safe. Can always nom the rest")),
            crate::recipe::Ingredient::new("Olive Oil"),
            crate::recipe::Ingredient::opt("Cherry Tomatoes", None, Some("Halved"), Some("Optional")),
            crate::recipe::Ingredient::opt("Fresh Basil", None, None, Some("Optional")),
        ]),
        method: vec![
            "Pre-heat the oven to 200C/fan 180C/gas 6.".to_string(),
            "Cut the peppers into large chunks. Cut the aubergines into slices about 1/2cm thick.".to_string(),
            "Lightly grease 2 large baking trays, then place peppers and aubergines on top. Toss with the olive oil, season well, then roast for 25 mins until lightly browned. Meanwhile, make the red and white sauces.".to_string(),
            "To make the red sauce, add the garlic and some oil to a saucepan over a medium heat.".to_string(),
            "Add the onions and carrots and cook until softened.".to_string(),
            "Turn up the heat, add the tomato purée, then cook for a further minute.".to_string(),
            "Add the chopped tomatoes, dried mixed herbs and a generous slosh of wine. Bring to the boil then leave to simmer for at least 20 minutes.".to_string(),
            "For the white sauce, melt the butter in a saucepan over a low heat.".to_string(),
            "Sift in the flour, stirring regularly, until you have a very thick mixture.".to_string(),
            "Switch to a medium heat and slowly add the milk until the sauce pours off the spoon. Be warned, there's a delay after adding the milk before it thickens, turn up the heat slightly if it's taking a long time.".to_string(),
            "Reduce the oven to 180C/fan 160C/gas 4 and lightly oil an ovenproof serving dish (30 x 20cm).".to_string(),
            "Arrange a layer of the roasted vegetables on the bottom, then pour over a third of the red sauce. Top with a layer of lasagne, then drizzle over a quarter of the white sauce. Repeat until you have 3 layers of pasta.".to_string(),
            "To finish, spoon remaining white sauce over the pasta, making sure the whole surface is covered. Scatter mozzarella over the top. Bake for 45 mins until bubbling and golden.".to_string(),
            "Scatter the cherry tomatoes and basil over the top then serve.".to_string(),
        ],
    }
}
