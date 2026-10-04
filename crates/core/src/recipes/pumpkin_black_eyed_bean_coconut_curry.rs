//! `se.reciba.api.recipes.PumpkinBlackEyedBeanCoconutCurry`.
//! chaos-tag: casa-chaos

use crate::recipe::RecipeDef;
use crate::tag::Tag;
use chrono::NaiveDate;

pub fn recipe() -> RecipeDef {
    RecipeDef {
        object_name: "PumpkinBlackEyedBeanCoconutCurry".to_string(),
        name: "Pumpkin, Black-Eyed Bean + Coconut Curry".to_string(),
        created_at: NaiveDate::from_ymd_opt(2026, 10, 4).unwrap(),
        permalink_override: None,
        source: None,
        description: None,
        tagline: None,
        notes: vec![
            "Serves 4 as a main course".to_string(),
        ],
        tags: vec![
            Tag::Vegan,
            Tag::GlutenFree,
            Tag::ColdWeather,
            Tag::LowEffort,
            Tag::Scales,
        ],
        image: None,
        ingredients_blocks: crate::recipe::IngredientsBlock::simple(vec![
            crate::recipe::Ingredient::q("Pumpkin or squash", "1.2kg"),
            crate::recipe::Ingredient::new("Rapeseed oil"),
            crate::recipe::Ingredient::q("Garam masala", "1 tablespoon"),
            crate::recipe::Ingredient::new("Salt and ground black pepper"),
            crate::recipe::Ingredient::new("Coconut or rapeseed oil"),
            crate::recipe::Ingredient::q("Mustard seeds", "1 teaspoon"),
            crate::recipe::Ingredient::qp("Green finger chillies", "2", "slit lengthways"),
            crate::recipe::Ingredient::qp("Large onion", "1", "halved and thinly sliced"),
            crate::recipe::Ingredient::qp("Garlic", "3 cloves", "crushed"),
            crate::recipe::Ingredient::qp("Black-eyed beans", "1 x 400g tin", "drained"),
            crate::recipe::Ingredient::qp("Ripe tomatoes", "150g", "cut into wedges"),
            crate::recipe::Ingredient::q("Ground turmeric", "1/3 teaspoon"),
            crate::recipe::Ingredient::q("Coconut milk", "1 x 400ml tin"),
            crate::recipe::Ingredient::opt("Fresh curry leaves", Some("10"), None, Some("Optional")),
        ]),
        method: vec![
            "Preheat the oven to 200°C/400°F/gas 6 and line two baking trays with foil. Cut the pumpkin in half, scoop out and discard the seeds, then cut it into crescents around 2cm at the widest part. Transfer to a big bowl, drizzle with oil, and sprinkle with the garam masala, 1 teaspoon of salt and 1/2 teaspoon of black pepper. Toss to coat evenly, then arrange in a single layer. Roast for 30 minutes, or until soft and tender.".to_string(),
            "Meanwhile, put 2 tablespoons of oil into a large lidded frying pan over a medium heat and, when hot, add the mustard seeds. When they pop, add the slit green chillies and the onion. Cook for 12 minutes, or until the onion is soft and golden, then add the garlic. Cook for another couple of minutes, then add the drained beans and stir to mix together. Add the tomatoes and cook for a few more minutes until soft and jammy around the edges.".to_string(),
            "Next, add the turmeric, 1/3 teaspoon of black pepper, 1/2 teaspoon of salt and the coconut milk. Tip the roasted pumpkin into the pan and stir to mix. Cover with the lid and leave to heat through for 5 minutes. Check for salt and chilli, adjusting if you wish, then transfer to a serving dish.".to_string(),
            "If you like, you can finish off the dish with a quick curry leaf tarka: put 2 tablespoons of oil into a small frying pan over a medium to high heat. When hot, throw in the curry leaves and let them crackle and turn translucent in the oil. Pour over the pumpkin, then serve.".to_string(),
            "This dish goes well with elephant ear naan (see page 220), tamarind and caramelised red onion rice (page 192) and some cucumber and mint raita (page 247).".to_string(),
        ],
    }
}
