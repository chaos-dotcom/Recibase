//! `se.reciba.api.model.Permalink`.

use crate::stop_words;
use unicode_normalization::UnicodeNormalization;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Permalink {
    pub value: String,
}

impl Permalink {
    pub fn new(value: &str) -> Self {
        Permalink {
            value: value.to_string(),
        }
    }

    pub fn from_raw_string(raw: &str) -> Permalink {
        Permalink {
            value: from_raw_string(raw),
        }
    }
}

/// `Permalink.fromRawString`.
pub fn from_raw_string(raw: &str) -> String {
    // "fôô AND  bär!"
    let lowered = raw.to_lowercase();
    // "fôô and  bär!"
    let lower_no_accents = strip_accents(&lowered);
    // "foo and  bar!"
    let latin_only: String = lower_no_accents
        .chars()
        .filter(|c| matches!(c, 'a'..='z' | ' ' | '-'))
        .collect();
    // "foo and  bar"
    let no_and_word = latin_only
        .split(' ')
        .filter(|word| !stop_words::words().iter().any(|w| w == word))
        .collect::<Vec<_>>()
        .join(" ");
    // "foo  bar"
    let no_white_space_blocks = normalize_space(&no_and_word);
    // "foo bar"
    no_white_space_blocks.replace(' ', "-")
}

/// `org.apache.commons.lang3.StringUtils.normalizeSpace`.
pub fn normalize_space(input: &str) -> String {
    input.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// `org.apache.commons.lang3.StringUtils.stripAccents`: NFD, then drop the
/// combining diacritical marks.
pub fn strip_accents(input: &str) -> String {
    input
        .nfd()
        .filter(|c| {
            let cp = *c as u32;
            !(0x0300..=0x036f).contains(&cp)
        })
        .collect()
}
