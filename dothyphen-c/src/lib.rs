use std::ffi::{c_char, CStr};

/// Get the buffer size needed to hold the ASCII to Morse translation.
///
/// @param input A pointer to the string to be translated.
/// @return The size of the buffer.
#[no_mangle]
pub extern "C" fn to_morse_buffer_size(input: *const c_char) -> usize {
    let intput_to_translate = get_input_as_string(input);
    let input_translated = dothyphen::translate::to_morse(&intput_to_translate);

    input_translated.len() + 1
}

/// Get the buffer size needed to hold the Morse to ASCII translation.
///
/// @param input A pointer to the string to be translated.
/// @return The size of the buffer.
#[no_mangle]
pub extern "C" fn to_ascii_buffer_size(input: *const c_char) -> usize {
    let intput_to_translate = get_input_as_string(input);
    let input_translated = dothyphen::translate::to_ascii(&intput_to_translate);

    input_translated.len() + 1
}

/// Translate an ASCII string to Morse string.

/// Only ASCII content will be translated, non ASCII content will be skipped.
///
/// @param input A pointer to the string to be translated.
/// @param output_buffer A pointer to the buffer that will contain the string translated.
/// @param output_buffer_size The size of the buffer that will contain the string translated.
/// @return
///     0 if the translation was successful.
///     > 0 if the translation was wrong.
#[no_mangle]
pub extern "C" fn to_morse(
    input: *const c_char,
    output_buffer: *mut u8,
    output_buffer_size: usize,
) -> isize {
    let intput_to_translate = get_input_as_string(input);
    let input_translated = dothyphen::translate::to_morse(&intput_to_translate);

    // To avoid a buffer overflow, we check that the output buffer has enough space to hold the translated input.
    if input_translated.len() + 1 > output_buffer_size {
        return 1;
    }

    // We walk translated input byte per byte and copy each byte to the output buffer.
    for (index, value) in input_translated.as_bytes().iter().enumerate() {
        let output_buffer_writter = (output_buffer as usize + index) as *mut u8;
        unsafe { *output_buffer_writter = *value };
    }
    // Finally we add a valid nul terminator at the end of the output buffer.
    let output_buffer_writter = (output_buffer as usize + input_translated.len()) as *mut u8;
    unsafe { *output_buffer_writter = b'\0' };

    0
}

/// Translate an Morse string to ASCII string.

/// @param input A pointer to the string to be translated.
/// @param output_buffer A pointer to the buffer that will contain the string translated.
/// @param output_buffer_size The size of the buffer that will contain the string translated.
/// @return
///     0 if the translation was successful.
///     > 0 if the translation was wrong.
#[no_mangle]
pub extern "C" fn to_ascii(
    input: *const c_char,
    output_buffer: *mut u8,
    output_buffer_size: usize,
) -> isize {
    let intput_to_translate = get_input_as_string(input);
    let input_translated = dothyphen::translate::to_ascii(&intput_to_translate);

    // To avoid a buffer overflow, we check that the output buffer has enough space to hold the translated input.
    if input_translated.len() + 1 > output_buffer_size {
        return 1;
    }

    // We walk translated input byte per byte and copy each byte to the output buffer.
    for (index, value) in input_translated.as_bytes().iter().enumerate() {
        let output_buffer_writter = (output_buffer as usize + index) as *mut u8;
        unsafe { *output_buffer_writter = *value };
    }
    // Finally we add a valid nul terminator at the end of the output buffer.
    let output_buffer_writter = (output_buffer as usize + input_translated.len()) as *mut u8;
    unsafe { *output_buffer_writter = b'\0' };

    0
}

/// Transform a char pointer to a Rust String
///
/// @param input A char pointer to a string
/// @return a Rust String containing the string pointed by input
fn get_input_as_string(input: *const c_char) -> String {
    let intput_to_translate =
        String::from_utf8_lossy(unsafe { CStr::from_ptr(input) }.to_bytes()).to_string();

    intput_to_translate
}

#[cfg(test)]
mod dothyphen_c_tests {
    use crate::*;

    #[test]
    fn to_morse_size_is_returned() {
        let str = "Hello Samuel".as_bytes();
        let result = to_morse_buffer_size(str.as_ptr() as *const i8);
        assert_eq!(result, 44);
    }

    #[test]
    fn to_ascii_size_is_returned() {
        let str = ".... . .-.. .-.. --- / .-- --- .-. .-.. -..".as_bytes();
        let result = to_ascii_buffer_size(str.as_ptr() as *const i8);
        assert!(result >= 11 && result <= 12);
    }

    #[test]
    fn from_pointer_to_string() {
        let str = "Hello Samuel".as_bytes();
        let result = get_input_as_string(str.as_ptr() as *const i8);
        assert_eq!(String::from("Hello Samuel"), result);
    }
}
