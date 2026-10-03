//! A port of `TemperatureUtilsSpec` and the `StringUtils` / `PermalinkSpec`
//! cases, plus temperature strings taken from the byte-exact capture in
//! `capture-scala/` (a Scala server's own output).
//!
//! OWNER: workstream W6.

use recibase_core::permalink::from_raw_string;
use recibase_core::utils::int_utils::{celsius, fahrenheit, simple_fan_instruction};
use recibase_core::utils::string_utils::unpluralise;

// ---------------------------------------------------------------------------
// IntUtils.TemperatureUtils
// ---------------------------------------------------------------------------

#[test]
fn temperature_utils_converts_celsius_to_a_formatted_string() {
    // `TemperatureUtilsSpec`: F is rounded to the nearest multiple of 5 and
    // the gas mark is an int.
    assert_eq!(celsius(190), "190°C (375°F, gas mark 5)");
}

#[test]
fn temperature_utils_converts_fahrenheit_ignoring_the_gas_mark_when_there_is_none() {
    assert_eq!(fahrenheit(380), "195°C (380°F)");
}

#[test]
fn temperature_utils_matches_the_scala_servers_own_output() {
    // Every temperature string the Scala server printed for the capture
    // (`capture-scala/*.norm`, the recipe bodies).
    assert_eq!(celsius(160), "160°C (320°F, gas mark 3)");
    assert_eq!(celsius(180), "180°C (355°F, gas mark 4)");
    assert_eq!(celsius(190), "190°C (375°F, gas mark 5)");
    assert_eq!(celsius(200), "200°C (390°F, gas mark 6)");
    assert_eq!(simple_fan_instruction(200), "200°C (fan 180°C)");
    assert_eq!(simple_fan_instruction(220), "220°C (fan 200°C)");
}

#[test]
fn temperature_utils_walks_the_whole_gas_mark_table() {
    assert_eq!(celsius(140), "140°C (285°F, gas mark 1)");
    assert_eq!(celsius(150), "150°C (300°F, gas mark 2)");
    assert_eq!(celsius(170), "170°C (340°F, gas mark 3.5)");
    assert_eq!(celsius(210), "210°C (410°F, gas mark 7)");
    assert_eq!(celsius(220), "220°C (430°F, gas mark 8)");
    assert_eq!(celsius(230), "230°C (445°F, gas mark 8.5)");
    assert_eq!(celsius(240), "240°C (465°F, gas mark 9)");
    // No clean gas mark conversion.
    assert_eq!(celsius(250), "250°C (480°F)");
    assert_eq!(celsius(100), "100°C (210°F)");
}

#[test]
fn temperature_utils_fahrenheit_table() {
    assert_eq!(fahrenheit(300), "150°C (300°F, gas mark 2)");
    assert_eq!(fahrenheit(325), "165°C (325°F)");
    assert_eq!(fahrenheit(350), "175°C (350°F)");
    // The rounded celsius value is what selects the gas mark.
    assert_eq!(fahrenheit(375), "190°C (375°F, gas mark 5)");
    assert_eq!(fahrenheit(400), "205°C (400°F)");
    assert_eq!(fahrenheit(425), "220°C (425°F, gas mark 8)");
    assert_eq!(fahrenheit(450), "230°C (450°F, gas mark 8.5)");
    assert_eq!(fahrenheit(475), "245°C (475°F)");
    assert_eq!(fahrenheit(250), "120°C (250°F)");
}

#[test]
fn temperature_utils_carries_the_degree_sign_not_an_ordinal() {
    // The separator is U+00B0 DEGREE SIGN followed by "C"/"F".
    assert_eq!(celsius(200).chars().nth(3), Some('\u{00B0}'));
}

// ---------------------------------------------------------------------------
// StringUtils.PluralUtils
// ---------------------------------------------------------------------------

#[test]
fn unpluralise_strips_a_trailing_s() {
    assert_eq!(unpluralise("carrots"), "carrot");
    assert_eq!(unpluralise("carrot"), "carrot");
    // `stripSuffix("es")` only runs when the string does not end in "s",
    // which cannot happen for a string ending in "es".
    assert_eq!(unpluralise("tomatoes"), "tomatoe");
    assert_eq!(unpluralise(""), "");
    // Case matters: "S" does not end in a lowercase "s".
    assert_eq!(unpluralise("S"), "S");
}

// ---------------------------------------------------------------------------
// Permalink.fromRawString
// ---------------------------------------------------------------------------

#[test]
fn permalink_lowercases_the_input() {
    assert_eq!(from_raw_string("FooBar"), "foobar");
}

#[test]
fn permalink_replaces_spaces_with_dashes() {
    assert_eq!(from_raw_string("foo bar"), "foo-bar");
}

#[test]
fn permalink_trims_whitespace_from_the_start_and_end() {
    assert_eq!(from_raw_string(" foo "), "foo");
}

#[test]
fn permalink_removes_accents() {
    assert_eq!(from_raw_string("fôôbär"), "foobar");
}

#[test]
fn permalink_removes_non_latin_characters() {
    assert_eq!(from_raw_string(".f$^b@r!"), "fbr");
}

#[test]
fn permalink_removes_duplicate_whitespace_left_after_other_transforms() {
    assert_eq!(from_raw_string("foo $ bar"), "foo-bar");
}

#[test]
fn permalink_removes_stop_words() {
    assert_eq!(from_raw_string("and bar with"), "bar");
}

#[test]
fn permalink_preserves_pre_existing_dashes() {
    assert_eq!(from_raw_string("foo-bar"), "foo-bar");
}

#[test]
fn permalink_keeps_duplicate_dashes() {
    // `PermalinkSpec` marks this case `pendingUntilFixed`, so the Scala
    // currently leaves the double dash in place. Copied as-is.
    assert_eq!(from_raw_string("foo--bar"), "foo--bar");
}

#[test]
fn permalink_removes_adjectives() {
    assert_eq!(from_raw_string("creamy foo and bar with"), "foo-bar");
}

#[test]
fn permalink_matches_the_captures_recipe_permalinks() {
    // Recipe permalinks from the Scala server's `/recipes/` capture that are
    // derived from the recipe name (73 of the 95 are; the rest override it).
    assert_eq!(
        from_raw_string("Vegetable Primavera"),
        "vegetable-primavera"
    );
    assert_eq!(
        from_raw_string("Goan King Prawn Balchão"),
        "goan-king-prawn-balchao"
    );
    assert_eq!(
        from_raw_string("Christmas Naanwidge"),
        "christmas-naanwidge"
    );
    assert_eq!(
        from_raw_string("Blue Cheese Gnocchi"),
        "blue-cheese-gnocchi"
    );
    assert_eq!(
        from_raw_string("Smoky Sweet potato and chickpea stew"),
        "smoky-sweet-potato-chickpea-stew"
    );
    assert_eq!(
        from_raw_string("Pomegranate Persian Halloumi"),
        "pomegranate-persian-halloumi"
    );
    // `Chicken Curry (WIP)` and `Chilli con Carne` override their permalinks in
    // Scala ("chicken-curry", "chilli-con-carne"); "con" is a stop word, so the
    // name-derived value drops it.
    assert_eq!(from_raw_string("Chicken Curry (WIP)"), "chicken-curry-wip");
    assert_eq!(from_raw_string("Chilli con Carne"), "chilli-carne");
}
