#include "libdotcyphen.h"
#include "munit/munit.h"

void test_buffer_size_successfully() {
  const char* string_to_translate = "Hello world";
  int size = get_buffer_size(string_to_translate);
  munit_assert_int(size, ==, 44);
}

void test_translation_successfully() {
  const char* string_to_translate = "Hello world";
  int size = get_buffer_size(string_to_translate);
  char translation[size] = "";
  int result = translate(string_to_translate, (uint8_t*) &translation, size);
  munit_assert_int(result, ==, 0);
}

void test_translation_with_wrong_size() {
  const char* string_to_translate = "Hello world";
  char translation[10] = "";
  int result = translate(string_to_translate, (uint8_t*) &translation, 10);
  munit_assert_int(result, ==, 1);
}

int main(void) {  
  test_buffer_size_successfully();
  test_translation_successfully();
  test_translation_with_wrong_size();

  return 0;
}