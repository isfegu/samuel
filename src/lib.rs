pub mod morse {
    use std::collections::HashMap;

    pub fn translate(input: &str) -> String {
        let tranlation_table = get_latin_to_morse_table();
        let mut translated_string: String = "".to_string();

        for character in input.chars() {
            match get_valid_character(&character) {
                Some(c) => match tranlation_table.get(&c) {
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

    fn get_latin_to_morse_table() -> HashMap<&'static char, &'static str> {
        let mut latin_to_morse_table: HashMap<&char, &str> = HashMap::new();

        latin_to_morse_table.insert(&'a', ".-");
        latin_to_morse_table.insert(&'b', "-...");
        latin_to_morse_table.insert(&'c', "-.-.");
        latin_to_morse_table.insert(&'d', "-..");
        latin_to_morse_table.insert(&'e', ".");
        latin_to_morse_table.insert(&'f', "..-.");
        latin_to_morse_table.insert(&'g', "--.");
        latin_to_morse_table.insert(&'h', "....");
        latin_to_morse_table.insert(&'i', "..");
        latin_to_morse_table.insert(&'j', ".---");
        latin_to_morse_table.insert(&'k', "-.-");
        latin_to_morse_table.insert(&'l', ".-..");
        latin_to_morse_table.insert(&'m', "--");
        latin_to_morse_table.insert(&'n', "-.");
        latin_to_morse_table.insert(&'o', "---");
        latin_to_morse_table.insert(&'p', ".--.");
        latin_to_morse_table.insert(&'q', "--.-");
        latin_to_morse_table.insert(&'r', ".-.");
        latin_to_morse_table.insert(&'s', "...");
        latin_to_morse_table.insert(&'t', "-");
        latin_to_morse_table.insert(&'u', "..-");
        latin_to_morse_table.insert(&'v', "...-");
        latin_to_morse_table.insert(&'w', ".--");
        latin_to_morse_table.insert(&'x', "-..-");
        latin_to_morse_table.insert(&'y', "-.--");
        latin_to_morse_table.insert(&'z', "--..");
        latin_to_morse_table.insert(&'0', "-----");
        latin_to_morse_table.insert(&'1', ".----");
        latin_to_morse_table.insert(&'2', "..---");
        latin_to_morse_table.insert(&'3', "...--");
        latin_to_morse_table.insert(&'4', "....-");
        latin_to_morse_table.insert(&'5', ".....");
        latin_to_morse_table.insert(&'6', "-....");
        latin_to_morse_table.insert(&'7', "--...");
        latin_to_morse_table.insert(&'8', "---..");
        latin_to_morse_table.insert(&'9', "----.");
        latin_to_morse_table.insert(&' ', "/");

        latin_to_morse_table
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
}

#[cfg(test)]
mod tests {
    use crate::morse;

    #[test]
    fn nonascii_is_not_tranlated() {
        let result = morse::translate("áéíóú");
        assert_eq!(result, "");
    }

    #[test]
    fn space_is_tranlated() {
        let result = morse::translate(" ");
        assert_eq!(result, "/");
    }

    #[test]
    fn alphabetic_is_tranlated() {
        let result = morse::translate("Hello Samuel");
        assert_eq!(result, ".... . .-.. .-.. --- / ... .- -- ..- . .-..");
    }

    #[test]
    fn numeric_is_tranlated() {
        let result = morse::translate("123456789");
        assert_eq!(
            result,
            ".---- ..--- ...-- ....- ..... -.... --... ---.. ----."
        );
    }

    #[test]
    fn alphanumeric_is_tranlated() {
        let result = morse::translate("Hello 123456789 Samuel");
        assert_eq!(
            result,
            ".... . .-.. .-.. --- / .---- ..--- ...-- ....- ..... -.... --... ---.. ----. / ... .- -- ..- . .-.."
        );
    }
}
