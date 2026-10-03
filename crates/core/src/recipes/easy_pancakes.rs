//! `se.reciba.api.recipes.EasyPancakes`.
//! chaos-tag: casa-chaos

use crate::recipe::RecipeDef;
use crate::tag::Tag;
use chrono::NaiveDate;

pub fn recipe() -> RecipeDef {
    RecipeDef {
        object_name: "EasyPancakes".to_string(),
        name: "Easy Pancakes".to_string(),
        created_at: NaiveDate::from_ymd_opt(2026, 10, 3).unwrap(),
        permalink_override: None,
        source: Some("Waitrose".to_string()),
        description: Some("Whether you like to roll or fold, this flexible recipe is fab with sweet or savoury fillings.".to_string()),
        tagline: None,
        notes: vec![
            "Serves 2 (4-6 pancakes)".to_string(),
            "Prepare 5 minutes + resting; cook 20 minutes.".to_string(),
            "Per serving: 1863kJ/445kcals/20g fat/4.1g saturated fat/51g carbs/7.1g sugars/2.5g fibre/15g protein/0.3g salt; vegetarian.".to_string(),
            "Scan the QR code overleaf to see Waitrose cook Charmaine Katz making these pancakes with toppings and fillings.".to_string(),
        ],
        tags: vec![
            Tag::Vegetarian,
            Tag::Pudding,
            Tag::Quick,
            Tag::LowEffort,
            Tag::Scales,
        ],
        image: None,
        ingredients_blocks: crate::recipe::IngredientsBlock::simple(vec![
            crate::recipe::Ingredient::q("Plain Flour", "120g"),
            crate::recipe::Ingredient::opt("Egg", Some("1"), None, Some("Medium, free range")),
            crate::recipe::Ingredient::opt("Milk", Some("300ml"), None, Some("plus extra if needed")),
            crate::recipe::Ingredient::q("Sunflower Oil", "1-2 tbsp"),
        ]),
        method: vec![
            "Sift the flour and a pinch of salt into a bowl. Make a well in the centre. Crack in the egg and add 50ml of the milk, then use a balloon whisk to start whisking, incorporating the flour into the liquid. Add the remaining milk gradually, whisking continually, until the batter is slightly thicker than single cream. Add a touch more milk if needed. Alternatively, whizz up the batter in a blender until smooth.".to_string(),
            "Cover the batter and leave it to rest for 30 minutes. If it thickens while resting, add a little more milk to bring it back to a thin consistency.".to_string(),
            "Heat an 18-20cm nonstick frying pan over a medium heat. Add 1 tbsp oil, swirl it around, then tip out any excess into a heatproof bowl. Ladle 60-80ml of batter into the pan, just enough to coat the base of the pan, swirling it around quickly to make a thin, even layer. Cook for 1-2 minutes, or until the edges begin to curl and the base is golden brown. Use a fish slice or palette knife to flip the pancake. Cook for 30 seconds to 1 minute more, then transfer to a warm plate. Repeat with the rest of the batter, adding more oil if needed. Serve hot with your favourite savoury or sweet toppings.".to_string(),
        ],
    }
}
