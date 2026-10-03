//! `se.reciba.api.submit.ScalaLiteral` - a Scala string literal, quoted exactly
//! as the Scala implementation quotes it.

/// Quotes a string as a Scala `"..."` literal.
///
/// Scala replaces `\uXXXX` escapes before it tokenises, including inside
/// strings, and one or more `u` characters are allowed. A user backslash must
/// never appear as a raw `\` in the file: a value such as `\u0022` would
/// otherwise close the literal during that pass. Backslash and quote are
/// therefore written as `\u005c` sequences that decode to ordinary string
/// escapes (`\\` and `\"`).
pub struct ScalaLiteral;

impl ScalaLiteral {
    pub fn quote(raw: &str) -> String {
        let mut quoted = String::with_capacity(raw.len() + 2);
        quoted.push('"');
        for character in raw.chars() {
            match character {
                '"' => quoted.push_str("\\u005c\\u0022"),
                '\\' => quoted.push_str("\\u005c\\u005c"),
                '\n' => quoted.push_str("\\n"),
                '\r' => quoted.push_str("\\r"),
                '\t' => quoted.push_str("\\t"),
                other => quoted.push(other),
            }
        }
        quoted.push('"');
        quoted
    }
}
