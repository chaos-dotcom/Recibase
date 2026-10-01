//! `se.reciba.api.recipes.RhubarbCrumble`.

use crate::recipe::RecipeDef;
use crate::tag::Tag;
use chrono::NaiveDate;

pub fn recipe() -> RecipeDef {
    RecipeDef {
        object_name: "RhubarbCrumble".to_string(),
        name: "Rhubarb & Date Crumble".to_string(),
        created_at: NaiveDate::from_ymd_opt(2023, 4, 5).unwrap(),
        permalink_override: Some("rhubarb-crumble".to_string()),
        source: Some("https://web.archive.org/web/20210227223551/http://www.claudiandfin.co.uk/healthy-rhubarb-date-crumble-recipe/".to_string()),
        description: None,
        tagline: None,
        notes: vec![
            "Make sure to keep the butter in the fridge until right before you need it, to avoid it melting.".to_string(),
            "You can leave out the dates or substitute the filling for another fruit combination. We use this recipe as the template for all our crumbles.".to_string(),
            "We mix the fruit in the ovenproof dish because ours is basically a glass mixing bowl. If your ovenproof dish is a metal tin you might want to use a mixing bowl first.".to_string(),
        ],
        tags: vec![Tag::Pudding],
        image: None,
        ingredients_blocks: crate::recipe::IngredientsBlock::simple(vec![
            crate::recipe::Ingredient::q("Plain Flour", "120g"),
            crate::recipe::Ingredient::q("Dark brown muscovado sugar", "50g"),
            crate::recipe::Ingredient::qp("Butter", "90g", "cut into small pieces"),
            crate::recipe::Ingredient::qp("Rhubarb", "3 sticks", "cut into 2-3cm pieces"),
            crate::recipe::Ingredient::qp("Pitted Dates", "50g", "chopped"),
            crate::recipe::Ingredient::qp("Fresh Ginger", "2cm", "grated"),
            crate::recipe::Ingredient::q("Golden Syrup", "1-2 tbsp"),
        ]),
        method: vec![
            "Pre-heat the oven to 180C.".to_string(),
            "Rub the butter and flour together in a mixing bowl until it disappears.".to_string(),
            "Mix in the brown sugar.".to_string(),
            "In an ovenproof dish combine the rhubarb, dates, ginger and golden syrup.".to_string(),
            "Pour the crumble topping evenly over the fruit mixture.".to_string(),
            "Cook in the oven until the topping is golden and the fruit is bubbling around the edges. This will take 20 minutes in a shallow dish or up to 45 minutes for a deep dish.".to_string(),
            "Serve with cream or ice cream. Rhubarb is very acidic so dairy is a must.".to_string(),
        ],
    }
}
