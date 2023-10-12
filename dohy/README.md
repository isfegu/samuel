# DoHy

A CLI for _DotHyphen_.

_DotHyphen_ is a basic ASCII to [Morse](https://en.wikipedia.org/wiki/Morse_code) and vice versa translator developed in Rust.

## Usage

From source, using cargo:

```bash
dohy~ cargo run -- --translate "Hello world" --output morse # Should print: .... . .-.. .-.. --- / .-- --- .-. .-.. -..
```

```bash
dohy~ cargo run -- --translate ".... . .-.. .-.. --- / .-- --- .-. .-.. -.." --output ascii # Should print: hello world
```

Using the binary:

```bash
~ dohy --translate "Hello world" --output morse # Should print: .... . .-.. .-.. --- / .-- --- .-. .-.. -..
```

```bash
~ dohy --translate --translate ".... . .-.. .-.. --- / .-- --- .-. .-.. -.." --output ascii # Should print: hello world
```

## Contributing

_DoHy_ is a part of [Samuel](https://github.com/isfegu/samuel), a project focused on learning how to use Rust to build a crate that can be used from other languages and multiple environments.

Please read the [contributing guidelines](https://github.com/isfegu/samuel#contributing) and the [contributing documentation](./CONTRIBUTING.md) to contribute to _DoHy_.

Moreover, you can use the _Samuel_ [Github Discussions](https://github.com/isfegu/samuel/discussions).

All help will be appreciated.
