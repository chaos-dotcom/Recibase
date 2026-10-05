//! The `scaler.py` port: recipe quantity parsing, scaling and copy-text
//! merging, matching the Python module.
//!
//! The Python's arithmetic is not the arithmetic the shape of the code
//! suggests, and the golden data pins these quirks down.
//!
//! * A `Fraction` times a `float` is *not* exact.  CPython's
//!   `Fraction.__mul__` falls back to `float(fraction) * value` for a float
//!   operand, and `FractionQuantity.__mul__` only then wraps that float back
//!   up with `Fraction(...)`.  So `1/2 * 1.8` lands on the exact binary rational
//!   `8106479329266893/9007199254740992`, whose denominator sends it down the
//!   `denominator > 10` rendering branch: `0.90`, not `9/10`.  Likewise
//!   `1/3 * 2.0` is `6004799503160661/36028797018963968`, which renders as
//!   `0.67`, not `2/3`.
//! * `MixedFractionQuantity.__mul__` keeps whatever that product is: a `float`
//!   here, never a `Fraction`, so the scaled quantity renders through
//!   `SimpleQuantity` (`1 1/2 tsp * 3.0` is `4.50 tsp`, not `4 1/2 tsp`).
//! * `get_scale_factor` always returns `float(raw_scale)`, so the float path
//!   is the only one the application ever takes; this port scales every
//!   quantity in floating point.  `Fraction * int` is the exact path, and it
//!   still matters for `add_quantities`, which adds two parsed quantities.

use serde_json::Value;

// ---------------------------------------------------------------------------
// Python `float()`
// ---------------------------------------------------------------------------

/// `scaler.get_scale_factor`: the `scale` request parameter as a factor.
///
/// `None` when the parameter is missing, unparseable, or a scale of 1, 0,
/// more than 50 or less than 0.  `"nan"` parses and is returned: both range
/// comparisons are false for NaN, exactly as in the Python.
pub fn get_scale_factor(raw: Option<&str>) -> Option<f64> {
    let scale = python_float(raw?)?;

    if scale == 1.0 || scale == 0.0 {
        return None;
    }

    if scale > 50.0 || scale < 0.0 {
        return None;
    }

    Some(scale)
}

/// Python's `float(str)`, which is more permissive than `str::parse::<f64>()`:
/// surrounding whitespace, `_` separators between digits, `inf`, `infinity` and
/// `nan` in any case, and any Unicode decimal digit.
fn python_float(raw: &str) -> Option<f64> {
    let text = raw.trim();

    let (negative, digits) = match text.as_bytes().first() {
        Some(b'-') => (true, &text[1..]),
        Some(b'+') => (false, &text[1..]),
        _ => (false, text),
    };

    let lowered = digits.to_ascii_lowercase();

    if lowered == "inf" || lowered == "infinity" {
        return Some(if negative {
            f64::NEG_INFINITY
        } else {
            f64::INFINITY
        });
    }

    if lowered == "nan" {
        return Some(f64::NAN);
    }

    let value = python_decimal_literal(digits)?;

    Some(if negative { -value } else { value })
}

/// The numeric form Python's `float()` accepts: `digits ['.' digits]
/// [('e'|'E') ['+'|'-'] digits]`, with `_` allowed between digits.
fn python_decimal_literal(text: &str) -> Option<f64> {
    let chars: Vec<char> = text.chars().collect();
    let mut index = 0;

    let mut integer = String::new();
    index = scan_digits(&chars, index, &mut integer);

    let mut fraction = String::new();
    let mut has_point = false;

    if index < chars.len() && chars[index] == '.' {
        has_point = true;
        index += 1;
        index = scan_digits(&chars, index, &mut fraction);
    }

    if integer.is_empty() && fraction.is_empty() {
        return None;
    }

    let mut exponent = String::new();

    if index < chars.len() && (chars[index] == 'e' || chars[index] == 'E') {
        index += 1;

        let mut sign = String::new();

        if index < chars.len() && (chars[index] == '+' || chars[index] == '-') {
            sign.push(chars[index]);
            index += 1;
        }

        let mut digits = String::new();
        index = scan_digits(&chars, index, &mut digits);

        if digits.is_empty() {
            return None;
        }

        exponent = format!("e{sign}{digits}");
    }

    if index != chars.len() {
        return None;
    }

    let integer = if integer.is_empty() {
        "0".to_string()
    } else {
        integer
    };

    let fraction = if has_point {
        format!(".{}", if fraction.is_empty() { "0" } else { &fraction })
    } else {
        String::new()
    };

    format!("{integer}{fraction}{exponent}").parse::<f64>().ok()
}

/// Consumes `[0-9](_?[0-9])*` into `out`, returning the new index.  An
/// underscore is only taken when a digit follows it, so `"1_"` and `"1__0"`
/// stop early and are rejected by the caller.
///
/// `float()` takes any Unicode decimal digit (`float("١٢")` is 12.0), unlike
/// the `[0-9]` of the module's regular expressions, so the digits are
/// rewritten to ASCII here.
fn scan_digits(chars: &[char], mut index: usize, out: &mut String) -> usize {
    while index < chars.len() {
        let character = chars[index];

        if let Some(digit) = decimal_digit(character) {
            out.push(char::from_digit(digit, 10).unwrap_or(character));
            index += 1;
        } else if character == '_'
            && !out.is_empty()
            && index + 1 < chars.len()
            && chars[index + 1].is_ascii_digit()
        {
            index += 1;
        } else {
            break;
        }
    }

    index
}

/// The Unicode decimal-digit blocks (`Nd`), each a run of ten from digit zero.
/// Python's `float()` accepts every one of them (`float("\u{661}\u{662}")`
/// is 12.0) and rewrites them to ASCII internally; neither `str::parse::<f64>`
/// nor `char::to_digit` in the standard library sees them.
const DECIMAL_DIGIT_ZEROS: [u32; 66] = [
    0x30, 0x660, 0x6f0, 0x7c0, 0x966, 0x9e6, 0xa66, 0xae6, 0xb66, 0xbe6, 0xc66, 0xce6, 0xd66,
    0xde6, 0xe50, 0xed0, 0xf20, 0x1040, 0x1090, 0x17e0, 0x1810, 0x1946, 0x19d0, 0x1a80, 0x1a90,
    0x1b50, 0x1bb0, 0x1c40, 0x1c50, 0xa620, 0xa8d0, 0xa900, 0xa9d0, 0xa9f0, 0xaa50, 0xabf0, 0xff10,
    0x104a0, 0x10d30, 0x11066, 0x110f0, 0x11136, 0x111d0, 0x112f0, 0x11450, 0x114d0, 0x11650,
    0x116c0, 0x11730, 0x118e0, 0x11950, 0x11c50, 0x11d50, 0x11da0, 0x16a60, 0x16ac0, 0x16b50,
    0x1d7ce, 0x1d7d8, 0x1d7e2, 0x1d7ec, 0x1d7f6, 0x1e140, 0x1e2f0, 0x1e950, 0x1fbf0,
];

/// The value of a Unicode decimal digit, as `Py_UNICODE_TODECIMAL` gives it.
fn decimal_digit(character: char) -> Option<u32> {
    let code = character as u32;

    DECIMAL_DIGIT_ZEROS.iter().find_map(|&zero| {
        if code >= zero && code < zero + 10 {
            Some(code - zero)
        } else {
            None
        }
    })
}

// ---------------------------------------------------------------------------
// Exact rationals
// ---------------------------------------------------------------------------

/// An exact rational, like Python's `Fraction`: `den > 0` and reduced.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Frac {
    num: i128,
    den: i128,
}

impl Frac {
    /// `Fraction(num, den)`, or `None` for `den == 0`.  The Python raises
    /// `ZeroDivisionError` there, so `"1/0"` is a crash rather than a parse
    /// failure; this port treats the quantity as unparseable instead.
    fn new(num: i128, den: i128) -> Option<Frac> {
        if den == 0 {
            return None;
        }

        let (mut num, mut den) = (num, den);

        if den < 0 {
            num = -num;
            den = -den;
        }

        let divisor = gcd(num.unsigned_abs(), den.unsigned_abs());

        if divisor > 1 {
            num /= divisor as i128;
            den /= divisor as i128;
        }

        Some(Frac { num, den })
    }

    fn from_int(value: i128) -> Frac {
        Frac { num: value, den: 1 }
    }

    /// `Fraction(float)`: the float as the exact rational its mantissa and
    /// exponent spell out.  `None` for a non-finite value (`Fraction(nan)`
    /// raises in the Python) or one outside `i128` range.
    fn from_f64(value: f64) -> Option<Frac> {
        if !value.is_finite() {
            return None;
        }

        if value == 0.0 {
            return Some(Frac::from_int(0));
        }

        let bits = value.to_bits();
        let biased = ((bits >> 52) & 0x7ff) as i32;
        let mantissa = (bits & 0x000f_ffff_ffff_ffff) as i128;

        let (mantissa, exponent) = if biased == 0 {
            (mantissa, -1074)
        } else {
            (mantissa | (1i128 << 52), biased - 1075)
        };

        let mantissa = if bits >> 63 == 1 { -mantissa } else { mantissa };

        if exponent >= 0 {
            let shift = 1i128.checked_shl(exponent as u32)?;

            Frac::new(mantissa.checked_mul(shift)?, 1)
        } else {
            let shift = 1i128.checked_shl((-exponent) as u32)?;

            Frac::new(mantissa, shift)
        }
    }

    /// `float(fraction)`, correctly rounded: `num as f64 / den as f64` is one
    /// rounded division of two rounded operands, which agrees with Python's
    /// single-rounding conversion for the magnitudes recipes reach.
    fn to_f64(self) -> f64 {
        self.num as f64 / self.den as f64
    }

    /// `fraction + fraction`, kept exact.  `None` on `i128` overflow.
    fn add(self, other: Frac) -> Option<Frac> {
        let left = self.num.checked_mul(other.den)?;
        let right = other.num.checked_mul(self.den)?;

        Frac::new(left.checked_add(right)?, self.den.checked_mul(other.den)?)
    }

    /// `fraction - int`, kept exact.
    fn sub_int(self, other: i128) -> Option<Frac> {
        Frac::new(
            self.num.checked_sub(other.checked_mul(self.den)?)?,
            self.den,
        )
    }

    /// `fraction % 1 == 0`.
    fn is_integral(self) -> bool {
        self.den == 1
    }
}

fn gcd(mut left: u128, mut right: u128) -> u128 {
    while right != 0 {
        let remainder = left % right;
        left = right;
        right = remainder;
    }

    left
}

// ---------------------------------------------------------------------------
// Quantities
// ---------------------------------------------------------------------------

/// Python's `int`/`float` split, kept so `format_number` can tell the two
/// apart: `CompoundQuantity.count` starts life as an `int` and only becomes a
/// `float` once a factor is applied.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Num {
    Int(i128),
    Float(f64),
}

impl Num {
    fn to_f64(self) -> f64 {
        match self {
            Num::Int(value) => value as f64,
            Num::Float(value) => value,
        }
    }

    fn is_one(self) -> bool {
        match self {
            Num::Int(value) => value == 1,
            Num::Float(value) => value == 1.0,
        }
    }

    /// `scaler.format_number`: a whole number loses its decimal point, the
    /// rest are rendered with two decimals.
    fn format(self) -> String {
        match self {
            Num::Int(value) => value.to_string(),
            Num::Float(value) => format_number_f64(value),
        }
    }
}

/// `scaler.format_number` for a plain `float`.  `value % 1 == 0` goes through
/// `str(int(value))`, which is the exact integer the float spells out.
fn format_number_f64(value: f64) -> String {
    if value.fract() != 0.0 {
        // `'{:.2f}'.format(value)`, whose spelling for a NaN is `nan` where
        // Rust's is `NaN`.  `inf` and `-inf` already agree.
        return if value.is_nan() {
            "nan".to_string()
        } else {
            format!("{:.2}", value)
        };
    }

    match Frac::from_f64(value) {
        Some(exact) => exact.num.to_string(),
        None => format!("{value}"),
    }
}

/// The payload of a `MixedFractionQuantity`.  The Python one holds a
/// `Fraction` when it was parsed or scaled by an `int` and a `float` when it
/// was scaled by a `float`, and `__str__` branches on exactly that.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MixedAmount {
    Fraction(Frac),
    Float(f64),
}

impl MixedAmount {
    fn to_f64(self) -> f64 {
        match self {
            MixedAmount::Fraction(value) => value.to_f64(),
            MixedAmount::Float(value) => value,
        }
    }

    /// `scaled % 1 == 0`.
    fn is_integral(self) -> bool {
        match self {
            MixedAmount::Fraction(value) => value.is_integral(),
            MixedAmount::Float(value) => value.fract() == 0.0,
        }
    }

    /// The whole value, as the `SimpleQuantity` the Python returns for it.
    fn as_num(self) -> Num {
        match self {
            MixedAmount::Fraction(value) => Num::Int(value.num),
            MixedAmount::Float(value) => Num::Float(value),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ApproximateStyle {
    Tilde,
    Suffix,
    Upto,
}

/// A parsed quantity.  One variant per parser in the Python.
#[derive(Clone, Debug, PartialEq)]
pub enum Quantity {
    Simple {
        quantity: Num,
        suffix: Option<String>,
    },
    Range {
        lower: f64,
        upper: f64,
        suffix: Option<String>,
    },
    Fraction {
        quantity: Frac,
        suffix: Option<String>,
    },
    MixedFraction {
        quantity: MixedAmount,
        suffix: Option<String>,
    },
    Compound {
        count: Num,
        size: String,
        unit: String,
        explicit_count: bool,
    },
    Parenthetical {
        count: Num,
        unit: String,
        size: String,
        approx: bool,
    },
    Length {
        amount: f64,
        spacing: String,
        has_piece: bool,
    },
    Descriptive {
        count: f64,
        description: String,
    },
    Approximate {
        style: ApproximateStyle,
        amount: f64,
        unit: String,
    },
}

impl Quantity {
    /// `scaler.Quantity.__mul__`.
    pub fn scale(&self, by: f64) -> Quantity {
        match self {
            Quantity::Simple { quantity, suffix } => Quantity::Simple {
                quantity: mul_num(*quantity, by),
                suffix: suffix.clone(),
            },
            Quantity::Range {
                lower,
                upper,
                suffix,
            } => Quantity::Range {
                lower: lower * by,
                upper: upper * by,
                suffix: suffix.clone(),
            },
            Quantity::Fraction { quantity, suffix } => {
                let scaled = quantity.to_f64() * by;

                match Frac::from_f64(scaled) {
                    Some(exact) => Quantity::Fraction {
                        quantity: exact,
                        suffix: suffix.clone(),
                    },
                    None => Quantity::Simple {
                        quantity: Num::Float(scaled),
                        suffix: suffix.clone(),
                    },
                }
            }
            Quantity::MixedFraction { quantity, suffix } => {
                let scaled = MixedAmount::Float(quantity.to_f64() * by);

                if scaled.is_integral() {
                    Quantity::Simple {
                        quantity: scaled.as_num(),
                        suffix: suffix.clone(),
                    }
                } else {
                    Quantity::MixedFraction {
                        quantity: scaled,
                        suffix: suffix.clone(),
                    }
                }
            }
            Quantity::Compound {
                count, size, unit, ..
            } => Quantity::Compound {
                count: mul_num(*count, by),
                size: size.clone(),
                unit: unit.clone(),
                explicit_count: true,
            },
            Quantity::Parenthetical {
                count,
                unit,
                size,
                approx,
            } => Quantity::Parenthetical {
                count: mul_num(*count, by),
                unit: unit.clone(),
                size: if *approx {
                    scale_approximate_size(size, by).unwrap_or_else(|| size.clone())
                } else {
                    size.clone()
                },
                approx: *approx,
            },
            Quantity::Length {
                amount,
                spacing,
                has_piece,
            } => Quantity::Length {
                amount: amount * by,
                spacing: spacing.clone(),
                has_piece: *has_piece,
            },
            Quantity::Descriptive { count, description } => Quantity::Descriptive {
                count: count * by,
                description: description.clone(),
            },
            Quantity::Approximate {
                style,
                amount,
                unit,
            } => Quantity::Approximate {
                style: *style,
                amount: amount * by,
                unit: unit.clone(),
            },
        }
    }

    /// The Rust spelling of Python's `str(quantity)`.
    pub fn to_text(&self) -> String {
        match self {
            Quantity::Simple { quantity, suffix } => {
                format!("{}{}", quantity.format(), format_suffix(suffix))
            }
            Quantity::Range {
                lower,
                upper,
                suffix,
            } => format!(
                "{}-{}{}",
                format_number_f64(*lower),
                format_number_f64(*upper),
                format_suffix(suffix)
            ),
            Quantity::Fraction { quantity, suffix } => {
                format!("{}{}", fraction_text(*quantity), format_suffix(suffix))
            }
            Quantity::MixedFraction { quantity, suffix } => match quantity {
                MixedAmount::Fraction(exact) => {
                    format!("{}{}", fraction_text(*exact), format_suffix(suffix))
                }
                MixedAmount::Float(value) => {
                    format!("{}{}", format_number_f64(*value), format_suffix(suffix))
                }
            },
            Quantity::Compound {
                count,
                size,
                unit,
                explicit_count,
            } => {
                let unit = pluralize_unit(unit, *count);

                if count.is_one() && !explicit_count {
                    format!("{size} {unit}")
                } else {
                    format!("{} {size} {unit}", count.format())
                }
            }
            Quantity::Parenthetical {
                count,
                unit,
                size,
                approx,
            } => {
                let unit = pluralize_unit(unit, *count);

                if *approx {
                    format!("{} {unit}, approx {size}", count.format())
                } else {
                    format!("{} {unit} ({size})", count.format())
                }
            }
            Quantity::Length {
                amount,
                spacing,
                has_piece,
            } => {
                let scaled_amount = format_number_f64(*amount);

                if *has_piece {
                    let piece = pluralize_unit("piece", Num::Float(*amount));

                    format!("{scaled_amount}{spacing}cm {piece}")
                } else {
                    format!("{scaled_amount}{spacing}cm")
                }
            }
            Quantity::Descriptive { count, description } => {
                let mut words: Vec<String> = description.split(' ').map(str::to_string).collect();

                if *count != 1.0 {
                    let count = Num::Float(*count);
                    let last = words.last().map(|word| word.to_lowercase());

                    if last.as_deref() == Some("minimum") && words.len() >= 2 {
                        let index = words.len() - 2;
                        words[index] = pluralize_unit(&words[index], count);
                    } else if let Some(last) = words.last_mut() {
                        *last = pluralize_unit(last, count);
                    }
                }

                format!("{} {}", format_number_f64(*count), words.join(" "))
            }
            Quantity::Approximate {
                style,
                amount,
                unit,
            } => {
                let amount = format_number_f64(*amount);

                match style {
                    ApproximateStyle::Tilde => format!("~{amount}{unit}"),
                    ApproximateStyle::Suffix => format!("{amount}{unit} approx"),
                    ApproximateStyle::Upto => format!("up to {amount}{unit}"),
                }
            }
        }
    }
}

/// `scaler.format_suffix`.
fn format_suffix(suffix: &Option<String>) -> &str {
    match suffix {
        Some(suffix) => suffix,
        None => "",
    }
}

/// `scaler.pluralize_unit`.
fn pluralize_unit(unit: &str, count: Num) -> String {
    if count.is_one() {
        return unit.to_string();
    }

    if unit.ends_with('s') {
        return unit.to_string();
    }

    format!("{unit}s")
}

/// `scaler.FractionQuantity.__str__`.
fn fraction_text(quantity: Frac) -> String {
    let remainder = quantity.num.div_euclid(quantity.den);

    if quantity.den > 10 {
        return format!("{:.2}", quantity.to_f64());
    }

    if remainder != 0 && quantity.num == quantity.den {
        return remainder.to_string();
    }

    if remainder != 0 {
        let even_fraction = quantity.sub_int(remainder).unwrap_or(quantity);

        format!("{} {}/{}", remainder, even_fraction.num, even_fraction.den)
    } else {
        format!("{}/{}", quantity.num, quantity.den)
    }
}

/// `quantity * factor` for the `int`/`float`-valued fields: the Python's
/// `int * float` is a `float`, and the factor is always a `float` here.
fn mul_num(number: Num, by: f64) -> Num {
    Num::Float(number.to_f64() * by)
}

/// `ParentheticalQuantity.__mul__`'s re-match of its own `size`.
fn scale_approximate_size(size: &str, by: f64) -> Option<String> {
    let chars: Vec<char> = size.chars().collect();
    let end = number_ends(&chars, 0).into_iter().next()?;
    let unit_end = unit_g_ml(&chars, end)?;

    if unit_end != chars.len() {
        return None;
    }

    Some(format!(
        "{}{}",
        format_number_f64(text_to_f64(&chars, 0, end) * by),
        chars[end..unit_end].iter().collect::<String>()
    ))
}

// ---------------------------------------------------------------------------
// The regular expressions, hand rolled
// ---------------------------------------------------------------------------
//
// Every pattern in the Python is anchored with `^...$`, so each of these is a
// full match.  `$` also matches just before a final newline, which
// `parse_quantity` reproduces by dropping one trailing newline up front.

fn digits_end(chars: &[char], start: usize) -> usize {
    let mut index = start;

    while index < chars.len() && chars[index].is_ascii_digit() {
        index += 1;
    }

    index
}

/// The ends of `[0-9]+(?:\.[0-9]+)?` at `start`, longest first: the regex
/// tries the fraction part before giving it up.
fn number_ends(chars: &[char], start: usize) -> Vec<usize> {
    let integer_end = digits_end(chars, start);

    if integer_end == start {
        return Vec::new();
    }

    let mut ends = Vec::new();

    if integer_end < chars.len() && chars[integer_end] == '.' {
        let fraction_end = digits_end(chars, integer_end + 1);

        if fraction_end > integer_end + 1 {
            ends.push(fraction_end);
        }
    }

    ends.push(integer_end);
    ends
}

fn letters_end(chars: &[char], start: usize) -> usize {
    let mut index = start;

    while index < chars.len() && chars[index].is_ascii_alphabetic() {
        index += 1;
    }

    index
}

/// `\s+` and `\s*` (Python's whitespace class, which `is_whitespace` covers).
fn spaces_end(chars: &[char], start: usize) -> usize {
    let mut index = start;

    while index < chars.len() && chars[index].is_whitespace() {
        index += 1;
    }

    index
}

/// `\w`: word characters, Python-flavoured (letters, digits, underscore).
fn is_word_char(character: char) -> bool {
    character.is_alphanumeric() || character == '_'
}

fn word_end(chars: &[char], start: usize) -> usize {
    let mut index = start;

    while index < chars.len() && is_word_char(chars[index]) {
        index += 1;
    }

    index
}

/// `( {0,1}[a-z]+)` anchored to the end of the string: the optional single
/// space is part of the suffix the Python keeps (`" tbsp"`).
fn suffix_at(chars: &[char], start: usize) -> Option<String> {
    let mut index = start;

    if index < chars.len() && chars[index] == ' ' {
        index += 1;
    }

    let end = letters_end(chars, index);

    if end > index && end == chars.len() {
        Some(chars[start..end].iter().collect())
    } else {
        None
    }
}

/// `(g|ml)`, case-insensitively, at `start`; the alternation tries `g` first.
fn unit_g_ml(chars: &[char], start: usize) -> Option<usize> {
    if start < chars.len() && (chars[start] == 'g' || chars[start] == 'G') {
        return Some(start + 1);
    }

    if start + 1 < chars.len()
        && (chars[start] == 'm' || chars[start] == 'M')
        && (chars[start + 1] == 'l' || chars[start + 1] == 'L')
    {
        return Some(start + 2);
    }

    None
}

/// `re.IGNORECASE` for the literal words in the patterns (all ASCII).
fn is_literal_at(chars: &[char], start: usize, literal: &str) -> bool {
    let end = start + literal.len();

    end <= chars.len()
        && chars[start..end]
            .iter()
            .collect::<String>()
            .eq_ignore_ascii_case(literal)
}

fn text_to_f64(chars: &[char], start: usize, end: usize) -> f64 {
    chars[start..end]
        .iter()
        .collect::<String>()
        .parse::<f64>()
        .unwrap_or(f64::NAN)
}

/// Python's `int(...)` on a run of digits.  Absurdly long digit runs saturate
/// rather than panic; nothing in the corpus comes close.
fn text_to_int(chars: &[char], start: usize, end: usize) -> i128 {
    chars[start..end]
        .iter()
        .collect::<String>()
        .parse::<i128>()
        .unwrap_or(i128::MAX)
}

// ---------------------------------------------------------------------------
// The parsers
// ---------------------------------------------------------------------------

/// `SimpleQuantity.parser`.
fn parse_simple(chars: &[char]) -> Option<Quantity> {
    for end in number_ends(chars, 0) {
        let suffix = if end == chars.len() {
            Some(None)
        } else {
            suffix_at(chars, end).map(Some)
        };

        if let Some(suffix) = suffix {
            return Some(Quantity::Simple {
                quantity: Num::Float(text_to_f64(chars, 0, end)),
                suffix,
            });
        }
    }

    None
}

/// `RangeQuantity.parser`.
fn parse_range(chars: &[char]) -> Option<Quantity> {
    for lower_end in number_ends(chars, 0) {
        if lower_end >= chars.len() || chars[lower_end] != '-' {
            continue;
        }

        for upper_end in number_ends(chars, lower_end + 1) {
            let suffix = if upper_end == chars.len() {
                Some(None)
            } else {
                suffix_at(chars, upper_end).map(Some)
            };

            if let Some(suffix) = suffix {
                return Some(Quantity::Range {
                    lower: text_to_f64(chars, 0, lower_end),
                    upper: text_to_f64(chars, lower_end + 1, upper_end),
                    suffix,
                });
            }
        }
    }

    None
}

/// `FractionQuantity.parser`.
fn parse_fraction_quantity(chars: &[char]) -> Option<Quantity> {
    let numerator_end = digits_end(chars, 0);

    if numerator_end == 0 || numerator_end >= chars.len() || chars[numerator_end] != '/' {
        return None;
    }

    let denominator_end = digits_end(chars, numerator_end + 1);

    if denominator_end == numerator_end + 1 {
        return None;
    }

    let suffix = if denominator_end == chars.len() {
        None
    } else {
        Some(suffix_at(chars, denominator_end)?)
    };

    let quantity = Frac::new(
        text_to_int(chars, 0, numerator_end),
        text_to_int(chars, numerator_end + 1, denominator_end),
    )?;

    Some(Quantity::Fraction { quantity, suffix })
}

/// `MixedFractionQuantity.parser`.
fn parse_mixed_fraction(chars: &[char]) -> Option<Quantity> {
    let whole_end = digits_end(chars, 0);

    if whole_end == 0 {
        return None;
    }

    let fraction_start = spaces_end(chars, whole_end);

    if fraction_start == whole_end {
        return None;
    }

    let numerator_end = digits_end(chars, fraction_start);

    if numerator_end == fraction_start
        || numerator_end >= chars.len()
        || chars[numerator_end] != '/'
    {
        return None;
    }

    let denominator_end = digits_end(chars, numerator_end + 1);

    if denominator_end == numerator_end + 1 {
        return None;
    }

    let suffix = if denominator_end == chars.len() {
        None
    } else {
        Some(suffix_at(chars, denominator_end)?)
    };

    let whole = Frac::from_int(text_to_int(chars, 0, whole_end));
    let fraction = Frac::new(
        text_to_int(chars, fraction_start, numerator_end),
        text_to_int(chars, numerator_end + 1, denominator_end),
    )?;

    Some(Quantity::MixedFraction {
        quantity: MixedAmount::Fraction(whole.add(fraction)?),
        suffix,
    })
}

/// `CompoundQuantity.parser`.
fn parse_compound(chars: &[char]) -> Option<Quantity> {
    let count_end = digits_end(chars, 0);

    if count_end > 0 {
        let size_start = spaces_end(chars, count_end);

        if size_start > count_end
            && let Some((size, unit)) = compound_size_and_unit(chars, size_start)
        {
            return Some(Quantity::Compound {
                count: Num::Int(text_to_int(chars, 0, count_end)),
                size,
                unit,
                explicit_count: true,
            });
        }
    }

    let (size, unit) = compound_size_and_unit(chars, 0)?;

    Some(Quantity::Compound {
        count: Num::Int(1),
        size,
        unit,
        explicit_count: false,
    })
}

/// `(\d+(?:g|ml))\s+(\w+)` anchored to the end, as the middle of the compound
/// pattern.
fn compound_size_and_unit(chars: &[char], start: usize) -> Option<(String, String)> {
    let digits_end = digits_end(chars, start);

    if digits_end == start {
        return None;
    }

    let size_end = unit_g_ml(chars, digits_end)?;
    let unit_start = spaces_end(chars, size_end);

    if unit_start == size_end || unit_start >= chars.len() {
        return None;
    }

    if !chars[unit_start..]
        .iter()
        .all(|&character| is_word_char(character))
    {
        return None;
    }

    Some((
        chars[start..size_end].iter().collect(),
        chars[unit_start..].iter().collect(),
    ))
}

/// `ParentheticalQuantity.parser`.
fn parse_parenthetical(chars: &[char]) -> Option<Quantity> {
    let count_end = digits_end(chars, 0);

    if count_end == 0 {
        return None;
    }

    let unit_start = spaces_end(chars, count_end);

    if unit_start == count_end {
        return None;
    }

    let unit_end = word_end(chars, unit_start);

    if unit_end == unit_start {
        return None;
    }

    let count = Num::Int(text_to_int(chars, 0, count_end));
    let unit: String = chars[unit_start..unit_end].iter().collect();

    // `,\s*approx\.\s*(\d+g)`
    if unit_end < chars.len() && chars[unit_end] == ',' {
        let mut index = spaces_end(chars, unit_end + 1);

        if is_literal_at(chars, index, "approx.") {
            index = spaces_end(chars, index + "approx.".len());

            let size_end = digits_end(chars, index);

            if size_end > index
                && unit_g_ml(chars, size_end) == Some(size_end + 1)
                && size_end + 1 == chars.len()
            {
                return Some(Quantity::Parenthetical {
                    count,
                    unit,
                    size: chars[index..size_end + 1].iter().collect(),
                    approx: true,
                });
            }
        }
    }

    // `\s+\((\d+g)\)`
    let paren_start = spaces_end(chars, unit_end);

    if paren_start > unit_end && paren_start < chars.len() && chars[paren_start] == '(' {
        let size_start = paren_start + 1;
        let size_end = digits_end(chars, size_start);

        if size_end > size_start
            && unit_g_ml(chars, size_end) == Some(size_end + 1)
            && size_end + 1 < chars.len()
            && chars[size_end + 1] == ')'
            && size_end + 2 == chars.len()
        {
            return Some(Quantity::Parenthetical {
                count,
                unit,
                size: chars[size_start..size_end + 1].iter().collect(),
                approx: false,
            });
        }
    }

    None
}

/// `LengthQuantity.parser`.
fn parse_length(chars: &[char]) -> Option<Quantity> {
    for amount_end in number_ends(chars, 0) {
        let spacing_end = spaces_end(chars, amount_end);

        if !is_literal_at(chars, spacing_end, "cm") {
            continue;
        }

        let after = spacing_end + 2;
        let spacing: String = chars[amount_end..spacing_end].iter().collect();
        let amount = text_to_f64(chars, 0, amount_end);

        if after == chars.len() {
            return Some(Quantity::Length {
                amount,
                spacing,
                has_piece: false,
            });
        }

        let piece_start = spaces_end(chars, after);

        if piece_start > after
            && is_literal_at(chars, piece_start, "piece")
            && piece_start + "piece".len() == chars.len()
        {
            return Some(Quantity::Length {
                amount,
                spacing,
                has_piece: true,
            });
        }
    }

    None
}

/// `DescriptiveQuantity.parser`.
fn parse_descriptive(chars: &[char]) -> Option<Quantity> {
    for count_end in number_ends(chars, 0) {
        let description_start = spaces_end(chars, count_end);

        if description_start == count_end || description_start >= chars.len() {
            continue;
        }

        let mut description: Vec<char> = chars[description_start..].to_vec();

        // `.+$`: `.` does not cross a newline, but a final one is allowed.
        if let Some(newline) = description.iter().position(|&character| character == '\n') {
            if newline + 1 == description.len() {
                description.truncate(newline);
            } else {
                continue;
            }
        }

        if description.is_empty() {
            continue;
        }

        let description: String = description.iter().collect();

        if !description.contains(' ') && !description.contains('-') {
            continue;
        }

        return Some(Quantity::Descriptive {
            count: text_to_f64(chars, 0, count_end),
            description,
        });
    }

    None
}

/// `ApproximateQuantity.parse`.
fn parse_approximate(chars: &[char]) -> Option<Quantity> {
    // `^~(\d+(?:\.\d+)?)(g|ml)$`
    if chars.first() == Some(&'~') {
        for amount_end in number_ends(chars, 1) {
            if unit_g_ml(chars, amount_end) == Some(chars.len()) {
                return Some(Quantity::Approximate {
                    style: ApproximateStyle::Tilde,
                    amount: text_to_f64(chars, 1, amount_end),
                    unit: chars[amount_end..].iter().collect(),
                });
            }
        }
    }

    // `^(\d+(?:\.\d+)?)(g|ml)\s+approx$`
    for amount_end in number_ends(chars, 0) {
        let Some(unit_end) = unit_g_ml(chars, amount_end) else {
            continue;
        };

        let approx_start = spaces_end(chars, unit_end);

        if approx_start > unit_end
            && is_literal_at(chars, approx_start, "approx")
            && approx_start + "approx".len() == chars.len()
        {
            return Some(Quantity::Approximate {
                style: ApproximateStyle::Suffix,
                amount: text_to_f64(chars, 0, amount_end),
                unit: chars[amount_end..unit_end].iter().collect(),
            });
        }
    }

    // `^up to (\d+(?:\.\d+)?)(g|ml)$`
    if is_literal_at(chars, 0, "up to ") {
        for amount_end in number_ends(chars, "up to ".len()) {
            if unit_g_ml(chars, amount_end) == Some(chars.len()) {
                return Some(Quantity::Approximate {
                    style: ApproximateStyle::Upto,
                    amount: text_to_f64(chars, "up to ".len(), amount_end),
                    unit: chars[amount_end..].iter().collect(),
                });
            }
        }
    }

    None
}

/// `scaler.QUANTITY_PARSERS`, in order.
pub fn parse_quantity(raw_quantity: &str) -> Option<Quantity> {
    // `$` matches just before a final newline, so the patterns see the string
    // without it.
    let raw_quantity = raw_quantity.strip_suffix('\n').unwrap_or(raw_quantity);
    let chars: Vec<char> = raw_quantity.chars().collect();

    parse_approximate(&chars)
        .or_else(|| parse_compound(&chars))
        .or_else(|| parse_parenthetical(&chars))
        .or_else(|| parse_mixed_fraction(&chars))
        .or_else(|| parse_range(&chars))
        .or_else(|| parse_fraction_quantity(&chars))
        .or_else(|| parse_length(&chars))
        .or_else(|| parse_descriptive(&chars))
        .or_else(|| parse_simple(&chars))
}

// ---------------------------------------------------------------------------
// Scaling, merging and the copy text
// ---------------------------------------------------------------------------

/// `scaler.scale_ingredient`: mutates one ingredient object, setting
/// `quantity` to the scaled string and `scaled` to `true` when the quantity
/// parsed.
pub fn scale_ingredient(ingredient: &mut Value, factor: f64) {
    let Some(object) = ingredient.as_object_mut() else {
        return;
    };

    let Some(raw_quantity) = object.get("quantity").and_then(|value| value.as_str()) else {
        return;
    };

    let Some(quantity) = parse_quantity(raw_quantity) else {
        return;
    };

    object.insert("scaled".to_string(), Value::Bool(true));
    object.insert(
        "quantity".to_string(),
        Value::String(quantity.scale(factor).to_text()),
    );
}

/// `scaler.normalize_unit`.
fn normalize_unit(suffix: &Option<String>) -> String {
    suffix.clone().unwrap_or_default().trim().to_lowercase()
}

/// `scaler.is_shopping_omitted_quantity`.
fn is_shopping_omitted_quantity(raw_quantity: Option<&str>) -> bool {
    let Some(raw_quantity) = raw_quantity else {
        return true;
    };

    if raw_quantity.is_empty() {
        return true;
    }

    let lowered = raw_quantity.trim().to_lowercase();

    lowered.ends_with("tsp") || lowered.ends_with("tbsp")
}

/// `quantity + quantity` for the `int`/`float`-valued fields.
fn add_nums(left: Num, right: Num) -> Num {
    match (left, right) {
        (Num::Int(left), Num::Int(right)) => Num::Int(left.saturating_add(right)),
        (left, right) => Num::Float(left.to_f64() + right.to_f64()),
    }
}

/// `mixed + mixed`: exact for two `Fraction` payloads, floating point as soon
/// as a `float` payload is involved.
fn add_mixed(left: MixedAmount, right: MixedAmount) -> MixedAmount {
    match (left, right) {
        (MixedAmount::Fraction(left), MixedAmount::Fraction(right)) => match left.add(right) {
            Some(sum) => MixedAmount::Fraction(sum),
            None => MixedAmount::Float(left.to_f64() + right.to_f64()),
        },
        (left, right) => MixedAmount::Float(left.to_f64() + right.to_f64()),
    }
}

/// The `(suffix, value)` of the three kinds the Python sums as a mixture.
fn summable_parts(quantity: &Quantity) -> Option<(&Option<String>, f64)> {
    match quantity {
        Quantity::Simple { quantity, suffix } => Some((suffix, quantity.to_f64())),
        Quantity::Fraction { quantity, suffix } => Some((suffix, quantity.to_f64())),
        Quantity::MixedFraction { quantity, suffix } => Some((suffix, quantity.to_f64())),
        _ => None,
    }
}

/// `scaler.add_quantities`: sum two parsed quantities when the units are
/// compatible, else `None`.
#[must_use]
pub fn add_quantities(first: &Quantity, second: &Quantity) -> Option<Quantity> {
    if let (
        Quantity::Simple {
            quantity: left,
            suffix: left_suffix,
        },
        Quantity::Simple {
            quantity: right,
            suffix: right_suffix,
        },
    ) = (first, second)
    {
        if normalize_unit(left_suffix) != normalize_unit(right_suffix) {
            return None;
        }

        return Some(Quantity::Simple {
            quantity: add_nums(*left, *right),
            suffix: left_suffix.clone(),
        });
    }

    if let (
        Quantity::Fraction {
            quantity: left,
            suffix: left_suffix,
        },
        Quantity::Fraction {
            quantity: right,
            suffix: right_suffix,
        },
    ) = (first, second)
    {
        if normalize_unit(left_suffix) != normalize_unit(right_suffix) {
            return None;
        }

        return Some(Quantity::Fraction {
            quantity: left.add(*right)?,
            suffix: left_suffix.clone(),
        });
    }

    if let (
        Quantity::MixedFraction {
            quantity: left,
            suffix: left_suffix,
        },
        Quantity::MixedFraction {
            quantity: right,
            suffix: right_suffix,
        },
    ) = (first, second)
    {
        if normalize_unit(left_suffix) != normalize_unit(right_suffix) {
            return None;
        }

        return Some(Quantity::MixedFraction {
            quantity: add_mixed(*left, *right),
            suffix: left_suffix.clone(),
        });
    }

    if let (Some((left_suffix, left)), Some((right_suffix, right))) =
        (summable_parts(first), summable_parts(second))
    {
        if normalize_unit(left_suffix) != normalize_unit(right_suffix) {
            return None;
        }

        return Some(Quantity::Simple {
            quantity: Num::Float(left + right),
            suffix: left_suffix.clone(),
        });
    }

    match (first, second) {
        (
            Quantity::Range {
                lower: lower_left,
                upper: upper_left,
                suffix: left_suffix,
            },
            Quantity::Range {
                lower: lower_right,
                upper: upper_right,
                suffix: right_suffix,
            },
        ) => {
            if normalize_unit(left_suffix) != normalize_unit(right_suffix) {
                return None;
            }

            Some(Quantity::Range {
                lower: lower_left + lower_right,
                upper: upper_left + upper_right,
                suffix: left_suffix.clone(),
            })
        }
        (
            Quantity::Approximate {
                style: left_style,
                amount: left,
                unit: left_unit,
            },
            Quantity::Approximate {
                style: right_style,
                amount: right,
                unit: right_unit,
            },
        ) => {
            if left_style != right_style || left_unit.to_lowercase() != right_unit.to_lowercase() {
                return None;
            }

            Some(Quantity::Approximate {
                style: *left_style,
                amount: left + right,
                unit: left_unit.clone(),
            })
        }
        (
            Quantity::Length {
                amount: left,
                spacing,
                has_piece,
            },
            Quantity::Length {
                amount: right,
                has_piece: right_has_piece,
                ..
            },
        ) => {
            if has_piece != right_has_piece {
                return None;
            }

            Some(Quantity::Length {
                amount: left + right,
                spacing: spacing.clone(),
                has_piece: *has_piece,
            })
        }
        (
            Quantity::Compound {
                count: left,
                size,
                unit,
                ..
            },
            Quantity::Compound {
                count: right,
                size: right_size,
                unit: right_unit,
                ..
            },
        ) => {
            if size.to_lowercase() != right_size.to_lowercase()
                || unit.to_lowercase() != right_unit.to_lowercase()
            {
                return None;
            }

            Some(Quantity::Compound {
                count: add_nums(*left, *right),
                size: size.clone(),
                unit: unit.clone(),
                explicit_count: true,
            })
        }
        (
            Quantity::Parenthetical {
                count: left,
                unit,
                size,
                approx,
            },
            Quantity::Parenthetical {
                count: right,
                unit: right_unit,
                size: right_size,
                approx: right_approx,
            },
        ) => {
            if unit.to_lowercase() != right_unit.to_lowercase()
                || size.to_lowercase() != right_size.to_lowercase()
                || approx != right_approx
            {
                return None;
            }

            Some(Quantity::Parenthetical {
                count: add_nums(*left, *right),
                unit: unit.clone(),
                size: size.clone(),
                approx: *approx,
            })
        }
        (
            Quantity::Descriptive {
                count: left,
                description,
            },
            Quantity::Descriptive {
                count: right,
                description: right_description,
            },
        ) => {
            if description.to_lowercase() != right_description.to_lowercase() {
                return None;
            }

            Some(Quantity::Descriptive {
                count: left + right,
                description: description.clone(),
            })
        }
        _ => None,
    }
}

/// `scaler.merge_quantity_strings`: the merged raw string and whether the two
/// were compatible at all.
fn merge_quantity_strings(
    existing_raw: Option<&str>,
    new_raw: Option<&str>,
) -> (Option<String>, bool) {
    if is_shopping_omitted_quantity(existing_raw) {
        if is_shopping_omitted_quantity(new_raw) {
            return (None, true);
        }

        return (new_raw.map(str::to_string), true);
    }

    if is_shopping_omitted_quantity(new_raw) {
        return (existing_raw.map(str::to_string), true);
    }

    let existing_parsed = parse_quantity(existing_raw.unwrap_or_default());
    let new_parsed = parse_quantity(new_raw.unwrap_or_default());

    match (existing_parsed, new_parsed) {
        (Some(existing), Some(new)) => match add_quantities(&existing, &new) {
            Some(combined) => (Some(combined.to_text()), true),
            None => (None, false),
        },
        _ => {
            if existing_raw == new_raw {
                (existing_raw.map(str::to_string), true)
            } else {
                (None, false)
            }
        }
    }
}

/// `scaler.format_ingredient_for_copy`.
fn format_ingredient_for_copy(name: &str, raw_quantity: Option<&str>) -> String {
    if is_shopping_omitted_quantity(raw_quantity) {
        return name.to_string();
    }

    format!("{} {}", raw_quantity.unwrap_or_default(), name)
}

/// `scaler.ingredients_copy_text`: the clipboard text, with duplicate
/// ingredients merged across blocks.
pub fn ingredients_copy_text(ingredients_blocks: &Value) -> String {
    let mut merged: Vec<(String, Option<String>)> = Vec::new();

    for block in ingredients_blocks.as_array().into_iter().flatten() {
        for ingredient in block
            .get("ingredients")
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
        {
            let name = ingredient
                .get("name")
                .and_then(Value::as_str)
                .unwrap_or_default()
                .to_string();
            let raw_quantity = ingredient
                .get("quantity")
                .and_then(Value::as_str)
                .map(str::to_string);

            let mut merge_target = None;
            let mut combined_quantity = None;

            for (index, item) in merged.iter().enumerate() {
                if item.0.to_lowercase() != name.to_lowercase() {
                    continue;
                }

                let (combined, compatible) =
                    merge_quantity_strings(item.1.as_deref(), raw_quantity.as_deref());

                if compatible {
                    merge_target = Some(index);
                    combined_quantity = Some(combined);
                    break;
                }
            }

            match merge_target {
                Some(index) => {
                    merged[index].1 = combined_quantity.unwrap_or(None);
                }
                None => merged.push((name, raw_quantity)),
            }
        }
    }

    merged
        .iter()
        .map(|(name, quantity)| format_ingredient_for_copy(name, quantity.as_deref()))
        .collect::<Vec<String>>()
        .join("\n")
}
