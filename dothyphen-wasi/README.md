# DotHyphen WASI

_DotHyphen WASI_ is a WebAssembly wrapper of _DotHyphen_ ready to be used from a WebAssembly runtime.

_DotHyphen_ is a basic ASCII to [Morse](https://en.wikipedia.org/wiki/Morse_code) and vice versa translator developed in Rust.

## Usage

Using wasmtime runtime:

```bash
samuel~ wasmtime target/wasm32-wasi/release/dothyphen-wasi.wasm -- --translate "Hello" --output morse # Should print: .... . .-.. .-.. --- / .-- --- .-. .-.. -..
```

```bash
samuel~ wasmtime target/wasm32-wasi/release/dothyphen-wasi.wasm -- --translate ".... . .-.. .-.. --- / .-- --- .-. .-.. -.." --output ascii # Should print: hello world
```

Or using wasmer runtime:

```bash
samuel~ wasmer run target/wasm32-wasi/release/dothyphen-wasi.wasm -- --translate "Hello" --output morse # Should print: .... . .-.. .-.. --- / .-- --- .-. .-.. -..
```

```bash
samuel~ wasmer run target/wasm32-wasi/release/dothyphen-wasi.wasm -- --translate ".... . .-.. .-.. --- / .-- --- .-. .-.. -.." --output ascii # Should print: hello world
```

## Contributing

_DoHy_ is a part of [Samuel](https://github.com/isfegu/samuel), a project focused on learning how to use Rust to build a crate that can be used from other languages and multiple environments.

Please read the [contributing guidelines](https://github.com/isfegu/samuel#contributing) and the [contributing documentation](./CONTRIBUTING.md) to contribute to _DoHy_.

Moreover, you can use the _Samuel_ [Github Discussions](https://github.com/isfegu/samuel/discussions).

All help will be appreciated.
