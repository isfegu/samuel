# DotHyphen

A basic ASCII to [Morse](https://en.wikipedia.org/wiki/Morse_code) and vice versa translator.

## Usage

```rust
use dothyphen::*;

let morse_string = translate::to_morse("Hello World");
println!("{}", morse_string); // Should print: .... . .-.. .-.. --- / .-- --- .-. .-.. -..

let ascii_string = translate::to_morse(".... . .-.. .-.. --- / .-- --- .-. .-.. -..");
println!("{}", ascii_string); // Should print: Hello World
```

## Contributing

_DotHyphen_ is a part of [Samuel](https://github.com/isfegu/samuel), a project focused on learning how to use Rust to build a crate that can be used from other languages and multiple environments.

Please read the [contributing guidelines](https://github.com/isfegu/samuel#contributing) and the [contributing documentation](./CONTRIBUTING.md) to contribute to _DotHyphen_.

Moreover, you can use the _Samuel_ [Github Discussions](https://github.com/isfegu/samuel/discussions).

All help will be appreciated.
