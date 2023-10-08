//! Simple ASCII to Morse and vice versa translator.
//!
//! Provides a way to translate an ASCII input to their Morse code representation and a way to translate a Morse input to their ASCII representation.
pub mod translate {
    //! Provides methods to translate ASCII to Morse code and to translate Morse to ASCII.
    use lazy_static::lazy_static;
    use std::collections::HashMap;

    lazy_static! {
        static ref ASCII_MORSE_TABLE: HashMap<&'static char, &'static str> = {
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
    pub fn to_morse(input: &str) -> String {
        let mut translated_string: String = "".to_string();

        for character in input.chars() {
            if let Some(c) = get_valid_character(&character) {
                if let Some(morse_str) = ASCII_MORSE_TABLE.get(&c) {
                    translated_string.push(' ');
                    translated_string.push_str(morse_str);
                }
            };
        }
        translated_string.trim().to_string()
    }

    fn get_valid_character(character: &char) -> Option<char> {
        let lowercase_character = character.to_ascii_lowercase();

        if !lowercase_character.is_ascii_alphanumeric()
            && !lowercase_character.is_ascii_whitespace()
        {
            return None;
        }

        Some(lowercase_character)
    }

    /// Translate a Morse code text into a ASCII text.
    ///
    /// Only ASCII characters will be translated. Non ASCII characters will be skipped.
    pub fn to_ascii(input: &str) -> String {
        let trimmed_input = input.trim();
        if trimmed_input.len() == 0 {
            return "".to_string();
        }

        let mut translated_string: String = "".to_string();
        let words: Vec<&str> = input.split("/").collect();
        let morse_ascii_table = flip_morse_table();

        for word in words {
            let letters: Vec<&str> = word.split_ascii_whitespace().collect();

            for letter in letters {
                if let Some(ascii_char) = morse_ascii_table.get(&letter) {
                    translated_string.push(**ascii_char);
                }
            }
            translated_string.push(' ');
        }

        return translated_string.trim_end().to_string();
    }

    fn flip_morse_table() -> HashMap<&'static str, &'static char> {
        let mut morse_ascii_table: HashMap<&str, &char> = HashMap::new();

        for (key, value) in ASCII_MORSE_TABLE.iter() {
            morse_ascii_table.insert(value, key);
        }

        return morse_ascii_table;
    }
}

#[cfg(test)]
mod translate_tests {
    use crate::*;

    #[test]
    fn empty_is_translated_to_morse() {
        let result = translate::to_morse("");
        assert_eq!(result, "");
    }

    #[test]
    fn space_is_translated_to_morse() {
        let result = translate::to_morse(" ");
        assert_eq!(result, "/");
    }

    #[test]
    fn nonascii_is_not_translated_to_morse() {
        let result = translate::to_morse("áéíóú");
        assert_eq!(result, "");
    }

    #[test]
    fn alphabetic_is_translated_to_morse() {
        let result = translate::to_morse("Hello Samuel");
        assert_eq!(result, ".... . .-.. .-.. --- / ... .- -- ..- . .-..");
    }

    #[test]
    fn numeric_is_translated_to_morse() {
        let result = translate::to_morse("123456789");
        assert_eq!(
            result,
            ".---- ..--- ...-- ....- ..... -.... --... ---.. ----."
        );
    }

    #[test]
    fn alphanumeric_is_translated_to_morse() {
        let result = translate::to_morse("hello 123456789 samuel");
        assert_eq!(
            result,
            ".... . .-.. .-.. --- / .---- ..--- ...-- ....- ..... -.... --... ---.. ----. / ... .- -- ..- . .-.."
        );
    }

    #[test]
    fn non_morse_is_not_translated_to_ascii() {
        let result = translate::to_ascii("Hello Samuel");
        assert_eq!(result, "");
    }

    #[test]
    fn empty_is_translated_to_ascii() {
        let result = translate::to_ascii("");
        assert_eq!(result, "");
    }

    #[test]
    fn spaces_is_translated_to_ascii() {
        let result = translate::to_ascii("    ");
        assert_eq!(result, "");
    }

    #[test]
    fn slashes_is_translated_to_ascii() {
        let result = translate::to_ascii("/ / / /");
        assert_eq!(result, "");
    }

    #[test]
    fn alphabetic_is_translated_to_ascii() {
        let result = translate::to_ascii(".... . .-.. .-.. --- / ... .- -- ..- . .-..");
        assert_eq!(result, "hello samuel");
    }

    #[test]
    fn numeric_is_translated_to_ascii() {
        let result = translate::to_ascii(".---- ..--- ...-- ....- ..... -.... --... ---.. ----.");
        assert_eq!(result, "123456789");
    }

    #[test]
    fn alphanumeric_is_translated_to_ascii() {
        let result = translate::to_ascii(".... . .-.. .-.. --- / .---- ..--- ...-- ....- ..... -.... --... ---.. ----. / ... .- -- ..- . .-..");
        assert_eq!(result, "hello 123456789 samuel");
    }
}
