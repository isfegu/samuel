# DoHy

A CLI for [DotHyphen](https://crates.io/crates/dothyphen). _DotHyphen_ is a basic ASCII to [Morse](https://en.wikipedia.org/wiki/Morse_code) translator.

## Usage

From source, you can use cargo:

```bash
dohy~ cargo run -- --translate "Hello world"
```

Using the binary:

```bash
~ dohy --translate "Hello world"  # Should print .... . .-.. .-.. --- / .-- --- .-. .-.. -..
```

## Contributing

_DoHy_ is a part of [Samuel](https://github.com/isfegu/samuel), a project focused on learning how to use Rust to build a crate that can be used from other languages.

Please read the [contributing guidelines](https://github.com/isfegu/samuel#contributing) to contribute to _DoHy_. All help will be appreciated.
