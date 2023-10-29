#include "libdotcyphen.h"
#include "munit/munit.h"

void test_to_morse_buffer_size_successfully() {
  const char* string_to_translate = "Hello world";
  int size = to_morse_buffer_size(string_to_translate);
  munit_assert_int(size, ==, 44);
}

void test_to_morse_translation_successfully() {
  const char* string_to_translate = "Hello world";
  int size = to_morse_buffer_size(string_to_translate);
  char translation[size] = "";
  int result = to_morse(string_to_translate, (uint8_t*) &translation, size);
  munit_assert_int(result, ==, 0);
}

void test_to_morse_with_wrong_size() {
  const char* string_to_translate = "Hello world";
  char translation[10] = "";
  int result = to_morse(string_to_translate, (uint8_t*) &translation, 10);
  munit_assert_int(result, ==, 1);
}

void test_to_ascii_buffer_size_successfully() {
  const char* string_to_translate = ".... . .-.. .-.. --- / .-- --- .-. .-.. -..";
  int size = to_ascii_buffer_size(string_to_translate);
  munit_assert_int(size, ==, 12);
}

void test_to_ascii_translation_successfully() {
  const char* string_to_translate = ".... . .-.. .-.. --- / .-- --- .-. .-.. -..";
  int size = to_morse_buffer_size(string_to_translate);
  char translation[size] = "";
  int result = to_ascii(string_to_translate, (uint8_t*) &translation, size);
  munit_assert_int(result, ==, 0);
}

void test_to_ascii_with_wrong_size() {
  const char* string_to_translate = ".... . .-.. .-.. --- / .-- --- .-. .-.. -..";
  char translation[16] = "";
  int result = to_morse(string_to_translate, (uint8_t*) &translation, 10);
  munit_assert_int(result, ==, 1);
}

int main(void) {  
  test_to_morse_buffer_size_successfully();
  test_to_morse_translation_successfully();
  test_to_morse_with_wrong_size();
  test_to_ascii_buffer_size_successfully();
  test_to_ascii_translation_successfully();
  test_to_ascii_with_wrong_size();

  return 0;
}