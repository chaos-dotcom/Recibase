//! `se.reciba.api.recipes.RasElHanoutSlowRoastedMushroomsWithPinkPeppercornsHalloumiFlatLeafParsley`.
//! chaos-tag: casa-chaos

use crate::recipe::RecipeDef;
use crate::tag::Tag;
use chrono::NaiveDate;

pub fn recipe() -> RecipeDef {
    RecipeDef {
        object_name: "RasElHanoutSlowRoastedMushroomsWithPinkPeppercornsHalloumiFlatLeafParsley".to_string(),
        name: "Ras El Hanout Slow-Roasted Mushrooms with Pink Peppercorns, Halloumi & Flat-Leaf Parsley".to_string(),
        created_at: NaiveDate::from_ymd_opt(2026, 10, 4).unwrap(),
        permalink_override: None,
        source: None,
        description: Some("These mushrooms are as good as part of a vegetarian feasting menu as they are as antipasti - and of course if you're building a non-vegetarian feast, they go wonderfully alongside the slow-roasted harissa lamb on page 102 or warm flatbreads.".to_string()),
        tagline: None,
        notes: vec![
            "Serves 4".to_string(),
            "Prepare 10 minutes; cook 1 hour 20 minutes.".to_string(),
        ],
        tags: vec![
            Tag::Vegetarian,
            Tag::Slow,
            Tag::ColdWeather,
            Tag::Scales,
        ],
        image: None,
        ingredients_blocks: crate::recipe::IngredientsBlock::simple(vec![
            crate::recipe::Ingredient::opt("Whole mushrooms", Some("600g"), None, Some("Portabellini or chestnut")),
            crate::recipe::Ingredient::qp("Whole shallots", "300g", "peeled and halved"),
            crate::recipe::Ingredient::q("Butter", "70g"),
            crate::recipe::Ingredient::q("Ras el hanout", "4 teaspoons"),
            crate::recipe::Ingredient::qp("Lemon", "1", "zest and juice"),
            crate::recipe::Ingredient::qp("Garlic", "4 cloves", "crushed"),
            crate::recipe::Ingredient::qp("Halloumi", "250g", "cut into 1cm cubes"),
            crate::recipe::Ingredient::q("Pine nuts", "30g"),
            crate::recipe::Ingredient::qp("Flat-leaf parsley", "Large handful", "roughly chopped"),
            crate::recipe::Ingredient::opt("Pink peppercorns", Some("2 teaspoons"), None, Some("Optional but very nice")),
            crate::recipe::Ingredient::new("Sea salt"),
            crate::recipe::Ingredient::new("Freshly ground black pepper"),
        ]),
        method: vec![
            "Preheat your oven to 130°C fan/150°C/gas 2. Place the butter, ras el hanout, lemon zest and crushed garlic in the roasting tin, then transfer to the oven for 5 minutes to melt the butter and to allow the spices to toast a little.".to_string(),
            "Meanwhile, trim the mushrooms and peel the shallots. After 5 minutes, pop them into the tin with the melted spice butter, season well with sea salt and freshly ground black pepper and mix everything together really well with your hands. Cover in tinfoil, then place in the oven to cook for 1 hour.".to_string(),
            "After 1 hour, increase the heat to 150°C fan/170°C/gas 3 and remove the tinfoil. Squeeze over the lemon juice, add the halloumi and mix well before scattering with the pine nuts. Return to the oven to cook uncovered for a further 15 minutes.".to_string(),
        ],
    }
}
