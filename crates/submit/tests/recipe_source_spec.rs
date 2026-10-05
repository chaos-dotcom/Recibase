//! Port of `se.reciba.api.RecipeSourceSpec`.

use chrono::NaiveDate;
use recibase_core::recipe::RecipeDef;
use recibase_core::tag::Tag;
use recibase_submit::recipe_source::{GeneratedRecipe, RecipeSource, SubmitRejection};
use recibase_submit::recipe_submission::{IngredientSubmission, RecipeSubmission};

fn today() -> NaiveDate {
    NaiveDate::from_ymd_opt(2026, 9, 7).expect("a real date")
}

fn generate(
    submission: &RecipeSubmission,
    existing: &[RecipeDef],
) -> Result<GeneratedRecipe, SubmitRejection> {
    RecipeSource::generate(submission, today(), existing)
}

fn named(name: &str) -> RecipeSubmission {
    RecipeSubmission {
        name: name.to_string(),
        source: None,
        description: None,
        notes: Vec::new(),
        tags: Vec::new(),
        ingredients: vec![IngredientSubmission {
            name: "Onion".to_string(),
            quantity: Some("1".to_string()),
            ..Default::default()
        }],
        method: vec!["Simmer.".to_string()],
    }
}

fn sample() -> RecipeSubmission {
    RecipeSubmission {
        name: "Chilli con Carne".to_string(),
        source: Some("Kit's Dad".to_string()),
        description: Some("A weeknight chilli.".to_string()),
        notes: vec!["Better the next day.".to_string()],
        tags: vec![
            "VegetarianIsh".to_string(),
            "Spicy".to_string(),
            "Spicy".to_string(),
        ],
        ingredients: vec![
            IngredientSubmission {
                name: "Mince".to_string(),
                quantity: Some("500g".to_string()),
                ..Default::default()
            },
            IngredientSubmission {
                name: "Honey".to_string(),
                quantity: Some("1 tbsp".to_string()),
                notes: Some("Optional".to_string()),
                ..Default::default()
            },
            IngredientSubmission {
                name: "Oil".to_string(),
                ..Default::default()
            },
            IngredientSubmission {
                name: "Garlic".to_string(),
                quantity: Some("2".to_string()),
                prep: Some("crushed".to_string()),
                notes: Some("Fresh".to_string()),
            },
            IngredientSubmission {
                name: " ".to_string(),
                ..Default::default()
            },
        ],
        method: vec!["Brown the mince.".to_string()],
    }
}

/// The fields the conflict checks read, as the two Scala case objects the spec
/// refers to declare them.
fn chilli_con_carne() -> RecipeDef {
    RecipeDef {
        object_name: "ChilliConCarne".to_string(),
        name: "Chilli con Carne".to_string(),
        permalink_override: Some("chilli-con-carne".to_string()),
        ..Default::default()
    }
}

fn crunch_chocolate_chip_coffee_cake() -> RecipeDef {
    RecipeDef {
        object_name: "CrunchChocolateChipCoffeeCake".to_string(),
        name: "Crunch Chocolate Chip Coffee Cake".to_string(),
        permalink_override: Some("coffee-cake".to_string()),
        ..Default::default()
    }
}

#[test]
fn writes_one_case_object_with_an_explicit_permalink() {
    let generated = generate(&sample(), &[]).expect("generated");
    assert_eq!(generated.object_name, "ChilliConCarne");
    assert_eq!(generated.permalink, "chilli-con-carne");
    assert_eq!(generated.branch, "recipe/chilli-con-carne");
    assert_eq!(
        generated.path,
        "src/main/scala/se/reciba/api/recibase/recipes/ChilliConCarne.scala"
    );
    assert_eq!(generated.source, EXPECTED_SAMPLE);
}

#[test]
fn keeps_stop_words_that_permalink_from_raw_string_would_drop() {
    let generated = generate(&named("Toad in the Hole"), &[]).expect("generated");
    assert_eq!(generated.object_name, "ToadInTheHole");
    assert_eq!(generated.permalink, "toad-in-the-hole");
}

#[test]
fn strips_accents_when_naming_the_file() {
    let generated = generate(&named("Crème Brûlée"), &[]).expect("generated");
    assert_eq!(generated.object_name, "CremeBrulee");
    assert_eq!(generated.permalink, "creme-brulee");
}

#[test]
fn omits_empty_optional_fields() {
    let generated = generate(&named("Phone Test Soup"), &[]).expect("generated");
    assert_eq!(generated.source, EXPECTED_PLAIN);
    assert!(!generated.source.contains("s\""));
    assert!(!generated.source.contains(".celsius"));
}

#[test]
fn keeps_newlines_in_a_description() {
    let submission = RecipeSubmission {
        description: Some("A weeknight soup.\nBetter the next day.".to_string()),
        ..named("Phone Test Soup")
    };
    let generated = generate(&submission, &[]).expect("generated");
    assert!(generated.source.contains(
        r#"override val description: Option[String] = "A weeknight soup.\nBetter the next day.".some"#
    ));
}

#[test]
fn stores_temperature_text_as_a_plain_string() {
    let submission = RecipeSubmission {
        method: vec!["Preheat to ${180.celsius}.".to_string()],
        ..named("Phone Test Soup")
    };
    let generated = generate(&submission, &[]).expect("generated");
    assert!(generated.source.contains("${180.celsius}."));
    assert!(!generated.source.contains("s\""));
}

#[test]
fn rejects_a_name_that_cannot_be_a_scala_identifier() {
    assert_eq!(
        generate(&named("2 eggs"), &[]),
        Err(SubmitRejection::InvalidSubmission(
            "Recipe name cannot be turned into a Scala file name".to_string()
        ))
    );
}

#[test]
fn rejects_names_that_would_shadow_the_generated_file() {
    assert_eq!(
        generate(&named("Set"), &[]),
        Err(SubmitRejection::InvalidSubmission(
            "Recipe name cannot be turned into a Scala file name".to_string()
        ))
    );
}

#[test]
fn rejects_automatic_tags() {
    let submission = RecipeSubmission {
        tags: vec!["New".to_string()],
        ..named("Phone Test Soup")
    };
    assert_eq!(
        generate(&submission, &[]),
        Err(SubmitRejection::InvalidSubmission(
            "Tag New is assigned automatically".to_string()
        ))
    );
}

#[test]
fn rejects_tag_display_names() {
    let submission = RecipeSubmission {
        tags: vec!["Vegetarian-ish".to_string()],
        ..named("Phone Test Soup")
    };
    assert_eq!(
        generate(&submission, &[]),
        Err(SubmitRejection::InvalidSubmission(
            "Unknown tag: Vegetarian-ish".to_string()
        ))
    );
}

#[test]
fn rejects_a_duplicate_name() {
    assert_eq!(
        generate(&sample(), &[chilli_con_carne()]),
        Err(SubmitRejection::ConflictingSubmission(
            "A recipe named Chilli con Carne already exists".to_string()
        ))
    );
}

#[test]
fn rejects_a_permalink_already_used_by_another_recipe() {
    assert_eq!(
        generate(
            &named("Coffee Cake"),
            &[crunch_chocolate_chip_coffee_cake()]
        ),
        Err(SubmitRejection::ConflictingSubmission(
            "A recipe with permalink coffee-cake already exists".to_string()
        ))
    );
}

#[test]
fn rejects_an_object_name_that_already_has_a_file() {
    assert_eq!(
        generate(
            &named("Crunch Chocolate Chip Coffee Cake!"),
            &[crunch_chocolate_chip_coffee_cake()]
        ),
        Err(SubmitRejection::ConflictingSubmission(
            "A recipe file named CrunchChocolateChipCoffeeCake.scala already exists".to_string()
        ))
    );
}

#[test]
fn tag_object_names_match_the_identifiers_the_generator_emits() {
    assert_eq!(Tag::VeganIsh.object_name(), "VeganIsh");
    assert_eq!(Tag::New.object_name(), "New");
    // The generator accepts the object name of every tag except the automatic
    // ones, which are rejected earlier.
    assert!(
        generate(
            &RecipeSubmission {
                tags: vec!["VegetarianIsh".to_string()],
                ..named("Phone Test Soup")
            },
            &[]
        )
        .is_ok()
    );
    assert!(
        generate(
            &RecipeSubmission {
                tags: vec!["Not a Meal".to_string()],
                ..named("Phone Test Soup")
            },
            &[]
        )
        .is_err()
    );
}

const EXPECTED_SAMPLE: &str = "\
package se.reciba.api.recipes

import cats.syntax.option._
import se.reciba.api.model.{Ingredient, IngredientsBlock, Permalink, Recipe, Tag}
import java.time.LocalDate

case object ChilliConCarne extends Recipe {
  val name = \"Chilli con Carne\"
  val createdAt = LocalDate.of(2026, 9, 7)
  override val permalink: Permalink = Permalink(\"chilli-con-carne\")

  override val source: Option[String] = \"Kit's Dad\".some
  override val description: Option[String] = \"A weeknight chilli.\".some
  override val notes: List[String] = List(
    \"Better the next day.\"
  )

  val tags = Set(Tag.VegetarianIsh, Tag.Spicy)

  val ingredientsBlocks = IngredientsBlock.simple(
    Ingredient(\"Mince\", \"500g\"),
    Ingredient(\"Honey\", \"1 tbsp\".some, None, \"Optional\".some),
    Ingredient(\"Oil\"),
    Ingredient(\"Garlic\", \"2\", \"crushed\", \"Fresh\")
  )

  val method = List(
    \"Brown the mince.\"
  )
}
";

const EXPECTED_PLAIN: &str = "\
package se.reciba.api.recipes

import se.reciba.api.model.{Ingredient, IngredientsBlock, Permalink, Recipe, Tag}
import java.time.LocalDate

case object PhoneTestSoup extends Recipe {
  val name = \"Phone Test Soup\"
  val createdAt = LocalDate.of(2026, 9, 7)
  override val permalink: Permalink = Permalink(\"phone-test-soup\")

  val tags = Set.empty[Tag]

  val ingredientsBlocks = IngredientsBlock.simple(
    Ingredient(\"Onion\", \"1\")
  )

  val method = List(
    \"Simmer.\"
  )
}
";
