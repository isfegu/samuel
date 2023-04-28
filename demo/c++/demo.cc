#include "libdotcyphen.h"

int main () {
  const char* string_to_translate = "Hello world";
  int size = get_buffer_size(string_to_translate);
  char translation[size] = "";
  int result = translate(string_to_translate, (uint8_t*) &translation, size);
  switch (result) {
    case 0:
      printf("%s\n", translation);
      break;
    case 1:
      printf("The buffer size is small to hold the translation.\n");
      break;
  }

  return 0;
}