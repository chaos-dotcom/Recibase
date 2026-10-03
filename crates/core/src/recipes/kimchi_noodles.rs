//! `se.reciba.api.recipes.KimchiNoodles`.
//! chaos-tag: casa-chaos

use crate::recipe::RecipeDef;
use crate::tag::Tag;
use chrono::NaiveDate;

pub fn recipe() -> RecipeDef {
    RecipeDef {
        object_name: "KimchiNoodles".to_string(),
        name: "Kimchi Noodles".to_string(),
        created_at: NaiveDate::from_ymd_opt(2026, 10, 3).unwrap(),
        permalink_override: None,
        source: None,
        description: Some("Who says you have to go out for ramen? Here's how to make your own in 20 minutes.".to_string()),
        tagline: None,
        notes: vec![
            "Serves 2".to_string(),
            "Prepare 5 minutes; cook 20 minutes.".to_string(),
            "Per serving: 1944kJ/463kcals/15g fat/2.4g saturated fat/52g carbs/6.7g sugars/8.9g fibre/25g protein/4.8g salt; vegetarian.".to_string(),
            "Leftovers: kimchi. A traditional Korean accompaniment of salted, fermented vegetables, kimchi has a punchy, piquant flavour that makes a great addition to fried rice, toasted cheese sandwiches, gratins and scrambled eggs.".to_string(),
        ],
        tags: vec![
            Tag::Soup,
            Tag::Vegetarian,
            Tag::Quick,
            Tag::LowEffort,
            Tag::Scales,
            Tag::ColdWeather,
        ],
        image: None,
        ingredients_blocks: crate::recipe::IngredientsBlock::simple(vec![
            crate::recipe::Ingredient::qp("Essential Edamame Beans", "75g", "frozen"),
            crate::recipe::Ingredient::q("Essential Free Range White Eggs", "2"),
            crate::recipe::Ingredient::q("Itsu Ramen Noodles", "1/2 x 250g pack"),
            crate::recipe::Ingredient::opt("Vegetable Oil", Some("1/2 tbsp"), None, Some("Plus a dash")),
            crate::recipe::Ingredient::opt("Kimchi", Some("100g"), None, Some("We used Vadasz Raw Kimchi")),
            crate::recipe::Ingredient::q("Itsu Classic Ramen Brilliant Broth", "500ml pack"),
            crate::recipe::Ingredient::qp("Shiitake Mushrooms", "1/2 x 150g pack", "trimmed and finely sliced"),
            crate::recipe::Ingredient::qp("Essential Salad Onions", "2", "finely sliced"),
            crate::recipe::Ingredient::opt("Itsu Crispy Seaweed Thins", Some("6"), Some("finely snipped"), Some("Taken from a 4 x 5g pack")),
        ]),
        method: vec![
            "Cook the edamame beans according to pack instructions. Bring a large saucepan of water to the boil, add the eggs and simmer for 6 1/2 minutes, then drain and put in a bowl of ice-cold water. Meanwhile, bring another pan of water to the boil, add the noodles and simmer for 5 minutes. Once cooked, rinse under cold water to cool completely, drain well, then toss with a dash of oil to stop them from sticking and set aside.".to_string(),
            "Rinse out the pan; return to a medium-high heat with 1/2 tbsp oil. Add the kimchi and fry for 2 minutes, then add the ramen broth and shiitake mushrooms. Bring to the boil, then simmer briskly for 4 minutes, adding the cooked edamame at the end. Meanwhile, peel and halve the eggs.".to_string(),
            "Divide the noodles between 2 large bowls and pour over the piping hot broth. Top with the halved eggs, salad onions and seaweed. Serve immediately.".to_string(),
        ],
    }
}
