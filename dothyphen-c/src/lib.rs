use std::ffi::{c_char, CStr};

/// Get the buffer size needed to hold a Morse translation.
///
/// @param input A pointer to the string to be translated.
/// @return The size of the buffer.
#[no_mangle]
pub extern "C" fn get_buffer_size(input: *const c_char) -> usize {
    let intput_to_translate = get_input_as_string(input);
    let input_translated = dothyphen::translate(&intput_to_translate);

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
pub extern "C" fn translate(
    input: *const c_char,
    output_buffer: *mut u8,
    output_buffer_size: usize,
) -> isize {
    let intput_to_translate = get_input_as_string(input);
    let input_translated = dothyphen::translate(&intput_to_translate);

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

fn get_input_as_string(input: *const c_char) -> String {
    let intput_to_translate =
        String::from_utf8_lossy(unsafe { CStr::from_ptr(input) }.to_bytes()).to_string();

    intput_to_translate
}

#[cfg(test)]
mod dothyphen_c_tests {
    use crate::*;

    #[test]
    fn size_is_returned() {
        let str = "Hello Samuel".as_bytes();
        let result = get_buffer_size(str.as_ptr() as *const i8);
        assert_eq!(result, 44);
    }

    #[test]
    fn string_is_returned() {
        let str = "Hello Samuel".as_bytes();
        let result = get_input_as_string(str.as_ptr() as *const i8);
        assert_eq!(String::from("Hello Samuel"), result);
    }
}
