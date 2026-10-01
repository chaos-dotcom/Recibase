//! `se.reciba.api.utils`.

pub mod string_utils {
    /// `StringUtils.PluralUtils.unpluralise`.
    ///
    /// Strips a trailing `s`, otherwise strips a trailing `es` (which is
    /// almost always a no-op, but is what the Scala does).
    pub fn unpluralise(s: &str) -> String {
        if let Some(stripped) = s.strip_suffix('s') {
            stripped.to_string()
        } else if let Some(stripped) = s.strip_suffix("es") {
            stripped.to_string()
        } else {
            s.to_string()
        }
    }
}

pub mod int_utils {
    /// `IntUtils.TemperatureUtils`.
    pub fn simple_fan_instruction(t: i32) -> String {
        format!("{}°C (fan {}°C)", t, t - 20)
    }

    pub fn celsius(t: i32) -> String {
        let as_fahrenheit = (t as f32) * 9.0 / 5.0 + 32.0;
        let rounded_fahrenheit = nearest_multiple_of(as_fahrenheit, 5);
        let gas_mark = celsius_to_gas_mark(t);
        formatted_temperature_string(t, rounded_fahrenheit, gas_mark)
    }

    pub fn fahrenheit(t: i32) -> String {
        let as_celsius = (t - 32) as f32 * 5.0 / 9.0;
        let rounded_celsius = nearest_multiple_of(as_celsius, 5);
        let gas_mark = celsius_to_gas_mark(rounded_celsius);
        formatted_temperature_string(rounded_celsius, t, gas_mark)
    }

    fn celsius_to_gas_mark(c: i32) -> Option<&'static str> {
        match c {
            140 => Some("1"),
            150 => Some("2"),
            160 => Some("3"),
            170 => Some("3.5"),
            180 => Some("4"),
            190 => Some("5"),
            200 => Some("6"),
            210 => Some("7"),
            220 => Some("8"),
            230 => Some("8.5"),
            240 => Some("9"),
            _ => None,
        }
    }

    fn nearest_multiple_of(input: f32, of: i32) -> i32 {
        of * (input / of as f32).round() as i32
    }

    fn formatted_temperature_string(c: i32, f: i32, gas_mark: Option<&str>) -> String {
        let formatted_gas_mark = gas_mark.map(|g| format!(", gas mark {}", g)).unwrap_or_default();
        format!("{}°C ({}°F{})", c, f, formatted_gas_mark)
    }
}
