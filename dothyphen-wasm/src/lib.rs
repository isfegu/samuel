use dothyphen;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn translate(input: &str) -> String {
    dothyphen::translate(input)
}

#[cfg(test)]
mod dothyphen_wasm_tests {
    use crate::*;

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
