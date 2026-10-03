//! Port of `se.reciba.api.ScalaLiteralSpec`.

use recibase_submit::ScalaLiteral;

/// Built with concatenation so that a compiler's own `\u` pass cannot rewrite
/// them - the Scala spec does the same.
fn samples() -> Vec<String> {
    let slash = "\\";
    vec![
        String::new(),
        "plain".to_string(),
        "purée".to_string(),
        "quote\"here".to_string(),
        format!("back{}slash", slash),
        "line\nbreak".to_string(),
        "tab\there".to_string(),
        "cr\rhere".to_string(),
        "${180.celsius}".to_string(),
        "\"\"\"".to_string(),
        format!("{}u0022; evil; {}u0022", slash, slash),
        format!("{}u005c{}u0022", slash, slash),
        format!("{}uu0022", slash),
        format!("{}{}", slash, slash),
        "say \"hi\"".to_string(),
    ]
}

#[test]
fn round_trips_through_scalas_unicode_pass_and_string_escapes() {
    for raw in samples() {
        assert_eq!(decode(&ScalaLiteral::quote(&raw)), raw);
    }
}

#[test]
fn never_leaves_a_raw_quote_or_a_user_backslash_in_the_literal() {
    for raw in samples() {
        let quoted = ScalaLiteral::quote(&raw);
        assert!(structural(&quoted), "not structural: {}", quoted);
    }
}

#[test]
fn quotes_exactly_as_the_scala_does() {
    assert_eq!(ScalaLiteral::quote(""), r#""""#);
    assert_eq!(ScalaLiteral::quote("plain"), "\"plain\"");
    assert_eq!(ScalaLiteral::quote("purée"), "\"purée\"");
    assert_eq!(ScalaLiteral::quote("say \"hi\""), r#""say \u005c\u0022hi\u005c\u0022""#);
    assert_eq!(ScalaLiteral::quote("line\nbreak"), r#""line\nbreak""#);
    assert_eq!(ScalaLiteral::quote("cr\rhere"), r#""cr\rhere""#);
    assert_eq!(ScalaLiteral::quote("tab\there"), r#""tab\there""#);
    assert_eq!(ScalaLiteral::quote("a\\b"), r#""a\u005c\u005cb""#);
    assert_eq!(ScalaLiteral::quote("\"\"\""), r#""\u005c\u0022\u005c\u0022\u005c\u0022""#);
}

/// Scala translates `\u` escapes, including `\uuXXXX`, before tokenising, and
/// does not rescan the result.
fn unicode_pass(source: &str) -> String {
    let characters: Vec<char> = source.chars().collect();
    let mut out = String::new();
    let mut index = 0;
    while index < characters.len() {
        let multi_u = characters[index] == '\\'
            && index + 1 < characters.len()
            && characters[index + 1] == 'u';
        if multi_u {
            let mut hex = index + 2;
            while hex < characters.len() && characters[hex] == 'u' {
                hex += 1;
            }
            let complete =
                hex + 4 <= characters.len() && (0..4).all(|offset| is_hex(characters[hex + offset]));
            if complete {
                let digits: String = characters[hex..hex + 4].iter().collect();
                let value = u32::from_str_radix(&digits, 16).expect("four hex digits");
                out.push(char::from_u32(value).unwrap_or('\u{fffd}'));
                index = hex + 4;
            } else {
                out.push(characters[index]);
                index += 1;
            }
        } else {
            out.push(characters[index]);
            index += 1;
        }
    }
    out
}

fn decode(quoted: &str) -> String {
    let source: Vec<char> = unicode_pass(quoted).chars().collect();
    let mut out = String::new();
    let mut index = 1;
    while index < source.len() {
        match source[index] {
            '"' => {
                if index != source.len() - 1 {
                    panic!("trailing text in {}", quoted);
                }
                return out;
            }
            '\\' => {
                match source[index + 1] {
                    '\\' => out.push('\\'),
                    '"' => out.push('"'),
                    'n' => out.push('\n'),
                    'r' => out.push('\r'),
                    't' => out.push('\t'),
                    other => panic!("bad escape {} in {}", other, quoted),
                }
                index += 2;
            }
            character => {
                out.push(character);
                index += 1;
            }
        }
    }
    panic!("unterminated {}", quoted);
}

fn structural(quoted: &str) -> bool {
    if !quoted.starts_with('"') || !quoted.ends_with('"') {
        return false;
    }
    let characters: Vec<char> = quoted.chars().skip(1).collect();
    let inner = &characters[..characters.len() - 1];
    let mut index = 0;
    while index < inner.len() {
        match inner[index] {
            '"' => return false,
            '\\' => {
                let rest: String = inner[index..].iter().collect();
                if rest.starts_with("\\u005c") || rest.starts_with("\\u0022") {
                    index += 6;
                } else if rest.starts_with("\\n")
                    || rest.starts_with("\\r")
                    || rest.starts_with("\\t")
                {
                    index += 2;
                } else {
                    return false;
                }
            }
            _ => index += 1,
        }
    }
    index == inner.len()
}

fn is_hex(character: char) -> bool {
    character.is_ascii_hexdigit()
}
