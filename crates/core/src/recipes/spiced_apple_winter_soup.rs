//! `se.reciba.api.recipes.SpicedAppleWinterSoup`.

use crate::recipe::RecipeDef;
use crate::tag::Tag;
use chrono::NaiveDate;

pub fn recipe() -> RecipeDef {
    RecipeDef {
        object_name: "SpicedAppleWinterSoup".to_string(),
        name: "Spiced Apple Winter Soup".to_string(),
        created_at: NaiveDate::from_ymd_opt(2026, 9, 27).unwrap(),
        permalink_override: Some("spiced-apple-winter-soup".to_string()),
        source: Some("Stasis/Bel".to_string()),
        description: Some("Winter apple soup, scales nicely, very good in the cooler months. Can use most winter root veg to get added textures/flavours.".to_string()),
        tagline: None,
        notes: vec![
            "Gwen rating 8.5/10.".to_string(),
        ],
        tags: vec![
            Tag::Soup,
            Tag::Vegan,
            Tag::ColdWeather,
            Tag::Slow,
            Tag::Freezes,
            Tag::BetterNextDay,
        ],
        image: None,
        ingredients_blocks: crate::recipe::IngredientsBlock::simple(vec![
            crate::recipe::Ingredient::qp("Apples (Red)", "4", "Peeled, cored, chopped finely"),
            crate::recipe::Ingredient::qp("Onions (white or brown)", "4 medium or 2 large", "peeled, chopped finely (1-2cm cubes)"),
            crate::recipe::Ingredient::qp("Carrots", "2", "peeled, chooped intp 1-2cm cubes"),
            crate::recipe::Ingredient::opt("Apple juice", Some("2lt"), None, Some("Preferably not from concentrate/added sugar. DO NOT USE SUGAR FREE.")),
            crate::recipe::Ingredient::qpn("Parsnips", "2 small or 1 large", "peeled, chopped 4s then cut into 0.5 - 1 inch segments", "Use fresh, not frozen for best results"),
            crate::recipe::Ingredient::qpn("Fresh ginger", "1 knob", "Peeled, chop into 1 inch cubes", "Put into a mesh bag to be removed at the end."),
            crate::recipe::Ingredient::opt("Cinamon", Some("1 3 inch stem crush into rough segments"), None, Some("Put into a mesh bag to be removed at the end.")),
            crate::recipe::Ingredient::opt("Cloves", Some("5 cloves"), None, Some("Put into a mesh bag to be removed at the end.")),
            crate::recipe::Ingredient::opt("Star anise", Some("2 cloves"), None, Some("Put into a mesh bag to be removed at the end.")),
            crate::recipe::Ingredient::opt("Fennel", Some("2 tsp"), None, Some("Put into a mesh bag to be removed at the end.")),
            crate::recipe::Ingredient::opt("Thyme", Some("4 sprigs, or 2 tbsp dried"), None, Some("Fresh prefered, Put into a mesh bag to be removed at the end.")),
            crate::recipe::Ingredient::opt("Juniper berries (Dried)", Some("3 - 5"), None, Some("Put into a mesh bag to be removed at the end.")),
            crate::recipe::Ingredient::opt("Vegetable stock pots", Some("2"), None, Some("Replace with 3 cubes if unavailable.")),
            crate::recipe::Ingredient::opt("Cooking oil of choice", Some("as needed to prevent stickage"), None, Some("personal preference is gee for an added richness, but olive or avacardo works fine.")),
        ]),
        method: vec![
            "Add onions, carrots and parsnips into a medium high heat in a large, 3-5lt container or slow cooker.".to_string(),
            "After 7 minutes, while the veg is sweating, add in your chopped apples and then sweat until all veg is fork soft (another 5-10 minutes)".to_string(),
            "Heat 200ml of apple juice to dissolve stock into, then add all apple juice to the pot. add in your spice mesh bag and bring to simmer.".to_string(),
            "Reduce heat to medium low, and leave to simmer for at least 30 minutes. Once reduced to desired apple-tensisity, serve.".to_string(),
        ],
    }
}
