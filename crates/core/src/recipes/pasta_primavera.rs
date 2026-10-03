//! `se.reciba.api.recipes.PastaPrimavera`.
//! chaos-tag: casa-chaos

use crate::recipe::RecipeDef;
use crate::tag::Tag;
use chrono::NaiveDate;

pub fn recipe() -> RecipeDef {
    RecipeDef {
        object_name: "PastaPrimavera".to_string(),
        name: "Pasta Primavera".to_string(),
        created_at: NaiveDate::from_ymd_opt(2026, 10, 3).unwrap(),
        permalink_override: None,
        source: Some("https://www.loveandlemons.com/pasta-primavera/".to_string()),
        description: Some("Packed with seasonal vegetables, this pasta primavera is a simple, fresh spring or summer dinner.".to_string()),
        tagline: None,
        notes: vec![
            "Serves 4".to_string(),
            "Prepare 10 minutes; cook 20 minutes.".to_string(),
            "Tarragon is optional but highly recommended.".to_string(),
        ],
        tags: vec![
            Tag::Quick,
            Tag::Vegetarian,
            Tag::HotWeather,
            Tag::LowEffort,
            Tag::Scales,
        ],
        image: None,
        ingredients_blocks: crate::recipe::IngredientsBlock::simple(vec![
            crate::recipe::Ingredient::q("Penne pasta", "280g"),
            crate::recipe::Ingredient::opt("Extra-virgin olive oil", Some("2 tbsp"), None, Some("Plus more for drizzling")),
            crate::recipe::Ingredient::qp("Garlic", "4 cloves", "sliced"),
            crate::recipe::Ingredient::opt("Yellow squash", Some("1"), Some("sliced into thin half-moons"), Some("Use just squash or courgette if you can't find both")),
            crate::recipe::Ingredient::qp("Courgette", "1", "sliced into thin half-moons"),
            crate::recipe::Ingredient::qp("Asparagus", "1 bunch", "chopped into 2.5cm pieces"),
            crate::recipe::Ingredient::qp("Cherry tomatoes", "150g", "halved"),
            crate::recipe::Ingredient::qp("Red onion", "1", "thinly sliced"),
            crate::recipe::Ingredient::q("Sea salt", "1 tsp"),
            crate::recipe::Ingredient::qp("Frozen peas", "75g", "thawed"),
            crate::recipe::Ingredient::opt("Pecorino cheese", Some("75g"), Some("grated"), Some("Parmesan works well too")),
            crate::recipe::Ingredient::q("Fresh lemon juice", "3 tbsp"),
            crate::recipe::Ingredient::q("Red pepper flakes", "Pinch"),
            crate::recipe::Ingredient::opt("Fresh basil", Some("25g"), None, Some("Plus more for garnish")),
            crate::recipe::Ingredient::opt("Fresh tarragon", Some("10g"), None, Some("Optional")),
            crate::recipe::Ingredient::new("Black pepper"),
        ]),
        method: vec![
            "Bring a large pot of salted water to the boil. Cook the pasta according to the pack instructions until al dente. Drain and toss with a drizzle of olive oil to stop it sticking.".to_string(),
            "Heat the oil in a large, deep frying pan over a medium heat. Add the garlic, squash, courgette, asparagus, tomatoes, onion, salt and several grinds of pepper and sauté for 3-4 minutes, or until the vegetables are tender.".to_string(),
            "Add the pasta, peas, cheese, lemon juice and a pinch of red pepper flakes and toss to combine. Stir in the basil and tarragon, if using.".to_string(),
            "Season to taste, garnish with more basil and serve.".to_string(),
        ],
    }
}
