# DotHyphen

A basic ASCII to [Morse](https://en.wikipedia.org/wiki/Morse_code) translator.

## Building

As many others Rust crates you must use cargo to build `DotHyphen` library.

```bash
dothyphen~ cargo build --target x86_64-unknown-linux-gnu --release
```

> Change the target if you want to build the crate to other platforms like Windows or macOS.
> Remove --release if you want a debug version.
> Remember, the build output will be placed in the root of the workspace. For example, `target/x86_64-unknown-linux-gnu/release/libdothyphen.rlib`

## Usage

Add `dothyphen` as a dependency, then:

```rust
use dothyphen::translate;

let translated_string = translate("Hello World");
println!("{}", translated_string); // Should print .... . .-.. .-.. --- / .-- --- .-. .-.. -..
```

## Publish

This crate follows the [official documentation](https://doc.rust-lang.org/cargo/reference/publishing.html) to be published into crates.io.
