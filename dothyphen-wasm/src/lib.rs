use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn to_morse(input: &str) -> String {
    dothyphen::translate::to_morse(input)
}

#[wasm_bindgen]
pub fn to_ascii(input: &str) -> String {
    dothyphen::translate::to_ascii(input)
}

#[cfg(test)]
mod dothyphen_wasm_tests {
    use crate::*;

    #[test]
    fn nonascii_is_not_tranlated_to_morse() {
        let result = to_morse("áéíóú");
        assert_eq!(result, "");
    }

    #[test]
    fn space_is_tranlated_to_morse() {
        let result = to_morse(" ");
        assert_eq!(result, "/");
    }

    #[test]
    fn alphabetic_is_tranlated_to_morse() {
        let result = to_morse("Hello Samuel");
        assert_eq!(result, ".... . .-.. .-.. --- / ... .- -- ..- . .-..");
    }

    #[test]
    fn numeric_is_tranlated_to_morse() {
        let result = to_morse("123456789");
        assert_eq!(
            result,
            ".---- ..--- ...-- ....- ..... -.... --... ---.. ----."
        );
    }

    #[test]
    fn alphanumeric_is_tranlated_to_morse() {
        let result = to_morse("Hello 123456789 Samuel");
        assert_eq!(
            result,
            ".... . .-.. .-.. --- / .---- ..--- ...-- ....- ..... -.... --... ---.. ----. / ... .- -- ..- . .-.."
        );
    }

    #[test]
    fn nonmorse_is_not_tranlated_to_ascii() {
        let result = to_ascii("Hello Samuel");
        assert_eq!(result, "");
    }

    #[test]
    fn alphabetic_is_tranlated_to_ascii() {
        let result = to_ascii(".... . .-.. .-.. --- / ... .- -- ..- . .-..");
        assert_eq!(result, "hello samuel");
    }

    #[test]
    fn numeric_is_tranlated_to_ascii() {
        let result = to_ascii(".---- ..--- ...-- ....- ..... -.... --... ---.. ----.");
        assert_eq!(result, "123456789");
    }

    #[test]
    fn alphanumeric_is_tranlated_to_ascii() {
        let result = to_ascii(".... . .-.. .-.. --- / .---- ..--- ...-- ....- ..... -.... --... ---.. ----. / ... .- -- ..- . .-..");
        assert_eq!(result, "hello 123456789 samuel");
    }
}
