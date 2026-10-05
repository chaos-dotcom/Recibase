//! The `scaler.py` port: the ported `test_scaler.py` cases plus the golden
//! data generated from the Python by `tools/golden/gen_scaler_golden.py`.
//!
//! The golden files are the acceptance criterion: `quantities.json`,
//! `parse_quantity.json` and `scale_factors.json` are the Python's own output
//! for the whole recipe corpus, and `copy_text.json` is the copy text of every
//! recipe in it.  A missing file, or a shrinking file, fails the suite rather
//! than quietly checking less.

use std::path::{Path, PathBuf};

use recibase_frontend::scaler::{
    Quantity, get_scale_factor, ingredients_copy_text, parse_quantity, scale_ingredient,
};
use serde_json::{Value, json};

/// `scale_factors.json` cases.
const SCALE_FACTOR_CASES: usize = 31;
/// `parse_quantity.json` cases.
const PARSE_CASES: usize = 212;
/// `quantities.json` cases.
const QUANTITY_CASES: usize = 2544;
/// `quantities.json` cases whose quantity parses at all (20 vague quantities
/// times the 12 factors do not).
const SCALED_QUANTITY_CASES: usize = 2304;
/// `copy_text.json` cases.
const COPY_TEXT_CASES: usize = 95;

fn golden_path(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("golden")
        .join(name)
}

/// Reads a golden file, or fails loudly: a missing file is a broken build,
/// not a skipped test.
fn load_golden_text(name: &str) -> String {
    let path = golden_path(name);

    std::fs::read_to_string(&path).unwrap_or_else(|error| {
        panic!(
            "golden file {} is missing or unreadable: {error}",
            path.display()
        )
    })
}

/// Parses a golden file.  `json.dump` wrote a bare `NaN` for the `"nan"` scale
/// factor, which is not JSON, and `serde_json` cannot hold a NaN in a `Value`
/// anyway; the token is neutralised here.  Where it matters, the scale-factor
/// test reads it off the file text instead.
fn parse_golden(text: &str, name: &str) -> Value {
    let text = text.replace("NaN", "null");

    serde_json::from_str(&text)
        .unwrap_or_else(|error| panic!("golden file {name} is not valid JSON: {error}"))
}

fn load_golden(name: &str) -> Value {
    parse_golden(&load_golden_text(name), name)
}

/// The cases of a golden array, checked against the count in the brief.
fn golden_cases(name: &str, expected: usize) -> Vec<Value> {
    let value = load_golden(name);
    let cases = value
        .as_array()
        .unwrap_or_else(|| panic!("golden file {name} is not a JSON array"));

    assert_eq!(
        cases.len(),
        expected,
        "golden file {name} has {} cases, expected {expected}",
        cases.len()
    );

    cases.clone()
}

/// The Python class name of a parsed quantity, as the golden `kind` records
/// it.  Informational: it says which parser won, not what the text is.
fn kind_name(quantity: &Quantity) -> &'static str {
    match quantity {
        Quantity::Simple { .. } => "SimpleQuantity",
        Quantity::Range { .. } => "RangeQuantity",
        Quantity::Fraction { .. } => "FractionQuantity",
        Quantity::MixedFraction { .. } => "MixedFractionQuantity",
        Quantity::Compound { .. } => "CompoundQuantity",
        Quantity::Parenthetical { .. } => "ParentheticalQuantity",
        Quantity::Length { .. } => "LengthQuantity",
        Quantity::Descriptive { .. } => "DescriptiveQuantity",
        Quantity::Approximate { .. } => "ApproximateQuantity",
    }
}

/// Every golden file is present and still holds the number of cases the brief
/// promises.
#[test]
fn golden_files_hold_the_documented_case_counts() {
    let counted = [
        ("quantities.json", QUANTITY_CASES),
        ("parse_quantity.json", PARSE_CASES),
        ("scale_factors.json", SCALE_FACTOR_CASES),
        ("copy_text.json", COPY_TEXT_CASES),
    ];

    for (name, expected) in counted {
        assert_eq!(
            golden_cases(name, expected).len(),
            expected,
            "golden file {name} no longer holds {expected} cases"
        );
    }
}

// ---------------------------------------------------------------------------
// The ported `test_scaler.py`
// ---------------------------------------------------------------------------

mod test_get_scale_factor {
    use super::*;

    #[test]
    fn return_nothing_if_no_param() {
        assert_eq!(get_scale_factor(None), None);
    }

    #[test]
    fn return_nothing_if_non_numeric() {
        assert_eq!(get_scale_factor(Some("lol")), None);
    }

    #[test]
    fn return_number_if_numeric_scale() {
        assert_eq!(get_scale_factor(Some("2")), Some(2.0));
        assert_eq!(get_scale_factor(Some("5")), Some(5.0));
        assert_eq!(get_scale_factor(Some("20")), Some(20.0));
    }

    #[test]
    fn return_number_if_decimal_number_scale() {
        assert_eq!(get_scale_factor(Some("0.5")), Some(0.5));
        assert_eq!(get_scale_factor(Some(".5")), Some(0.5));
        assert_eq!(get_scale_factor(Some("1.5")), Some(1.5));
    }

    #[test]
    fn ignore_silly_scale() {
        for raw in ["1", "0", "1.0", "0.0", "-1", "-1.0"] {
            assert_eq!(get_scale_factor(Some(raw)), None, "scale {raw:?}");
        }
    }

    #[test]
    fn ignore_large_scales() {
        assert_eq!(get_scale_factor(Some("50")), Some(50.0));
        assert_eq!(get_scale_factor(Some("51")), None);
        assert_eq!(get_scale_factor(Some("1000")), None);
    }
}

mod test_scale_ingredient {
    use super::*;

    /// `TestScaleIngredient.make_ingredient`.
    fn make_ingredient(quantity: Value) -> Value {
        json!({
            "name": "Onion",
            "prep": "chopped",
            "notes": Value::Null,
            "quantity": quantity,
        })
    }

    /// Scales a fresh ingredient and returns it.
    fn scaled(quantity: &str, factor: f64) -> Value {
        let mut ingredient = make_ingredient(json!(quantity));

        scale_ingredient(&mut ingredient, factor);

        ingredient
    }

    /// `scale_ingredient` returns the ingredient untouched: in the Python the
    /// test compares the object with itself, which only proves that the
    /// quantity string survived and that nothing was marked `scaled`.
    fn assert_untouched(quantity: &str, factor: f64) {
        let ingredient = scaled(quantity, factor);

        assert_eq!(
            ingredient["quantity"].clone(),
            json!(quantity),
            "quantity {quantity:?} should not change"
        );
        assert!(
            ingredient.get("scaled").is_none(),
            "quantity {quantity:?} should not be marked scaled"
        );
    }

    fn assert_scaled_to(quantity: &str, factor: f64, expected: &str) {
        let ingredient = scaled(quantity, factor);

        assert_eq!(
            ingredient["quantity"].clone(),
            json!(expected),
            "quantity {quantity:?} scaled by {factor}"
        );
        assert_eq!(
            ingredient.get("scaled"),
            Some(&json!(true)),
            "quantity {quantity:?} scaled by {factor} should be marked scaled"
        );
    }

    #[test]
    fn ignore_ingredients_with_no_quantity() {
        let mut ingredient = json!({"name": "Onion", "quantity": Value::Null});

        scale_ingredient(&mut ingredient, 2.0);

        assert_eq!(
            ingredient,
            json!({"name": "Onion", "quantity": Value::Null})
        );
    }

    #[test]
    fn ignore_ingredients_with_unparseable_quantity() {
        let mut ingredient = json!({"name": "Onion", "quantity": "About 3"});

        scale_ingredient(&mut ingredient, 2.0);

        assert_eq!(ingredient, json!({"name": "Onion", "quantity": "About 3"}));
    }

    // The Python's `TestScaleIngredient.test_scale_ingredient`, renamed: a
    // test function cannot share the name of the function it calls.
    #[test]
    fn scale_a_simple_quantity() {
        assert_scaled_to("2", 2.0, "4");
    }

    #[test]
    fn scale_ingredient_by_fraction() {
        assert_scaled_to("1", 1.5, "1.50");
    }

    #[test]
    fn scale_ingredient_with_suffix() {
        assert_scaled_to("4g", 2.0, "8g");
        assert_scaled_to("2 tbsp", 2.0, "4 tbsp");
    }

    #[test]
    fn scale_ingredient_with_fraction() {
        assert_scaled_to("1/4", 2.0, "1/2");
    }

    #[test]
    fn scale_ingredient_with_uneven_fraction() {
        assert_scaled_to("1/4", 5.0, "1 1/4");
    }

    #[test]
    fn scale_fractional_ingredient_to_int() {
        assert_scaled_to("1/2", 2.0, "1");
    }

    #[test]
    fn scale_messy_fraction_to_decimal() {
        assert_scaled_to("1/2", 1.8, "0.90");
    }

    #[test]
    fn scale_ingredient_with_uneven_fraction_with_suffix() {
        assert_scaled_to("1/4 tsp", 5.0, "1 1/4 tsp");
    }

    #[test]
    fn scale_ingredient_with_range() {
        assert_scaled_to("2-3", 2.0, "4-6");
    }

    #[test]
    fn scale_decimal_quantity() {
        assert_scaled_to("4.5 tsp", 2.0, "9 tsp");
    }

    #[test]
    fn scale_decimal_quantity_without_space() {
        assert_scaled_to("12.5g", 2.0, "25g");
    }

    #[test]
    fn scale_mixed_fraction_quantity() {
        assert_scaled_to("1 1/2 tsp", 2.0, "3 tsp");
    }

    #[test]
    fn scale_compound_tin_quantity() {
        assert_scaled_to("2 400g tins", 2.0, "4 400g tins");
    }

    #[test]
    fn scale_single_compound_tin_quantity() {
        assert_scaled_to("1 400g tin", 2.0, "2 400g tins");
    }

    #[test]
    fn scale_implicit_compound_tin_quantity() {
        assert_scaled_to("400g tin", 2.0, "2 400g tins");
    }

    #[test]
    fn scale_compound_fillet_quantity() {
        assert_scaled_to("2 110g fillets", 2.0, "4 110g fillets");
    }

    #[test]
    fn scale_parenthetical_tin_quantity() {
        assert_scaled_to("1 tin (400g)", 2.0, "2 tins (400g)");
    }

    #[test]
    fn scale_parenthetical_block_quantity() {
        assert_scaled_to("1 block (225g)", 2.0, "2 blocks (225g)");
    }

    #[test]
    fn scale_approximate_block_quantity() {
        assert_scaled_to("1 block, approx. 200g", 2.0, "2 blocks, approx 400g");
    }

    #[test]
    fn scale_tilde_approximate_quantity() {
        assert_scaled_to("~70g", 2.0, "~140g");
    }

    #[test]
    fn scale_suffix_approximate_quantity() {
        assert_scaled_to("25g approx", 2.0, "50g approx");
    }

    #[test]
    fn scale_up_to_approximate_quantity() {
        assert_scaled_to("up to 75g", 2.0, "up to 150g");
    }

    #[test]
    fn scale_descriptive_quantities() {
        let cases = [
            ("1 large clove", "2 large cloves"),
            ("1 large tin", "2 large tins"),
            ("1 large bag minimum", "2 large bags minimum"),
            ("1 heaped tablespoon", "2 heaped tablespoons"),
            ("2 fork-fulls", "4 fork-fulls"),
        ];

        for (quantity, expected) in cases {
            assert_scaled_to(quantity, 2.0, expected);
        }
    }

    #[test]
    fn scale_length_quantities() {
        let cases = [
            ("1 cm piece", "2 cm pieces"),
            ("1cm piece", "2cm pieces"),
            ("2.5cm", "5cm"),
        ];

        for (quantity, expected) in cases {
            assert_scaled_to(quantity, 2.0, expected);
        }
    }

    // The Python's `TestScaleIngredient.test_ignore_vague_quantities`.  Its
    // assertion is `scale_ingredient(ingredient, 2.0) == ingredient`, which the
    // mutation of that very object makes trivially true: it only proves that
    // the quantity string survived for the quantities no parser claims.
    // `1 per person` *is* claimed, by the `DescriptiveQuantity` parser, and does
    // scale - the golden data records `2 per persons` - so it is asserted at
    // its real value rather than at the Python's vacuous one.
    #[test]
    fn ignore_vague_quantities() {
        let untouched = [
            "pinch",
            "handful",
            "knob",
            "generous dash",
            "Several teaspoons",
        ];

        for quantity in untouched {
            assert_untouched(quantity, 2.0);
        }

        assert_scaled_to("1 per person", 2.0, "2 per persons");
    }
}

mod test_ingredients_copy_text {
    use super::*;

    #[test]
    fn sums_matching_quantities_case_insensitively() {
        let blocks = json!([
            {"ingredients": [{"name": "Butter", "quantity": "50g"}]},
            {"ingredients": [{"name": "butter", "quantity": "150g"}]},
        ]);

        assert_eq!(ingredients_copy_text(&blocks), "200g Butter");
    }

    #[test]
    fn keeps_incompatible_units_separate() {
        let blocks = json!([{
            "ingredients": [
                {"name": "Milk", "quantity": "100ml"},
                {"name": "Milk", "quantity": "50g"},
            ],
        }]);

        assert_eq!(ingredients_copy_text(&blocks), "100ml Milk\n50g Milk");
    }

    #[test]
    fn omits_tsp_tbsp_and_missing_quantities() {
        let blocks = json!([{
            "ingredients": [
                {"name": "Salt", "quantity": "1tsp"},
                {"name": "salt", "quantity": "1/2tsp"},
                {"name": "Pepper", "quantity": Value::Null},
            ],
        }]);

        assert_eq!(ingredients_copy_text(&blocks), "Salt\nPepper");
    }

    #[test]
    fn merges_omitted_quantity_into_concrete_quantity() {
        let blocks = json!([{
            "ingredients": [
                {"name": "Butter", "quantity": Value::Null},
                {"name": "Butter", "quantity": "50g"},
            ],
        }]);

        assert_eq!(ingredients_copy_text(&blocks), "50g Butter");
    }

    #[test]
    fn sums_fraction_quantities() {
        let blocks = json!([{
            "ingredients": [
                {"name": "Flour", "quantity": "1/2 cup"},
                {"name": "Flour", "quantity": "1/2 cup"},
            ],
        }]);

        assert_eq!(ingredients_copy_text(&blocks), "1 cup Flour");
    }

    #[test]
    fn preserves_order_of_first_occurrence() {
        let blocks = json!([{
            "ingredients": [
                {"name": "Butter", "quantity": "50g"},
                {"name": "Sugar", "quantity": "100g"},
                {"name": "butter", "quantity": "150g"},
            ],
        }]);

        assert_eq!(ingredients_copy_text(&blocks), "200g Butter\n100g Sugar");
    }
}

// ---------------------------------------------------------------------------
// Python semantics the golden data pins down
// ---------------------------------------------------------------------------

mod python_semantics {
    use super::*;

    /// One quantity scaled, as the rendered string.
    fn scaled(quantity: &str, factor: f64) -> Value {
        let mut ingredient = json!({"name": "X", "quantity": quantity});

        scale_ingredient(&mut ingredient, factor);

        ingredient["quantity"].clone()
    }

    /// `get_scale_factor` always hands `scale_ingredient` a `float`, so the
    /// fraction arithmetic goes through binary floating point.  `1 1/2 tsp`
    /// times `3.0` is the float 4.5, which renders through `SimpleQuantity`,
    /// and `1/3 tsp` times `2.0` is 0.6666666666666666, whose exact rational has
    /// a denominator far above ten, so it renders as `0.67`.  The golden data
    /// covers both; they are spelled out here because they are the easiest
    /// things in the module to get wrong.
    #[test]
    fn fraction_scaling_goes_through_a_float() {
        let cases = [
            ("1 1/2 tsp", 3.0, "4.50 tsp"),
            ("1 1/2 tsp", 2.0, "3 tsp"),
            ("1 1/2 tsp", 1.5, "2.25 tsp"),
            ("1/3 tsp", 2.0, "0.67 tsp"),
            ("1/3 tsp", 3.0, "1 tsp"),
            ("1/2", 1.8, "0.90"),
            ("1/4", 5.0, "1 1/4"),
            ("1/4", 2.0, "1/2"),
        ];

        for (quantity, factor, expected) in cases {
            assert_eq!(
                scaled(quantity, factor),
                json!(expected),
                "quantity {quantity:?} scaled by {factor}"
            );
        }
    }

    /// `MixedFraction + MixedFraction` stays a `Fraction`, and
    /// `FractionQuantity.__str__` renders a whole one through its
    /// `numerator == denominator` branch: Python really does say
    /// `3 0/1 cups`.
    #[test]
    fn merging_two_mixed_fractions_keeps_the_fraction_spelling() {
        let blocks = json!([{"ingredients": [
            {"name": "X", "quantity": "1 1/2 cups"},
            {"name": "X", "quantity": "1 1/2 cups"},
        ]}]);

        assert_eq!(ingredients_copy_text(&blocks), "3 0/1 cups X");
    }

    /// `get_scale_factor("nan")` returns a NaN, and `format_number` spells the
    /// result `nan` - `str.format` and Rust's `{:.2}` disagree here.  The live
    /// Flask app renders `/chicken-curry?scale=nan` as `nan Chicken Breasts`.
    #[test]
    fn a_nan_factor_renders_the_way_python_spells_it() {
        let factor = get_scale_factor(Some("nan")).expect("nan is a scale factor");

        assert!(factor.is_nan());
        assert_eq!(scaled("2", factor), json!("nan"));
    }
}

// ---------------------------------------------------------------------------
// The golden data
// ---------------------------------------------------------------------------

/// `get_scale_factor` for every raw `scale` in the golden.  `serde_json`
/// turns the file's non-standard `NaN` into `Null`, so the expected token is
/// read off the file text instead of the parsed `Value`.
#[test]
fn golden_scale_factors() {
    let text = load_golden_text("scale_factors.json");
    let cases = golden_cases("scale_factors.json", SCALE_FACTOR_CASES);
    let tokens = out_tokens(&text);

    assert_eq!(
        tokens.len(),
        SCALE_FACTOR_CASES,
        "scale_factors.json holds {} `out` tokens, expected {SCALE_FACTOR_CASES}",
        tokens.len()
    );

    let mut checked = 0;

    for (case, token) in cases.iter().zip(tokens.iter()) {
        let raw = case["raw"]
            .as_str()
            .unwrap_or_else(|| panic!("scale_factors.json case without a raw string: {case}"));
        let got = get_scale_factor(Some(raw));

        if token == "null" {
            assert!(got.is_none(), "scale {raw:?} should not be a factor");
        } else {
            let expected: f64 = token.parse().unwrap_or_else(|_| {
                panic!("scale_factors.json `out` token {token:?} is not a number")
            });
            let got = got.unwrap_or_else(|| panic!("scale {raw:?} should be {expected}"));

            assert!(
                got == expected || (got.is_nan() && expected.is_nan()),
                "scale {raw:?}: got {got}, expected {expected}"
            );
        }

        checked += 1;
    }

    assert_eq!(
        checked, SCALE_FACTOR_CASES,
        "checked every scale factor case"
    );
}

/// The `out` tokens of a golden file, in file order.
fn out_tokens(text: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut rest = text;

    while let Some(index) = rest.find("\"out\":") {
        rest = &rest[index + "\"out\":".len()..];

        let end = rest.find([',', '}']).unwrap_or(rest.len());

        tokens.push(rest[..end].trim().to_string());
        rest = &rest[end..];
    }

    tokens
}

/// `parse_quantity` for every unique quantity in the corpus.
#[test]
fn golden_parse_quantity() {
    let cases = golden_cases("parse_quantity.json", PARSE_CASES);

    let mut checked = 0;

    for case in &cases {
        let raw = case["quantity"].as_str().unwrap();
        let parsed = parse_quantity(raw);

        match case["str"].as_str() {
            Some(expected) => {
                let parsed =
                    parsed.unwrap_or_else(|| panic!("{raw:?} should parse to {expected:?}"));

                assert_eq!(parsed.to_text(), expected, "quantity {raw:?}");

                let kind = case["kind"].as_str().unwrap();

                assert_eq!(kind_name(&parsed), kind, "quantity {raw:?}");
            }
            None => assert!(parsed.is_none(), "quantity {raw:?} should not parse"),
        }

        checked += 1;
    }

    assert_eq!(checked, PARSE_CASES, "checked every parse_quantity case");
}

/// `scale_ingredient` for every quantity in the corpus at every factor the
/// generator used.
#[test]
fn golden_scaled_quantities() {
    let cases = golden_cases("quantities.json", QUANTITY_CASES);

    let mut checked = 0;
    let mut scaled_cases = 0;

    for case in &cases {
        let quantity = &case["quantity"];
        let factor = case["factor"].as_f64().unwrap();
        let mut ingredient = json!({"name": "X", "quantity": quantity.clone()});

        // `scale_ingredient` mutates and hands back the same object, which the
        // generator recorded as `same_object`.
        assert_eq!(case["same_object"], json!(true), "case {case}");

        scale_ingredient(&mut ingredient, factor);

        assert_eq!(
            ingredient["quantity"].clone(),
            case["out"],
            "quantity {quantity} scaled by {factor}"
        );

        if case["scaled"].as_bool().unwrap() {
            assert_eq!(
                ingredient.get("scaled"),
                Some(&json!(true)),
                "quantity {quantity} scaled by {factor} should be marked scaled"
            );

            scaled_cases += 1;
        } else {
            assert!(
                ingredient.get("scaled").is_none(),
                "quantity {quantity} scaled by {factor} should not be marked scaled"
            );
        }

        checked += 1;
    }

    assert_eq!(checked, QUANTITY_CASES, "checked every quantity case");
    assert_eq!(
        scaled_cases, SCALED_QUANTITY_CASES,
        "the golden holds {SCALED_QUANTITY_CASES} scalable quantity/factor pairs"
    );
}

// ---------------------------------------------------------------------------
// `copy_text.json`: the whole corpus
// ---------------------------------------------------------------------------

/// The input side of `copy_text.json`: for every recipe, its
/// `ingredients_blocks` reduced to the `(name, quantity)` pair of each
/// ingredient, in order.  `copy_text.json` itself only carries the permalink
/// and the expected text, and the repository has no dump of the full recipes
/// (`recipes.json` at the repository root is the permalink list only), so the
/// blocks the golden was generated from are embedded here.  They were read
/// from the live API backend, `GET http://127.0.0.1:8081/recipes/<permalink>`,
/// which `tools/golden/gen_scaler_golden.py` used too.
const CORPUS_BLOCKS: &str = r##"[["baked-rigatoni-aubergine",[[["Onion","1"],["Garlic","2-3 cloves"],["Red wine",null],["Chopped tomatoes","2 400g tins"],["Sun-dried tomatoes","5"],["Aubergine","1"],["Oregano","2 tbsp"],["Rigatoni","225g"],["Breadcrumbs","30g"],["Parmesan","30g"],["Salt",null],["Pepper",null]]]],["salmon-olive-spaghetti",[[["Salmon fillets","1 per person"],["Capers",null],["Olive oil",null],["Onion","1"],["Pimento-stuffed olives",null],["Lemon juice",null],["Thyme",null],["Spaghetti",null],["Cherry tomatoes",null]]]],["baobab-ice-cream",[[["Baobab powder","20g"],["Mascarpone","230g"],["Eggs","2"],["Icing Sugar","60g"]]]],["basa-pathia",[[["Brown onion","1"],["Cumin seeds","1 tsp"],["Ground cumin","1 tsp"],["Cayenne pepper","1/2 tsp"],["Curry powder","1/2 tbsp"],["Coriander","5g"],["Basa fillets","200g"],["Tamarind paste","15g"],["Ground turmeric","1/2 tsp"],["Tomato paste","32g"],["Basmati rice","100g"],["Garlic Clove","2"],["Fresh root ginger","15g"],["Naan","2"],["Salt",null],["Sugar",null],["Oil",null]]]],["beef-stroganoff",[[["Beef","500g"],["Garlic Clove","1"],["Onion","1"],["Mushrooms","5-6"],["Soured Cream","350ml"],["Beef or Pork Stock Cube",null],["Salt","Pinch"],["Pepper","Pinch"],["Paprika","Pinch"],["Butter","Wedge"]]]],["beef-wraps",[[["Vegetarian Beef Strips","175g"],["Red Pepper","1"],["Spring Onions","3"],["Tortilla Wraps","4"],["Cherry Tomatoes",null],["Black Beans","1 400g tin"],["Cayenne pepper",null],["Smoked Paprika",null],["Mayonnaise",null],["Oil",null]]]],["beetroot-ice-cream",[[["Beetroot Powder","25g"],["Mascarpone","230g"],["Eggs","2"],["Icing Sugar","60g"]]]],["birthday-cake",[[["Unsalted Butter","200g"],["Caster Sugar","200g"],["Eggs","2 medium"],["Lemon","1/2"],["Milk","2 tbsp"],["Plain Flour","186g"],["Baking Powder","3 tsp"]],[["Unsalted butter","150g"],["Icing Sugar","300g"],["Vanilla Extract","1 tsp"],["Milk","1 tbsp"],["Smarties","200g"],["Cocoa powder","4.5 tsp"],["Marzipan",null]]]],["birthday-cake-classic",[[["Butter","5oz"],["Caster Sugar","5oz"],["Eggs","3"],["Self raising flour","8oz"],["Lemon essence","1/4 tsp"],["Unsalted Butter","100g"],["Icing Sugar","200g"],["Milk","1 tbsp"],["Smarties","136g"],["Vanilla Extract","1 tsp"],["Marzipan",null]]]],["blue-cheese-gnocchi",[[["Gnocchi","500g"],["Parmesan","~70g"],["Creme Fraiche","150ml"],["Spinach","200g"],["Stilton","200g"],["Pimento Stuffed olives",null],["Cherry Tomatoes",null],["Fresh bread",null]]]],["broccoli-salmon-quiche",[[["Broccoli Florets","100g"],["Smoked Salmon","120g"],["Stilton","50g"],["Shortcrust Pastry Sheet","230g"],["Eggs","3"],["Mascarpone Cheese","2 tbsp"],["Pepper",null]]]],["broccoli-stilton-soup",[[["Broccoli","2 heads"],["Stilton","220g"],["Onion","1 onion"],["Garlic","3 cloves"],["Stock Cube","1"],["Boiled water","1L"],["Butter","1 knob"],["Ground Nutmeg",null],["Lemon Juice",null],["Salt",null]]]],["brownies",[[["Unsalted Butter","300g"],["Dark Chocolate","300g"],["Eggs","5 large"],["Granulated Sugar","450g"],["Vanilla Extract","1 tbsp"],["Plain Flour","200g"],["Salt","1 tsp"]]]],["butternut-chilli",[[["Onion","1"],["Butternut Squash","1"],["Vine Tomatoes","400g"],["Red Wine","150ml"],["Vegetable stock cube","1/4"],["Black turtle beans","400g tin"],["Soured Cream","2 tbsp"],["Piquillo peppers","6"],["Garlic","1 large clove"],["Cayenne Pepper","1 tsp"],["Oregano","1 tsp"],["Red chilli","1"],["Pitted Green Olives","6"],["Bay Leaf",null],["Chives",null],["Lemon Juice",null],["Olive Oil",null]]]],["butternut-squash-pad-thai",[[["Butternut squash","120g"],["Lime","1"],["Red chilli","1"],["Red pepper","1"],["Garlic","1 cloves"],["Roasted peanuts","25g"],["Soy sauce","2 tbsp"],["Tamarind paste","3 tsp"],["Toasted sesame oil","1 tsp"],["Tenderstem broccoli","80g"],["Red curry paste","4 tsp"],["Sriracha hot chilli sauce","1 tbsp"],["Fried onions","3 tsp"],["Thai rice noodles","200g"],["Salt",null],["Sugar",null],["Vegetable oil",null]]]],["cheese-scones",[[["Plain Flour","208g"],["Baking Powder","21g"],["Cayenne pepper","pinch"],["Butter","55g"],["Mature Cheddar","120g"],["Milk","90-100ml"],["Salt","pinch"]]]],["cheesy-cod",[[["Cod","2 110g fillets"],["Stock cube","1"],["Spinach","80g"],["Panko breadcrumbs","30g"],["Cheddar","40g"],["Soft cheese","50g"],["Water","150ml"],["Crispy Potato Slices",null]]]],["chicken-curry",[[["Chicken Breasts","4"],["Large Onions","2"],["Garlic","6 cloves"],["Green Beans","220g"],["Chestnut Mushrooms","485g"],["Double Cream","75ml"],["Sweetcorn","1 large tin"],["Lemongrass","2 stems"],["Kaffir lime leaves",null],["Salt",null],["Sugar",null],["Smoked Paprika",null],["Cumin",null],["Cloves",null],["Nutmeg",null],["Turmeric",null],["Red Peppers","2"],["Chillies",null],["Naan Bread",null]]]],["chilli-con-carne",[[["Oil",null],["Salt",null],["Pepper",null],["Chilli Powder","1 tsp"],["Crushed or whole dried chillies","1 tsp"],["Red Pepper","1"],["Frozen Sweetcorn",null],["Paprika",null],["Tomato Puree",null],["Peeled plum tomatoes","1 400g tin"],["Italian Herbs",null],["Brown Sauce",null],["Beef stock cube or Bovril",null],["Rice",null],["Olive Oil",null],["Onion","1"],["Garlic Clove","1"],["Kidney Beans","1 400g tin"],["Honey","1 tbsp"],["Dark cooking chocolate",null],["Mince","500g"]]]],["chinese-fusion",[[["Butternut Squash","1"],["Lamb Mince","500g"],["Leeks","2 large"],["Shiitake mushrooms","125g"],["Garlic","4 cloves"],["Jar Ginger","2 fork-fulls"],["Sliced Water chestnuts","1 tin"],["Sliced bamboo","1 tin"],["Five spice",null],["Rich hoisin sauce",null],["Noodles",null],["Oil",null],["Salt",null]]]],["chipotle-burgers",[[["Beyond Meat Burgers","4"],["Brioche buns","4"],["Applewood Smoked Cheddar","4 slices"],["Vegan Streaky Bacon Rashers","105g"],["Ketchup",null],["Mayonnaise",null],["Chipotle paste","30g"],["Oil",null]]]],["chipotle-mac-n-cheese",[[["Yellow pepper","1"],["Red pepper","1"],["Red onion","1"],["Macaroni","150g"],["Vegetable stock cube","1"],["Spring onion","1"],["Smoked paprika","2 tsp"],["Chipotle paste","40g"],["Tomato paste","1 tbsp"],["Cheddar cheese","80g"],["Panko breadcrumbs","30g"],["Creme fraiche","200g"],["Salt",null],["Vegetable oil",null]]]],["christmas-naanwidge",[[["Coriander","10g"],["Ground turmeric","1 tsp"],["Dried chilli flakes","1/2 tsp"],["Curry powder","1 tbsp"],["Carrot","1"],["Parsnip","1"],["Potatoes","3"],["Paneer","200g"],["Greek-style yoghurt","100g"],["Smoked paprika","1 tsp"],["Cranberry sauce","40g"],["Tamarind paste","15g"],["Plain naans","2"],["Salt",null],["Pepper",null],["Oil",null]]]],["vegetable-crumble",[[["Onion","1"],["Garlic","3 minimum"],["Carrots","3"],["Parsnips","2"],["Baby turnips","350g"],["New potatoes","5-6"],["Vegetable stock","450ml"],["Worcestershire sauce","generous dash"],["Tomato pur\u00e9e","1 tbsp"],["Bay leaves","2"],["Butter beans","1 410g tin"],["Thyme","1-2 tsp"],["Salt",null],["Pepper",null],["Plain Flour","85g"],["Butter","30g"],["Mature cheddar cheese","75g"],["Sunflower seeds","30g"]]]],["coconut-lime-dahl",[[["Lentils","450g"],["Turmeric","1 tsp"],["Garlic","2 Cloves"],["Coconut Milk","1 Tin"],["Boiling Water","600ml"],["Onion","1/2"],["Cumin","1 tsp"],["Lime","1"],["Spinach","9 handfuls"],["Oil",null]]]],["cod-lentils",[[["Red Onion","1"],["Cod Fillets","2"],["Green Lentils","2 400g tin"],["Butter",null],["Salt",null],["Pepper",null],["Lemon Juice",null]]]],["courgette-broccoli-pasta",[[["Courgette","1"],["Tenderstem broccoli","120g"],["Fresh tagliatelle","190g"],["Garlic","3 cloves"],["Baby spinach","80g"],["Parmesan",null],["Lemon juice",null],["Dried chilli flakes",null],["Flaked almonds","handful"],["Soured cream","1 tbsp"],["Vegetable stock",null],["Olive oil",null]]]],["courgette-spinach-pasties",[[["Courgettes","2"],["Feta","200g"],["Spinach","3 handfulls"],["All Butter Puff Pastry","320g"],["Plain Flour",null],["Nutmeg",null],["Lemon Juice",null],["Salt",null],["Pepper",null]]]],["cranberry-relish",[[["Red onions","2"],["Brown sugar","3 tbsp"],["Cranberries","450g"],["Redcurrant jelly","2 tbsp"],["Cinnamon stick","1"],["Vegetable oil","tbsp"]]]],["creamy-cauliflower-cheese",[[["Cauliflower","1"],["Creme Fraiche","300g"],["Dijon Mustard","1 tsp"],["Blue Cheese","125g"],["Walnuts","25g"],["Cheddar Cheese","50g"],["Salt",null],["Black pepper",null]]]],["coffee-cake",[[["Plain Flour","375g"],["Baking Powder","2 tsp"],["Baking Soda","1/2 tsp"],["Salt","1/4 tsp"],["Butter","180g"],["Cream Cheese","250g"],["Granulated Sugar","300g"],["Vanilla Extract","1 tsp"],["Eggs","3 large"],["Milk","180ml"]],[["Dark Brown Sugar","100g"],["Plain Flour","75g"],["Butter","60g"],["Dark Chocolate Chips","180g"],["Walnuts","60g"]]]],["dahl",[[["Red lentils","350g"],["Water","800ml"],["Ground turmeric","1 tsp"],["Chilli powder","1/2 tsp"],["Ginger","1 cm piece"],["Garlic","2 cloves"],["Garam masala","1/2 tsp"],["Salt",null],["Butter","25g approx"],["Ground cumin","1 tsp"],["Onion","1"]]]],["dukaten-cookies",[[["Plain Flour","250g"],["Baking Powder","1tsp"],["Sugar","75g"],["Vanilla Extract","1/2 tsp"],["Egg","1"],["Milk","1 tbsp"],["Butter","125g"]],[["Butter","125g"],["Icing Sugar","130g"],["Cocoa","1 heaped tablespoon"],["Rum flavouring","1 top"],["Egg","1"]],[["Powdered Sugar","100g"],["Cocoa","1 heaped tablespoon"],["Water","1-2 tbsp"],["Butter","28g"]]]],["egg-tapas",[[["Brown onion","1"],["Eggs","2"],["Red pepper","1"],["Garlic clove","1"],["Spring onion","1"],["Coriander","10g"],["Stock Cube","1"],["Mayonnaise","4 tbsp"],["Chipotle paste","5 tsp"],["White potatoes","4"],["Cayenne pepper","1/2 tsp"],["Cannellini beans","1 tin (400g)"],["Olive oil",null],["Pepper",null],["Salt",null],["Sugar",null]]]],["goan-king-prawn-balchao",[[["Red onions","2"],["Garlic","3 cloves"],["Fresh root ginger","15g"],["Ground coriander","1 tsp"],["Ground cumin","1 tsp"],["Dried chilli flakes","1/2 tsp"],["Cayenne pepper","1/2 tsp"],["Tomato paste","32g"],["King prawns","171g"],["Tamarind paste","15g"],["Cider vinegar","30ml"],["Sugar","1 tsp"],["Baby leaf spinach","80g"],["Basmati rice","130g"],["Salt",null],["Vegetable oil",null],["Water","300ml"]]]],["greek-wraps",[[["Vivera Veggie Greek Kebab","125g"],["Red Pepper","1"],["Spring Onions","3"],["Tortilla Wraps","4"],["Cherry Tomatoes",null],["Sriracha",null],["Mayonnaise",null],["Oil",null]]]],["halloumi-wraps",[[["Halloumi","225g"],["Red Pepper","1"],["Spring Onions","3"],["Tortilla Wraps","4"],["Cherry Tomatoes",null],["Yoghurt",null],["Garlic",null],["Oil",null]]]],["duck-wraps",[[["Vegetarian Shredded Hoisin Duck","200g"],["Red Pepper","1"],["Spring Onions","3"],["Tortilla Wraps","4"],["Cherry Tomatoes",null],["Hoisin sauce",null],["Mayonnaise",null],["Oil",null]]]],["indian-patties",[[["Red lentils","225g"],["Whole cloves","1-2"],["Coriander seeds","2 tsp"],["Cumin seeds","1-2 tsp"],["Black peppercorns","1 tsp"],["Garlic","2-3 cloves"],["Ginger","1cm piece"],["Onion","1"],["Spinach","225g"],["Coriander leaves",null],["Mint leaves",null],["Chillies","1-2"],["Ground cinnamon",null],["Egg","1"],["Salt",null]]]],["kashtouri",[[["Risotto rice","150g"],["Macaroni","150g"],["Red lentils","150g"],["Onion","1"],["Garlic",null],["Chopped tomatoes","1 400g tin"],["Cayenne pepper","1-2 tsp"],["Ground coriander","1 tsp"],["Lemon juice",null],["Salt",null],["Black pepper",null]]]],["lamb-aubergine-daube",[[["Aubergine","350g"],["Onions","2"],["Lamb Neck Fillet","750g"],["Chopped Tomatoes","1 400g tin"],["Chicken Stock","250ml"],["Cumin Seed","1/2 tsp"],["Fennel Seed","1/2 tsp"],["Dried Chilli Flakes","1/2 tsp"],["Ginger","10g"],["Olive Oil",null],["Cinamon Stick",null],["Salt",null],["Lemon",null]]]],["lemon-feta-pasta",[[["Pine nuts",null],["Cashew nuts",null],["Brazil nuts",null],["Feta","1 block, approx. 200g"],["Tuna","1 80g tin"],["Olives",null],["Lemon juice",null],["Thyme",null],["Fusilli",null]]]],["lentil-spinach-stew",[[["Garlic","1 clove"],["Onion","1"],["Carrots","2-3"],["Salad Tomatoes",null],["Cream of coconut",null],["Frozen Spinach",null],["Lentils","150g"],["Water","400ml"],["Worcestershire Sauce",null],["Stock cube","1"],["Honey",null],["Chili flakes",null],["Cinnamon",null],["Nutmeg",null],["Olive Oil",null]]]],["macaroni",[[["Butter","25g"],["Plain Flour","2 tbsp"],["Milk","1/2 pint (or more)"],["Medium Cheddar Cheese",null],["Macaroni Pasta","275g"]]]],["marmalade-ice-cream",[[["Marmalade","125g"],["Mascarpone","230g"],["Eggs","2"],["Icing Sugar","60g"]]]],["mascarpone-ice-cream",[[["Mascarpone","230g"],["Eggs","2"],["Icing Sugar","60g"],["Vanilla Essence","1 tsp"]]]],["mead",[[["Honey","340g"],["Tap Water","416ml"],["Boiling Water","630ml"],["Champagne Yeast","1/3 tsp"]]]],["melty-mushroom-wellingtons",[[["Chestnut mushrooms","250g"],["Spinach",null],["Stilton","220g"],["Garlic",null],["Butter","knob"],["Black pepper",null],["Puff pastry","1 sheet"]]]],["mexican-polenta-pie",[[["Sunflower Oil","1 tbsp"],["Celery","1 stick"],["Garlic","1 large clove"],["Onions","150g"],["Green Pepper","1/2"],["Cayenne Pepper","1 tsp"],["Red Kidney Beans","400g"],["Green Olives","12"],["Jalape\u00f1o Peppers","1 tbsp"],["Sweetcorn","75g"],["Chopped Tomatoes","400g"],["Tomato Pur\u00e9e","1 tbsp"],["Salt",null],["Black Pepper",null]],[["Cornmeal or Polenta","125g"],["Plain White Flour","1 tbsp"],["Baking Powder","2 tsp"],["Egg","1"],["Skimmed Milk","100ml"],["Half-fat Mature Cheddar Cheese","25g"]]]],["mushroom-quiche",[[["Chestnut mushrooms","250g"],["Dried mushrooms","handful"],["Red onion","1"],["Parmesan",null],["Mascarpone","2 tbsp"],["Eggs","2"],["Salt",null],["Black pepper",null],["Wholegrain Mustard","2 tsp"],["Shortcrust pastry sheet","230g"]]]],["mushroom-risotto",[[["Chestnut mushrooms","250g"],["Dried mushrooms","handful"],["Arborio rice","1 cup"],["White wine","A decent slosh"],["Black pepper",null],["Stock cube","1"],["Water","700ml"],["Stilton","100g"],["Butter","knob"]]]],["mushroom-stroganoff",[[["Butter",null],["Large Onion","1"],["Brown Mushrooms","250g"],["Shiitake or Porchini Mushrooms","Some"],["Vegetable or Mushroom stock cube","1"],["Soured Cream","350ml"],["Plain Flour","3 tbsp"],["Salt",null],["Black Pepper",null],["Garlic","1 clove"],["Nutmeg",null],["Brandy",null],["Marjoram",null],["Worcestershire sauce",null]]]],["new-york-bagels",[[["Bagels","4"],["Squeaky Bean Deli Pastrami","180g"],["Cooked Beetroot","180g"],["Pickles",null],["Cheese",null],["Mayonnaise",null]]]],["nut-roast",[[["Mixed Roasted Nuts","500g"],["Rosemary","5 sprigs"],["Cranberries","100g"],["Eggs","4-6"],["Onion","1"],["Chestnut mushrooms","250g"],["Brown crusty bread","2 slices"],["Puff Pastry","2 sheets"],["Salt",null],["Oil",null]]]],["pancakes",[[["Flour","200g"],["Milk","400ml"],["Butter",null]]]],["paneer-jalfrezi",[[["Onion","1"],["Green pepper","1"],["Chilli flakes","1/2 tsp"],["Ginger","15g"],["Paneer","1 block (225g)"],["Garlic","2-3 cloves"],["Salad Tomatoes","2"],["Curry powder","1 tbsp"],["Tomato pur\u00e9e","32g"],["Brown sugar","2 tsp"],["Stock cube","1"],["Water","300ml"]]]],["parsnip-and-ginger-soup",[[["Parnsips","450g"],["Onion","1"],["Potato","1"],["Orange","1"],["Fresh Ginger","2.5cm"],["Stock cube",null],["Butter",null],["Single Cream","250ml"]]]],["parsnip-and-lentil-lasagne",[[["Lasagne sheets","200g"],["Soft Goats cheese","150g"],["Feta","200g"],["Milk","100ml"],["Parsnips","400g"],["Red Onion","1 large"],["Red lentils","100g"],["Red peppers","2 large"],["Carrot","1 large"],["Vegetable Stock","300ml"],["Passata","250ml"],["Kidney Beans","2 400g tins"],["Sunflower Oil",null],["Bay Leaf",null],["Salt",null],["Pepper",null],["Nutmeg",null]]]],["peanut-butter-biscuits",[[["Crunchy peanut butter","250g"],["Light Brown soft sugar","200g"],["Egg","1 medium"]]]],["pistachio-ice-cream",[[["Pistachios","175g"],["Mascarpone","230g"],["Eggs","2"],["Icing Sugar","60g"]]]],["pomegranate-lime-ice-cream",[[["Pomegranate powder","20g"],["Lime Juice","2tsp"],["Mascarpone","230g"],["Eggs","2"],["Icing Sugar","60g"]]]],["pomegranate-persian-halloumi",[[["Halloumi","200g"],["Couscous","125g"],["Boiled water","170ml"],["Red onion","2"],["Sultanas","30g"],["Pomegranate molasses","15g"],["Pomegranate seeds","10g"],["Ras el hanout","1 tbsp"],["Mint","10g"],["Natural yoghurt","80g"],["Olive oil",null],["Salt",null],["Pepper",null]]]],["quesadillas",[[["Mixed beans","1 400ml tin"],["Kidney beans","1 400ml tin"],["Frozen Sweetcorn",null],["Cheddar","60g"],["Spring Onions","8"],["Chipotle paste","5 tsp"],["Tomato Puree","2 tbsp"],["Tortillas","4"],["Soured Cream",null],["Oil",null],["Salt",null],["Pepper",null]]]],["red-pepper-soup",[[["Butter","Knob"],["Onion","1"],["Red Pepper","1"],["Apple","1"],["Carrot","1"],["Water","600ml"],["Stock Cube",null],["Mixed Herbs",null],["Extra vegetables",null]]]],["rhubarb-crumble",[[["Plain Flour","120g"],["Dark brown muscovado sugar","50g"],["Butter","90g"],["Rhubarb","3 sticks"],["Pitted Dates","50g"],["Fresh Ginger","2cm"],["Golden Syrup","1-2 tbsp"]]]],["roast-beetroot-dahl",[[["Beetroot","500g"],["Onion","1"],["Spring Onions","2"],["Lime","1"],["Garlic","2 cloves"],["Flatbreads","4"],["Sri Lankan Curry Powder","a few tablespoons"],["Stock cube","1"],["Coconut Milk","1 400ml tin"],["Red Lentils","150g"],["Water","200ml"],["Oil",null]]]],["beetroot-risotto",[[["Beetroot","5"],["Arborio rice","1 cup"],["White wine","A decent slosh"],["Thyme",null],["Stock cube","1"],["Water","700ml"],["Soft Goats Cheese","up to 75g"]]]],["roasted-artichoke-pasta",[[["Sun-dried Tomatoes","100g"],["Roasted Artichokes","140g"],["Pine Nuts",null],["Ricotta","80-125g"],["Pasta",null],["Mixed Herbs",null],["Salt",null]]]],["roasted-vegetable-lasagne",[[["Red Peppers","3"],["Aubergines","2"],["Onions","2"],["Garlic Cloves","2"],["Carrot","1"],["Tomato Puree","2 tbsp"],["Red or White Wine",null],["Chopped Tomatoes","3 tins"],["Dried mixed herbs","handful"],["Butter","knob"],["Plain Flour",null],["Milk",null],["Fresh Lasagne sheets","300g"],["Mozzarella","125g"],["Olive Oil",null],["Cherry Tomatoes",null],["Fresh Basil",null]]]],["roasted-vegetable-moroccan-tagine",[[["Aubergine","1"],["Carrots","2"],["Pepper","1"],["Sweet potato","1"],["Garlic","4 cloves"],["Broccoli","1/2"],["Onion","1"],["Rosemary","1/2 tsp"],["Ground Cumin","1/2 tsp"],["Ground Coriander","1/2 tsp"],["Turmeric Powder","1/2 tsp"],["Chickpeas","1 tin"],["Tinned Tomatoes","1 tin"],["Harissa Paste","1 tsp"],["Honey","1 tsp"],["Stock Cube","1"],["Couscous","200g"],["Olive Oil",null],["Salt",null]]]],["roasted-vegetable-tart",[[["Aubergines","400g"],["Courgettes","400g"],["Red Pepper","1"],["Yellow Pepper","1"],["Red Onions","150g"],["Garlic","1 clove"],["Fresh Rosemary or Thyme","1 tsp"],["Olive Oil","3 tbsp"],["Salt",null],["Black Pepper",null],["Filo Pastry","4 sheets"],["Feta","150g"]]]],["russian-mushroom-julienne",[[["Mushrooms","250g"],["Onion",null],["White wine",null],["Soured cream","150ml"],["Double cream","120ml"],["Mozzarella cheese","240g"],["Butter",null]]]],["saag-paneer",[[["Paneer","1 block"],["Ground turmeric","2 tsp"],["Chilli powder","1/2 tsp"],["Onion","1"],["Ground cumin","1/2 tsp"],["Ground cinnamon","1/2 tsp"],["Tomatoes","2"],["Spinach","1 large bag minimum"],["Peas",null]]]],["scotch-pancakes",[[["Plain flour","200g"],["Baking powder","1/2 tbsp"],["Fine salt","1/2 tsp"],["Golden caster sugar","50g"],["Egg","1"],["Milk","200ml"],["Butter",null]]]],["scrambled-eggs",[[["Butter",null],["Eggs","2"],["Milk",null],["Salt",null],["Black pepper",null]]]],["seafood-lasagne",[[["Semi-skimmed milk","1L"],["Mixed seafood","400g"],["Garlic clove","1"],["Plain flour","50g"],["English mustard","1 tsp"],["Fresh lasagne sheets","400g"],["Tarragon","2 tbsp"],["Baby spinach","large handful"],["Cheddar","150g"],["Parmesan","75g"],["Butter",null]]]],["shawarma-wraps",[[["Sweet Potato","1"],["Red Pepper","1"],["Red Onion","1"],["Vivera Plant-Based Shawarma Kebab","1"],["Garlic","3 cloves"],["Wraps",null],["Siracha Sauce",null],["Yoghurt",null],["Mayonaise",null],["Fresh coriander","10g"],["Olive Oil",null],["Salt",null],["Pepper",null]],[["Cumin","1/2 tsp"],["Ground Coriander","1/2 tsp"],["Smoked Paprika","1/2 tsp"],["Ground Cinnamon","1/4 tsp"],["Allspice","1/4 tsp"],["Cayenne Pepper","1/4 tsp"]]]],["vegetarian-mexican",[[["Black Beans","2 tins"],["Frozen sweetcorn","260g"],["Squash","1"],["Garlic",null],["Cheese",null],["Tortillas",null],["Cinnamon",null],["Cumin",null],["Salt",null],["Soured Cream",null]]]],["smoky-sweet-potato-chickpea-stew",[[["Sweet Potatoes","620g"],["Garlic Cloves","4"],["Carrots","2"],["Onions","2"],["Red pepper","1"],["Fresh ginger","2 tsp"],["Spinach","a large handful"],["Sundried tomatoes","285g"],["Vegetarian chorizo sausages","6"],["Kidney Beans","1 400g tin"],["Chopped tomatoes","1 400g tin"],["Chickpeas","2 400g tins"],["Curry powder","2 tsps"],["Smoked paprika","several teaspoons"],["Stock cube",null],["Olive oil",null]]]],["smoky-fish-curry",[[["Onion","1"],["Garlic","1 clove"],["Smoked fish","200g"],["Coriander","5g"],["Red chilli","1"],["Creamed coconut","50g"],["Ginger","30g"],["Stock cube","1"],["Ground coriander","1 tsp"],["Ground turmeric","1 tsp"],["Squash","200g"],["Water","350ml"],["Naan Bread",null]]]],["spanakopita",[[["Frozen spinach","300g"],["Fresh Parsley","30g"],["Onion","1"],["Garlic","2 cloves"],["Feta","200g"],["Eggs","2"],["Fillo Pastry","125g"],["Fennel Seeds","1 tsp"],["Salt",null],["Pepper",null],["Olive Oil",null]]]],["spiced-apple-winter-soup",[[["Apples (Red)","4"],["Onions (white or brown)","4 medium or 2 large"],["Carrots","2"],["Apple juice","2lt"],["Parsnips","2 small or 1 large"],["Fresh ginger","1 knob"],["Cinamon","1 3 inch stem crush into rough segments"],["Cloves","5 cloves"],["Star anise","2 cloves"],["Fennel","2 tsp"],["Thyme","4 sprigs, or 2 tbsp dried"],["Juniper berries (Dried)","3 - 5"],["Vegetable stock pots","2"],["Cooking oil of choice","as needed to prevent stickage"]]]],["spicy-smoked-paprika-chorizo",[[["Spanish Chorizo","225g"],["Red Onions","4"],["Garlic Cloves","Several"],["Pointed Red Peppers","2"],["Carrots","2-3"],["Fresh Spinach","100g"],["Tinned Tomatoes","3"],["Tomato Paste","4 Inches"],["Red Wine","10-20CL"],["Smoked Paprika","Several teaspoons"],["Olive Oil",null],["Chilli Flakes",null],["Honey","2 tsp"],["Dried Oregano","1 tbsp"],["Stick of Cinnamon","1"],["Pitted Black Olives","6 tbsp"],["Cannellini beans","1 400g tin"],["Celery",null],["Whole Cloves","2-3"],["Mascarpone","2 tbsp"]]]],["squash-gnocchi-gratin",[[["Squash","1"],["Rosemary","1 sprig"],["Chilli Flakes","1 tsp"],["Gnocchi","500g"],["Kale","125g"],["Creme fraiche","200ml"],["Feta","100g"],["Panko breadcrumbs ","100g"],["Salt",null],["Olive Oil",null],["Black pepper",null]]]],["strawberry-basil-ice-cream",[[["Strawberry powder","12.5g"],["Fresh Basil","15g"],["Black pepper",null],["Green Food Colouring","1tsp"],["Mascarpone","230g"],["Eggs","2"],["Icing Sugar","60g"]]]],["sweet-chilli-feta-pasta",[[["Pine nuts",null],["Cashew nuts",null],["Brazil nuts",null],["Feta","1 block, approx. 200g"],["Tuna chunks","1 80g tin"],["Sweet chilli sauce",null],["Black pepper",null],["Fusilli",null]]]],["tangy-vegetable-pad-thai",[[["Butternut squash cubes","160g"],["Rice noodles","150g"],["Red pepper","1"],["Courgette","1"],["Garlic","2 cloves"],["Lime","1"],["Tamarind paste","15g"],["Soy sauce","30ml"],["Mirin","15ml"],["Sriracha hot chilli sauce","8ml"],["Thai basil","5g"],["Coriander","5g"],["Roasted peanuts","25g"],["Salt",null],["Pepper",null],["Sugar",null],["Vegetable oil",null]]]],["toad-in-the-hole",[[["Sausages","6"],["Plain Flour","150g"],["Eggs","2"],["Milk","125ml"],["Water","125ml"],["Salt","A pinch"],["Gravy granules","7 tsp"],["Red wine",null],["Marmite","1/2 tsp"]]]],["tofu-katsu-curry",[[["Plain tofu","280g"],["White long grain rice","130g"],["Tenderstem broccoli","160g"],["Panko breadcrumbs","40g"],["Vegan mayonnaise","25ml"],["Fresh root ginger","15g"],["Curry powder","1 tbsp"],["Plain flour","2 tbsp"],["Soy sauce","15ml"],["Mango chutney","20g"],["Salt",null],["Vegetable oil",null]]]],["truffle-burgers",[[["Beyond Meat Burgers","4"],["Brioche buns","4"],["Manchego","4 slices"],["Jam",null],["Mayonnaise",null],["Truffle oil",null],["Oil",null]]]],["turmeric-ginger-ice-cream",[[["Mascarpone","460g"],["Eggs","4"],["Icing Sugar","120g"],["Honey","1-2 tbsp"],["Turmeric","1 1/2 tsp"],["Fresh Ginger","1 tsp"],["Vanilla Extract","1/2 tsp"],["Cardamon","1/2 tsp"],["Cinnamon","1 tsp"],["Chilli Powder","a pinch"]]]],["vegan-brownies",[[["Vitalite","200g"],["Dark Chocolate","300g"],["Aquafaba","214ml"],["Cream of tartar","1/4 tsp"],["Granulated Sugar","450g"],["Vanilla Extract","1 tbsp"],["Plain Flour","200g"],["Salt","1/2 tsp"]]]],["vegetable-primavera",[[["Baby vegetables","3-4 varieties"],["Tortellini or ravioli","400g"],["Olive oil","1 tbsp"],["Lemon","1/2"],["Wholegrain mustard","1-2 tbsp"],["Salt",null],["Black pepper",null]]]],["veggie-shepherds-pie",[[["Onions","1"],["Carrots","4"],["Celery","1 head"],["Garlic","4 Cloves"],["Chestnut mushrooms","200g"],["Red Lentils","230g"],["Butter",null],["Bay Leaf","2"],["Thyme","1 tbsp"],["Red wine","100ml"],["Stock cube","1"],["Tomato pur\u00e9e","3 tbsp"],["King Edwards Potatoes","500g"],["Butter","85g"],["Milk","100ml"],["Cheddar","50g"]]]],["wasabi-ice-cream",[[["Wasabi paste","2 tsp"],["Green food colouring","1/2 tsp"],["Mascarpone","230g"],["Eggs","2"],["Icing Sugar","60g"]]]]]"##;

/// `[{"ingredients": [{"name": ..., "quantity": ...}, ...]}, ...]` for one
/// corpus entry.
fn blocks_of(entry: &Value) -> Value {
    let blocks = entry[1]
        .as_array()
        .unwrap_or_else(|| panic!("corpus entry {entry} is not a block list"));

    Value::Array(
        blocks
            .iter()
            .map(|block| {
                let ingredients = block
                    .as_array()
                    .unwrap_or_else(|| panic!("corpus block {block} is not an ingredient list"));

                json!({
                    "ingredients": ingredients
                        .iter()
                        .map(|ingredient| {
                            json!({"name": ingredient[0], "quantity": ingredient[1]})
                        })
                        .collect::<Vec<Value>>(),
                })
            })
            .collect(),
    )
}

/// The copy text of every recipe in the corpus.
#[test]
fn golden_ingredients_copy_text() {
    let cases = golden_cases("copy_text.json", COPY_TEXT_CASES);
    let corpus: Value = serde_json::from_str(CORPUS_BLOCKS).expect("the embedded corpus is JSON");
    let corpus = corpus.as_array().expect("the embedded corpus is an array");

    assert_eq!(
        corpus.len(),
        COPY_TEXT_CASES,
        "the embedded corpus holds {} recipes, expected {COPY_TEXT_CASES}",
        corpus.len()
    );

    let mut checked = 0;

    for case in &cases {
        let permalink = case["permalink"].as_str().unwrap();
        let entry = corpus
            .iter()
            .find(|entry| entry[0].as_str() == Some(permalink))
            .unwrap_or_else(|| panic!("no embedded blocks for recipe {permalink}"));

        assert_eq!(
            ingredients_copy_text(&blocks_of(entry)),
            case["text"].as_str().unwrap(),
            "copy text of {permalink}"
        );

        checked += 1;
    }

    assert_eq!(checked, COPY_TEXT_CASES, "checked every copy text case");
}
