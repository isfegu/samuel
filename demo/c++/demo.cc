#include "libdotcyphen.h"

int main () {
  const char* string_to_translate_to_morse = "Hello world";
  int size = to_morse_buffer_size(string_to_translate_to_morse);
  char translation_to_morse[size] = "";
  int to_morse_result = to_morse(string_to_translate_to_morse, (uint8_t*) &translation_to_morse, size);
  switch (to_morse_result) {
    case 0:
      printf("ASCII to Morse: %s\n", translation_to_morse);
      break;
    case 1:
      printf("The buffer size is small to hold the translation.\n");
      break;
  }

  const char* string_to_translate_to_ascii = ".... . .-.. .-.. --- / .-- --- .-. .-.. -..";
  size = to_ascii_buffer_size(string_to_translate_to_ascii);
  char translation_to_ascii[size] = "";
  int to_ascii_result = to_ascii(string_to_translate_to_ascii, (uint8_t*) &translation_to_ascii, size);
  switch (to_ascii_result) {
    case 0:
      printf("Morse to ASCII: %s\n", translation_to_ascii);
      break;
    case 1:
      printf("The buffer size is small to hold the translation.\n");
      break;
  }

  return 0;
}