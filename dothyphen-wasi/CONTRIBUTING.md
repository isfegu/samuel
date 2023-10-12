# Contributing

_DotHyphen WASI_ is a part of [Samuel](https://github.com/isfegu/samuel), to contribute:

1. Read the [Samuel documentation](https://github.com/isfegu/samuel/blob/main/README.md).
2. Download the [Samuel repository](https://github.com/isfegu/samuel).
3. Install the [Samuel requirements](https://github.com/isfegu/samuel/blob/main/README.md#requirements).
4. Follow the [Samuel contributing guidelines](https://github.com/isfegu/samuel/blob/main/README.md#guidelines).

## Building

As many others Rust crates you must use cargo to build _DotHyphen WASI_ bin.

```bash
samuel/dothyphen-wasi~ cargo build --target wasm32-wasi --release
```

or

```bash
samuel~ cargo build --target wasm32-wasi --release -p dothyphen-wasi
```

> Remove --release if you want a debug version.
> Remember, the build output will be placed in the root of the workspace. For example, `target/wasm32-wasi/release/dothyphen-wasi.wasm`

Read the [official documentation](https://doc.rust-lang.org/cargo/commands/cargo-build.html) to learn more about how to build a crate with cargo.

## Publishing

`dothyphen-wasi` it is not published anywhere, yet.
