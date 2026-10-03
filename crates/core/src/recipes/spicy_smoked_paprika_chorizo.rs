//! `se.reciba.api.recipes.SpicySmokedPaprikaChorizo`.

use crate::recipe::RecipeDef;
use crate::tag::Tag;
use chrono::NaiveDate;

pub fn recipe() -> RecipeDef {
    RecipeDef {
        object_name: "SpicySmokedPaprikaChorizo".to_string(),
        name: "Spicy Smoked Paprika Chorizo".to_string(),
        created_at: NaiveDate::from_ymd_opt(2020, 4, 24).unwrap(),
        permalink_override: None,
        source: Some("Kit's Dad".to_string()),
        description: None,
        tagline: None,
        notes: vec![
            "If your chorio is very spicy you might not need any chilli flakes. Similarly if you use chilli oil.".to_string(),
            "This dish tastes best if you leave it to cool then reheat it.".to_string(),
            "You could try serving this dish over grilled aubergines.".to_string(),
        ],
        tags: vec![
            Tag::Spicy,
            Tag::Scales,
            Tag::BetterNextDay,
            Tag::Slow,
        ],
        image: None,
        ingredients_blocks: crate::recipe::IngredientsBlock::simple(vec![
            crate::recipe::Ingredient::opt("Spanish Chorizo", Some("225g"), None, Some("ideally good quality")),
            crate::recipe::Ingredient::opt("Red Onions", Some("4"), Some("finely diced"), None),
            crate::recipe::Ingredient::opt("Garlic Cloves", Some("Several"), Some("fined diced or crushed in a press"), None),
            crate::recipe::Ingredient::opt("Pointed Red Peppers", Some("2"), Some("sliced"), None),
            crate::recipe::Ingredient::opt("Carrots", Some("2-3"), Some("diced"), None),
            crate::recipe::Ingredient::opt("Fresh Spinach", Some("100g"), Some("torn"), Some("frozen would also work")),
            crate::recipe::Ingredient::q("Tinned Tomatoes", "3"),
            crate::recipe::Ingredient::q("Tomato Paste", "4 Inches"),
            crate::recipe::Ingredient::opt("Red Wine", Some("10-20CL"), None, Some("something in the £5-10 range")),
            crate::recipe::Ingredient::q("Smoked Paprika", "Several teaspoons"),
            crate::recipe::Ingredient::new("Olive Oil"),
            crate::recipe::Ingredient::new("Chilli Flakes"),
            crate::recipe::Ingredient::opt("Honey", Some("2 tsp"), None, Some("anything but that Rowse shit")),
            crate::recipe::Ingredient::q("Dried Oregano", "1 tbsp"),
            crate::recipe::Ingredient::opt("Stick of Cinnamon", Some("1"), None, Some("Ground also works")),
            crate::recipe::Ingredient::opt("Pitted Black Olives", Some("6 tbsp"), Some("halved"), None),
            crate::recipe::Ingredient::opt("Cannellini beans", Some("1 400g tin"), Some("drained and rinsed"), Some("Optional")),
            crate::recipe::Ingredient::opt("Celery", None, Some("chopped into 1cm pieces"), Some("Optional")),
            crate::recipe::Ingredient::opt("Whole Cloves", Some("2-3"), None, Some("Optional")),
            crate::recipe::Ingredient::opt("Mascarpone", Some("2 tbsp"), None, Some("Optional")),
        ]),
        method: vec![
            "Cut the chorizo into half centimeter thick semi-circles.".to_string(),
            "Add the oil and smoked paprika to a wide pan.".to_string(),
            "Cook the carrots and celery on a medium to high heat for a minute or so. Stir regularly to avoid sticking.".to_string(),
            "Add the red onions and garlic then cook for minute.".to_string(),
            "Stir in the chorizo, peppers and tomato paste then cook for a few more minutes.".to_string(),
            "Add the pasata, tinned tomatoes, cloves, chilli flakes, red wine, oregano, cinamon stick, and honey. If the pan is looking full then wait until the mixure has reduced before adding the rest of the tinned tomatoes/wine.".to_string(),
            "Wait until bubbling then turn down the heat and simmer for at least 20 minutes.".to_string(),
            "Stir in spinach, mascarpone and cannellini beans.".to_string(),
            "Serve with pasta".to_string(),
        ],
    }
}
