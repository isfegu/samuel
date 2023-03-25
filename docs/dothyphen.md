# DotHyphen

A simple ASCII to Morse translator and the core of Samuel project.

## Building

As many others Rust crates you must use cargo to build `DotHyphen` library.

```bash
dothyphen~ cargo build --target x86_64-unknown-linux-gnu --release
```

> Change the target if you want to build the crate to other platforms like Windows or macOS.
> Remove --release if you want a debug version.
> Remember, the build output will be placed in the root of the workspace. For example, `target/x86_64-unknown-linux-gnu/release/libdothyphen.rlib`

Read the [official documentation](https://doc.rust-lang.org/cargo/commands/cargo-build.html) to learn more about how to build a crate with cargo.

## Testing

To execute the crate unit tests, run:

```bash
dothyphen~ cargo test
```
