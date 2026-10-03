//! The circe acceptance rules the Scala `Decoder`s have, and `Passcode.equal`.

use recibase_submit::recipe_submission::{IngredientSubmission, Passcode, RecipeSubmission};
use serde_json::{json, Value};

fn full() -> Value {
    json!({
        "name": "Phone Test Soup",
        "source": "Kit's Dad",
        "description": "A weeknight soup.",
        "notes": ["Better the next day."],
        "tags": ["Spicy"],
        "ingredients": [
            {"name": "Onion", "quantity": "1", "prep": "chopped", "notes": "Fresh"},
            {"name": "Oil"}
        ],
        "method": ["Simmer."]
    })
}

#[test]
fn decodes_every_field() {
    let submission = RecipeSubmission::from_json(&full()).expect("decoded");
    assert_eq!(
        submission,
        RecipeSubmission {
            name: "Phone Test Soup".to_string(),
            source: Some("Kit's Dad".to_string()),
            description: Some("A weeknight soup.".to_string()),
            notes: vec!["Better the next day.".to_string()],
            tags: vec!["Spicy".to_string()],
            ingredients: vec![
                IngredientSubmission {
                    name: "Onion".to_string(),
                    quantity: Some("1".to_string()),
                    prep: Some("chopped".to_string()),
                    notes: Some("Fresh".to_string()),
                },
                IngredientSubmission { name: "Oil".to_string(), ..Default::default() },
            ],
            method: vec!["Simmer.".to_string()],
        }
    );
}

#[test]
fn unknown_fields_are_ignored() {
    let mut value = full();
    value["extra"] = json!("ignored");
    assert!(RecipeSubmission::from_json(&value).is_some());
}

#[test]
fn null_and_missing_optionals_become_none_or_empty_lists() {
    let value = json!({
        "name": "Soup",
        "source": null,
        "description": null,
        "notes": null,
        "tags": null,
        "ingredients": [{"name": "Onion", "quantity": null, "prep": null, "notes": null}],
        "method": ["Simmer."]
    });
    let submission = RecipeSubmission::from_json(&value).expect("decoded");
    assert_eq!(submission.source, None);
    assert_eq!(submission.description, None);
    assert_eq!(submission.notes, Vec::<String>::new());
    assert_eq!(submission.tags, Vec::<String>::new());
    assert_eq!(submission.ingredients[0], IngredientSubmission { name: "Onion".to_string(), ..Default::default() });

    let value = json!({
        "name": "Soup",
        "ingredients": [{"name": "Onion"}],
        "method": ["Simmer."]
    });
    let submission = RecipeSubmission::from_json(&value).expect("decoded");
    assert_eq!(submission.notes, Vec::<String>::new());
    assert_eq!(submission.tags, Vec::<String>::new());
}

#[test]
fn a_missing_or_null_ingredients_or_method_field_fails() {
    for field in ["ingredients", "method"] {
        let mut missing = full();
        missing.as_object_mut().expect("object").remove(field);
        assert!(RecipeSubmission::from_json(&missing).is_none(), "missing {}", field);

        let mut null = full();
        null[field] = Value::Null;
        assert!(RecipeSubmission::from_json(&null).is_none(), "null {}", field);
    }
}

#[test]
fn a_name_of_the_wrong_type_or_a_missing_name_fails() {
    let mut missing = full();
    missing.as_object_mut().expect("object").remove("name");
    assert!(RecipeSubmission::from_json(&missing).is_none());

    let mut null = full();
    null["name"] = Value::Null;
    assert!(RecipeSubmission::from_json(&null).is_none());

    let mut number = full();
    number["name"] = json!(7);
    assert!(RecipeSubmission::from_json(&number).is_none());
}

#[test]
fn an_ingredient_must_be_an_object_with_a_string_name() {
    let mut value = full();
    value["ingredients"] = json!([{"quantity": "1"}]);
    assert!(RecipeSubmission::from_json(&value).is_none());

    let mut value = full();
    value["ingredients"] = json!(["Onion"]);
    assert!(RecipeSubmission::from_json(&value).is_none());

    let mut value = full();
    value["ingredients"] = json!([{"name": "Onion", "quantity": 4}]);
    assert!(RecipeSubmission::from_json(&value).is_none());
}

#[test]
fn lists_of_strings_reject_other_element_types_and_other_shapes() {
    let mut value = full();
    value["method"] = json!(["Simmer.", 4]);
    assert!(RecipeSubmission::from_json(&value).is_none());

    let mut value = full();
    value["tags"] = json!("Spicy");
    assert!(RecipeSubmission::from_json(&value).is_none());

    let mut value = full();
    value["ingredients"] = json!({"name": "Onion"});
    assert!(RecipeSubmission::from_json(&value).is_none());
}

#[test]
fn a_body_that_is_not_an_object_fails() {
    assert!(RecipeSubmission::from_json(&json!([])).is_none());
    assert!(RecipeSubmission::from_json(&json!("soup")).is_none());
    assert!(RecipeSubmission::from_json(&json!(null)).is_none());
}

#[test]
fn decode_reports_a_circe_style_failure() {
    let error =
        RecipeSubmission::decode(&json!({"name": "Soup", "method": ["x"]})).expect_err("failed");
    assert_eq!(
        error.message,
        "DecodingFailure at .ingredients: Attempt to decode value on failed cursor"
    );

    let error = RecipeSubmission::decode(&json!({"name": 1})).expect_err("failed");
    assert_eq!(error.message, "DecodingFailure at .name: String");
}

#[test]
fn passcode_equal_compares_the_utf8_bytes() {
    assert!(Passcode::equal("abc", "abc"));
    assert!(Passcode::equal("", ""));
    assert!(Passcode::equal("café", "café"));
    assert!(!Passcode::equal("abc", "abd"));
    assert!(!Passcode::equal("abc", "abcd"));
    assert!(!Passcode::equal("", "a"));
    assert!(!Passcode::equal("a", ""));
    assert!(!Passcode::equal("secret", "Secret"));
    // A multi-byte character is compared as bytes, so a prefix cannot pass.
    assert!(!Passcode::equal("é", "e"));
}
