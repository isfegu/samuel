# DotCyphen

_DotHyphen_ is a basic ASCII to [Morse](https://en.wikipedia.org/wiki/Morse_code) translator developed in Rust. _DotCyphen_ is a C/C++ wrapper of _DotHyphen_.

## Usage

```c++
#include "libdotcyphen.h"

int main () {
  const char* string_to_translate = "Hello world";
  int size = get_buffer_size(string_to_translate);
  char translation[size] = "";
  translate(string_to_translate, (uint8_t*) &translation, size);
  printf("%s\n", translation); // Should print .... . .-.. .-.. --- / .-- --- .-. .-.. -..

  return 0;
}
```

Look at the [demo](../demo/c++/) to see how to use _DotCyphen_ from C++ using `gcc`.

## Contributing

_DotCyphen_ is a part of [Samuel](https://github.com/isfegu/samuel), a project focused on learning how to use Rust to build a crate that can be used from other languages.

Please read the [contributing guidelines](https://github.com/isfegu/samuel#contributing) to contribute to _DotCyphen_. All help will be appreciated.