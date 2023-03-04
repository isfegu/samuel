# DoHy

A CLI for DotHyphen.

## Building

As many others Rust crates you must use cargo to build `DotHyphen` bin.

```bash
dohy~ cargo build --target x86_64-unknown-linux-gnu --release
```

> Change the target to build the crate in other platforms like Windows or macOS.
> Remove --release to build a debug version.
> Remember, the build output will be placed in the root of the workspace. For example, `target/x86_64-unknown-linux-gnu/release/dohy`

## Usage

You can use cargo:

```bash
dohy~ cargo run -p dothyphen -- --translate "Hello world"
```

Or you can execute the binary built previously:

```bash
~ ./target/x86_64-unknown-linux-gnu/release/dohy --translate "Hello world"
```

### Install

DoHy is available from crates.io, therefore, it can be installed using cargo:

```bash
~ cargo install dohy
```

## Publish

This crate follows the [official documentation](https://doc.rust-lang.org/cargo/reference/publishing.html) to be published into crates.io.
