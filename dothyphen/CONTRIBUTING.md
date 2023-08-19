# Contributing

_DotHyphen_ is a part of [Samuel](https://github.com/isfegu/samuel), to contribute:

1. Read the [Samuel documentation](https://github.com/isfegu/samuel/blob/main/README.md).
2. Download the [Samuel repository](https://github.com/isfegu/samuel).
3. Install the [Samuel requirements](https://github.com/isfegu/samuel/blob/main/README.md#requirements).
4. Follow the [Samuel contributing guidelines](https://github.com/isfegu/samuel/blob/main/README.md#guidelines).

## Building

As many others Rust crates you must use cargo to build `DotHyphen` library.

```bash
dothyphen~ cargo build --target x86_64-unknown-linux-gnu --release
```

> Change the target if you want to build the crate to other platforms like Windows or macOS.
> Remove --release if you want a debug version.
> Remember, the build output will be placed in the root of the workspace. For example, `target/x86_64-unknown-linux-gnu/release/libdothyphen.rlib`

Read the [official documentation](https://doc.rust-lang.org/cargo/commands/cargo-build.html) to learn more about how to build a crate with cargo.

## Publishing

Read the `dothyphen` [RELEASE](./RELEASE.md) file.
