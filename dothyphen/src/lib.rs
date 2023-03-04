//! Simple ASCII to Morse translator.
//!
//! Provides a way to translate an ASCII input to their Morse code representation.
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    static ref MORSE_TABLE: HashMap<&'static char, &'static str> = {
        let mut morse_table = HashMap::new();
        morse_table.insert(&'a', ".-");
        morse_table.insert(&'b', "-...");
        morse_table.insert(&'c', "-.-.");
        morse_table.insert(&'d', "-..");
        morse_table.insert(&'e', ".");
        morse_table.insert(&'f', "..-.");
        morse_table.insert(&'g', "--.");
        morse_table.insert(&'h', "....");
        morse_table.insert(&'i', "..");
        morse_table.insert(&'j', ".---");
        morse_table.insert(&'k', "-.-");
        morse_table.insert(&'l', ".-..");
        morse_table.insert(&'m', "--");
        morse_table.insert(&'n', "-.");
        morse_table.insert(&'o', "---");
        morse_table.insert(&'p', ".--.");
        morse_table.insert(&'q', "--.-");
        morse_table.insert(&'r', ".-.");
        morse_table.insert(&'s', "...");
        morse_table.insert(&'t', "-");
        morse_table.insert(&'u', "..-");
        morse_table.insert(&'v', "...-");
        morse_table.insert(&'w', ".--");
        morse_table.insert(&'x', "-..-");
        morse_table.insert(&'y', "-.--");
        morse_table.insert(&'z', "--..");
        morse_table.insert(&'0', "-----");
        morse_table.insert(&'1', ".----");
        morse_table.insert(&'2', "..---");
        morse_table.insert(&'3', "...--");
        morse_table.insert(&'4', "....-");
        morse_table.insert(&'5', ".....");
        morse_table.insert(&'6', "-....");
        morse_table.insert(&'7', "--...");
        morse_table.insert(&'8', "---..");
        morse_table.insert(&'9', "----.");
        morse_table.insert(&' ', "/");
        morse_table
    };
}

/// Translate an ASCII text into a Morse code text.
///
/// Only ASCII characters will be translated. Non ASCII characters will be skipped.
/// Each letter will be separated by _spaces_ and words will be separated by _/_ character.
pub fn translate(input: &str) -> String {
    let mut translated_string: String = "".to_string();

    for character in input.chars() {
        match get_valid_character(&character) {
            Some(c) => match MORSE_TABLE.get(&c) {
                Some(morse_str) => {
                    translated_string.push(' ');
                    translated_string.push_str(&morse_str);
                }
                _ => {}
            },
            _ => {}
        };
    }
    translated_string.trim().to_string()
}

fn get_valid_character(character: &char) -> Option<char> {
    let lowercase_character = character.to_ascii_lowercase();
    if !lowercase_character.is_ascii_alphanumeric() && !lowercase_character.is_ascii_whitespace() {
        return None;
    }

    Some(lowercase_character)
}

#[cfg(test)]
mod dothyphen_tests {
    use crate::translate;

    #[test]
    fn nonascii_is_not_tranlated() {
        let result = translate("áéíóú");
        assert_eq!(result, "");
    }

    #[test]
    fn space_is_tranlated() {
        let result = translate(" ");
        assert_eq!(result, "/");
    }

    #[test]
    fn alphabetic_is_tranlated() {
        let result = translate("Hello Samuel");
        assert_eq!(result, ".... . .-.. .-.. --- / ... .- -- ..- . .-..");
    }

    #[test]
    fn numeric_is_tranlated() {
        let result = translate("123456789");
        assert_eq!(
            result,
            ".---- ..--- ...-- ....- ..... -.... --... ---.. ----."
        );
    }

    #[test]
    fn alphanumeric_is_tranlated() {
        let result = translate("Hello 123456789 Samuel");
        assert_eq!(
            result,
            ".... . .-.. .-.. --- / .---- ..--- ...-- ....- ..... -.... --... ---.. ----. / ... .- -- ..- . .-.."
        );
    }
}
