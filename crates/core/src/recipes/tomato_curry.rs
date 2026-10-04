//! `se.reciba.api.recipes.TomatoCurry`.
//! chaos-tag: casa-chaos

use crate::recipe::RecipeDef;
use crate::tag::Tag;
use chrono::NaiveDate;

pub fn recipe() -> RecipeDef {
    RecipeDef {
        object_name: "TomatoCurry".to_string(),
        name: "Tomato Curry".to_string(),
        created_at: NaiveDate::from_ymd_opt(2026, 10, 4).unwrap(),
        permalink_override: None,
        source: None,
        description: Some("There are just a few Indian dishes that truly celebrate the tomato, such as Keralan tomato fry and Gujarati sev tameta nu shaak (a sweet and sour tomato curry), but it's thakkali kuzhambu, from Tamil Nadu, on which this recipe is (very) loosely based. The sweetness and acidity of tomatoes is married to classic pickling spices, then tempered with curry leaves, tamarind and coconut: the ingredients that define South Indian cooking. This dish has a magic moment when all the water in the coconut milk evaporates to render the oil, leaving you with a silky, luxurious heap of deliciousness that's perfect for scooping up with naan bread or mixing into hot rice.".to_string()),
        tagline: None,
        notes: vec![
            "Serves 4 as a main".to_string(),
            "Note: You'll need two large frying pans for this recipe.".to_string(),
        ],
        tags: vec![Tag::Vegan, Tag::Spicy, Tag::Scales],
        image: None,
        ingredients_blocks: crate::recipe::IngredientsBlock::simple(vec![
            crate::recipe::Ingredient::q("Fennel Seeds", "1 1/4 tsp"),
            crate::recipe::Ingredient::q("Black Mustard Seeds", "1 1/4 tsp"),
            crate::recipe::Ingredient::q("Cumin Seeds", "1 1/4 tsp"),
            crate::recipe::Ingredient::q("Coriander Seeds", "1 1/4 tsp"),
            crate::recipe::Ingredient::new("Rapeseed Oil"),
            crate::recipe::Ingredient::qp("Onions", "2", "halved and finely sliced"),
            crate::recipe::Ingredient::q("Salt", "1 1/4 tsp"),
            crate::recipe::Ingredient::opt(
                "Fresh Curry Leaves",
                Some("8"),
                None,
                Some("plus extra to garnish if you like"),
            ),
            crate::recipe::Ingredient::opt(
                "Tomatoes",
                Some("1.2 kg"),
                None,
                Some("ideally 1kg vine and 200g yellow baby plum"),
            ),
            crate::recipe::Ingredient::qp("Green Finger Chillies", "1 1/2", "very finely chopped"),
            crate::recipe::Ingredient::qp("Garlic", "4 cloves", "crushed"),
            crate::recipe::Ingredient::q("Tamarind Paste", "2 1/2 tsp"),
            crate::recipe::Ingredient::q("Coconut Milk", "1 x 400ml tin"),
        ]),
        method: vec![
            "Heat a large frying pan on a medium flame and, when hot, toast the fennel, mustard, cumin and coriander seeds for a minute or two, shaking the pan every few seconds, until the coriander seeds turn golden (coriander always takes first). Tip the seeds into a mortar and bash until fairly well ground.".to_string(),
            "Heat 4 tablespoons of oil in the same pan and, when hot, return the ground spices with the onions, salt and curry leaves. Fry for 10 to 12 minutes, until the onions are golden and crisp-edged. Meanwhile, cut the vine tomatoes into eighths and the baby tomatoes in half.".to_string(),
            "Add the chillies and garlic to the pan and cook, stirring, for 2 minutes.".to_string(),
            "Then add the tamarind and coconut milk, stir, and transfer half the mixture into your second large frying pan.".to_string(),
            "Divide the tomatoes between both pans, so they sit in one layer. Set both pans on a medium heat and cook for 20 to 25 minutes without stirring: you want the tomatoes to keep their shape while driving off the water in the coconut milk. You'll know there's none left when you can see oil at the sides of the pan. (The curry won't be dry: the tomatoes contain a lot of juice, which will come out while they're resting.) Now tip the contents of the second pan gently back into the first.".to_string(),
            "If you'd like to add a final bit of pizazz, heat a little oil in a saucepan and, when hot, drop in a handful of extra curry leaves. Let them crackle and crisp, then take off the heat and pour over the tomatoes.".to_string(),
            "Serve with naan or rice.".to_string(),
        ],
    }
}
